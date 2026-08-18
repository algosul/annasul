use std::{cell::Cell, fmt::Display, rc::Rc};

use gtk4::{Orientation, prelude::*};
use serde::Serialize;

use crate::Error;

/// Serializes a Rust value into a tree of editable `gtk4` widgets.
///
/// Every primitive maps to a single editing widget loaded with the current
/// value (`ToggleButton`, `SpinButton`, `Entry`, ...). Each variant carries the
/// callback invoked when the user edits that widget: it receives the widget's
/// new value and returns the value to apply back. `None` builds the widgets
/// without any callback and is used when recursing into nested values.
pub enum Serializer
{
  None,
  Bool(Box<dyn Fn(bool) -> bool>),
  I8(Box<dyn Fn(i8) -> i8>),
  I16(Box<dyn Fn(i16) -> i16>),
  I32(Box<dyn Fn(i32) -> i32>),
  I64(Box<dyn Fn(i64) -> i64>),
  U8(Box<dyn Fn(u8) -> u8>),
  U16(Box<dyn Fn(u16) -> u16>),
  U32(Box<dyn Fn(u32) -> u32>),
  U64(Box<dyn Fn(u64) -> u64>),
  F32(Box<dyn Fn(f32) -> f32>),
  F64(Box<dyn Fn(f64) -> f64>),
  Char(Box<dyn Fn(char) -> char>),
  Str(Box<dyn Fn(String) -> String>),
}

/// Shared collector for all composite serialization states (`seq`, `tuple`,
/// `map`, `struct`, ... variants). A single type keeps the seven serde
/// "Serialize*" traits from requiring seven near-identical structs;
/// `pending_key` is only used by the map state.
pub struct Node
{
  gbox:        gtk4::Box,
  pending_key: Option<gtk4::Box>,
}

