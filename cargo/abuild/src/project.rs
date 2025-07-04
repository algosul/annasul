use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{
    profile::Profile,
    utils::{Builder, HasBuilder},
};
/// ```
/// # use std::sync::Arc;
/// use abuild::{
///     profile::{DevProfile, ReleaseProfile},
///     project::Project,
///     utils::{Builder, HasBuilder},
/// };
/// Project::builder()
///     .profile("debug", Arc::new(DevProfile::default()))
///     .profile("release", Arc::new(ReleaseProfile::default()))
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Project {
    commands: HashMap<String, Arc<dyn ProjectCommand>>,
    profiles: HashMap<String, Arc<dyn Profile>>,
}
#[derive(Debug, Clone)]
pub struct ProjectBuilder {
    commands: HashMap<String, Arc<dyn ProjectCommand>>,
    profiles: HashMap<String, Arc<dyn Profile>>,
}
pub trait ProjectCommand: Debug {
    fn name(&self) -> String;
    fn try_do(
        &self, project: &mut Project, args: &str,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;
    fn try_undo(
        &self, project: &mut Project, cache: &mut dyn std::any::Any, args: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn try_redo(
        &self, project: &mut Project, cache: &mut dyn std::any::Any, args: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
impl Builder for ProjectBuilder {
    type Error = ();
    type Output = Project;

    fn new() -> Self {
        Self { commands: HashMap::new(), profiles: HashMap::new() }
    }

    fn build(&self) -> Result<Self::Output, Self::Error> {
        Ok(Project {
            commands: self.commands.clone(),
            profiles: self.profiles.clone(),
        })
    }
}
impl HasBuilder for Project {
    type Builder = ProjectBuilder;
}
impl ProjectBuilder {
    pub fn command(&mut self, command: Arc<dyn ProjectCommand>) -> &mut Self {
        self.commands.insert(command.name(), command);
        self
    }

    pub fn commands(
        &mut self, commands: &[Arc<dyn ProjectCommand>],
    ) -> &mut Self {
        for command in commands {
            self.command(command.clone());
        }
        self
    }

    pub fn profile(
        &mut self, name: impl Into<String>, profile: Arc<dyn Profile>,
    ) -> &mut Self {
        self.profiles.insert(name.into(), profile);
        self
    }

    pub fn profiles(
        &mut self, profiles: &[(String, Arc<dyn Profile>)],
    ) -> &mut Self {
        for (name, profile) in profiles {
            self.profile(name.clone(), profile.clone());
        }
        self
    }
}
