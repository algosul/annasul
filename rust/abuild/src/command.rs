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
//! > > ![feature] auto-completion (install|reinstall|uninstall) script
//! > > ```shell
//! > > $ abuild auto-complete install bash
//! > > the auto-completion script for bash was installed in '/etc/bash_completion.d/abuild' successfully.
//! > > $ abuild auto-complete reinstall zsh
//! > > the auto-completion script for zsh was installed in '/usr/local/share/zsh/site-functions/_abuild' successfully.
//! > > $ abuild auto-complete uninstall bash
//! > > the auto-completion script for bash was uninstalled from '/etc/bash_completion.d/abuild' successfully.
//! > > ```
//! >
//! > > ![feature] init/create/remove (workspace|project|profile)
//! > > + ![note] init profile: unsupported yet.
//! > > + ![note] init/create: The workspace directory must be empty.
//! > > + ![note] create/remove profile: The current folder must be a (workspace|project), or the (-w|-j) option must be provided.
//! > > ```shell
//! > > $ abuild init
//! > > workspace '<current_directory>' was initialized successfully.
//! > > $ abuild create
//! > > workspace '<workspace_name>' was created successfully.
//! > > $ abuild remove
//! > > workspace '<workspace_name>' was removed successfully.
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
//! > > the workspace '<workspace_name>' config.author was set to "your_name".
//! > > $ abuild set -j my_project config.version "1.0.0"
//! > > the project '<project_name>' config.version was set to "1.0.0".
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

use clap::{value_parser, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::aot::generate;
use clap_complete::Generator;
use io::Write;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    sub_command: SubCommand,
}

pub type Shell = clap_complete::Shell;

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
pub enum AutoCompleteCommand {
    /// install auto-completion script
    Install,
    /// reinstall auto-completion script
    Reinstall,
    /// uninstall auto-completion script
    Uninstall,
    /// output auto-completion script
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SubCommand {
    /// auto-completion script
    AutoComplete {
        #[clap(value_parser=value_parser!(AutoCompleteCommand))]
        command: AutoCompleteCommand,
        #[clap(value_parser=value_parser!(Shell))]
        shell: Shell,
    },
    /// initialize a new workspace or project
    Init {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// create a new workspace or project or profile
    Create {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// remove a workspace or project or profile
    Remove {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// undo the last command in workspace or project or profile
    Undo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// redo the last command in workspace or project or profile
    Redo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// build workspace or project or profile
    Build {
        /// the binary to build (default: build all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// clean workspace or project or profile
    Clean {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// run binary in a workspace or project or profile
    Run {
        /// the binary to run (default: run all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        /// the arguments to pass to the binary
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<OsString>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// rebuild workspace or project or profile
    Rebuild {
        /// the binary to rebuild (default: rebuild all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
}

/// Options for the scope of the command (workspace, project, profile)
#[derive(Default, Debug, Clone, PartialEq, Eq, Parser)]
pub struct ScopeOptions {
    /// set the workspace directory
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub workspace: Option<PathBuf>,
    /// set the project name
    #[clap(short = 'j', long, value_hint = ValueHint::DirPath)]
    pub project: Option<String>,
    /// set the profile name
    #[clap(short, long, value_hint = ValueHint::Unknown)]
    pub profile: Option<String>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub fn generate_completion<G: Generator>(generator: G, bin_name: &str, buf: &mut dyn Write) {
    generate(generator, &mut Cli::command(), bin_name, buf);
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        self.sub_command().run()
    }
    pub fn sub_command(&self) -> &SubCommand {
        &self.sub_command
    }
    pub fn sub_command_mut(&mut self) -> &mut SubCommand {
        &mut self.sub_command
    }
}

impl SubCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            SubCommand::AutoComplete { command, shell } => {
                let mut buffer = Vec::new();
                generate_completion(*shell, &crate::app_name(), &mut buffer);
                match command {
                    AutoCompleteCommand::Output => {
                        io::stdout().write_all(&buffer).map_err(Error::IOError)?;
                        Ok(())
                    }
                    _ => todo!(),
                }
            }
            SubCommand::Init { scope } => {
                todo!()
            }
            SubCommand::Create { scope } => {
                todo!()
            }
            SubCommand::Remove { scope } => {
                todo!()
            }
            SubCommand::Undo { scope } => todo!(),
            SubCommand::Redo { scope } => {
                todo!()
            }
            SubCommand::Build { binary, scope } => {
                todo!()
            }
            SubCommand::Clean { scope } => {
                todo!()
            }
            SubCommand::Run {
                binary,
                args,
                scope,
            } => {
                todo!()
            }
            SubCommand::Rebuild { binary, scope } => {
                todo!()
            }
        }
    }
}
