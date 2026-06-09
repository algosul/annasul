use core::{
  fmt::{Display, Formatter},
  str::FromStr,
};

use unic_langid::{LanguageIdentifier as UNICLangID, LanguageIdentifierError};

#[derive(Debug)]
pub enum Error
{
  Unknown,
  ParserError(ParserError),
}
#[derive(Debug)]
pub enum ParserError
{
  InvalidLanguage,
  InvalidSubtag,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LanguageIdentifier
{
  inner: UNICLangID,
}

impl From<LanguageIdentifierError> for Error
{
  fn from(value: LanguageIdentifierError) -> Self
  {
    match value
    {
      LanguageIdentifierError::Unknown => Self::Unknown,
      LanguageIdentifierError::ParserError(parser_error) =>
      {
        Self::ParserError(parser_error.into())
      }
    }
  }
}

impl From<unic_langid::parser::ParserError> for ParserError
{
  fn from(value: unic_langid::parser::ParserError) -> Self
  {
    use unic_langid::parser::ParserError;
    match value
    {
      ParserError::InvalidLanguage => Self::InvalidLanguage,
      ParserError::InvalidSubtag => Self::InvalidSubtag,
    }
  }
}

impl FromStr for LanguageIdentifier
{
  type Err = Error;

  fn from_str(s: &str) -> Result<Self>
  {
    Ok(Self { inner: UNICLangID::from_str(s).map_err(Into::<Error>::into)? })
  }
}

impl Display for LanguageIdentifier
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
  {
    self.inner.fmt(f)
  }
}
