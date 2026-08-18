use rust_serde_type_editor::Editor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum ShapeB
{
  Circle
  {
    radius: f64,
  },
  Square(i32),
}

#[derive(Debug, Serialize, Deserialize)]
enum Shape
{
  Circle
  {
    radius: f64,
  },
  Square(i32),
  SquareB(ShapeB),
}

fn main()
{
  Editor::builder()
    .edit(Shape::Square(0), |t: Shape| {
      println!("new Shape is {t:?}");
      t
    })
    .unwrap()
    .build()
    .unwrap()
    .run()
    .unwrap();
}
