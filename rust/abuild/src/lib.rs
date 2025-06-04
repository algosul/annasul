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

pub mod command;

pub mod lang;
