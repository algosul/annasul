use std::{
  borrow::Borrow,
  ops::{Add, Div, Mul, Rem, Sub},
  simd::{
    Mask,
    MaskElement,
    Simd,
    SimdElement,
    cmp::SimdPartialEq,
    num::{SimdFloat, SimdInt, SimdUint},
    simd_swizzle,
  },
};

use algosul_core::wrapper::prelude::*;
use algosul_derive::Wrapper;
use num::{One, traits::ConstOne};
use num_traits::{ConstZero, Zero};

use crate::ops::{Cross, Dot};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[cfg(not(feature = "__feature-algosul-wrapper"))]
compile_error!("no feature '__feature-algosul-wrapper'");

#[cfg(not(feature = "__feature-const_trait_impl"))]
compile_error!("no feature '__feature-const_trait_impl'");

#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Wrapper,
)]
pub struct Vector<T, const N: usize>
where T: SimdElement
{
  #[wrapper(inner)]
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

impl<T: SimdElement, const N: usize> AsRef<[T; N]> for Vector<T, N>
{
  #[inline]
  fn as_ref(&self) -> &[T; N]
  {
    self.inner.as_ref()
  }
}

impl<T: SimdElement, const N: usize> AsMut<[T; N]> for Vector<T, N>
{
  #[inline]
  fn as_mut(&mut self) -> &mut [T; N]
  {
    self.inner.as_mut()
  }
}

impl<T, const N: usize> Vector<T, N>
where T: SimdElement
{
  #[inline]
  pub const fn from_array(array: [T; N]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_array(array))
  }

  #[inline]
  pub const fn as_array(&self) -> &[T; N]
  {
    self.inner.as_array()
  }

  #[inline]
  pub const fn as_mut_array(&mut self) -> &mut [T; N]
  {
    self.inner.as_mut_array()
  }

  #[inline]
  pub const fn to_array(self) -> [T; N]
  {
    self.inner.to_array()
  }

  #[inline]
  pub const fn from_slice(slice: &[T]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_slice(slice))
  }
}

const impl<T, const N: usize> FromInner for Vector<T, N>
where T: SimdElement
{
  #[inline]
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner }
  }
}

impl<T, const N: usize> Add for Vector<T, N>
where
  T: ConstOne + SimdElement,
  <Self as Wrapper>::Inner: Add<Output = <Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.add(rhs.inner))
  }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
  T: ConstOne + SimdElement,
  <Self as Wrapper>::Inner: Sub<Output = <Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.sub(rhs.inner))
  }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
  T: ConstOne + SimdElement,
  <Self as Wrapper>::Inner: Mul<Output = <Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.mul(rhs.inner))
  }
}

impl<T, const N: usize> Div for Vector<T, N>
where
  T: ConstOne + SimdElement,
  <Self as Wrapper>::Inner: Div<Output = <Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn div(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.div(rhs.inner))
  }
}

impl<T, const N: usize> Rem for Vector<T, N>
where
  T: ConstOne + SimdElement,
  <Self as Wrapper>::Inner: Rem<Output = <Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn rem(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.rem(rhs.inner))
  }
}

impl<T, M, const N: usize> Zero for Vector<T, N>
where
  T: SimdElement + ConstZero,
  M: MaskElement,
  Self: Add<Output = Self>,
  <Self as Wrapper>::Inner: SimdPartialEq<Mask = Mask<M, N>>,
{
  #[inline]
  fn zero() -> Self
  {
    Self::ZERO
  }

  fn is_zero(&self) -> bool
  {
    self.inner.simd_eq(Self::ZERO.inner).all()
  }
}

impl<T, M, const N: usize> ConstZero for Vector<T, N>
where
  T: SimdElement + ConstZero,
  M: MaskElement,
  Self: Add<Output = Self>,
  <Self as Wrapper>::Inner: SimdPartialEq<Mask = Mask<M, N>>,
{
  const ZERO: Self = Self::from_array([T::ZERO; N]);
}

