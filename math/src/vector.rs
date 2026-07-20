use std::simd::{Simd, SimdElement, num::SimdFloat};

use algosul_core::wrapper::{FromInner, Inner, InnerMut, IntoInner, Wrapper};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector<T, const N: usize>
where T: SimdElement
{
  inner: Simd<T, N>,
}

crate::type_defines! {
  Vector {
    VectorI8x2(i8, 2),
    VectorI8x3(i8, 3),
    VectorI8x4(i8, 4),
    VectorU8x2(u8, 2),
    VectorU8x3(u8, 3),
    VectorU8x4(u8, 4),
    VectorI16x2(i16, 2),
    VectorI16x3(i16, 3),
    VectorI16x4(i16, 4),
    VectorU16x2(u16, 2),
    VectorU16x3(u16, 3),
    VectorU16x4(u16, 4),
    VectorI32x2(i32, 2),
    VectorI32x3(i32, 3),
    VectorI32x4(i32, 4),
    VectorU32x2(u32, 2),
    VectorU32x3(u32, 3),
    VectorU32x4(u32, 4),
    VectorI64x2(i64, 2),
    VectorI64x3(i64, 3),
    VectorI64x4(i64, 4),
    VectorU64x2(u64, 2),
    VectorU64x3(u64, 3),
    VectorU64x4(u64, 4),
    VectorF32x1(f32, 1),
    VectorF32x2(f32, 2),
    VectorF32x3(f32, 3),
    VectorF32x4(f32, 4),
    VectorF64x2(f64, 2),
    VectorF64x3(f64, 3),
    VectorF64x4(f64, 4),
  }
}

impl<T, const N: usize> Vector<T, N>
where T: SimdElement
{
  #[inline]
  pub fn from_array(array: [T; N]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_array(array))
  }

  #[inline]
  pub fn from_slice(slice: &[T]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_slice(slice))
  }
}

impl<T, const N: usize> Wrapper for Vector<T, N>
where T: SimdElement
{
  type Inner = Simd<T, N>;
}

impl<T, const N: usize> Inner for Vector<T, N>
where T: SimdElement
{
  fn inner(&self) -> &Self::Inner
  {
    &self.inner
  }
}

impl<T, const N: usize> InnerMut for Vector<T, N>
where T: SimdElement
{
  fn inner_mut(&mut self) -> &mut Self::Inner
  {
    &mut self.inner
  }
}

impl<T, const N: usize> IntoInner for Vector<T, N>
where T: SimdElement + SimdFloat
{
  fn into_inner(self) -> Self::Inner
  {
    self.inner
  }
}

impl<T, const N: usize> FromInner for Vector<T, N>
where T: SimdElement
{
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner }
  }
}

macro_rules! impl_for_floats {
  ($($ty:ty)+ $(,)?) => {
    $(impl_for_float!($ty);)+
  };
}

macro_rules! impl_for_float {
  ($ty:ty) => {
    impl<const N: usize> Vector<$ty, N>
    {
      /// vector norm (length)
      pub fn norm(&self) -> $ty
      {
        self.norm_squared().sqrt()
      }

      /// Square of the vector norm (length)
      pub fn norm_squared(&self) -> $ty
      {
        (self.inner * self.inner).reduce_sum()
      }

      /// vector norm (length)
      /// see [Self::norm]
      pub fn length(&self) -> $ty
      {
        self.norm()
      }

      /// Square of the vector norm (length)
      /// see [Self::norm_squared]
      pub fn length_squared(&self) -> $ty
      {
        self.norm_squared()
      }
    }
  };
}
impl_for_floats!(f32 f64);

crate::impl_element_getter! {
  Vector {
    x: 1,
    y: 2,
    z: 3,
    w: 4,
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  #[test]
  fn test_norm()
  {
    let vec = VectorF32x2::from_array([1.0, 8.0]);
    let norm_squared = vec.norm_squared();
    let norm = vec.norm();
    assert_eq!(norm * norm, norm_squared);
    assert_eq!(norm_squared, 65.0);
    let vec = VectorF32x3::from_array([3.0, 4.0, 5.0]);
    let norm_squared = vec.norm_squared();
    let norm = vec.norm();
    assert_eq!(norm * norm, norm_squared);
    assert_eq!(norm_squared, 50.0);
    let vec = VectorF32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let norm_squared = vec.norm_squared();
    let norm = vec.norm();
    assert_eq!(norm * norm, norm_squared);
    assert_eq!(norm_squared, 54.0);
  }
}
