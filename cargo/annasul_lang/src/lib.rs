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
#![doc = include_str!("lib.md")]
#![cfg_attr(feature = "unstable-f16", feature(f16))]
#![cfg_attr(feature = "unstable-f128", feature(f128))]
#![feature(string_into_chars)]
pub mod codegen;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod types;
pub mod utils;
