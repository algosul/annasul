use rust_serde_type_editor::Editor;

#[test]
fn test_base()
{
  let editor = Editor::builder()
    .edit(0, |v: i32| {
      println!("New value is: {v}");
      v
    })
    .unwrap()
    .build()
    .unwrap();

  editor.run().unwrap();
}
