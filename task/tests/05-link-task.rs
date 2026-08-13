use std::collections::HashMap;

use algosul_task::prelude::*;

#[test]
fn task_link_new()
{
  let map = HashMap::from([("input", "output"), ("INPUT", "OUTPUT")]);
  let task = (|_: ()| "input").map(str::to_uppercase);
  // .map(|s: String| map.get(&s).cloned());
  let result = task.run_once(()).unwrap();
  assert_eq!(result, "OUTPUT");
}
