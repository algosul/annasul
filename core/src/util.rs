pub mod r#async;
#[cfg(feature = "std")]
pub mod std;

#[cfg(feature = "util-derive")]
pub mod derive;

#[cfg(not(feature = "std"))]
use core::{option::Option, result::Result};

pub trait Take
{
  type Inner;
  fn take(self) -> Option<Self::Inner>;
}

pub trait TryTake
{
  type Inner;
  type Error;
  fn try_take(self) -> Result<Self::Inner, Self::Error>;
}

pub trait Taker
{
  fn take<T: Take>(&mut self, take: T) -> Option<T::Inner>;
}

pub trait TryTaker
{
  fn try_take<T: TryTake>(&mut self, try_take: T)
  -> Result<T::Inner, T::Error>;
}

impl<T> Take for Option<T>
{
  type Inner = T;

  fn take(self) -> Option<Self::Inner>
  {
    self
  }
}

impl<T> Take for &mut Option<T>
{
  type Inner = T;

  fn take(self) -> Option<Self::Inner>
  {
    Option::take(self)
  }
}
