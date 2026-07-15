use std::simd::{Simd, SimdElement};

use algosul_core::wrapper::{Wrapper, WrapperOwned};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "unstable-portable_simd"))]
compile_error!("no feature 'unstable-portable_simd'");

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color<T, const N: usize>
where T: SimdElement
{
  inner: Simd<T, N>,
}

crate::type_defines! {
  Color {
    ColorI8x2(i8, 2),
    ColorI8x3(i8, 3),
    ColorI8x4(i8, 4),
    ColorU8x2(u8, 2),
    ColorU8x3(u8, 3),
    ColorU8x4(u8, 4),
    ColorI16x2(i16, 2),
    ColorI16x3(i16, 3),
    ColorI16x4(i16, 4),
    ColorU16x2(u16, 2),
    ColorU16x3(u16, 3),
    ColorU16x4(u16, 4),
    ColorI32x2(i32, 2),
    ColorI32x3(i32, 3),
    ColorI32x4(i32, 4),
    ColorU32x2(u32, 2),
    ColorU32x3(u32, 3),
    ColorU32x4(u32, 4),
    ColorI64x2(i64, 2),
    ColorI64x3(i64, 3),
    ColorI64x4(i64, 4),
    ColorU64x2(u64, 2),
    ColorU64x3(u64, 3),
    ColorU64x4(u64, 4),
    ColorF32x2(f32, 2),
    ColorF32x3(f32, 3),
    ColorF32x4(f32, 4),
    ColorF64x2(f64, 2),
    ColorF64x3(f64, 3),
    ColorF64x4(f64, 4),
  }
}

impl<T, const N: usize> Wrapper<Simd<T, N>> for Color<T, N>
where T: SimdElement
{
  fn inner(&self) -> &Simd<T, N>
  {
    &self.inner
  }

  fn inner_mut(&mut self) -> &mut Simd<T, N>
  {
    &mut self.inner
  }
}

impl<T, const N: usize> WrapperOwned<Simd<T, N>> for Color<T, N>
where T: SimdElement
{
  fn into_inner(self) -> Simd<T, N>
  {
    self.inner
  }
}

crate::impl_element_getter! {
  Color {
    r: 1,
    g: 2,
    b: 3,
    a: 4,
  }
}
