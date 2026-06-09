#[cfg(feature = "i18n-fluent")]
pub use fluent;

pub mod langid;

#[cfg(test)]
mod tests
{
  use alloc::{string::ToString, vec};

  use fluent::{fluent_args, FluentBundle, FluentResource};
  use unic_langid::LanguageIdentifier;

  #[test]
  fn test_fluent()
  {
    let args = fluent_args! {
      "name" => "Alice",
      "age" => 18,
    };
    let resource = FluentResource::try_new(
      r#"
hello = Hello, { $name }. You are { $age }.
"#
      .to_string(),
    )
    .unwrap();
    let lang_id: LanguageIdentifier = "en-US".parse().unwrap();
    let mut bundle = FluentBundle::new(vec![lang_id]);
    bundle.add_resource(resource).expect("Can not to add resource");
    let mut errors = vec![];
    let msg =
      bundle.get_message("hello").expect("Failed to retrieve a FluentMessage.");
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(pattern, Some(&args), &mut errors);
    assert_eq!(
      value,
      "Hello, \u{2068}Alice\u{2069}. You are \u{2068}18\u{2069}."
    );
  }
}
