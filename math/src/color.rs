use std::{
  marker::PhantomData,
  simd::{Simd, SimdElement},
};

use algosul_core::wrapper::prelude::*;
use algosul_derive::Wrapper;

use crate::color::space::{Bgr, Bgra, ColorSpace, G, Ga, Hsv, Hsva, Rgb, Rgba};

#[cfg(not(feature = "std"))]
compile_error!("no feature 'std'");

#[cfg(not(feature = "__feature-portable_simd"))]
compile_error!("no feature '__feature-portable_simd'");

#[cfg(not(feature = "__feature-algosul-wrapper"))]
compile_error!("no feature '__feature-algosul-wrapper'");

#[cfg(not(feature = "__feature-const_trait_impl"))]
compile_error!("no feature '__feature-const_trait_impl'");

pub mod space;
pub mod traits;

#[derive(
  Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Wrapper,
)]
pub struct Color<T, S, const N: usize>
where
  T: SimdElement,
  S: ColorSpace<N>,
{
  #[wrapper(inner)]
  inner:   Simd<T, N>,
  _marker: PhantomData<S>,
}

crate::type_defines! {
  Color {

    RgbI8(i8, Rgb, 3),
    RgbaI8(i8, Rgba, 4),
    BgrI8(i8, Bgr, 3),
    BgraI8(i8, Bgra, 4),
    HsvI8(i8, Hsv, 3),
    HsvaI8(i8, Hsva, 4),
    GI8(i8, G, 1),
    GaI8(i8, Ga, 2),

    RgbU8(u8, Rgb, 3),
    RgbaU8(u8, Rgba, 4),
    BgrU8(u8, Bgr, 3),
    BgraU8(u8, Bgra, 4),
    HsvU8(u8, Hsv, 3),
    HsvaU8(u8, Hsva, 4),
    GU8(u8, G, 1),
    GaU8(u8, Ga, 2),

    RgbI16(i16, Rgb, 3),
    RgbaI16(i16, Rgba, 4),
    BgrI16(i16, Bgr, 3),
    BgraI16(i16, Bgra, 4),
    HsvI16(i16, Hsv, 3),
    HsvaI16(i16, Hsva, 4),
    GI16(i16, G, 1),
    GaI16(i16, Ga, 2),

    RgbU16(u16, Rgb, 3),
    RgbaU16(u16, Rgba, 4),
    BgrU16(u16, Bgr, 3),
    BgraU16(u16, Bgra, 4),
    HsvU16(u16, Hsv, 3),
    HsvaU16(u16, Hsva, 4),
    GU16(u16, G, 1),
    GaU16(u16, Ga, 2),

    RgbI32(i32, Rgb, 3),
    RgbaI32(i32, Rgba, 4),
    BgrI32(i32, Bgr, 3),
    BgraI32(i32, Bgra, 4),
    HsvI32(i32, Hsv, 3),
    HsvaI32(i32, Hsva, 4),
    GI32(i32, G, 1),
    GaI32(i32, Ga, 2),

    RgbU32(u32, Rgb, 3),
    RgbaU32(u32, Rgba, 4),
    BgrU32(u32, Bgr, 3),
    BgraU32(u32, Bgra, 4),
    HsvU32(u32, Hsv, 3),
    HsvaU32(u32, Hsva, 4),
    GU32(u32, G, 1),
    GaU32(u32, Ga, 2),

    RgbI64(i64, Rgb, 3),
    RgbaI64(i64, Rgba, 4),
    BgrI64(i64, Bgr, 3),
    BgraI64(i64, Bgra, 4),
    HsvI64(i64, Hsv, 3),
    HsvaI64(i64, Hsva, 4),
    GI64(i64, G, 1),
    GaI64(i64, Ga, 2),

    RgbU64(u64, Rgb, 3),
    RgbaU64(u64, Rgba, 4),
    BgrU64(u64, Bgr, 3),
    BgraU64(u64, Bgra, 4),
    HsvU64(u64, Hsv, 3),
    HsvaU64(u64, Hsva, 4),
    GU64(u64, G, 1),
    GaU64(u64, Ga, 2),

    RgbF32(f32, Rgb, 3),
    RgbaF32(f32, Rgba, 4),
    BgrF32(f32, Bgr, 3),
    BgraF32(f32, Bgra, 4),
    HsvF32(f32, Hsv, 3),
    HsvaF32(f32, Hsva, 4),
    GF32(f32, G, 1),
    GaF32(f32, Ga, 2),

    RgbF64(f64, Rgb, 3),
    RgbaF64(f64, Rgba, 4),
    BgrF64(f64, Bgr, 3),
    BgraF64(f64, Bgra, 4),
    HsvF64(f64, Hsv, 3),
    HsvaF64(f64, Hsva, 4),
    GF64(f64, G, 1),
    GaF64(f64, Ga, 2),
  }
}

impl<T, S, const N: usize> Color<T, S, N>
where
  T: SimdElement,
  S: ColorSpace<N>,
{
  #[inline]
  pub const fn from_array(array: [T; N]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_array(array))
  }

  #[inline]
  pub const fn from_slice(slice: &[T]) -> Self
  {
    Self::from_inner(<Self as Wrapper>::Inner::from_slice(slice))
  }
}
const impl<T, S, const N: usize> FromInner for Color<T, S, N>
where
  T: SimdElement,
  S: ColorSpace<N>,
{
  #[inline]
  fn from_inner(inner: Self::Inner) -> Self
  {
    Self { inner, _marker: PhantomData }
  }
}

macro_rules! impl_consts_for_all {
  ($ty:ty, $zero:literal) => {
    impl<S, const N: usize> Color<$ty, S, N>
    where
      S: ColorSpace<N>,
    {
      pub const ZERO: Self = Self::from_array([$zero; N]);
    }
  };
  ($($ty:ty, $zero:literal)+) => {
    $(impl_consts_for_all!($ty, $zero);)+
  };
}
impl_consts_for_all! {
  i8, 0i8
  u8, 0u8
  i16, 0i16
  u16, 0u16
  i32, 0i32
  u32, 0u32
  i64, 0i64
  u64, 0u64
}
macro_rules! impl_base_for_all {
  ($ty:ty, $zero:literal, $one:literal) => {
    impl Color<$ty, Rgb, 3>
    {
      pub const R: Self = Self::from_array([$one, $zero, $zero]);
      pub const G: Self = Self::from_array([$zero, $one, $zero]);
      pub const B: Self = Self::from_array([$zero, $zero, $one]);
    }
    impl Color<$ty, Rgba, 4>
    {
      pub const R: Self = Self::from_array([$one, $zero, $zero, $zero]);
      pub const G: Self = Self::from_array([$zero, $one, $zero, $zero]);
      pub const B: Self = Self::from_array([$zero, $zero, $one, $zero]);
      pub const RA: Self = Self::from_array([$one, $zero, $zero, $one]);
      pub const GA: Self = Self::from_array([$zero, $one, $zero, $one]);
      pub const BA: Self = Self::from_array([$zero, $zero, $one, $one]);
      pub const A: Self = Self::from_array([$zero, $zero, $zero, $one]);
    }
  };
  ($($ty:ty, $zero:literal, $one:literal)+) => {
    $(impl_base_for_all!($ty, $zero, $one);)+
  };
}

impl_base_for_all! {
  i8, 0i8, 1i8
  u8, 0u8, 1u8
  i16, 0i16, 1i16
  u16, 0u16, 1u16
  i32, 0i32, 1i32
  u32, 0u32, 1u32
  i64, 0i64, 1i64
  u64, 0u64, 1u64
}
