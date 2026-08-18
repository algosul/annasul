use std::sync::Once;

use gtk4::{glib::object::Cast, prelude::WidgetExt};
use rust_serde_type_editor::{de::Deserializer, ser::Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point
{
  x: i32,
  y: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Shape
{
  Circle
  {
    radius: f64,
  },
  Square(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Config
{
  name:  String,
  shape: Shape,
}

fn child_count(gbox: &gtk4::Box) -> u32
{
  let mut count = 0;
  let mut child = gbox.first_child();
  while child.is_some()
  {
    count += 1;
    child = child.and_then(|w| w.next_sibling());
  }
  count
}

/// GTK may only be used from the thread that initialized it, and the test
/// harness spawns one thread per `#[test]`. Keeping every assertion in a
/// single test function guarantees they all run on the same (init) thread.
#[test]
fn serialize_builds_editable_widgets()
{
  static INIT: Once = Once::new();
  INIT.call_once(|| gtk4::init().expect("failed to initialize gtk"));

  // Primitive: bool -> ToggleButton
  let gbox = true.serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "bool");
  assert_eq!(child_count(&gbox), 1);

  // Primitive: integer -> SpinButton
  let gbox = 42i32.serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "i32");
  assert_eq!(child_count(&gbox), 1);

  // Primitive: string -> Entry
  let gbox = "hello".serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "str");
  assert_eq!(child_count(&gbox), 1);

  // Option
  let gbox = Some(5u8).serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "some");
  let gbox = None::<u8>.serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "none");

  // Seq: one row per element
  let gbox = vec![1i32, 2, 3].serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "seq");
  assert_eq!(child_count(&gbox), 3);

  // Struct: one labeled row per field
  let gbox = Point { x: 1, y: 2 }.serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "Point");
  assert_eq!(child_count(&gbox), 2);

  // Struct variant: header row + one field row
  let gbox = Shape::Circle { radius: 1.5 }.serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "struct_variant");
  assert_eq!(child_count(&gbox), 2);

  // Newtype variant: one labeled row
  let gbox = Shape::Square(7).serialize(Serializer::None).unwrap();
  assert_eq!(gbox.widget_name().as_str(), "newtype_variant");
  assert_eq!(child_count(&gbox), 1);

  // Callback wiring: setting the SpinButton emits `value-changed`, which runs
  // the callback and applies its result back to the widget.
  let gbox =
    5i32.serialize(Serializer::I32(Box::new(|v| v.clamp(0, 3)))).unwrap();
  let spin =
    gbox.first_child().unwrap().downcast::<gtk4::SpinButton>().unwrap();
  spin.set_value(10.0);
  assert_eq!(spin.value(), 3.0);

  // Round-trip: ser builds widgets, de reads the same value back out.
  assert!(round_trip::<bool>(true.serialize(Serializer::None).unwrap()));
  assert_eq!(round_trip::<i32>(42i32.serialize(Serializer::None).unwrap()), 42);
  assert_eq!(
    round_trip::<String>("hello".serialize(Serializer::None).unwrap()),
    "hello"
  );
  assert_eq!(
    round_trip::<Option<u8>>(Some(5u8).serialize(Serializer::None).unwrap()),
    Some(5)
  );
  assert_eq!(
    round_trip::<Vec<i32>>(
      vec![1i32, 2, 3].serialize(Serializer::None).unwrap()
    ),
    vec![1, 2, 3]
  );
  assert_eq!(
    round_trip::<Point>(
      Point { x: 1, y: 2 }.serialize(Serializer::None).unwrap()
    ),
    Point { x: 1, y: 2 }
  );
  match round_trip::<Shape>(
    Shape::Circle { radius: 1.5 }.serialize(Serializer::None).unwrap(),
  )
  {
    Shape::Circle { radius } => assert_eq!(radius, 1.5),
    _ => panic!("expected Circle"),
  }

  // Substitute: switching a (possibly nested) enum variant preserves the other
  // fields' widget values and constructs the target variant from defaults.
  let cfg = Config { name: "hello".to_string(), shape: Shape::Square(7) };
  let gbox = cfg.serialize(Serializer::None).unwrap();
  let switched: Config =
    Config::deserialize(Deserializer::with_substitute(gbox, "Shape", "Circle"))
      .unwrap();
  assert_eq!(switched.name, "hello");
  match switched.shape
  {
    Shape::Circle { radius } => assert_eq!(radius, 0.0),
    _ => panic!("expected Circle"),
  }
}

/// Serializes `T` into widgets and deserializes them back into `T`.
fn round_trip<T: serde::de::DeserializeOwned>(gbox: gtk4::Box) -> T
{
  T::deserialize(Deserializer::from_box(gbox)).expect("round-trip failed")
}
