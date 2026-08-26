use thiserror::Error;

use crate::connector::Connector;

mod _impl;
pub mod connector;
pub mod prelude;

#[derive(Debug, Error)]
pub enum Error
{
  #[cfg(feature = "serialization-rykv")]
  #[error("rkyv serialization: {0}")]
  RkyvSer(rkyv::rancor::Error),
  #[cfg(feature = "serialization-rykv")]
  #[error("rkyv deserialization: {0}")]
  RkyvDe(rkyv::rancor::Error),
  #[cfg(feature = "serialization-serde")]
  #[error("serde serialization: {0}")]
  SerdeSer(u8),
  #[cfg(feature = "serialization-serde")]
  #[error("serde deserialization: {0}")]
  SerdeDe(serde::de::value::Error),
  #[error("read error: {0}")]
  Read(std::io::Error),
  #[error("write error: {0}")]
  Write(std::io::Error),
  #[error("send error: {0}")]
  Send(Box<dyn std::error::Error>),
  #[error("thread unknown error")]
  Thread(Box<dyn std::any::Any>),
  #[error("join error: {0}")]
  Join(Box<dyn std::error::Error>),
}

pub type Result<T> = core::result::Result<T, Error>;

/// Two tasks linked one after the other: `b` runs on the output of `a`.
///
/// The input and output types are derived from the associated types of `a`
/// and `b`, so no extra type parameters are needed.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Dependent<A, B>
{
  a: A,
  b: B,
}

/// A chain of two sequential tasks that behaves as a single task.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Link<B, C>
{
  b: B,
  c: C,
}

/// Unwraps a task whose output is a [`Result`], falling back to `default` on
/// `Err`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnwrapOr<A, O>
{
  a:       A,
  default: O,
}

pub trait Runnable<Input>
{
  type Output;
  fn run_once(self, input: Input) -> Self::Output;
}

pub trait Connectable<Input>: Runnable<Input>
{
  fn connect(
    self, input: Input, connector: &mut impl Connector,
  ) -> Self::Output
  where Self: Sized;
}

pub trait TaskMap<B, Input>: Sized + Connectable<Input> + Send + 'static
where
  B: Send + 'static,
  Input: Send + 'static,
{
  type Mapped;
  fn map(self, b: B) -> Self::Mapped;
}

pub trait TaskUnwrapOr<Input, O>: Sized + Connectable<Input> + Send + 'static
where
  Input: Send + 'static,
  O: Send + 'static,
{
  type Mapped;
  fn unwrap_or(self, default: O) -> Self::Mapped;
}

impl<T: FnOnce(Input) -> Output, Input, Output> Runnable<Input> for T
{
  type Output = Output;

  fn run_once(self, input: Input) -> Self::Output
  {
    self(input)
  }
}

impl<T: FnOnce(Input) -> Output, Input, Output> Connectable<Input> for T
{
  fn connect(
    self, input: Input, _connector: &mut impl Connector,
  ) -> Self::Output
  {
    self(input)
  }
}

impl<B, C, Input> Runnable<Input> for Link<B, C>
where
  B: Connectable<Input> + Send + 'static,
  Input: Send + 'static,
  B::Output: Send + 'static,
  C: Connectable<B::Output> + Send + 'static,
  C::Output: Send + 'static,
{
  type Output = C::Output;

  fn run_once(self, input: Input) -> Self::Output
  {
    let mut connector = connector::SyncConnector::new();
    self.connect(input, &mut connector)
  }
}

impl<B, C, Input> Connectable<Input> for Link<B, C>
where
  B: Connectable<Input> + Send + 'static,
  Input: Send + 'static,
  B::Output: Send + 'static,
  C: Connectable<B::Output> + Send + 'static,
  C::Output: Send + 'static,
{
  fn connect(
    self, input: Input, connector: &mut impl Connector,
  ) -> Self::Output
  {
    self.c.connect(self.b.connect(input, connector), connector)
  }
}

impl<A, O, Input> Runnable<Input> for UnwrapOr<A, O>
where
  A: Connectable<Input, Output = Result<O>> + Send + 'static,
  Input: Send + 'static,
  O: Send + 'static,
{
  type Output = O;

  fn run_once(self, input: Input) -> Self::Output
  {
    match self.a.run_once(input) {
      Ok(value) => value,
      Err(_) => self.default,
    }
  }
}

impl<A, O, Input> Connectable<Input> for UnwrapOr<A, O>
where
  A: Connectable<Input, Output = Result<O>> + Send + 'static,
  Input: Send + 'static,
  O: Send + 'static,
{
  fn connect(
    self, input: Input, connector: &mut impl Connector,
  ) -> Self::Output
  {
    match self.a.connect(input, connector) {
      Ok(value) => value,
      Err(_) => self.default,
    }
  }
}

impl<A, B, Input> Runnable<Input> for Dependent<A, B>
where
  A: Connectable<Input> + Send + 'static,
  Input: Send + 'static,
  A::Output: Send + 'static,
  B: Connectable<A::Output> + Send + 'static,
  B::Output: Send + 'static,
{
  type Output = Result<B::Output>;

  fn run_once(self, input: Input) -> Self::Output
  {
    connector::SyncConnector::new().dependent(input, self)
  }
}

impl<A, B, Input> Connectable<Input> for Dependent<A, B>
where
  A: Connectable<Input> + Send + 'static,
  Input: Send + 'static,
  A::Output: Send + 'static,
  B: Connectable<A::Output> + Send + 'static,
  B::Output: Send + 'static,
{
  fn connect(self, input: Input, connector: &mut impl Connector)
  -> Self::Output
  {
    connector.dependent(input, self)
  }
}

impl<F, B, Input, Output> TaskMap<B, Input> for F
where
  F: FnOnce(Input) -> Output + Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
  B: Connectable<Output> + Send + 'static,
  B::Output: Send + 'static,
{
  type Mapped = Dependent<F, B>;

  fn map(self, right: B) -> Self::Mapped
  {
    Dependent { a: self, b: right }
  }
}

impl<A, B, C, Input> TaskMap<C, Input> for Dependent<A, B>
where
  A: Connectable<Input> + Send + 'static,
  Input: Send + 'static,
  A::Output: Send + 'static,
  B: Connectable<A::Output> + Send + 'static,
  B::Output: Send + 'static,
  C: Connectable<B::Output> + Send + 'static,
  C::Output: Send + 'static,
{
  type Mapped = Dependent<A, Link<B, C>>;

  fn map(self, right: C) -> Self::Mapped
  {
    Dependent {
      a: self.a,
      b: Link { b: self.b, c: right },
    }
  }
}

impl<A, O, C, Input> TaskMap<C, Input> for UnwrapOr<A, O>
where
  A: Connectable<Input, Output = Result<O>> + Send + 'static,
  Input: Send + 'static,
  O: Send + 'static,
  C: Connectable<O> + Send + 'static,
  C::Output: Send + 'static,
{
  type Mapped = Dependent<UnwrapOr<A, O>, C>;

  fn map(self, right: C) -> Self::Mapped
  {
    Dependent { a: self, b: right }
  }
}

impl<A, O, Input> TaskUnwrapOr<Input, O> for A
where
  A: Connectable<Input, Output = Result<O>> + Send + 'static,
  Input: Send + 'static,
  O: Send + 'static,
{
  type Mapped = UnwrapOr<A, O>;

  fn unwrap_or(self, default: O) -> Self::Mapped
  {
    UnwrapOr { a: self, default }
  }
}