impl<T, const N: usize> One for Vector<T, N>
where
  T: SimdElement + ConstOne,
  Self: Mul<Output = Self>,
{
  #[inline]
  fn one() -> Self
  {
    Self::ONE
  }
}

impl<T, const N: usize> ConstOne for Vector<T, N>
where
  T: SimdElement + ConstOne,
  Self: Mul<Output = Self>,
{
  const ONE: Self = Self::from_array([T::ONE; N]);
}

macro_rules! impl_for_base {
  ($ty:ty) => {
    impl<const N: usize> Vector<$ty, N>
    {
      /// vector norm (length)
      #[inline]
      pub fn inorm(&self) -> $ty
      {
        self.norm_squared().isqrt()
      }

      /// Square of the vector norm (length)
      #[inline]
      pub fn norm_squared(&self) -> $ty
      {
        (self.inner * self.inner).reduce_sum()
      }

      /// vector norm (length)
      /// see [Self::norm]
      #[inline]
      pub fn ilength(&self) -> $ty
      {
        self.inorm()
      }

      /// Square of the vector norm (length)
      /// see [Self::norm_squared]
      #[inline]
      pub fn length_squared(&self) -> $ty
      {
        self.norm_squared()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_for_base!($ty);)+
  };
}
macro_rules! impl_for_int {
  ($ty:ty) => {
    impl<const N: usize> Vector<$ty, N>
    {

      /// vector norm (length)
      #[inline]
      pub fn checked_inorm(&self) -> Option<$ty>
      {
        self.norm_squared().checked_isqrt()
      }

      /// vector norm (length)
      /// see [Self::norm]
      #[inline]
      pub fn checked_ilength(&self) -> Option<$ty>
      {
        self.checked_inorm()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_for_int!($ty);)+
  };
}

macro_rules! impl_for_float {
  ($ty:ty) => {
    impl<const N: usize> Vector<$ty, N>
    {
      /// vector norm (length)
      #[inline]
      pub fn norm(&self) -> $ty
      {
        self.norm_squared().sqrt()
      }

      /// Square of the vector norm (length)
      #[inline]
      pub fn norm_squared(&self) -> $ty
      {
        (self.inner * self.inner).reduce_sum()
      }

      /// vector norm (length)
      /// see [Self::norm]
      #[inline]
      pub fn length(&self) -> $ty
      {
        self.norm()
      }

      /// Square of the vector norm (length)
      /// see [Self::norm_squared]
      #[inline]
      pub fn length_squared(&self) -> $ty
      {
        self.norm_squared()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_for_float!($ty);)+
  };
}
macro_rules! impl_for_all {
  ($ty:ty) => {
    impl<const N: usize> Dot for Vector<$ty, N>
    {
      type Output = $ty;
      #[inline]
      fn dot(self, rhs: Self) -> Self::Output
      {
        self.mul(rhs).inner.reduce_sum()
      }
    }
    impl Cross for Vector<$ty, 2>
    {
      type Output = $ty;
      #[inline]
      fn cross(self, rhs: Self) -> Self::Output
      {
        self.x() * rhs.y() - self.y() * rhs.x()
      }
    }
    impl Cross for Vector<$ty, 3>
    {
      type Output = Self;
      #[inline]
      fn cross(self, rhs: Self) -> Self::Output
      {
        let y1z2_z1x2_x1y2 = simd_swizzle!(self.inner, [1, 2, 0])
          .mul(simd_swizzle!(rhs.inner, [2, 0, 1]));
        let z1y2_x1z2_y1x2 = simd_swizzle!(self.inner, [2, 0, 1])
          .mul(simd_swizzle!(rhs.inner, [1, 2, 0]));
        Self::from_inner(y1z2_z1x2_x1y2.sub(z1y2_x1z2_y1x2))
      }
    }
    impl Cross for Vector<$ty, 4>
    {
      type Output = Self;
      #[inline]
      fn cross(self, rhs: Self) -> Self::Output
      {
        let y1z2_z1x2_x1y2 = simd_swizzle!(self.inner, [1, 2, 0, 3])
          .mul(simd_swizzle!(rhs.inner, [2, 0, 1, 3]));
        let z1y2_x1z2_y1x2 = simd_swizzle!(self.inner, [2, 0, 1, 3])
          .mul(simd_swizzle!(rhs.inner, [1, 2, 0, 3]));
        Self::from_inner(y1z2_z1x2_x1y2.sub(z1y2_x1z2_y1x2))
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_for_all!($ty);)+
  };
}
impl_for_int!(i8 i16 i32 i64);
impl_for_base!(i8 i16 i32 i64 u8 u16 u32 u64);
impl_for_float!(f32 f64);
impl_for_all!(i8 i16 i32 i64 u8 u16 u32 u64 f32 f64);

macro_rules! impl_base_for_all_int {
  ($ty:ty) => {
    impl Vector<$ty, 2>
    {
      pub const X: Self = Self::from_array([<$ty>::ONE, 0]);
      pub const Y: Self = Self::from_array([0, 1]);
    }
    impl Vector<$ty, 3>
    {
      pub const X: Self = Self::from_array([1, 0, 0]);
      pub const Y: Self = Self::from_array([0, 1, 0]);
      pub const Z: Self = Self::from_array([0, 0, 1]);
    }
    impl Vector<$ty, 4>
    {
      pub const X: Self = Self::from_array([1, 0, 0, 0]);
      pub const Y: Self = Self::from_array([0, 1, 0, 0]);
      pub const Z: Self = Self::from_array([0, 0, 1, 0]);
      pub const W: Self = Self::from_array([0, 0, 0, 1]);
    }
  };
  ($($ty:ty)+) => {
    $(impl_base_for_all_int!($ty);)+
  };
}

macro_rules! impl_base_for_float {
  ($ty:ty) => {
    impl Vector<$ty, 2>
    {
      pub const X: Self = Self::from_array([1.0, 0.0]);
      pub const Y: Self = Self::from_array([0.0, 1.0]);
    }
    impl Vector<$ty, 3>
    {
      pub const X: Self = Self::from_array([1.0, 0.0, 0.0]);
      pub const Y: Self = Self::from_array([0.0, 1.0, 0.0]);
      pub const Z: Self = Self::from_array([0.0, 0.0, 1.0]);
    }
    impl Vector<$ty, 4>
    {
      pub const X: Self = Self::from_array([1.0, 0.0, 0.0, 0.0]);
      pub const Y: Self = Self::from_array([0.0, 1.0, 0.0, 0.0]);
      pub const Z: Self = Self::from_array([0.0, 0.0, 1.0, 0.0]);
      pub const W: Self = Self::from_array([0.0, 0.0, 0.0, 1.0]);
    }
  };
  ($($ty:ty)+) => {
    $(impl_base_for_float!($ty);)+
  };
}

impl_base_for_all_int!(i8 i16 i32 i64 u8 u16 u32 u64);
impl_base_for_float!(f32 f64);

crate::impl_element_getter! {
  Vector {
    x: 0 1 2 3 4 5 6 7 8,
    y: 1 2 3 4 5 6 7 8,
    z: 2 3 4 5 6 7 8,
    w: 3 4 5 6 7 8,
  }
}

#[cfg(test)]
mod tests
{
  use std::fmt::Debug;

  use num_traits::{ConstZero, Zero};

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
  #[test]
  fn test_inorm()
  {
    let vec = VectorI32x2::from_array([1, 7]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 50);
    assert_eq!(norm, norm_squared.isqrt());

    let vec = VectorI32x3::from_array([3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 50);
    assert_eq!(norm, norm_squared.isqrt());

    let vec = VectorI32x4::from_array([2, 3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 54);
    assert_eq!(norm, norm_squared.isqrt());
  }
  #[test]
  fn test_checked_inorm()
  {
    let vec = VectorI32x2::from_array([3, 4]);
    let norm_squared = vec.norm_squared();
    let norm = vec.checked_inorm();
    assert_eq!(norm_squared, 25);
    assert_eq!(norm, norm_squared.checked_isqrt());

    let vec = VectorI32x3::from_array([3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.checked_inorm();
    assert_eq!(norm_squared, 50);
    assert_eq!(norm, norm_squared.checked_isqrt());

    let vec = VectorI32x4::from_array([2, 3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.checked_inorm();
    assert_eq!(norm_squared, 54);
    assert_eq!(norm, norm_squared.checked_isqrt());
  }
  #[test]
  fn test_length()
  {
    let vec = VectorF32x2::from_array([1.0, 8.0]);
    let length_squared = vec.length_squared();
    let length = vec.length();
    assert_eq!(length * length, length_squared);
    assert_eq!(length_squared, 65.0);
    let vec = VectorF32x3::from_array([3.0, 4.0, 5.0]);
    let length_squared = vec.length_squared();
    let length = vec.length();
    assert_eq!(length * length, length_squared);
    assert_eq!(length_squared, 50.0);
    let vec = VectorF32x4::from_array([2.0, 3.0, 4.0, 5.0]);
    let length_squared = vec.length_squared();
    let length = vec.length();
    assert_eq!(length * length, length_squared);
    assert_eq!(length_squared, 54.0);
  }
  #[test]
  fn test_dot()
  {
    let vec1 = VectorF32x2::from_array([1.0, 2.0]);
    let vec2 = VectorF32x2::from_array([3.0, 4.0]);
    let d = vec1.dot(vec2);
    assert_eq!(d, 11.0);
    let vec1 = VectorF32x3::from_array([1.0, 2.0, 3.0]);
    let vec2 = VectorF32x3::from_array([4.0, 5.0, 6.0]);
    let d = vec1.dot(vec2);
    assert_eq!(d, 32.0);
    let vec1 = VectorF32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let vec2 = VectorF32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    let d = vec1.dot(vec2);
    assert_eq!(d, 70.0);
  }
  #[test]
  fn test_cross()
  {
    let vec1 = VectorF32x2::from_array([1.0, 2.0]);
    let vec2 = VectorF32x2::from_array([3.0, 4.0]);
    let right = -2.0;
    let d = vec1.cross(vec2);
    assert_eq!(d, right);
    let vec1 = VectorF32x3::from_array([1.0, 2.0, 3.0]);
    let vec2 = VectorF32x3::from_array([4.0, 5.0, 6.0]);
    let right = VectorF32x3::from_array([-3.0, 6.0, -3.0]);
    let d = vec1.cross(vec2);
    assert_eq!(d, right);
    let vec1 = VectorF32x4::from_array([1.0, 2.0, 3.0, 4.0]);
    let vec2 = VectorF32x4::from_array([5.0, 6.0, 7.0, 8.0]);
    let right = VectorF32x4::from_array([-4.0, 8.0, -4.0, 0.0]);
    let d = vec1.cross(vec2);
    assert_eq!(d, right);
  }

  fn test_template_const_zero<T, const N: usize>()
  where
    T: Debug + ConstZero + SimdElement + PartialEq,
    Vector<T, N>: ConstZero,
  {
    assert!(Vector::<T, N>::zero().as_array().iter().all(|&x| x == T::zero()));
    assert!(Vector::<T, N>::ZERO.as_array().iter().all(|&x| x == T::ZERO));
  }

  fn test_template_const_one<T, const N: usize>()
  where
    T: Debug + ConstOne + SimdElement + PartialEq,
    Vector<T, N>: ConstOne,
  {
    assert!(Vector::<T, N>::one().as_array().iter().all(|&x| x == T::one()));
    assert!(Vector::<T, N>::ONE.as_array().iter().all(|&x| x == T::ONE));
  }

  #[test]
  fn test_const_zero()
  {
    test_template_const_zero::<f32, 4>()
  }

  #[test]
  fn test_const_one()
  {
    test_template_const_one::<f32, 4>()
  }
}
