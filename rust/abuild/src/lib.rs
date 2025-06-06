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
//! structure of abuild workspace:
//! > workspace:
//! > > .abuild:
//! > > > config.toml
//! > >
//! >
//! > > project A:
//! > > > .abuild:
//! > > > > config.toml
//! > > >
//! > >
//! > > > src:
//! > > > > ...
//! > > >
//! > >
//! > > > rc:
//! > > > > ...
//! > > >
//! > >
//! >
//! > > project B:
//! > > > ...
//! > >
//! >
//! > > target:
//! > > > profile A:
//! > > > > build:
//! > > > > > ...
//! > > > >
//! > > > > deps:
//! > > > > > ...
//! > > > >
//! > > > > ...
//! > > > >
//! > > >
//! > >
//! > > > profile B:
//! > > > > ...
//! > > >
//! > >
//! > > > target A:
//! > > > > profile A:
//! > > > > > ...
//! > > > >
//! > > >
//! > > > > profile B:
//! > > > > > ...
//! > > > >
//! > > >
//! > >
//! > > > target B:
//! > > > > ...
//! > > >
//! > >
//! >
//!
//! [note]: https://img.shields.io/badge/note-orange.svg?color=ddbb00
//!
//! [bug]: https://img.shields.io/badge/bug-red.svg
//!
//! [feature]: https://img.shields.io/badge/feature-orange.svg
//!

use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub mod command;

pub mod lang;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_CONFIG_DIR: &str = concat!("./.", env!("CARGO_PKG_NAME"));

pub fn app_name() -> Cow<'static, str> {
    fn app_name_from_env() -> Option<Cow<'static, str>> {
        Some(Cow::Owned(
            std::env::current_exe()
                .ok()?
                .file_stem()?
                .to_str()?
                .to_owned(),
        ))
    }
    app_name_from_env().unwrap_or(Cow::Borrowed(APP_NAME))
}

pub fn app_config_dir() -> Cow<'static, Path> {
    fn app_config_dir_from_app_name() -> Option<Cow<'static, Path>> {
        Some(Cow::Owned(
            PathBuf::from(".").join(".".to_owned() + &app_name()),
        ))
    }
    app_config_dir_from_app_name().unwrap_or(Cow::Borrowed(Path::new(APP_CONFIG_DIR)))
}

pub fn make_app_config_dir() -> std::io::Result<Cow<'static, Path>> {
    let app_config_dir = app_config_dir();
    if !app_config_dir.exists() {
        std::fs::create_dir(&app_config_dir)?;
    }
    Ok(app_config_dir)
}
