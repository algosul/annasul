use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use gtk4::glib::object::Cast;
use gtk4::prelude::*;
use serde::de::{self, DeserializeSeed, Visitor};
use serde::forward_to_deserialize_any;

use crate::Error;

fn custom(msg: impl Display) -> Error
{
  Error::Custom(msg.to_string())
}

/// Deserializes Rust values out of a widget tree built by
/// [`ser::Serializer`](crate::ser::Serializer). Each primitive reads its
/// editing widget back (`ToggleButton`, `SpinButton`, `Entry`, ...); composite
/// types traverse the child rows.
pub struct Deserializer
{
  gbox:       gtk4::Box,
  substitute: Option<Rc<(String, String)>>,
}

impl Deserializer
{
  pub fn from_box(gbox: gtk4::Box) -> Self
  {
    Self { gbox, substitute: None }
  }

  /// Deserializes while substituting a specific enum variant: at
  /// `deserialize_enum(name)`, when `name` matches `enum_name`, the variant is
  /// constructed from defaults instead of reading its (not yet rebuilt)
  /// widgets. Used when the switcher selects a new variant.
  pub fn with_substitute(
    gbox: gtk4::Box, enum_name: &str, variant: &str,
  ) -> Self
  {
    Self {
      gbox,
      substitute: Some(Rc::new((enum_name.to_string(), variant.to_string()))),
    }
  }

  fn toggle(&self) -> Result<gtk4::ToggleButton, Error>
  {
    self.gbox
      .first_child()
      .and_then(|w| w.downcast::<gtk4::ToggleButton>().ok())
      .ok_or_else(|| custom("missing toggle button"))
  }

  fn spin(&self) -> Result<gtk4::SpinButton, Error>
  {
    self.gbox
      .first_child()
      .and_then(|w| w.downcast::<gtk4::SpinButton>().ok())
      .ok_or_else(|| custom("missing spin button"))
  }

  fn entry(&self) -> Result<gtk4::Entry, Error>
  {
    self.gbox
      .first_child()
      .and_then(|w| w.downcast::<gtk4::Entry>().ok())
      .ok_or_else(|| custom("missing entry"))
  }

  /// The first `gtk4::Box` child, skipping the switcher `DropDown` if present.
  fn first_box(&self) -> Result<gtk4::Box, Error>
  {
    let mut child = self.gbox.first_child();
    while let Some(w) = child
    {
      if let Some(boxed) = w.downcast_ref::<gtk4::Box>()
      {
        return Ok(boxed.clone());
      }
      child = w.next_sibling();
    }
    Err(custom("missing child box"))
  }
}

/// Collects all `gtk4::Box` children of a container (leaf labels are skipped).
fn children(gbox: &gtk4::Box) -> Vec<gtk4::Box>
{
  let mut out = Vec::new();
  let mut child = gbox.first_child();
  while let Some(w) = child
  {
    if let Some(boxed) = w.downcast_ref::<gtk4::Box>()
    {
      out.push(boxed.clone());
    }
    child = w.next_sibling();
  }
  out
}

/// Returns the `index`-th `gtk4::Box` child of a container.
fn nth_box(gbox: &gtk4::Box, index: u32) -> Result<gtk4::Box, Error>
{
  let mut child = gbox.first_child();
  let mut i = 0;
  while let Some(w) = child
  {
    if i == index
    {
      return w
        .downcast::<gtk4::Box>()
        .map_err(|_| custom("child is not a box"));
    }
    i += 1;
    child = w.next_sibling();
  }
  Err(custom("missing child box"))
}

/// Reads the text of a container's first `Label` child, skipping any leading
/// switcher `DropDown`.
fn header_text(gbox: &gtk4::Box) -> Result<String, Error>
{
  let mut child = gbox.first_child();
  while let Some(w) = child
  {
    if let Some(label) = w.downcast_ref::<gtk4::Label>()
    {
      return Ok(label.text().to_string());
    }
    child = w.next_sibling();
  }
  Err(custom("missing label"))
}

