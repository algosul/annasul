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
//! # commands:
//! > > ![feature] init/create/remove (workspace|project|profile)
//! > > + ![note] init profile: unsupported yet.
//! > > + ![note] init/create: The workspace directory must be empty.
//! > > + ![note] create/remove profile: The current folder must be a (workspace|project), or the (-w|-j) option must be provided.
//! > > ```shell
//! > > $ abuild init
//! > > workspace "<current_directory>" was initialized successfully.
//! > > $ abuild create
//! > > workspace "<workspace_name>" was created successfully.
//! > > $ abuild remove
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
//! > > ![feature] build/clean (workspace|project|profile)
//! > > ```shell
//! > > $ abuild build
//! > > building...
//! > > ... # output of the build process
//! > > building finished.
//! > > $ abuild clean
//! > > cleaning...
//! > > ... # output of the clean process
//! > > cleaning finished.
//! > > ```
//! >
//! > > ![feature] run (workspace|project|profile)
//! > > ```shell
//! > > $ abuild run
//! > > ... # output of the build process (if not already built)
//! > > running...
//! > > ... # output of the run process
//! > > the program is exited with code '<exit_code>'.
//! > > ```
//! >
//! > > ![feature] rebuild = clean \& build (workspace|project|profile)
//! > > ```shell
//! > > $ abuild rebuild
//! > > ... # output of the clean process
//! > > ... # output of the build process
//! > > ```
//! >
//! > > ![feature] set/unset (workspace|project|profile)
//! > > ```shell
//! > > $ abuild set -w . config.author "your_name"
//! > > the workspace '<workspace_name>' config "author" was set to "your_name".
//! > > $ abuild set -j my_project config.version "1.0.0"
//! > > the project '<project_name>' config "version" was set to "1.0.0".
//! > > ```
//! >
//!
//! # Options:
//! - `-w, --workspace <workspace_path>`: set the workspace directory.
//! - `-j, --project <project_name>`: set the project name.
//! - `-p, --profile <profile_name>`: set the profile name.
//! - `-b, --binary <binary_name>`: set the binary name to build or run.
//! - `-a, --args <args>`: set the arguments to pass to the binary.
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
