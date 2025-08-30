#![feature(string_from_utf8_lossy_owned)]
//! #Example
//! ```
//! # use algosul::{
//! #   app::{
//! #     AppOper,
//! #     apps::rust::{Error, Rustup},
//! #   },
//! #   process::Process,
//! # };
//! async {
//!   let mut installer = Rustup::installer().await?;
//!   installer.on_status_changed(|status| {
//!     println!("status: {status:?}");
//!     Ok(())
//!   })?;
//!   let rustup = installer.run().await?;
//!   println!("rustup installed: {rustup:?}");
//!   Ok::<(), Error>(())
//! };
//! ```
#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "asset")]
pub mod asset;
#[cfg(feature = "codegen")]
pub mod codegen;
#[cfg(feature = "deps")]
pub mod deps;
#[cfg(feature = "macros")]
pub mod macros;
#[cfg(not(feature = "macros"))]
pub(crate) mod macros;
#[cfg(feature = "process")]
pub mod process;
#[cfg(not(feature = "process"))]
pub(crate) mod process;
pub mod utils;
#[cfg(feature = "math")]
pub use algosul_math as math;

#[cfg(feature = "langs")]
pub mod langs;
