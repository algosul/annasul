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
//! application operation
//!

use std::borrow::Cow;
use std::path::Path;

#[cfg(any(doc, test, feature = "app-apps"))]
pub mod apps;
pub trait AppInfo<'a> {
    type Error: std::error::Error;
    async fn name(&self) -> Cow<'a, str>;
    async fn license(&self) -> Result<Option<Cow<'a, str>>, Self::Error>;
    async fn license_file(&self) -> Result<Option<Cow<'a, Path>>, Self::Error>;
    async fn description(&self) -> Result<Option<Cow<'a, str>>, Self::Error>;
    async fn documentation(&self) -> Result<Option<Cow<'a, str>>, Self::Error>;
    async fn home_page(&self) -> Result<Option<Cow<'a, str>>, Self::Error>;
    async fn version(&self) -> Result<Cow<'a, str>, Self::Error>;
}
pub trait AppPath<'a> {
    type Error: std::error::Error;
    async fn home_path(&self) -> Result<Option<Cow<'a, Path>>, Self::Error>;
    async fn bin_path(&self) -> Result<Option<Cow<'a, Path>>, Self::Error>;
}
pub trait AppOper: Sized {
    type Error: std::error::Error;
    type InstallInfo;
    type ReinstallInfo;
    type RemoveInfo;
    type UpdateInfo;
    async fn install(info: Self::InstallInfo) -> Result<Self, Self::Error>;
    async fn reinstall(self, info: Self::ReinstallInfo) -> Result<Self, Self::Error>;
    async fn remove(self, info: Self::RemoveInfo) -> Result<(), Self::Error>;
    async fn update(self, info: Self::UpdateInfo) -> Result<Self, Self::Error>;
}