/// Ser tags variant headers as `Enum::Variant`; the deserializer's variant
/// seed only knows the bare variant name, so strip the type-name prefix.
fn variant_name(header: &str) -> String
{
  header.rsplit("::").next().unwrap_or(header).to_string()
}

/// The value payload of an enum variant widget.
enum VariantContent
{
  Unit,
  Newtype(gtk4::Box),
  Tuple(Vec<gtk4::Box>),
  Struct(Vec<gtk4::Box>),
}

/// A deserializer that yields a fixed string (field names, variant names).
struct TextDeserializer
{
  text: String,
}

impl<'de> serde::Deserializer<'de> for TextDeserializer
{
  type Error = crate::Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_str(&self.text)
  }

  forward_to_deserialize_any! {
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    bytes byte_buf option unit unit_struct newtype_struct seq tuple
    tuple_struct map struct enum identifier ignored_any
  }
}

struct SeqAccess
{
  elements:   Vec<gtk4::Box>,
  index:      usize,
  substitute: Option<Rc<(String, String)>>,
}

impl SeqAccess
{
  fn new(gbox: &gtk4::Box, substitute: Option<Rc<(String, String)>>) -> Self
  {
    Self { elements: children(gbox), index: 0, substitute }
  }
}

impl<'de> de::SeqAccess<'de> for SeqAccess
{
  type Error = crate::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where T: DeserializeSeed<'de>
  {
    let element = match self.elements.get(self.index)
    {
      Some(element) => element.clone(),
      None => return Ok(None),
    };
    self.index += 1;
    seed.deserialize(Deserializer {
      gbox: element,
      substitute: self.substitute.clone(),
    }).map(Some)
  }
}

struct StructAccess
{
  rows:       Vec<gtk4::Box>,
  index:      usize,
  pending:    Option<gtk4::Box>,
  substitute: Option<Rc<(String, String)>>,
}

impl StructAccess
{
  fn new(gbox: &gtk4::Box, substitute: Option<Rc<(String, String)>>) -> Self
  {
    Self { rows: children(gbox), index: 0, pending: None, substitute }
  }
}

impl<'de> de::MapAccess<'de> for StructAccess
{
  type Error = crate::Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where K: DeserializeSeed<'de>
  {
    let row = match self.rows.get(self.index)
    {
      Some(row) => row.clone(),
      None => return Ok(None),
    };
    self.index += 1;
    self.pending = Some(row.clone());
    let key = header_text(&row)?;
    seed.deserialize(TextDeserializer { text: key }).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where V: DeserializeSeed<'de>
  {
    let row =
      self.pending.take().ok_or_else(|| custom("field value without key"))?;
    let value = nth_box(&row, 1)?;
    seed.deserialize(Deserializer {
      gbox: value,
      substitute: self.substitute.clone(),
    })
  }
}

struct MapAccess
{
  rows:       Vec<gtk4::Box>,
  index:      usize,
  pending:    Option<gtk4::Box>,
  substitute: Option<Rc<(String, String)>>,
}

impl MapAccess
{
  fn new(gbox: &gtk4::Box, substitute: Option<Rc<(String, String)>>) -> Self
  {
    Self { rows: children(gbox), index: 0, pending: None, substitute }
  }
}

impl<'de> de::MapAccess<'de> for MapAccess
{
  type Error = crate::Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where K: DeserializeSeed<'de>
  {
    let row = match self.rows.get(self.index)
    {
      Some(row) => row.clone(),
      None => return Ok(None),
    };
    self.index += 1;
    self.pending = Some(row.clone());
    let key = nth_box(&row, 0)?;
    seed.deserialize(Deserializer {
      gbox: key,
      substitute: self.substitute.clone(),
    }).map(Some)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where V: DeserializeSeed<'de>
  {
    let row =
      self.pending.take().ok_or_else(|| custom("map value without key"))?;
    let value = nth_box(&row, 1)?;
    seed.deserialize(Deserializer {
      gbox: value,
      substitute: self.substitute.clone(),
    })
  }
}

struct EnumAccess
{
  variant:    String,
  content:    VariantContent,
  substitute: Option<Rc<(String, String)>>,
}

impl<'de> de::EnumAccess<'de> for EnumAccess
{
  type Error = crate::Error;
  type Variant = VariantAccess;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
  where V: DeserializeSeed<'de>
  {
    let variant = seed.deserialize(TextDeserializer { text: self.variant })?;
    Ok((
      variant,
      VariantAccess { content: self.content, substitute: self.substitute },
    ))
  }
}

struct VariantAccess
{
  content:    VariantContent,
  substitute: Option<Rc<(String, String)>>,
}

impl<'de> de::VariantAccess<'de> for VariantAccess
{
  type Error = crate::Error;

