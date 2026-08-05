#[cfg(feature = "__feature-const_trait_impl")]
pub mod __unstable;

#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait Dot<Rhs = Self>
{
  type Output;
  /// vector dot product
  fn dot(self, rhs: Rhs) -> Self::Output;
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::Dot;

#[cfg(not(feature = "__feature-const_trait_impl"))]
pub trait Cross<Rhs = Self>
{
  type Output;
  /// vector cross product
  /// `(x, y, z) = (x1, y1, z1) x (x2, y2, z2)`
  fn cross(self, rhs: Rhs) -> Self::Output;
}
#[cfg(feature = "__feature-const_trait_impl")]
pub use __unstable::Cross;
