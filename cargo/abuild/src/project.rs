use std::{collections::HashMap, fmt::Debug, sync::Arc};

use crate::{
    profile::Profile,
    utils::{Builder, HasBuilder},
};
/// ```
/// # use abuild::project::Project;
/// use abuild::utils::{Builder, HasBuilder};
/// Project::builder().build().unwrap();
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

    fn build(self) -> Result<Self::Output, Self::Error> {
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
    fn command(&mut self, command: Arc<dyn ProjectCommand>) -> &mut Self {
        self.commands.insert(command.name(), command);
        self
    }

    fn commands(&mut self, commands: &[Arc<dyn ProjectCommand>]) -> &mut Self {
        for command in commands {
            self.command(command.clone());
        }
        self
    }

    fn profile(
        &mut self, name: String, profile: Arc<dyn Profile>,
    ) -> &mut Self {
        self.profiles.insert(name, profile);
        self
    }

    fn profiles(
        &mut self, profiles: &[(String, Arc<dyn Profile>)],
    ) -> &mut Self {
        for (name, profile) in profiles {
            self.profile(name.clone(), profile.clone());
        }
        self
    }
}
