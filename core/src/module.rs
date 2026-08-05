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
pub enum Error
{
  #[error("Failed to new Module ID")]
  FailedToNewModuleID,
  #[error("Non-existent Module ID")]
  NonExistentModuleID,
}

pub type Result<T> = core::result::Result<T, Error>;

/// Module Manager
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ModuleManager
{
  id:      ModuleID,
  modules: HashMap<ModuleID, ModuleWrapper>,
}

/// Module ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ModuleID(usize);

/// Module Data
pub trait Module: 'static
{
  fn name(&self) -> &str;
}

pub struct ModuleWrapper
{
  inner: Box<dyn Module>,
}

impl ModuleID
{
  const MAX_MODULE_ID: usize = u32::MAX as usize;

  fn first() -> Self
  {
    Self(0)
  }

  fn next(&mut self) -> Self
  {
    if self.0 == Self::MAX_MODULE_ID
    {
      panic!("Module ID exhausted");
    }
    let old = *self;
    *self = Self(self.0 + 1);
    old
  }
}

impl ModuleWrapper
{
  fn new(inner: impl Module) -> Self
  {
    Self { inner: Box::new(inner) }
  }
}

impl AsRef<dyn Module> for ModuleWrapper
{
  fn as_ref(&self) -> &dyn Module
  {
    self.inner.as_ref()
  }
}

impl AsMut<dyn Module> for ModuleWrapper
{
  fn as_mut(&mut self) -> &mut dyn Module
  {
    self.inner.as_mut()
  }
}

impl ModuleManager
{
  pub fn new() -> Result<Self>
  {
    Ok(Self { id: ModuleID::first(), modules: HashMap::new() })
  }

  pub fn register_module(&mut self, module: impl Module) -> ModuleID
  {
    let id = self.id.next();
    self.modules.insert(id, ModuleWrapper::new(module));
    id
  }

  pub fn unregister_module(&mut self, module: ModuleID) -> Result<()>
  {
    let _ = self.modules.remove(&module).ok_or(Error::NonExistentModuleID)?;
    Ok(())
  }

  pub fn get(&self, id: ModuleID) -> Option<&ModuleWrapper>
  {
    self.modules.get(&id)
  }

  pub fn get_mut(&mut self, id: ModuleID) -> Option<&mut ModuleWrapper>
  {
    self.modules.get_mut(&id)
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

#[cfg(test)]
mod tests
{
  use super::*;

  #[derive(Clone)]
  struct DummyModule(&'static str);

  impl Module for DummyModule
  {
    fn name(&self) -> &str
    {
      self.0
    }
  }

  #[test]
  fn register_and_get()
  {
    let mut manager = ModuleManager::new().unwrap();
    let id1 = manager.register_module(DummyModule("alpha"));
    let id2 = manager.register_module(DummyModule("beta"));
    // IDs must be strictly increasing and mutually distinct
    assert_ne!(id1, id2);

    let m = manager.get(id1).expect("module 1 registered");
    assert_eq!(m.as_ref().name(), "alpha");
    let m = manager.get(id2).expect("module 2 registered");
    assert_eq!(m.as_ref().name(), "beta");
  }

  #[test]
  fn get_returns_none_for_unknown()
  {
    let mut manager = ModuleManager::new().unwrap();
    let id = manager.register_module(DummyModule("x"));
    assert!(manager.get(id).is_some());
    // An unregistered ID
    assert!(manager.get(ModuleID(std::usize::MAX)).is_none());
  }

  #[test]
  fn unregister_removes_module()
  {
    let mut manager = ModuleManager::new().unwrap();
    let id = manager.register_module(DummyModule("x"));
    assert!(manager.get(id).is_some());
    assert!(manager.unregister_module(id).is_ok());
    assert!(manager.get(id).is_none());
    // Unregistering again errors
    assert!(matches!(manager.unregister_module(id), Err(Error::NonExistentModuleID)));
  }

  #[test]
  fn get_mut_can_mutate()
  {
    let mut manager = ModuleManager::new().unwrap();
    let id = manager.register_module(DummyModule("before"));
    {
      // get_mut only provides a mutable reference; DummyModule has no mutable
      // state, so this only exercises the return / unwind path
      let _m = manager.get_mut(id).expect("module present");
    }
    assert!(manager.get(id).is_some());
  }
}
