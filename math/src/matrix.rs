use std::simd::{Simd, SimdElement, num::SimdFloat};

use algosul_core::wrapper::{FromInner, Inner, InnerMut, IntoInner, Wrapper};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Matrix<T, const LINE: usize, const COL: usize>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  inner: Simd<T, const { LINE * COL }>,
}
crate::type_defines! {
  Matrix {
    MatrixI8x2x2(i8, 2, 2),
    MatrixI8x3x3(i8, 3, 3),
    MatrixI8x4x4(i8, 4, 4),
    MatrixU8x2x2(u8, 2, 2),
    MatrixU8x3x3(u8, 3, 3),
    MatrixU8x4x4(u8, 4, 4),
    MatrixI16x2x2(i16, 2, 2),
    MatrixI16x3x3(i16, 3, 3),
    MatrixI16x4x4(i16, 4, 4),
    MatrixU16x2x2(u16, 2, 2),
    MatrixU16x3x3(u16, 3, 3),
    MatrixU16x4x4(u16, 4, 4),
    MatrixI32x2x2(i32, 2, 2),
    MatrixI32x3x3(i32, 3, 3),
    MatrixI32x4x4(i32, 4, 4),
    MatrixU32x2x2(u32, 2, 2),
    MatrixU32x3x3(u32, 3, 3),
    MatrixU32x4x4(u32, 4, 4),
    MatrixI64x2x2(i64, 2, 2),
    MatrixI64x3x3(i64, 3, 3),
    MatrixI64x4x4(i64, 4, 4),
    MatrixU64x2x2(u64, 2, 2),
    MatrixU64x3x3(u64, 3, 3),
    MatrixU64x4x4(u64, 4, 4),
    MatrixF32x1x1(f32, 1, 1),
    MatrixF32x2x2(f32, 2, 2),
    MatrixF32x3x3(f32, 3, 3),
    MatrixF32x4x4(f32, 4, 4),
    MatrixF64x2x2(f64, 2, 2),
    MatrixF64x3x3(f64, 3, 3),
    MatrixF64x4x4(f64, 4, 4),
  }
}

impl<T, const LINE: usize, const COL: usize> Matrix<T, LINE, COL>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  #[inline]
  pub fn from_array(array: [T; const { LINE * COL }]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_array(array))
  }

  #[inline]
  pub fn from_slice(slice: &[T]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_slice(slice))
  }
}

impl<T, const LINE: usize, const COL: usize> Wrapper for Matrix<T, LINE, COL>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  type Inner = Simd<T, const { LINE * COL }>;
}

impl<T, const LINE: usize, const COL: usize> Inner for Matrix<T, LINE, COL>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  fn inner(&self) -> &Self::Inner
  {
    &self.inner
  }
}

impl<T, const LINE: usize, const COL: usize> InnerMut for Matrix<T, LINE, COL>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  fn inner_mut(&mut self) -> &mut Self::Inner
  {
    &mut self.inner
  }
}

impl<T, const LINE: usize, const COL: usize> IntoInner for Matrix<T, LINE, COL>
where
  T: SimdElement + SimdFloat,
  [(); const { LINE * COL }]:,
{
  fn into_inner(self) -> Self::Inner
  {
    self.inner
  }
}

impl<T, const LINE: usize, const COL: usize> FromInner for Matrix<T, LINE, COL>
where
  T: SimdElement,
  [(); const { LINE * COL }]:,
{
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner }
  }
}
