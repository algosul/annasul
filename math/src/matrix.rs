use std::simd::{Simd, SimdElement};

use algosul_core::wrapper::prelude::*;
use algosul_derive::Wrapper;

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[cfg(not(feature = "__feature-algosul-wrapper"))]
compile_error!("no feature '__feature-algosul-wrapper'");

#[cfg(not(feature = "__feature-const_trait_impl"))]
compile_error!("no feature '__feature-const_trait_impl'");

/// # Params
/// + `N`: must is `LINE * COL`.
///   Why not calculate automatically?
///   Calculating `LINE * COL` requires the `min_generics_const_param` feature
///   from the Rust nightly release
///   (https://github.com/rust-lang/rust/issues/132980).
///   Some macros derive (e.g. `algosul-derive::Wrapper`) uses the crate `syn`.
///   However, `syn` does not support `min_generics_const_param`.
#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Wrapper,
)]
pub struct Matrix<T, const LINE: usize, const COL: usize, const N: usize>
where T: SimdElement
{
  #[wrapper(inner)]
  inner: Simd<T, N>,
}
crate::type_defines! {
  Matrix {
    MatrixI8x2x2(i8, 2, 2, 4),
    MatrixI8x3x3(i8, 3, 3, 9),
    MatrixI8x4x4(i8, 4, 4, 16),

    MatrixU8x2x2(u8, 2, 2, 4),
    MatrixU8x3x3(u8, 3, 3, 9),
    MatrixU8x4x4(u8, 4, 4, 16),

    MatrixI16x2x2(i16, 2, 2, 4),
    MatrixI16x3x3(i16, 3, 3, 9),
    MatrixI16x4x4(i16, 4, 4, 16),

    MatrixU16x2x2(u16, 2, 2, 4),
    MatrixU16x3x3(u16, 3, 3, 9),
    MatrixU16x4x4(u16, 4, 4, 16),

    MatrixI32x2x2(i32, 2, 2, 4),
    MatrixI32x3x3(i32, 3, 3, 9),
    MatrixI32x4x4(i32, 4, 4, 16),

    MatrixU32x2x2(u32, 2, 2, 4),
    MatrixU32x3x3(u32, 3, 3, 9),
    MatrixU32x4x4(u32, 4, 4, 16),

    MatrixI64x2x2(i64, 2, 2, 4),
    MatrixI64x3x3(i64, 3, 3, 9),
    MatrixI64x4x4(i64, 4, 4, 16),

    MatrixU64x2x2(u64, 2, 2, 4),
    MatrixU64x3x3(u64, 3, 3, 9),
    MatrixU64x4x4(u64, 4, 4, 16),

    MatrixF32x2x2(f32, 2, 2, 4),
    MatrixF32x3x3(f32, 3, 3, 9),
    MatrixF32x4x4(f32, 4, 4, 16),

    MatrixF64x2x2(f64, 2, 2, 4),
    MatrixF64x3x3(f64, 3, 3, 9),
    MatrixF64x4x4(f64, 4, 4, 16),
  }
}

impl<T, const LINE: usize, const COL: usize, const N: usize>
  Matrix<T, LINE, COL, N>
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

impl<T, const LINE: usize, const COL: usize, const N: usize> FromInner
  for Matrix<T, LINE, COL, N>
where T: SimdElement
{
  #[inline]
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner }
  }
}