impl Node
{
  fn new(name: &'static str) -> Self
  {
    Self { gbox: container(name), pending_key: None }
  }

  /// Collects into a box that also carries a header label (e.g.
  /// `Enum::Variant`).
  fn with_header(tag: &'static str, header: &str) -> Self
  {
    let gbox = container(tag);
    gbox.append(&gtk4::Label::new(Some(header)));
    Self { gbox, pending_key: None }
  }
}

/// Extracts the callback for `$variant` out of a `Serializer`. Returns `None`
/// for `Serializer::None` (nested values carry no callback) and errors when
/// the variant does not match the value being serialized.
macro_rules! take_callback
{
  ($self:expr, $variant:ident, $label:expr) =>
  {
    match $self
    {
      Serializer::$variant(cb) => Some(cb),
      Serializer::None => None,
      _ => return Err(Error::SerializerCallbackNoMatch {
        except: $label.to_string(),
      }),
    }
  };
}

/// Creates a vertical container tagged with a type name (visible in the
/// debugger and used to identify the widget's role).
fn container(name: &'static str) -> gtk4::Box
{
  gtk4::Box::builder()
    .name(name)
    .orientation(Orientation::Vertical)
    .spacing(4)
    .build()
}

/// Appends a `label + value` row (struct fields, newtypes).
fn append_labeled_row(gbox: &gtk4::Box, label: &str, value: &gtk4::Box)
{
  let row = gtk4::Box::builder()
    .orientation(Orientation::Horizontal)
    .spacing(4)
    .build();
  row.append(&gtk4::Label::new(Some(label)));
  row.append(value);
  gbox.append(&row);
}

/// Appends a `key + value` row (map entries).
fn append_kv_row(gbox: &gtk4::Box, key: &gtk4::Box, value: &gtk4::Box)
{
  let row = gtk4::Box::builder()
    .orientation(Orientation::Horizontal)
    .spacing(4)
    .build();
  row.append(key);
  row.append(value);
  gbox.append(&row);
}

/// Builds a box wrapping a `SpinButton` with the given range and decimal
/// digits, loaded with `value`. Arrow steps move by one unit at the last
/// digit; `on_change` (if any) transforms the new value before it is applied.
fn spin_container(
  name: &'static str, min: f64, max: f64, value: f64, digits: u32,
  on_change: Option<Box<dyn Fn(f64) -> f64>>,
) -> gtk4::Box
{
  let gbox = container(name);
  let spin =
    gtk4::SpinButton::with_range(min, max, 10f64.powi(-(digits as i32)));
  spin.set_digits(digits);
  spin.set_value(value);
  if let Some(cb) = on_change
  {
    spin.connect_value_changed(move |spin| {
      spin.set_value(cb(spin.value()));
    });
  }
  gbox.append(&spin);
  gbox
}

impl serde::ser::Serializer for Serializer
{
  type Error = crate::Error;
  type Ok = gtk4::Box;
  type SerializeMap = Node;
  type SerializeSeq = Node;
  type SerializeStruct = Node;
  type SerializeStructVariant = Node;
  type SerializeTuple = Node;
  type SerializeTupleStruct = Node;
  type SerializeTupleVariant = Node;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>
  {
    let on_bool = take_callback!(self, Bool, "on_bool");
    let gbox = container("bool");
    let gbutton = gtk4::ToggleButton::with_label("bool");
    gbutton.set_active(v);
    if let Some(cb) = on_bool
    {
      gbutton.connect_clicked(move |btn| {
        btn.set_active(cb(btn.is_active()));
      });
    }
    gbox.append(&gbutton);
    Ok(gbox)
  }

  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>
  {
    let on_i8 = take_callback!(self, I8, "on_i8");
    let cb = on_i8.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as i8) as f64)
    });
    Ok(spin_container(
      "i8", i8::MIN as f64, i8::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>
  {
    let on_i16 = take_callback!(self, I16, "on_i16");
    let cb = on_i16.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as i16) as f64)
    });
    Ok(spin_container(
      "i16", i16::MIN as f64, i16::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>
  {
    let on_i32 = take_callback!(self, I32, "on_i32");
    let cb = on_i32.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as i32) as f64)
    });
    Ok(spin_container(
      "i32", i32::MIN as f64, i32::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>
  {
    let on_i64 = take_callback!(self, I64, "on_i64");
    let cb = on_i64.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as i64) as f64)
    });
    Ok(spin_container(
      "i64", i64::MIN as f64, i64::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>
  {
    let on_u8 = take_callback!(self, U8, "on_u8");
    let cb = on_u8.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as u8) as f64)
    });
    Ok(spin_container(
      "u8", u8::MIN as f64, u8::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>
  {
    let on_u16 = take_callback!(self, U16, "on_u16");
    let cb = on_u16.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as u16) as f64)
    });
    Ok(spin_container(
      "u16", u16::MIN as f64, u16::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>
  {
    let on_u32 = take_callback!(self, U32, "on_u32");
    let cb = on_u32.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as u32) as f64)
    });
    Ok(spin_container(
      "u32", u32::MIN as f64, u32::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>
  {
    let on_u64 = take_callback!(self, U64, "on_u64");
    let cb = on_u64.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as u64) as f64)
    });
    Ok(spin_container(
      "u64", u64::MIN as f64, u64::MAX as f64, v as f64, 0, cb,
    ))
  }

  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>
  {
    let on_f32 = take_callback!(self, F32, "on_f32");
    let cb = on_f32.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value as f32) as f64)
    });
    Ok(spin_container(
      "f32", -(f32::MAX as f64), f32::MAX as f64, v as f64, 6, cb,
    ))
  }

  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>
  {
    let on_f64 = take_callback!(self, F64, "on_f64");
    let cb = on_f64.map(|cb| -> Box<dyn Fn(f64) -> f64> {
      Box::new(move |value: f64| cb(value))
    });
    Ok(spin_container("f64", -f64::MAX, f64::MAX, v, 9, cb))
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>
  {
    let on_char = take_callback!(self, Char, "on_char");
    let gbox = container("char");
    let entry = gtk4::Entry::new();
    entry.set_max_length(1);
    entry.set_max_width_chars(2);
    entry.set_text(&v.to_string());
    if let Some(cb) = on_char
    {
      // `set_text` always emits `changed`, so guard against re-entrancy.
      let updating = Rc::new(Cell::new(false));
      let guard = updating.clone();
      entry.connect_changed(move |entry| {
        if guard.get()
        {
          return;
        }
        guard.set(true);
        let ch = entry.text().chars().next().unwrap_or('\0');
        entry.set_text(&cb(ch).to_string());
        guard.set(false);
      });
    }
    gbox.append(&entry);
    Ok(gbox)
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>
  {
    let on_str = take_callback!(self, Str, "on_str");
    let gbox = container("str");
    let entry = gtk4::Entry::new();
    entry.set_text(v);
    if let Some(cb) = on_str
    {
      // `set_text` always emits `changed`, so guard against re-entrancy.
      let updating = Rc::new(Cell::new(false));
      let guard = updating.clone();
      entry.connect_changed(move |entry| {
        if guard.get()
        {
          return;
        }
        guard.set(true);
        entry.set_text(&cb(entry.text().to_string()));
        guard.set(false);
      });
    }
    gbox.append(&entry);
    Ok(gbox)
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>
  {
    let gbox = container("bytes");
    let label = gtk4::Label::new(Some(&format!("[{} bytes]", v.len())));
    gbox.append(&label);
    Ok(gbox)
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error>
  {
    let gbox = container("none");
    let label = gtk4::Label::new(Some("None"));
    gbox.append(&label);
    Ok(gbox)
  }

  fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where T: ?Sized + Serialize
  {
    let gbox = container("some");
    gbox.append(&value.serialize(self)?);
    Ok(gbox)
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error>
  {
    let gbox = container("unit");
    let label = gtk4::Label::new(Some("()"));
    gbox.append(&label);
    Ok(gbox)
  }

  fn serialize_unit_struct(
    self, name: &'static str,
  ) -> Result<Self::Ok, Self::Error>
  {
    let gbox = container("unit_struct");
    let label = gtk4::Label::new(Some(name));
    gbox.append(&label);
    Ok(gbox)
  }

  fn serialize_unit_variant(
    self, name: &'static str, _variant_index: u32, variant: &'static str,
  ) -> Result<Self::Ok, Self::Error>
  {
    let gbox = container("unit_variant");
    let label = gtk4::Label::new(Some(&format!("{name}::{variant}")));
    gbox.append(&label);
    Ok(gbox)
  }

  fn serialize_newtype_struct<T>(
    self, name: &'static str, value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where T: ?Sized + Serialize
  {
    let gbox = container("newtype_struct");
    append_labeled_row(&gbox, name, &value.serialize(self)?);
    Ok(gbox)
  }

  fn serialize_newtype_variant<T>(
    self, name: &'static str, _variant_index: u32, variant: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + Serialize,
  {
    let gbox = container("newtype_variant");
    append_labeled_row(
      &gbox,
      &format!("{name}::{variant}"),
      &value.serialize(self)?,
    );
    Ok(gbox)
  }

  fn serialize_seq(
    self, _len: Option<usize>,
  ) -> Result<Self::SerializeSeq, Self::Error>
  {
    Ok(Node::new("seq"))
  }

  fn serialize_tuple(
    self, _len: usize,
  ) -> Result<Self::SerializeTuple, Self::Error>
  {
    Ok(Node::new("tuple"))
  }

  fn serialize_tuple_struct(
    self, name: &'static str, _len: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error>
  {
    Ok(Node::new(name))
  }

  fn serialize_tuple_variant(
    self, name: &'static str, _variant_index: u32, variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error>
  {
    Ok(Node::with_header("tuple_variant", &format!("{name}::{variant}")))
  }

  fn serialize_map(
    self, _len: Option<usize>,
  ) -> Result<Self::SerializeMap, Self::Error>
  {
    Ok(Node::new("map"))
  }

  fn serialize_struct(
    self, name: &'static str, _len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error>
  {
    Ok(Node::new(name))
  }

  fn serialize_struct_variant(
    self, name: &'static str, _variant_index: u32, variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error>
  {
    Ok(Node::with_header("struct_variant", &format!("{name}::{variant}")))
  }

  fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where T: ?Sized + Display
  {
    self.serialize_str(&value.to_string())
  }
}

impl serde::ser::SerializeSeq for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    self.gbox.append(&value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeTuple for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    self.gbox.append(&value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeTupleStruct for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    self.gbox.append(&value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeTupleVariant for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    self.gbox.append(&value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeMap for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    self.pending_key = Some(key.serialize(Serializer::None)?);
    Ok(())
  }

  fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    let value = value.serialize(Serializer::None)?;
    match self.pending_key.take()
    {
      Some(key) => append_kv_row(&self.gbox, &key, &value),
      None => self.gbox.append(&value),
    }
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeStruct for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_field<T>(
    &mut self, key: &'static str, value: &T,
  ) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    append_labeled_row(&self.gbox, key, &value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}

impl serde::ser::SerializeStructVariant for Node
{
  type Error = crate::Error;
  type Ok = gtk4::Box;

  fn serialize_field<T>(
    &mut self, key: &'static str, value: &T,
  ) -> Result<(), Self::Error>
  where T: ?Sized + Serialize
  {
    append_labeled_row(&self.gbox, key, &value.serialize(Serializer::None)?);
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error>
  {
    Ok(self.gbox)
  }
}
