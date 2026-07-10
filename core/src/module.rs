//! # Modules
//!
//! ## Module Manager
//!
//! + `register_module`
//!   + return:
//!     + [ModuleID]

use core::fmt::{Debug, Formatter};
use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;

/// Module Manager
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ModuleManager
{
  modules: HashMap<ModuleID, Box<ModuleWrapper>>,
}

/// Module ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModuleID(usize);

/// Module Data
pub trait Module
{
  fn name(&self) -> &str;
}

struct ModuleWrapper
{
  inner: dyn Module,
}

impl ModuleManager
{
  pub fn new() -> Result<Self>
  {
    Ok(Self { modules: HashMap::new() })
  }
}

#[cfg(debug_assertions)]
impl Debug for ModuleWrapper
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
  {
    f.debug_struct(stringify!(Module))
      .field("name", &self.inner.name())
      .finish()
  }
}
