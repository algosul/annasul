use std::borrow::Cow;
pub mod c;
pub mod cpp;
pub mod csharp;
pub mod rust;
pub trait LanguageInfo {
    fn lang_name() -> Cow<'static, str>;
}
pub use c::C;
pub use cpp::CPP;
pub use csharp::CSharp;
pub use rust::Rust;
/// langauge
pub enum Langauge {
    /// ISO C
    C,
    /// ISO C++
    CPP,
    /// C#
    CSharp,
    /// Rust
    Rust,
}
/// complier information
pub struct ComplierInfo {
    /// e.g. rustc/MSVC/MinGW
    name:     String,
    version:  String,
    langauge: Langauge,
}
