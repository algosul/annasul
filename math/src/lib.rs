#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable-portable_simd", feature(portable_simd))]
extern crate core;

#[cfg(feature = "std-unstable-vector")]
pub mod vector;

#[cfg(feature = "std-unstable-color")]
pub mod color;

#[cfg(feature = "unstable-portable_simd")]
pub(crate) mod simd;
