pub mod prelude;

pub trait Wrapper
{
  type Inner: ?Sized;
}

pub trait Inner: Wrapper
{
  fn inner(&self) -> &Self::Inner;
}

pub trait InnerMut: Inner
{
  fn inner_mut(&mut self) -> &mut Self::Inner;
}

pub trait IntoInner: Wrapper
{
  fn into_inner(self) -> Self::Inner;
}

#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait FromInner: Wrapper
{
  fn from_inner(inner: Self::Inner) -> Self;
}

#[cfg(feature = "__feature-const_trait_impl")]
pub mod __unstable;

use core::convert::Infallible;

#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::FromInner;

pub trait TryFromInner: Wrapper
where Self: Sized
{
  type Error;
  fn try_from_inner(inner: Self::Inner) -> Result<Self, Self::Error>;
}

impl<T: FromInner> TryFromInner for T
where T::Inner: Sized
{
  type Error = Infallible;

  fn try_from_inner(inner: Self::Inner) -> Result<Self, Self::Error>
  {
    Ok(Self::from_inner(inner))
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  // A simple newtype used to exercise the trait family
  #[derive(Debug, PartialEq)]
  struct Int(i32);

  impl Wrapper for Int
  {
    type Inner = i32;
  }

  impl Inner for Int
  {
    fn inner(&self) -> &Self::Inner
    {
      &self.0
    }
  }

  impl InnerMut for Int
  {
    fn inner_mut(&mut self) -> &mut Self::Inner
    {
      &mut self.0
    }
  }

  impl IntoInner for Int
  {
    fn into_inner(self) -> Self::Inner
    {
      self.0
    }
  }

  impl FromInner for Int
  {
    fn from_inner(inner: Self::Inner) -> Self
    {
      Self(inner)
    }
  }

  #[test]
  fn wrapper_inner_roundtrip()
  {
    let mut x = Int::from_inner(42);
    assert_eq!(x.inner(), &42);
    assert_eq!(x.inner_mut(), &mut 42);
    *x.inner_mut() = 7;
    assert_eq!(x.inner(), &7);
    assert_eq!(x.into_inner(), 7);
  }
}
