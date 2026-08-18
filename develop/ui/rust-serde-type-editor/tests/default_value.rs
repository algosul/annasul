use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rust_serde_type_editor::de::DefaultValue;
use serde::Deserialize;

#[derive(Deserialize)]
enum Shape
{
  Circle
  {
    radius: f64,
  },
  Square(i32),
}

#[derive(Deserialize)]
struct Point
{
  x: i32,
  y: i32,
}

#[test]
fn registry_discovers_enum_variants()
{
  let registry = Rc::new(RefCell::new(HashMap::new()));
  let _: Shape =
    Shape::deserialize(DefaultValue::registry(registry.clone())).unwrap();

  let expected: &'static [&'static str] = &["Circle", "Square"];
  assert_eq!(registry.borrow().get("Shape").copied(), Some(expected));
}

#[test]
fn registry_ignores_non_enum()
{
  let registry = Rc::new(RefCell::new(HashMap::new()));
  let point: Point =
    Point::deserialize(DefaultValue::registry(registry.clone())).unwrap();
  assert!(registry.borrow().is_empty());
  assert_eq!(point.x, 0);
  assert_eq!(point.y, 0);
}

#[test]
fn variant_constructs_target_variant()
{
  match Shape::deserialize(DefaultValue::variant("Square")).unwrap()
  {
    Shape::Square(value) => assert_eq!(value, 0),
    _ => panic!("expected Square"),
  }

  match Shape::deserialize(DefaultValue::variant("Circle")).unwrap()
  {
    Shape::Circle { radius } => assert_eq!(radius, 0.0),
    _ => panic!("expected Circle"),
  }
}
