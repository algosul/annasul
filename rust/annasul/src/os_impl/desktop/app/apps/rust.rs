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
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[derive(Debug)]
pub struct Rustup {
    home_path: PathBuf,
}

#[derive(Debug)]
pub enum Error {
    Unsupported(String),
    IOError(std::io::Error),
    TaskJoinError(tokio::task::JoinError),
    InnerError(String),
    Failed {
        exit_status: ExitStatus,
        stdin: String,
        stdout: String,
        stderr: String,
    },
    FailedToGetHomeDir,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unsupported(info) => f.write_fmt(format_args!("Unsupported: {}", info)),
            Error::IOError(e) => f.write_fmt(format_args!("IO error: {}", e)),
            Error::TaskJoinError(e) => f.write_fmt(format_args!("Task join error: {}", e)),
            Error::InnerError(info) => f.write_fmt(format_args!("Inner error: {}", info)),
            Error::Failed {
                exit_status,
                stdin,
                stdout,
                stderr,
            } => f.write_fmt(format_args!(
                "Failed:\n - exit status: {}\n - stdin:\n{}\n\n - stdout:\n{}\n\n - stderr:\n{}",
                exit_status, stdin, stdout, stderr
            )),
            Error::FailedToGetHomeDir => f.write_fmt(format_args!("failed to get HOME dir")),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Unsupported(_) => None,
            Error::IOError(e) => Some(e),
            Error::TaskJoinError(e) => Some(e),
            Error::InnerError(_) => None,
            Error::Failed { .. } => None,
            Error::FailedToGetHomeDir => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Default, Debug)]
pub enum Toolchain {
    #[default]
    Stable,
    Beta,
    Nightly,
    None,
}
#[derive(Default, Debug)]
pub enum HostTriple {
    #[default]
    Host,
    /// e.g. x86_64-unknown-linux-gnu
    Target(String),
}
#[derive(Default, Debug)]
pub enum Profile {
    Minimal,
    #[default]
    Default,
    Complete,
}
#[derive(Debug)]
pub struct InstallCustomInfo {
    default_host_triple: HostTriple,
    default_toolchain: Toolchain,
    profile: Profile,
    modify_path_variable: bool,
}
#[derive(Default, Debug)]
pub enum InstallInfo {
    #[default]
    Default,
    Custom(InstallCustomInfo),
}
impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Toolchain::Stable => f.write_str("stable"),
            Toolchain::Beta => f.write_str("beta"),
            Toolchain::Nightly => f.write_str("nightly"),
            Toolchain::None => f.write_str("none"),
        }
    }
}
impl Display for HostTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HostTriple::Host => f.write_str("host"),
            HostTriple::Target(target) => f.write_str(target),
        }
    }
}
impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Profile::Minimal => f.write_str("minimal"),
            Profile::Default => f.write_str("default"),
            Profile::Complete => f.write_str("complete"),
        }
    }
}
impl Default for InstallCustomInfo {
    fn default() -> Self {
        Self {
            default_host_triple: Default::default(),
            default_toolchain: Default::default(),
            profile: Default::default(),
            modify_path_variable: true,
        }
    }
}
impl<'a> crate::app::AppInfo<'a> for Rustup {
    type Error = Error;
    async fn name(&self) -> Cow<'a, str> {
        Cow::Borrowed("rustup")
    }

    async fn license(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn license_file(&self) -> Result<Option<Cow<'a, Path>>> {
        todo!()
    }

    async fn description(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn documentation(&self) -> Result<Option<Cow<'a, str>>> {
        todo!()
    }

    async fn home_page(&self) -> Result<Option<Cow<'a, str>>> {
        Ok(Some(Cow::Borrowed("https://rustup.rs/")))
    }

    async fn version(&self) -> Result<Cow<'a, str>> {
        todo!()
    }
}
impl<'a> crate::app::AppPath<'_> for Rustup {
    type Error = Error;
    async fn home_path(&self) -> std::result::Result<Option<Cow<'a, Path>>, Self::Error> {
        Ok(Some((&self.home_path).into()))
    }
    async fn bin_path(&self) -> std::result::Result<Option<Cow<'a, Path>>, Self::Error> {
        todo!()
    }
}
impl crate::app::AppOper for Rustup {
    type Error = Error;
    type InstallInfo = InstallInfo;
    type ReinstallInfo = ();
    type RemoveInfo = ();
    type UpdateInfo = ();
    async fn install(info: Self::InstallInfo) -> Result<Self> {
        // curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        if cfg!(unix) {
            let mut command = Command::new("curl")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .args([
                    "--proto",
                    "'=https'",
                    "--tlsv1.2",
                    "-sSf",
                    "https://sh.rustup.rs",
                ])
                .spawn()
                .map_err(Error::IOError)?;

            let (mut stdin, mut stdout, mut stderr) = (
                command.stdin.take().ok_or(Error::InnerError(
                    "Command 'curl': stdin is not available".to_string(),
                ))?,
                command.stdout.take().ok_or(Error::InnerError(
                    "Command 'curl': stdout is not available".to_string(),
                ))?,
                command.stderr.take().ok_or(Error::InnerError(
                    "Command 'curl': stderr is not available".to_string(),
                ))?,
            );

            let (mut stdout_buf, mut stderr_buf) = (Vec::new(), Vec::new());

            let stdin_buf: Cow<'static, str> = match info {
                InstallInfo::Default => "1\n".into(),
                InstallInfo::Custom(InstallCustomInfo {
                    default_host_triple,
                    default_toolchain,
                    profile,
                    modify_path_variable,
                }) => format!(
                    "2\n{}\n{}\n{}\n{}\n1\n",
                    default_host_triple,
                    default_toolchain,
                    profile,
                    if modify_path_variable { "Y" } else { "n" }
                )
                .into(),
            };

