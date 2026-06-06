# Task System

## Task Priority

1. Realtime
2. High
3. Normal
4. Low
5. Idle

## Task Scheduler

1. System

## SIMD

```rust
fn main() {
  let datas = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  datas.simd_task_iter()
    .add(1)
}
```

## Pipeline

```rust
use algosul::cpu::task::prelude::v1::*;
fn main() {
  let datas = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  datas.task_iter()
    .filter(|x| x % 2 == 0)
    .sync_task(100)
    .map(|x| x * 10)
}
```
