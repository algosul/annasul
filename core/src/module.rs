use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;

/// Module Manager
#[derive(Debug)]
pub struct ModuleManager
{
  modules: HashMap<ModuleID, ()>,
}

/// Module ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModuleID(usize);

/// Module Private ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModulePrivateID(Uuid);

impl ModuleManager
{
  pub fn new() -> Result<Self>
  {
    Ok(Self { modules: HashMap::new() })
  }
}
