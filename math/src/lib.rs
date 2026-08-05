#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "__feature-portable_simd", feature(portable_simd))]
#![cfg_attr(feature = "__feature-const_trait_impl", feature(const_trait_impl))]
#![cfg_attr(feature = "unstable-f16", feature(f16))]
extern crate core;

#[cfg(feature = "std-unstable-matrix")]
pub mod matrix;

#[cfg(feature = "std-unstable-vector")]
pub mod vector;

#[cfg(feature = "std-unstable-color")]
pub mod color;

#[cfg(feature = "std-unstable-shape")]
pub mod shape;

pub mod traits;

pub use ::num;

#[cfg(feature = "__feature-portable_simd")]
pub(crate) mod simd;
