#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "__feature-portable_simd", feature(portable_simd))]
#![cfg_attr(
  feature = "__feature-generic_const_args",
  feature(min_generic_const_args, generic_const_exprs),
  allow(incomplete_features)
)]
extern crate core;

#[cfg(feature = "std-unstable-matrix")]
pub mod matrix;

#[cfg(feature = "std-unstable-vector")]
pub mod vector;

#[cfg(feature = "std-unstable-color")]
pub mod color;

#[cfg(feature = "__feature-portable_simd")]
pub(crate) mod simd;
