use std::borrow::Cow;
pub mod c;
pub mod cpp;
pub mod csharp;
pub mod rust;
pub trait Language {
    fn lang_name() -> Cow<'static, str>;
}