  fn unit_variant(self) -> Result<(), Self::Error>
  {
    match self.content
    {
      VariantContent::Unit => Ok(()),
      _ => Err(custom("not a unit variant")),
    }
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
  where T: DeserializeSeed<'de>
  {
    match self.content
    {
      VariantContent::Newtype(value) => seed.deserialize(Deserializer {
        gbox: value,
        substitute: self.substitute,
      }),
      _ => Err(custom("not a newtype variant")),
    }
  }

  fn tuple_variant<V>(
    self, _len: usize, visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    match self.content
    {
      VariantContent::Tuple(elements) =>
        visitor.visit_seq(SeqAccess { elements, index: 0, substitute: self.substitute }),
      _ => Err(custom("not a tuple variant")),
    }
  }

  fn struct_variant<V>(
    self, _fields: &'static [&'static str], visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    match self.content
    {
      VariantContent::Struct(rows) => visitor.visit_map(StructAccess {
        rows,
        index: 0,
        pending: None,
        substitute: self.substitute,
      }),
      _ => Err(custom("not a struct variant")),
    }
  }
}

impl<'de> serde::Deserializer<'de> for Deserializer
{
  type Error = crate::Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    let name = self.gbox.widget_name().to_string();
    match name.as_str()
    {
      "bool" => visitor.visit_bool(self.toggle()?.is_active()),
      "i8" => visitor.visit_i8(self.spin()?.value() as i8),
      "i16" => visitor.visit_i16(self.spin()?.value() as i16),
      "i32" => visitor.visit_i32(self.spin()?.value() as i32),
      "i64" => visitor.visit_i64(self.spin()?.value() as i64),
      "u8" => visitor.visit_u8(self.spin()?.value() as u8),
      "u16" => visitor.visit_u16(self.spin()?.value() as u16),
      "u32" => visitor.visit_u32(self.spin()?.value() as u32),
      "u64" => visitor.visit_u64(self.spin()?.value() as u64),
      "f32" => visitor.visit_f32(self.spin()?.value() as f32),
      "f64" => visitor.visit_f64(self.spin()?.value()),
      "char" =>
        visitor.visit_char(self.entry()?.text().chars().next().unwrap_or('\0')),
      "str" => visitor.visit_str(&self.entry()?.text()),
      "bytes" => Err(custom("cannot deserialize bytes from a widget")),
      "none" => visitor.visit_none(),
      "some" => visitor.visit_some(Deserializer {
        gbox: self.first_box()?,
        substitute: self.substitute.clone(),
      }),
      "unit" | "unit_struct" => visitor.visit_unit(),
      "unit_variant" | "newtype_variant" | "tuple_variant" | "struct_variant" =>
        self.deserialize_enum("", &[], visitor),
      "newtype_struct" =>
      {
        let value = nth_box(&self.first_box()?, 1)?;
        visitor.visit_newtype_struct(Deserializer {
          gbox: value,
          substitute: self.substitute.clone(),
        })
      }
      "seq" | "tuple" =>
        visitor.visit_seq(SeqAccess::new(&self.gbox, self.substitute.clone())),
      "map" => visitor.visit_map(MapAccess::new(&self.gbox, self.substitute.clone())),
      // Structs are tagged with their type name. Their rows are `label + value`
      // boxes, which distinguishes them from tuple-struct element boxes.
      _ =>
      {
        let struct_like = children(&self.gbox).first().is_some_and(|row| {
          row.first_child().and_then(|w| w.downcast::<gtk4::Label>().ok()).is_some()
        });
        if struct_like
        {
          visitor.visit_map(StructAccess::new(&self.gbox, self.substitute.clone()))
        }
        else
        {
          visitor.visit_seq(SeqAccess::new(&self.gbox, self.substitute.clone()))
        }
      }
    }
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    if self.gbox.widget_name() == "none"
    {
      visitor.visit_none()
    }
    else
    {
      visitor.visit_some(Deserializer {
        gbox: self.first_box()?,
        substitute: self.substitute.clone(),
      })
    }
  }

