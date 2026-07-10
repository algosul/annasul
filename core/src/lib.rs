#![cfg_attr(not(feature = "std"), no_std)]
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
