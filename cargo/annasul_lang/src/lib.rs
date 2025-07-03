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