            let stdin_write_handle = {
                let stdin_buf = &stdin_buf;
                tokio::spawn(async move {
                    stdin.write_all(stdin_buf.as_bytes()).await?;
                    stdin.shutdown().await?;
                    Ok::<_, std::io::Error>(())
                })
            };

            let stdout_read_handle =
                tokio::spawn(async move { stdout.read_to_end(&mut stdout_buf).await });

            let stderr_read_handle =
                tokio::spawn(async move { stderr.read_to_end(&mut stderr_buf).await });

            let (stdin_write_result, stdout_read_handle, stderr_read_handle) =
                tokio::try_join!(stdin_write_handle, stdout_read_handle, stderr_read_handle)
                    .map_err(Error::TaskJoinError)?;
            stdin_write_result.map_err(Error::IOError)?;
            stdout_read_handle.map_err(Error::IOError)?;
            stderr_read_handle.map_err(Error::IOError)?;

            let exit_status = command.wait().await.map_err(Error::IOError)?;
            if exit_status.success() {
                Ok(Self {
                    // ~/.config
                    home_path: std::env::home_dir()
                        .ok_or(Error::FailedToGetHomeDir)?
                        .join(".config"),
                })
            } else {
                Err(Error::Failed {
                    exit_status,
                    stdin: stdin_buf.into_owned(),
                    stdout: OsStr::from_bytes(&stdout_buf)
                        .to_string_lossy()
                        .into_owned(),
                    stderr: OsStr::from_bytes(&stderr_buf)
                        .to_string_lossy()
                        .into_owned(),
                })
            }
        } else if cfg!(windows) {
            todo!()
        } else {
            Err(Error::Unsupported(format!(
                "unsupported platform '{}'",
                std::env::consts::OS
            )))
        }
    }

    async fn reinstall(self, info: Self::ReinstallInfo) -> Result<Self> {
        todo!()
    }

    async fn remove(self, info: Self::RemoveInfo) -> Result<()> {
        todo!()
    }

    async fn update(self, info: Self::UpdateInfo) -> Result<Self> {
        todo!()
    }
}