  fn deserialize_enum<V>(
    self, name: &'static str, variants: &'static [&'static str], visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    // When switching, the target variant is constructed from defaults instead
    // of reading its (not yet rebuilt) widgets.
    let substitute = self.substitute.clone();
    if let Some((_, variant)) = substitute
      .as_deref()
      .filter(|(target, _)| target.as_str() == name)
    {
      return DefaultValue::variant(variant)
        .deserialize_enum(name, variants, visitor);
    }
    let (variant, content) = match self.gbox.widget_name().as_str()
    {
      "unit_variant" =>
      {
        let variant = variant_name(&header_text(&self.gbox)?);
        (variant, VariantContent::Unit)
      }
      "newtype_variant" =>
      {
        let row = self.first_box()?;
        let variant = variant_name(&header_text(&row)?);
        let value = nth_box(&row, 1)?;
        (variant, VariantContent::Newtype(value))
      }
      "tuple_variant" =>
      {
        let variant = variant_name(&header_text(&self.gbox)?);
        (variant, VariantContent::Tuple(children(&self.gbox)))
      }
      "struct_variant" =>
      {
        let variant = variant_name(&header_text(&self.gbox)?);
        (variant, VariantContent::Struct(children(&self.gbox)))
      }
      _ => return Err(custom("widget is not an enum variant")),
    };
    visitor.visit_enum(EnumAccess {
      variant,
      content,
      substitute: self.substitute,
    })
  }

  forward_to_deserialize_any! {
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    bytes byte_buf unit unit_struct newtype_struct seq tuple tuple_struct
    map struct identifier ignored_any
  }
}

/// Extracts the current variant name from a widget tree built by
/// `ser::Serializer` for an enum variant (used to seed the switcher).
pub fn current_variant(gbox: &gtk4::Box) -> Option<String>
{
  let header = match gbox.widget_name().as_str()
  {
    "unit_variant" => header_text(gbox).ok()?,
    "newtype_variant" =>
    {
      let row = first_box(gbox).ok()?;
      header_text(&row).ok()?
    }
    "tuple_variant" | "struct_variant" => header_text(gbox).ok()?,
    _ => return None,
  };
  Some(variant_name(&header))
}

/// The enum's type name from a variant box's `Enum::Variant` header, or `None`
/// if `gbox` is not a variant widget.
pub fn variant_enum_name(gbox: &gtk4::Box) -> Option<String>
{
  let header = match gbox.widget_name().as_str()
  {
    "unit_variant" => header_text(gbox).ok()?,
    "newtype_variant" =>
    {
      let row = first_box(gbox).ok()?;
      header_text(&row).ok()?
    }
    "tuple_variant" | "struct_variant" => header_text(gbox).ok()?,
    _ => return None,
  };
  header.split("::").next().map(str::to_string)
}

/// The first `gtk4::Box` child of a container, skipping non-box widgets.
fn first_box(gbox: &gtk4::Box) -> Result<gtk4::Box, Error>
{
  let mut child = gbox.first_child();
  while let Some(w) = child
  {
    if let Some(boxed) = w.downcast_ref::<gtk4::Box>()
    {
      return Ok(boxed.clone());
    }
    child = w.next_sibling();
  }
  Err(custom("missing child box"))
}

/// Shared configuration for `DefaultValue`: whether to explore the full type
/// graph (discovering every reachable enum's variant names) and where to
/// record them.
#[derive(Clone)]
struct DefaultConfig
{
  explore:  bool,
  registry: Rc<RefCell<HashMap<&'static str, &'static [&'static str]>>>,
}

impl DefaultConfig
{
  fn plain() -> Self
  {
    Self { explore: false, registry: Rc::new(RefCell::new(HashMap::new())) }
  }
}

/// Deserializer that constructs a value out of defaults for every type. With
/// `registry`, it explores the whole type graph (collections yield one default
/// element, options yield `Some`) and records each enum type name's variants —
/// this is what powers the switcher for nested enums. With `variant(name)` it
/// constructs a specific enum variant.
pub struct DefaultValue
{
  config: DefaultConfig,
  pick:   Option<String>,
}

impl DefaultValue
{
  fn nested(&self) -> Self
  {
    Self { config: self.config.clone(), pick: None }
  }

