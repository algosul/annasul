use algosul_core::wrapper::prelude::*;
use algosul_derive::Wrapper;
use num::{One, integer::Roots, traits::ConstOne};
use num_traits::{ConstZero, MulAdd, MulAddAssign, Zero, real::Real};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use std::{
  ops::{Add, Div, Mul, Rem, Sub},
  simd::{
    Mask,
    MaskElement,
    Simd,
    SimdElement,
    cmp::SimdPartialEq,
    simd_swizzle,
  },
};

use crate::simd::{SimdMulAdd, SimdMulAddAssign};
use crate::{
  simd::SimdReduceSum,
  traits::{
    CheckedINorm,
    INorm,
    Norm,
    NormSquared,
    ops::{Cross, Dot},
  },
};

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
where
  T: SimdElement,
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
    #[cfg(feature = "unstable-f16")]
    VectorF16x2(f16, 2),
    #[cfg(feature = "unstable-f16")]
    VectorF16x3(f16, 3),
    #[cfg(feature = "unstable-f16")]
    VectorF16x4(f16, 4),
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
where
  T: SimdElement,
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
where
  T: SimdElement,
{
  #[inline]
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner }
  }
}

impl<T, const N: usize> Add for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Add<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.add(rhs.inner))
  }
}

impl<T, const N: usize> AddAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: AddAssign,
{
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.inner.add_assign(rhs.inner);
  }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Sub<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.sub(rhs.inner))
  }
}

impl<T, const N: usize> SubAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: SubAssign,
{
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    self.inner.sub_assign(rhs.inner);
  }
}

impl<T, const N: usize> Mul for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Mul<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.mul(rhs.inner))
  }
}

impl<T, const N: usize> MulAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: MulAssign,
{
  #[inline]
  fn mul_assign(&mut self, rhs: Self) {
    self.inner.mul_assign(rhs.inner);
  }
}

impl<T, const N: usize> MulAdd for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: SimdMulAdd<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn mul_add(self, a: Self, b: Self) -> Self::Output {
    Self::from_inner(self.inner.mul_add(a.inner, b.inner))
  }
}

impl<T, const N: usize> MulAddAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: SimdMulAddAssign,
{
  #[inline]
  fn mul_add_assign(&mut self, a: Self, b: Self) {
    self.inner.mul_add_assign(a.inner, b.inner);
  }
}

impl<T, const N: usize> Div for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Div<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn div(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.div(rhs.inner))
  }
}

impl<T, const N: usize> DivAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: DivAssign,
{
  #[inline]
  fn div_assign(&mut self, rhs: Self) {
    self.inner.div_assign(rhs.inner);
  }
}

impl<T, const N: usize> Rem for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Rem<Output=<Self as Wrapper>::Inner>,
{
  type Output = Self;

  #[inline]
  fn rem(self, rhs: Self) -> Self::Output
  {
    Self::from_inner(self.inner.rem(rhs.inner))
  }
}

impl<T, const N: usize> RemAssign for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: RemAssign,
{
  #[inline]
  fn rem_assign(&mut self, rhs: Self) {
    self.inner.rem_assign(rhs.inner);
  }
}

impl<T, M, const N: usize> Zero for Vector<T, N>
where
  T: SimdElement + ConstZero,
  M: MaskElement,
  Self: Add<Output=Self>,
  <Self as Wrapper>::Inner: SimdPartialEq<Mask=Mask<M, N>>,
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
  Self: Add<Output=Self>,
  <Self as Wrapper>::Inner: SimdPartialEq<Mask=Mask<M, N>>,
{
  const ZERO: Self = Self::from_array([T::ZERO; N]);
}

impl<T, const N: usize> One for Vector<T, N>
where
  T: SimdElement + ConstOne,
  Self: Mul<Output=Self>,
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
  Self: Mul<Output=Self>,
{
  const ONE: Self = Self::from_array([T::ONE; N]);
}

impl<T, const N: usize> Norm for Vector<T, N>
where
  T: SimdElement + Real,
  <Self as Wrapper>::Inner:
  SimdReduceSum<Scalar=T> + Mul<Output=<Self as Wrapper>::Inner>,
{
  type Output = T;

  #[inline]
  fn norm(self) -> T
  {
    self.norm_squared().sqrt()
  }
}

impl<T, const N: usize> NormSquared for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner:
  SimdReduceSum<Scalar=T> + Mul<Output=<Self as Wrapper>::Inner>,
{
  type Output = T;

  #[inline]
  fn norm_squared(self) -> T
  {
    (self.inner * self.inner).reduce_sum()
  }
}

impl<T, const N: usize> NormSquared for &Vector<T, N>
where
  T: SimdElement,
  <Vector<T, N> as Wrapper>::Inner:
  SimdReduceSum<Scalar=T> + Mul<Output=<Vector<T, N> as Wrapper>::Inner>,
{
  type Output = T;

  #[inline]
  fn norm_squared(self) -> T
  {
    (self.inner * self.inner).reduce_sum()
  }
}

