use std::fmt::{Display, Formatter, Pointer};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
  Attribute,
  DeriveInput,
  Field,
  ItemFn,
  parse_macro_input,
  spanned::Spanned,
};
use wrapper::WrapperParser;

mod wrapper;

mod template;

#[derive(Clone)]
enum Message
{
  Warn(Warn),
  Error(Error),
}

#[derive(Clone)]
enum Warn {}

#[derive(Clone)]
enum Error
{
  WrapperInnerUseForPublicField
  {
    field: Field
  },
  InvalidAttribute
  {
    attr: Attribute
  },
  DuplicateWrapperInnerAttribute
  {
    attr: Attribute
  },
  NoWrapperInner
  {
    span: Span
  },
}

impl Warn
{
  fn code(&self) -> u32
  {
    match self
    {
      _ => 0x000,
    }
  }

  fn code_str(&self) -> String
  {
    format!("algosul::W{:03X}", self.code())
  }
}

impl Error
{
  fn code(&self) -> u32
  {
    match self
    {
      Error::WrapperInnerUseForPublicField { .. } => 0x001,
      Error::InvalidAttribute { .. } => 0x002,
      Error::DuplicateWrapperInnerAttribute { .. } => 0x003,
      Error::NoWrapperInner { .. } => 0x004,
    }
  }

  fn code_str(&self) -> String
  {
    format!("algosul::E{:03X}", self.code())
  }

  fn to_syn(&self) -> syn::Error
  {
    let (span, message) = match self
    {
      Error::WrapperInnerUseForPublicField { field } => (
        field.vis.span(),
        field.ident.as_ref().map_or_else(
          || "#[wrapper(inner)] cannot use for public field".to_string(),
          |ident| {
            format!("#[wrapper(inner)] cannot use for public field `{ident}`")
          },
        ),
      ),
      Error::InvalidAttribute { attr } => (
        attr.span(),
        format!("Invalid attribute: `{}`", attr.to_token_stream()),
      ),
      Error::DuplicateWrapperInnerAttribute { attr } =>
      {
        (attr.span(), "Duplicate `#[wrapper(inner)]` attribute".to_string())
      }
      Error::NoWrapperInner { span } =>
      {
        (*span, "no set `#[wrapper(inner)]`".to_string())
      }
    };
    syn::Error::new(span, format!("[{}]: {message}", self.code_str()))
  }
}

impl Display for Warn
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    f.write_fmt(format_args!("{}", self.code_str()))
  }
}

impl Display for Error
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    f.write_fmt(format_args!("{}", self.code_str()))
  }
}

impl Display for Message
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
  {
    match self
    {
      Message::Warn(warn) => warn.fmt(f),
      Message::Error(error) => error.fmt(f),
    }
  }
}

impl ToTokens for Warn
{
  fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream)
  {
    todo!()
  }
}

impl ToTokens for Error
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream)
  {
    self.to_syn().to_compile_error().to_tokens(tokens)
  }
}

impl ToTokens for Message
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream)
  {
    match self
    {
      Message::Warn(warn) => warn.to_tokens(tokens),
      Message::Error(error) => error.to_tokens(tokens),
    }
  }
}

#[proc_macro_derive(Wrapper, attributes(wrapper))]
pub fn wrapper_derive(token_stream: TokenStream) -> TokenStream
{
  let input = parse_macro_input!(token_stream as DeriveInput);
  TokenStream::from(WrapperParser::new().parse(&input))
}

#[proc_macro]
pub fn template(token_stream: TokenStream) -> TokenStream
{
  TokenStream::from(template::TemplateParser::new().parse(token_stream.into()))
}

#[proc_macro_attribute]
pub fn template_fn(attr: TokenStream, item: TokenStream) -> TokenStream
{
  let parser = syn::meta::parser(|meta| {
    eprintln!("meta: {:#?}", meta.input);
    Ok(())
  });
  parse_macro_input!(attr with parser);
  let item_fn = parse_macro_input!(item as ItemFn);
  eprintln!("item_fn: {:#?}", item_fn.to_token_stream());
  item_fn.into_token_stream().into()
}
