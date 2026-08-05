use std::fmt::Display;

use algosul_core::{
  util::{Take, Taker, TryTake, TryTaker},
  wrapper::prelude::*,
};
use proc_macro2::{Span, TokenStream};
use proc_macro_warning::FormattedWarning;
use quote::{ToTokens, quote};
use syn::{Error, spanned::Spanned};

#[derive(Debug, Default, Clone)]
pub struct Logger
{
  formatted_warnings: Vec<FormattedWarning>,
  errors:             Vec<Error>,
}

pub struct LoggerTakerBuilder
{
  call_in_none: Option<Box<dyn FnMut()>>,
}

pub struct LoggerTryTakerBuilder
{
  call_in_err: Option<Box<dyn FnMut(&dyn std::error::Error)>>,
}

pub struct LoggerTaker<'a>
{
  logger:       &'a mut Logger,
  call_in_none: Box<dyn FnMut()>,
}

pub struct LoggerTryTaker<'a>
{
  logger:      &'a mut Logger,
  call_in_err: Box<dyn FnMut(&dyn std::error::Error)>,
}

#[derive(Debug, Default, Clone)]
pub struct ResultWithLogger<T: ?Sized>
{
  logger: Logger,
  inner:  T,
}

impl Logger
{
  pub fn taker() -> LoggerTakerBuilder
  {
    LoggerTakerBuilder { call_in_none: None }
  }

  pub fn try_taker() -> LoggerTryTakerBuilder
  {
    LoggerTryTakerBuilder { call_in_err: None }
  }
}

impl LoggerTakerBuilder
{
  pub fn call_in_none(
    &mut self, call_in_none: impl FnMut() + 'static,
  ) -> &mut Self
  {
    self.call_in_none = Some(Box::new(call_in_none));
    self
  }

  pub fn build<'a>(&mut self, logger: &'a mut Logger)
  -> Option<LoggerTaker<'a>>
  {
    Some(LoggerTaker {
      logger,
      call_in_none: Option::take(&mut self.call_in_none)?,
    })
  }
}

impl LoggerTryTakerBuilder
{
  pub fn call_in_err(
    &mut self, call_in_err: impl FnMut(&dyn std::error::Error) + 'static,
  ) -> &mut Self
  {
    self.call_in_err = Some(Box::new(call_in_err));
    self
  }

  pub fn build<'a>(
    &mut self, logger: &'a mut Logger,
  ) -> Option<LoggerTryTaker<'a>>
  {
    Some(LoggerTryTaker {
      logger,
      call_in_err: Option::take(&mut self.call_in_err)?,
    })
  }
}

impl Taker for LoggerTaker<'_>
{
  fn take<T: Take>(&mut self, take: T) -> Option<T::Inner>
  {
    take.take().or_else(|| {
      (self.call_in_none)();
      None
    })
  }
}

impl TryTaker for LoggerTryTaker<'_>
{
  fn try_take<T>(&mut self, try_take: T) -> Result<T::Inner, T::Error>
  where T: TryTake
  {
    try_take.try_take().or_else(|err| {
      (self.call_in_err)(todo!());
      Err(err)
    })
  }
}

impl<T> ResultWithLogger<T>
{
  pub fn new(logger: Logger, inner: T) -> Self
  {
    Self { logger, inner }
  }

  pub fn into_logger(self) -> Logger
  {
    self.logger
  }

  pub fn logger(&self) -> &Logger
  {
    &self.logger
  }

  pub fn logger_mut(&mut self) -> &mut Logger
  {
    &mut self.logger
  }

  pub fn into_result(self) -> Result<T, Logger>
  {
    if self.logger.is_empty() { Ok(self.inner) } else { Err(self.logger) }
  }
}

impl<T> AsRef<T> for ResultWithLogger<T>
{
  fn as_ref(&self) -> &T
  {
    &self.inner
  }
}

impl<T> AsMut<T> for ResultWithLogger<T>
{
  fn as_mut(&mut self) -> &mut T
  {
    &mut self.inner
  }
}

impl<T> Wrapper for ResultWithLogger<T>
{
  type Inner = T;
}

impl<T> IntoInner for ResultWithLogger<T>
{
  fn into_inner(self) -> T
  {
    self.inner
  }
}
impl<T> Inner for ResultWithLogger<T>
{
  fn inner(&self) -> &T
  {
    &self.inner
  }
}
impl<T> InnerMut for ResultWithLogger<T>
{
  fn inner_mut(&mut self) -> &mut T
  {
    &mut self.inner
  }
}
impl<T> FromInner for ResultWithLogger<T>
{
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { logger: Logger::default(), inner }
  }
}

impl ToTokens for Logger
{
  fn to_tokens(&self, tokens: &mut TokenStream)
  {
    let errors = self.errors.iter().map(Error::to_compile_error);
    let formatted_warnings =
      self.formatted_warnings.iter().map(ToTokens::to_token_stream);
    *tokens = quote! {
      #(#errors);*
      #(#formatted_warnings);*
    };
  }

  fn into_token_stream(self) -> TokenStream
  where Self: Sized
  {
    let errors = self.errors.into_iter().map(Error::into_compile_error);
    let formatted_warnings =
      self.formatted_warnings.into_iter().map(ToTokens::into_token_stream);
    quote! {
      #(#errors);*
      #(#formatted_warnings);*
    }
  }
}

impl Logger
{
  pub fn is_empty(&self) -> bool
  {
    self.formatted_warnings.is_empty() && self.errors.is_empty()
  }

  pub fn formatted_warning(&mut self, formatted_warning: FormattedWarning)
  {
    self.formatted_warnings.push(formatted_warning);
  }

  pub fn warn_deprecated(
    &mut self, span: Span, name: impl AsRef<str>, note: impl Display,
  )
  {
    let note = note.to_string();
    self.formatted_warning(FormattedWarning::new_deprecated(name, note, span));
  }

  pub fn warn_deprecated_spanned(
    &mut self, tokens: impl ToTokens, name: impl AsRef<str>, note: impl Display,
  )
  {
    self.warn_deprecated(tokens.span(), name, note)
  }

  pub fn syn_error(&mut self, error: Error)
  {
    self.errors.push(error);
  }

  pub fn error(&mut self, span: Span, message: impl Display)
  {
    self.syn_error(Error::new(span, message));
  }

  pub fn error_spanned(&mut self, tokens: impl ToTokens, message: impl Display)
  {
    self.syn_error(Error::new_spanned(tokens, message));
  }
}
