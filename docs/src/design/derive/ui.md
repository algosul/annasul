# User Interface(UI)

## v1

```rust
use std::net::SocketAddr;
use std::time::Duration;
use algosul::ui::*;
use algosul::derive::*;

#[derive(UI, DataSync)]
#[ui(key = "time-sync", widget = "Taps::new()")]
enum TimeSync {
  #[ui(title_key = "unknown")]
  #[ui_cfg(class = "show", widget = "Text::new()")]
  #[ui_cfg(class = "input", ignore)]
  Unknown,
  #[ui(title_key = "no-sync")]
  No,
  #[ui(title_key = "auto-sync")]
  Auto {
    url: String,
    refresh_duration: Duration,
  }
}
```

## v2

1. async + `Rc<RefCell<T>>`
2. `Sender` and `Receiver`

```rust
use algosul::{ui::*, derive::*};
#[derive(DataSync)]
enum TimeSync {
  Unknown,
  No,
  Auto {
    url: String,
    refresh_duration: Duration,
  }
}

impl Widget for TimeSync {
  fn build(&self) -> Box<dyn Widget> {}
}

```