impl<T, const N: usize> Dot for Vector<T, N>
where
  T: SimdElement,
  <Self as Wrapper>::Inner:
  SimdReduceSum<Scalar=T> + Mul<Output=<Self as Wrapper>::Inner>,
{
  type Output = T;

  #[inline]
  fn dot(self, rhs: Self) -> Self::Output
  {
    self.mul(rhs).inner.reduce_sum()
  }
}
impl<T> Cross for Vector<T, 2>
where
  T: SimdElement + Mul<Output=T> + Sub<Output=T>,
{
  type Output = T;

  #[inline]
  fn cross(self, rhs: Self) -> Self::Output
  {
    self.x() * rhs.y() - self.y() * rhs.x()
  }
}

impl<T> Cross for Vector<T, 3>
where
  T: SimdElement,
  <Self as Wrapper>::Inner: Sub<Output=<Self as Wrapper>::Inner>
  + Mul<Output=<Self as Wrapper>::Inner>,
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
impl<T> Cross for Vector<T, 4>
where
  T: SimdElement,
  <Vector<T, 4> as Wrapper>::Inner: Sub<Output=<Vector<T, 4> as Wrapper>::Inner>
  + Mul<Output=<Vector<T, 4> as Wrapper>::Inner>,
{
  type Output = Vector<T, 4>;

  #[inline]
  fn cross(self, rhs: Self) -> Self::Output
  {
    let y1z2_z1x2_x1y2 = simd_swizzle!(self.inner, [1, 2, 0, 3])
      .mul(simd_swizzle!(rhs.inner, [2, 0, 1, 3]));
    let z1y2_x1z2_y1x2 = simd_swizzle!(self.inner, [2, 0, 1, 3])
      .mul(simd_swizzle!(rhs.inner, [1, 2, 0, 3]));
    Vector::<T, 4>::from_inner(y1z2_z1x2_x1y2.sub(z1y2_x1z2_y1x2))
  }
}

impl<T, const N: usize> INorm for Vector<T, N>
where
  T: SimdElement + Roots,
  <Vector<T, N> as Wrapper>::Inner:
  SimdReduceSum<Scalar=T> + Mul<Output=<Vector<T, N> as Wrapper>::Inner>,
{
  type Output = T;

  #[inline]
  fn inorm(self) -> T
  {
    self.norm_squared().sqrt()
  }
}

