use core::{
  fmt::{Display, Formatter},
  str::FromStr,
};
use thiserror::Error;
use unic_langid::{LanguageIdentifier as UNICLangID, LanguageIdentifierError};

#[derive(Debug, Error)]
pub enum Error
{
  #[error("Unknown")]
  Unknown,
  #[error("Parser Error: {0}")]
  ParserError(#[source] ParserError),
}
#[derive(Debug, Error)]
pub enum ParserError
{
  #[error("Invalid Language")]
  InvalidLanguage,
  #[error("Invalid Subtag")]
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

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn parse_valid_identifier()
  {
    let id: LanguageIdentifier = "zh-Hans".parse().unwrap();
    assert_eq!(id.to_string(), "zh-Hans");
    let id: LanguageIdentifier = "en-US".parse().unwrap();
    assert_eq!(id.to_string(), "en-US");
    let id: LanguageIdentifier = "zh".parse().unwrap();
    assert_eq!(id.to_string(), "zh");
  }

  #[test]
  fn invalid_identifier_errors()
  {
    // Obviously invalid input should yield Err
    let r: Result<LanguageIdentifier> = "".parse();
    assert!(r.is_err());
    let r: Result<LanguageIdentifier> = "123".parse();
    assert!(r.is_err());
  }

  #[test]
  fn display_roundtrip_and_eq()
  {
    let a: LanguageIdentifier = "en-GB".parse().unwrap();
    let b: LanguageIdentifier = "en-gb".parse().unwrap();
    // Equal after normalization (unic-langid unifies case and region)
    assert_eq!(a, b);
    assert_eq!(a.to_string(), b.to_string());
  }
}