  /// Constructs the default value of a specific enum variant (for switching).
  pub fn variant(name: &str) -> Self
  {
    Self { config: DefaultConfig::plain(), pick: Some(name.to_string()) }
  }

  /// Explores `T`'s type graph, recording every enum type name's variant list
  /// into `registry`.
  pub fn registry(
    registry: Rc<RefCell<HashMap<&'static str, &'static [&'static str]>>>,
  ) -> Self
  {
    Self { config: DefaultConfig { explore: true, registry }, pick: None }
  }
}

macro_rules! default_primitive {
  ($method:ident, $visit:ident, $value:expr) => {
    fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where V: Visitor<'de>
    {
      let _ = &self;
      visitor.$visit($value)
    }
  };
}

impl<'de> serde::Deserializer<'de> for DefaultValue
{
  type Error = crate::Error;

  default_primitive!(deserialize_bool, visit_bool, false);
  default_primitive!(deserialize_i8, visit_i8, 0);
  default_primitive!(deserialize_i16, visit_i16, 0);
  default_primitive!(deserialize_i32, visit_i32, 0);
  default_primitive!(deserialize_i64, visit_i64, 0);
  default_primitive!(deserialize_i128, visit_i128, 0);
  default_primitive!(deserialize_u8, visit_u8, 0);
  default_primitive!(deserialize_u16, visit_u16, 0);
  default_primitive!(deserialize_u32, visit_u32, 0);
  default_primitive!(deserialize_u64, visit_u64, 0);
  default_primitive!(deserialize_u128, visit_u128, 0);
  default_primitive!(deserialize_f32, visit_f32, 0.0);
  default_primitive!(deserialize_f64, visit_f64, 0.0);
  default_primitive!(deserialize_char, visit_char, '\0');
  default_primitive!(deserialize_str, visit_str, "");
  default_primitive!(deserialize_string, visit_string, String::new());
  default_primitive!(deserialize_bytes, visit_bytes, b"");
  default_primitive!(deserialize_byte_buf, visit_byte_buf, Vec::new());
  default_primitive!(deserialize_identifier, visit_str, "");

  fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    Err(custom("DefaultValue: cannot deserialize an arbitrary value"))
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    if self.config.explore
    {
      visitor.visit_some(self.nested())
    }
    else
    {
      visitor.visit_none()
    }
  }

  fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_unit()
  }

  fn deserialize_unit_struct<V>(
    self, _name: &'static str, visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_unit()
  }

