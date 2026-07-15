use std::simd::{Simd, SimdElement};

use algosul_core::wrapper::{Wrapper, WrapperOwned};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "unstable-portable_simd"))]
compile_error!("no feature 'unstable-portable_simd'");

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

impl<T, const N: usize> Wrapper<Simd<T, N>> for Vector<T, N>
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

impl<T, const N: usize> WrapperOwned<Simd<T, N>> for Vector<T, N>
where T: SimdElement
{
  fn into_inner(self) -> Simd<T, N>
  {
    self.inner
  }
}

crate::impl_element_getter! {
  Vector {
    x: 1,
    y: 2,
    z: 3,
    w: 4,
  }
}
