use algosul_task::prelude::*;

#[test]
fn run_task()
{
  let task = |_: ()| {
    println!("Hello, world!");
  };
  task.run_once(());
}

// #[tokio::test]
// async fn async_run_task()
// {
//   let task = async || {
//     println!("Hello, world!");
//   };
//   task.run_once(()).await;
// }
