use std::{
  io::{Read, Write},
  marker::PhantomData,
};

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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Dependent<
  A: Connectable<Input, Output = Data> + Send + 'static,
  B: Connectable<Data, Output = Output> + Send + 'static,
  Data: Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
> {
  a:      A,
  b:      B,
  _data:  PhantomData<Data>,
  _input: PhantomData<Input>,
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

pub trait TaskMap<
  B: Connectable<Data, Output = Output> + Send + 'static,
  Data: Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
>: Sized + Connectable<Input, Output = Data> + Send + 'static
{
  fn map(self, b: B) -> Dependent<Self, B, Data, Input, Output>;
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

impl<
  A: Connectable<Input, Output = Data> + Send + 'static,
  B: Connectable<Data, Output = Output> + Send + 'static,
  Data: Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
> Runnable<Input> for Dependent<A, B, Data, Input, Output>
{
  type Output = Result<Output>;

  fn run_once(self, input: Input) -> Self::Output
  {
    connector::SyncConnector::new().dependent(input, self)
  }
}
impl<A, B, Data, Input, Output> Connectable<Input>
  for Dependent<A, B, Data, Input, Output>
where
  A: Connectable<Input, Output = Data> + Send + 'static,
  B: Connectable<Data, Output = Output> + Send + 'static,
  Data: Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
{
  fn connect(self, input: Input, connector: &mut impl Connector)
  -> Self::Output
  {
    connector.dependent(input, self)
  }
}

impl<A, B, Data, Input, Output> TaskMap<B, Data, Input, Output> for A
where
  A: Connectable<Input, Output = Data> + Send + 'static,
  B: Connectable<Data, Output = Output> + Send + 'static,
  Data: Send + 'static,
  Input: Send + 'static,
  Output: Send + 'static,
{
  fn map(self, right: B) -> Dependent<Self, B, Data, Input, Output>
  {
    Dependent {
      a:      self,
      b:      right,
      _data:  PhantomData,
      _input: PhantomData,
    }
  }
}
