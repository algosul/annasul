use std::collections::HashMap;

use algosul_task::prelude::*;

#[test]
fn task_link_new()
{
  let map = HashMap::from([("input", "output"), ("INPUT", "OUTPUT")]);
  let task = (|_: ()| "input")
    .map(str::to_uppercase)
    .map(move |s: String| map.get(s.as_str()).copied().unwrap_or(""))
    .unwrap_or("")
    .map(ToString::to_string);
  let result = task.run_once(()).unwrap();
  assert_eq!(result, "OUTPUT");
}

#[test]
fn task_link_returns_result()
{
  let task = (|_: ()| "input").map(str::to_uppercase);
  let result = task.run_once(());
  assert_eq!(result.unwrap(), "INPUT");
}

#[test]
fn task_unwrap_or_passthrough()
{
  let task =
    (|_: ()| -> algosul_task::Result<&str> { Ok("value") })
      .unwrap_or("default");
  assert_eq!(task.run_once(()), "value");
}

#[test]
fn task_unwrap_or_default()
{
  let task = (|_: ()| -> algosul_task::Result<String> {
    Err(algosul_task::Error::Send(Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "boom",
    ))))
  })
  .unwrap_or("fallback".to_string());
  assert_eq!(task.run_once(()), "fallback");
}
