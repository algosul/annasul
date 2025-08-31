//! #Example
//! ```
//! # use algosul_app::{
//! #   AppOper,
//! #   apps::rust::{Error, Rustup},
//! # };
//! # use algosul_core::process::Process;
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
pub mod asset;
pub mod langs;
pub use algosul_core::*;