impl<T> Vector<T, 2>
where
  T: SimdElement + ConstZero + ConstOne,
{
  pub const X: Self = Self::from_array([T::ONE, T::ZERO]);
  pub const Y: Self = Self::from_array([T::ZERO, T::ONE]);
}
impl<T> Vector<T, 3>
where
  T: SimdElement + ConstZero + ConstOne,
{
  pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO]);
  pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO]);
  pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE]);
}
impl<T> Vector<T, 4>
where
  T: SimdElement + ConstZero + ConstOne,
{
  pub const W: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
  pub const X: Self = Self::from_array([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
  pub const Y: Self = Self::from_array([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
  pub const Z: Self = Self::from_array([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
}

macro_rules! impl_for_int {
  ($ty:ty) => {
    impl<const N: usize> CheckedINorm for Vector<$ty, N>
    {
      type Output = $ty;
      #[inline]
      fn checked_inorm(self) -> Option<$ty>
      {
        self.norm_squared().checked_isqrt()
      }
    }
    impl<const N: usize> CheckedINorm for &Vector<$ty, N>
    {
      type Output = $ty;
      #[inline]
      fn checked_inorm(self) -> Option<$ty>
      {
        self.norm_squared().checked_isqrt()
      }
    }
  };
  ($($ty:ty)+) => {
    $(impl_for_int!($ty);)+
  };
}

impl_for_int!(i8 i16 i32 i64);

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

  macro_rules! test_for_all {
    ($func:ident($ty:ty)) => {
      $func::<$ty, 1>();
      $func::<$ty, 2>();
      $func::<$ty, 3>();
      $func::<$ty, 4>();
      $func::<$ty, 5>();
      $func::<$ty, 6>();
      $func::<$ty, 7>();
      $func::<$ty, 8>();
    };
    (@no_n $func:ident($ty:ty)) => {
      $func::<$ty>();
    };
    ($(@$meta:ident)? $id:ident : $func:ident(int)) => {
      #[test]
      fn $id()
      {
        test_for_all!($(@$meta)? $func(i8));
        test_for_all!($(@$meta)? $func(i16));
        test_for_all!($(@$meta)? $func(i32));
        test_for_all!($(@$meta)? $func(i64));
      }
    };
    ($(@$meta:ident)? $id:ident : $func:ident(uint)) => {
      #[test]
      fn $id()
      {
        test_for_all!($(@$meta)? $func(u8));
        test_for_all!($(@$meta)? $func(u16));
        test_for_all!($(@$meta)? $func(u32));
        test_for_all!($(@$meta)? $func(u64));
      }
    };
    ($(@$meta:ident)? $id:ident : $func:ident(int uint)) => {
      #[test]
      fn $id()
      {
        test_for_all!($(@$meta)? $func(i8));
        test_for_all!($(@$meta)? $func(u8));
        test_for_all!($(@$meta)? $func(i16));
        test_for_all!($(@$meta)? $func(u16));
        test_for_all!($(@$meta)? $func(i32));
        test_for_all!($(@$meta)? $func(u32));
        test_for_all!($(@$meta)? $func(i64));
        test_for_all!($(@$meta)? $func(u64));
        test_for_all!($(@$meta)? $func(f32));
        test_for_all!($(@$meta)? $func(f64));
      }
    };
    ($(@$meta:ident)? $id:ident : $func:ident(float)) => {
      #[test]
      fn $id()
      {
        test_for_all!($(@$meta)? $func(f32));
        test_for_all!($(@$meta)? $func(f64));
      }
    };
    ($(@$meta:ident)? $id:ident : $func:ident) => {
      #[test]
      fn $id()
      {
        test_for_all!($(@$meta)? $func(i8));
        test_for_all!($(@$meta)? $func(u8));
        test_for_all!($(@$meta)? $func(i16));
        test_for_all!($(@$meta)? $func(u16));
        test_for_all!($(@$meta)? $func(i32));
        test_for_all!($(@$meta)? $func(u32));
        test_for_all!($(@$meta)? $func(i64));
        test_for_all!($(@$meta)? $func(u64));
        test_for_all!($(@$meta)? $func(f32));
        test_for_all!($(@$meta)? $func(f64));
      }
    };
  }

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
    let vec = VectorU32x2::from_array([1, 7]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 50);
    assert_eq!(norm, norm_squared.isqrt());

    let vec = VectorU32x3::from_array([3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 50);
    assert_eq!(norm, norm_squared.isqrt());

    let vec = VectorU32x4::from_array([2, 3, 4, 5]);
    let norm_squared = vec.norm_squared();
    let norm = vec.inorm();
    assert_eq!(norm_squared, 54);
    assert_eq!(norm, norm_squared.isqrt());

    let vec = VectorU32x2::from_array([1, 7]);
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

  fn const_zero_n<T, const N: usize>()
  where
    T: Debug + ConstZero + SimdElement + PartialEq,
    Vector<T, N>: ConstZero,
  {
    assert!(Vector::<T, N>::zero().as_array().iter().all(|&x| x == T::zero()));
    assert!(Vector::<T, N>::ZERO.as_array().iter().all(|&x| x == T::ZERO));
    let mut vec = Vector::<T, N>::ZERO;
    assert!(vec.is_zero());
    vec.set_zero();
    assert!(vec.is_zero());
  }

  fn const_one_n<T, const N: usize>()
  where
    T: Debug + ConstOne + SimdElement + PartialEq,
    Vector<T, N>: ConstOne,
  {
    assert!(Vector::<T, N>::one().as_array().iter().all(|&x| x == T::one()));
    assert!(Vector::<T, N>::ONE.as_array().iter().all(|&x| x == T::ONE));
    let mut vec = Vector::<T, N>::ONE;
    assert!(vec.is_one());
    vec.set_one();
    assert!(vec.is_one());
  }

  test_for_all!(test_const_zero: const_zero_n);
  test_for_all!(test_const_one: const_one_n);

  fn base_2<T>()
  where
    T: Debug + ConstZero + ConstOne + SimdElement + PartialEq,
  {
    let array = Vector::<T, 2>::X.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(0));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 1);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 2>::Y.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(1));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 1);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);
  }

  fn base_3<T>()
  where
    T: Debug + ConstZero + ConstOne + SimdElement + PartialEq,
  {
    let array = Vector::<T, 3>::X.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(0));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 2);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 3>::Y.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(1));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 2);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 3>::Z.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(2));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 2);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);
  }

  fn base_4<T>()
  where
    T: Debug + ConstZero + ConstOne + SimdElement + PartialEq,
  {
    let array = Vector::<T, 4>::X.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(0));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 3);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 4>::Y.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(1));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 3);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 4>::Z.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(2));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 3);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);

    let array = Vector::<T, 4>::W.to_array();
    assert_eq!(array.iter().position(|x| x == &T::ONE), Some(3));
    assert_eq!(array.iter().filter(|&x| x == &T::ZERO).count(), 3);
    assert_eq!(array.iter().filter(|&x| x == &T::ONE).count(), 1);
  }

  test_for_all!(@no_n test_base_2: base_2);
  test_for_all!(@no_n test_base_3: base_3);
  test_for_all!(@no_n test_base_4: base_4);

  fn mul_add<T>()
  where
    T: Debug + ConstZero + ConstOne + SimdElement + PartialEq,
    Vector<T, 3>: Add<Output=Vector<T, 3>> + Mul<Output=Vector<T, 3>> + MulAdd<Output=Vector<T, 3>>,
  {
    let u = Vector::<T, 3>::from_array([T::ONE, T::ZERO, T::ZERO]);
    let v = Vector::<T, 3>::from_array([T::ZERO, T::ONE, T::ZERO]);
    let w = Vector::<T, 3>::from_array([T::ONE, T::ONE, T::ZERO]);
    assert_eq!(u + v, w);
    assert_eq!((u * v) + w, u.mul_add(v, w));
  }

  test_for_all!(@no_n test_mul_add: mul_add(float));
}
