#[test]
fn task_from_fn()
{
  let _task = |_: ()| {
    println!("Hello, world!");
  };
}

#[test]
fn async_task_from_fn()
{
  let _task = async {
    println!("Hello, world!");
  };
}
