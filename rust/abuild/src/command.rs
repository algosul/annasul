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

use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::aot::generate;
use clap_complete::{Generator, Shell};
use io::Write;
use std::ffi::OsString;
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ShellSubCommand {
    /// Bourne Again `SHell` (bash)
    #[clap(name = "bash")]
    Bash,
    /// Elvish shell
    #[clap(name = "elvish")]
    Elvish,
    /// Friendly Interactive `SHell` (fish)
    #[clap(name = "fish")]
    Fish,
    /// `PowerShell`
    #[clap(name = "powershell")]
    PowerShell,
    /// Z `SHell` (zsh)
    #[clap(name = "zsh")]
    Zsh,
}

impl Into<Shell> for ShellSubCommand {
    fn into(self) -> Shell {
        match self {
            ShellSubCommand::Bash => Shell::Bash,
            ShellSubCommand::Elvish => Shell::Elvish,
            ShellSubCommand::Fish => Shell::Fish,
            ShellSubCommand::PowerShell => Shell::PowerShell,
            ShellSubCommand::Zsh => Shell::Zsh,
        }
    }
}

impl TryFrom<Shell> for ShellSubCommand {
    type Error = &'static str;
    fn try_from(shell: Shell) -> Result<Self, Self::Error> {
        match shell {
            Shell::Bash => Ok(ShellSubCommand::Bash),
            Shell::Elvish => Ok(ShellSubCommand::Elvish),
            Shell::Fish => Ok(ShellSubCommand::Fish),
            Shell::PowerShell => Ok(ShellSubCommand::PowerShell),
            Shell::Zsh => Ok(ShellSubCommand::Zsh),
            _ => Err("unsupported shell"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct ShellOptions {
    /// the shell to generate the completion for
    #[clap(subcommand, action = ArgAction::Set, value_parser=value_parser!(Shell))]
    shell_sub_command: ShellSubCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum AutoCompleteSubCommand {
    /// install completion script
    #[clap(name = "install")]
    Install {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// reinstall completion script
    #[clap(name = "reinstall")]
    Reinstall {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// uninstall completion script
    #[clap(name = "uninstall")]
    Uninstall {
        #[clap(flatten)]
        shell: ShellOptions,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SubCommand {
    /// auto-completion script
    #[clap(name = "auto-complete")]
    AutoComplete {
        #[command(subcommand)]
        sub_command: AutoCompleteSubCommand,
    },
    /// initialize a new workspace or project
    #[clap(name = "init")]
    Init {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// create a new workspace or project or profile
    #[clap(name = "create")]
    Create {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// remove a workspace or project or profile
    #[clap(name = "remove")]
    Remove {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// undo the last command in workspace or project or profile
    #[clap(name = "undo")]
    Undo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// redo the last command in workspace or project or profile
    #[clap(name = "redo")]
    Redo {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// build workspace or project or profile
    #[clap(name = "build")]
    Build {
        /// the binary to build(default: build all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// clean workspace or project or profile
    #[clap(name = "clean")]
    Clean {
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// run binary in a workspace or project or profile
    #[clap(name = "run")]
    Run {
        /// the binary to run(default: run all binaries)
        #[clap(short, long)]
        binary: Option<String>,
        /// the arguments to pass to the binary
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<OsString>,
        #[clap(flatten)]
        scope: ScopeOptions,
    },
    /// rebuild workspace or project or profile
    #[clap(name = "rebuild")]
    Rebuild {
        /// the binary to rebuild(default: rebuild all binaries)
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
