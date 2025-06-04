// Copyright (c) 2025 air (https://yuanair.github.io).
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License only.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//!
//! > > ![feature] init/create/remove workspace
//! > > + ![note] init/create: The workspace directory must be empty.
//! > > ```shell
//! > > $ abuild init (-w|--workspace)
//! > > workspace "<current_directory>" was initialized successfully.
//! > > $ abuild create (-w|--workspace) <workspace_name>
//! > > workspace "<workspace_name>" was created successfully.
//! > > $ abuild remove (-w|--workspace) <workspace_name>
//! > > workspace "<workspace_name>" was removed successfully.
//! > > ```
//! >
//! > > ![feature] undo/redo
//! > > ```shell
//! > > $ abuild undo
//! > > the last operation is '<last_operation>'
//! > > ... # output of the undo operation
//! > > $ abuild redo
//! > > the last operation is '<last_operation>'
//! > > ... # output of the redo operation
//! > > ```
//! >
//! > > ![feature] init/create/remove project (workspace|none)
//! > > + ![note] The current folder must be a workspace, or the -w option must be provided.
//! > > ```shell
//! > > $ abuild init (-j|--project) ((-w|--workspace) <workspace_path>)?
//! > > project "<current_directory>" was initialized successfully.
//! > > it is added to workspace '<current_workspace>'.
//! > > $ abuild create (-j|--project) <project_name> ((-w|--workspace) <workspace_path>)?
//! > > project "<project_name>" was created successfully.
//! > > it is added to workspace '<current_workspace>'.
//! > > $ abuild remove (-j|--project) <project_name> ((-w|--workspace) <workspace_path>)?
//! > > project "<project_name>" was removed successfully.
//! > > it is removed from workspace '<current_workspace>'.
//! > > ```
//! >
//! > > ![feature] create/remove profile (workspace|project)
//! > > + ![note] The current folder must be a (workspace|project), or the (-w|-j) option must be provided.
//! > > ```shell
//! > > $ abuild create (-p|--profile) <profile_name> ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > profile "<profile_name>" was created successfully.
//! > > it is added to (project '<current_project>'|workspace '<current_workspace>').
//! > > $ abuild remove (-p|--profile) <profile_name> ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > profile "<profile_name>" was removed successfully.
//! > > it is removed from (project '<current_project>'|workspace '<current_workspace>').
//! > > ```
//! >
//! > > ![feature] build/clean (workspace|project|profile)
//! > > ```shell
//! > > $ abuild build ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > building...
//! > > ... # output of the build process
//! > > building finished.
//! > > $ abuild clean ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > cleaning...
//! > > ... # output of the clean process
//! > > cleaning finished.
//! > > ```
//! >
//! > > ![feature] run (workspace|project|profile)
//! > > ```shell
//! > > $ abuild run ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > ... # output of the build process (if not already built)
//! > > running...
//! > > ... # output of the run process
//! > > the program is exited with code '<exit_code>'.
//! > > ```
//! >
//! > > ![feature] rebuild = clean \& build (workspace|project|profile)
//! > > ```shell
//! > > $ abuild rebuild ((-p|--profile) <profile_name>)? ((-j|--project) <project_name>)? ((-w|--workspace) <workspace_path>)?
//! > > ... # output of the clean process
//! > > ... # output of the build process
//! > > ```
//!
//! [note]: https://img.shields.io/badge/note-orange.svg?color=ddbb00
//!
//! [bug]: https://img.shields.io/badge/bug-red.svg
//!
//! [feature]: https://img.shields.io/badge/feature-orange.svg
//!

use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
    #[clap(short = 'w', long, help = "set the workspace directory")]
    workspace: Option<PathBuf>,
    #[clap(short = 'j', long, help = "set the project name")]
    project: Option<String>,
    #[clap(short = 'p', long, help = "set the profile name")]
    profile: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// initialize a new workspace or project
    #[clap(name = "init")]
    Init {},
    /// create a new workspace or project or profile
    #[clap(name = "create")]
    Create {},
    /// remove a workspace or project or profile
    #[clap(name = "remove")]
    Remove {},
    /// undo the last command in workspace or project or profile
    #[clap(name = "undo")]
    Undo,
    /// redo the last command in workspace or project or profile
    #[clap(name = "redo")]
    Redo,
    /// build workspace or project or profile
    #[clap(name = "build")]
    Build {
        #[clap(short, long, help = "the binary to build(default: build all binaries)")]
        binary: Option<String>,
    },
    /// clean workspace or project or profile
    #[clap(name = "clean")]
    Clean {},
    /// run binary in a workspace or project or profile
    #[clap(name = "run")]
    Run {
        #[clap(short, long, help = "the binary to run(default: run all binaries)")]
        binary: Option<String>,
        #[clap(help = "the arguments to pass to the binary")]
        args: Vec<OsString>,
    },
    /// rebuild workspace or project or profile
    #[clap(name = "rebuild")]
    Rebuild {
        #[clap(
            short,
            long,
            help = "the binary to rebuild(default: rebuild all binaries)"
        )]
        binary: Option<String>,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
