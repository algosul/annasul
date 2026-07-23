#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "__feature-const_trait_impl", feature(const_trait_impl))]
extern crate alloc;
extern crate core;

#[cfg(feature = "i18n")]
pub mod i18n;

#[cfg(feature = "module")]
pub mod module;

#[cfg(feature = "message")]
pub mod message;

#[cfg(feature = "util")]
pub mod util;

#[cfg(feature = "wrapper")]
pub mod wrapper;
