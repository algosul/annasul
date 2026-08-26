use crate::{Connectable, Dependent, Result};

pub trait Connector
{
  fn dependent<A, B, Input>(
    &mut self, input: Input, dependent: Dependent<A, B>,
  ) -> Result<B::Output>
  where
    A: Connectable<Input> + Send + 'static,
    Input: Send + 'static,
    A::Output: Send + 'static,
    B: Connectable<A::Output> + Send + 'static,
    B::Output: Send + 'static;
}

#[derive(Debug, Clone)]
pub struct MutiThreadConnector {}

#[derive(Debug, Clone)]
pub struct SyncConnector {}

impl Default for MutiThreadConnector
{
  fn default() -> Self
  {
    Self {}
  }
}

impl MutiThreadConnector
{
  pub fn new() -> Self
  {
    Self::default()
  }
}

impl Connector for MutiThreadConnector
{
  fn dependent<A, B, Input>(
    &mut self, input: Input, dependent: Dependent<A, B>,
  ) -> Result<B::Output>
  where
    A: Connectable<Input> + Send + 'static,
    Input: Send + 'static,
    A::Output: Send + 'static,
    B: Connectable<A::Output> + Send + 'static,
    B::Output: Send + 'static,
  {
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut b_self = self.clone();
    let handle = std::thread::spawn(move || {
      let data: A::Output = receiver.recv().map_err(|err| Box::new(err) as _)?;
      Ok::<B::Output, Box<dyn std::error::Error + Send>>(
        dependent.b.connect(data, &mut b_self),
      )
    });
    sender
      .send(dependent.a.connect(input, self))
      .map_err(|err| Box::new(err) as _)
      .map_err(crate::Error::Send)?;
    handle
      .join()
      .map_err(|err| crate::Error::Thread(err))?
      .map_err(|err| crate::Error::Join(err))
  }
}

impl Default for SyncConnector
{
  fn default() -> Self
  {
    Self {}
  }
}

impl SyncConnector
{
  pub fn new() -> Self
  {
    Self::default()
  }
}

impl Connector for SyncConnector
{
  fn dependent<A, B, Input>(
    &mut self, input: Input, dependent: Dependent<A, B>,
  ) -> Result<B::Output>
  where
    A: Connectable<Input> + Send + 'static,
    Input: Send + 'static,
    A::Output: Send + 'static,
    B: Connectable<A::Output> + Send + 'static,
    B::Output: Send + 'static,
  {
    Ok(dependent.b.connect(dependent.a.connect(input, self), self))
  }
}
