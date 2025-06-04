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
        #[clap(short, long, help = "the binary to rebuild(default: rebuild all binaries)")]
        binary: Option<String>,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