  fn deserialize_newtype_struct<V>(
    self, _name: &'static str, visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_newtype_struct(self.nested())
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    let remaining = if self.config.explore { 1 } else { 0 };
    visitor.visit_seq(DefaultSeq { remaining, config: self.config.clone() })
  }

  fn deserialize_tuple<V>(
    self, len: usize, visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_seq(DefaultSeq { remaining: len, config: self.config.clone() })
  }

  fn deserialize_tuple_struct<V>(
    self, _name: &'static str, len: usize, visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_seq(DefaultSeq { remaining: len, config: self.config.clone() })
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    let generic_entries = if self.config.explore { 1 } else { 0 };
    visitor.visit_map(DefaultMap {
      fields: &[],
      generic_entries,
      index: 0,
      config: self.config.clone(),
    })
  }

  fn deserialize_struct<V>(
    self, _name: &'static str, fields: &'static [&'static str], visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_map(DefaultMap {
      fields,
      generic_entries: 0,
      index: 0,
      config: self.config.clone(),
    })
  }

  fn deserialize_enum<V>(
    self, name: &'static str, variants: &'static [&'static str], visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    if self.config.explore
    {
      self.config.registry.borrow_mut().entry(name).or_insert(variants);
    }
    let pick = self.pick.clone().unwrap_or_else(|| {
      variants.first().map(|s| s.to_string()).unwrap_or_default()
    });
    visitor.visit_enum(DefaultEnum { variant: pick, config: self.config.clone() })
  }

  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_unit()
  }
}

struct DefaultSeq
{
  remaining: usize,
  config:    DefaultConfig,
}

impl<'de> de::SeqAccess<'de> for DefaultSeq
{
  type Error = crate::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where T: DeserializeSeed<'de>
  {
    if self.remaining == 0
    {
      return Ok(None);
    }
    self.remaining -= 1;
    seed
      .deserialize(DefaultValue { config: self.config.clone(), pick: None })
      .map(Some)
  }
}

struct DefaultMap
{
  fields:          &'static [&'static str],
  generic_entries: usize,
  index:           usize,
  config:          DefaultConfig,
}

impl<'de> de::MapAccess<'de> for DefaultMap
{
  type Error = crate::Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where K: DeserializeSeed<'de>
  {
    if self.index < self.fields.len()
    {
      let key = self.fields[self.index];
      return seed
        .deserialize(TextDeserializer { text: key.to_string() })
        .map(Some);
    }
    if self.index < self.fields.len() + self.generic_entries
    {
      return seed
        .deserialize(DefaultValue { config: self.config.clone(), pick: None })
        .map(Some);
    }
    Ok(None)
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where V: DeserializeSeed<'de>
  {
    self.index += 1;
    seed.deserialize(DefaultValue { config: self.config.clone(), pick: None })
  }
}

struct DefaultEnum
{
  variant: String,
  config:  DefaultConfig,
}

impl<'de> de::EnumAccess<'de> for DefaultEnum
{
  type Error = crate::Error;
  type Variant = DefaultVariant;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
  where V: DeserializeSeed<'de>
  {
    let variant = seed.deserialize(TextDeserializer { text: self.variant })?;
    Ok((variant, DefaultVariant { config: self.config }))
  }
}

struct DefaultVariant
{
  config: DefaultConfig,
}

impl<'de> de::VariantAccess<'de> for DefaultVariant
{
  type Error = crate::Error;

  fn unit_variant(self) -> Result<(), Self::Error>
  {
    Ok(())
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
  where T: DeserializeSeed<'de>
  {
    seed.deserialize(DefaultValue { config: self.config, pick: None })
  }

  fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_seq(DefaultSeq { remaining: len, config: self.config })
  }

  fn struct_variant<V>(
    self, fields: &'static [&'static str], visitor: V,
  ) -> Result<V::Value, Self::Error>
  where V: Visitor<'de>
  {
    visitor.visit_map(DefaultMap {
      fields,
      generic_entries: 0,
      index: 0,
      config: self.config,
    })
  }
}
