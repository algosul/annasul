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

#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::FromInner;

pub trait TryFromInner: Wrapper
where Self: Sized
{
  type Error;
  fn try_from_inner(inner: Self::Inner) -> Result<Self, Self::Error>;
}
