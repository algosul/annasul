#[cfg(feature = "__feature-const_trait_impl")]
pub mod __unstable;
pub mod ops;
#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait NormSquared
{
  type Output;
  /// Square of the vector norm (length)
  fn norm_squared(self) -> Self::Output;
  /// Square of the vector norm (length)
  /// see [Self::norm_squared]
  fn length_squared(self) -> Self::Output
  where Self: Sized
  {
    self.norm_squared()
  }
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::NormSquared;
#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait Norm
{
  type Output;
  /// vector norm (length)
  fn norm(self) -> Self::Output;
  /// vector norm (length)
  /// see [Self::norm]
  fn length(self) -> Self::Output
  where Self: Sized
  {
    self.norm()
  }
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::Norm;
#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait INorm
{
  type Output;
  /// vector norm (length)
  fn inorm(self) -> Self::Output;
  /// vector norm (length)
  /// see [Self::inorm]
  fn ilength(self) -> Self::Output
  where Self: Sized
  {
    self.inorm()
  }
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::INorm;
#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait CheckedINorm
{
  type Output;
  /// vector norm (length)
  fn checked_inorm(self) -> Option<Self::Output>;
  /// vector norm (length)
  /// see [Self::checked_inorm]
  fn checked_ilength(self) -> Option<Self::Output>
  where Self: Sized
  {
    self.checked_inorm()
  }
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::CheckedINorm;

// The only implementor of the norm / dot / cross traits is `Vector`
// (see vector.rs), so these tests require the vector-related feature
// to be enabled in order to compile and run.
#[cfg(test)]
#[cfg(feature = "std-unstable-vector")]
mod tests
{
  use super::ops::{Cross, Dot};
  use super::{CheckedINorm, INorm, Norm, NormSquared};
  use crate::vector::Vector;

  #[test]
  fn norm_and_norm_squared()
  {
    let v = Vector::<f32, 3>::from_array([3.0, 4.0, 0.0]);
    assert_eq!(NormSquared::norm_squared(v), 25.0);
    assert_eq!(Norm::norm(v), 5.0);
    assert_eq!(Norm::length(v), 5.0);
  }

  #[test]
  fn inorm_and_checked()
  {
    let v = Vector::<i32, 3>::from_array([3, 4, 0]);
    assert_eq!(INorm::inorm(v), 5);
    assert_eq!(CheckedINorm::checked_inorm(v), Some(5));
    // A two's-complement case without a square root must not panic
    let _ = CheckedINorm::checked_inorm(Vector::<i32, 2>::from_array([-1, 0]));
  }

  #[test]
  fn dot_product()
  {
    let a = Vector::<f32, 2>::from_array([1.0, 2.0]);
    let b = Vector::<f32, 2>::from_array([3.0, 4.0]);
    assert_eq!(Dot::dot(a, b), 11.0);
  }

  #[test]
  fn cross_product_3d()
  {
    let a = Vector::<f32, 3>::from_array([1.0, 2.0, 3.0]);
    let b = Vector::<f32, 3>::from_array([4.0, 5.0, 6.0]);
    assert_eq!(Cross::cross(a, b).to_array(), [-3.0, 6.0, -3.0]);
  }
}
