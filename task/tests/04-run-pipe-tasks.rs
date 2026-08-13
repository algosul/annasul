// use algosul_task::tasks;
//
// #[test]
// fn sync_run_pipe_tasks()
// {
//   let tasks = tasks![
//     || {
//       println!("Hello, pipe! From task1.");
//     },
//     || {
//       println!("Hello, pipe! From task2.");
//     },
//     async {
//       println!("Hello, pipe! From async task1.");
//     }
//   ];
//   tasks.sync_run().unwrap();
// }
//
// #[tokio::test]
// async fn async_run_pipe_tasks()
// {
//   let tasks = tasks![
//     || {
//       println!("Hello, pipe! From task1.");
//     },
//     || {
//       println!("Hello, pipe! From task2.");
//     },
//     async {
//       println!("Hello, pipe! From async task1.");
//     }
//   ];
//   tasks.async_run().await.unwrap();
// }
