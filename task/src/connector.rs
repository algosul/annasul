use crate::{Connectable, Dependent, Result};

pub trait Connector
{
  fn dependent<
    A: Connectable<Input, Output = Data> + Send + 'static,
    B: Connectable<Data, Output = Output> + Send + 'static,
    Data: Send + 'static,
    Input: Send + 'static,
    Output: Send + 'static,
  >(
    &mut self, input: Input, dependent: Dependent<A, B, Data, Input, Output>,
  ) -> Result<Output>;
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
  fn dependent<
    A: Connectable<Input, Output = Data> + Send + 'static,
    B: Connectable<Data, Output = Output> + Send + 'static,
    Data: Send + 'static,
    Input: Send + 'static,
    Output: Send + 'static,
  >(
    &mut self, input: Input, dependent: Dependent<A, B, Data, Input, Output>,
  ) -> Result<Output>
  {
    let (sender, receiver) = std::sync::mpsc::channel();
    let mut b_self = self.clone();
    let handle = std::thread::spawn(move || {
      let data: Data = receiver.recv().map_err(|err| Box::new(err) as _)?;
      Ok::<Output, Box<dyn std::error::Error + Send>>(
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
  fn dependent<
    A: Connectable<Input, Output = Data> + Send,
    B: Connectable<Data, Output = Output> + Send,
    Data: Send,
    Input: Send,
    Output: Send,
  >(
    &mut self, input: Input, dependent: Dependent<A, B, Data, Input, Output>,
  ) -> Result<Output>
  {
    Ok(dependent.b.connect(dependent.a.connect(input, self), self))
  }
}
