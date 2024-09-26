---
title: Using as a Rust Library
---

It is possible to use `binsider` in your Rust/TUI application if you are already using [Ratatui](https://ratatui.rs).

See the [API documentation](https://docs.rs/binsider) for getting information about the available functions and structures.

To integrate it with your TUI application, add the following to your `Cargo.toml`:

```toml
[dependencies]
binsider = "0.1"
ratatui = "0.28"
# use the latest versions above
```

Then you can create a `Analyzer` struct and `State` for managing the TUI state:

```rust
use binsider::prelude::*;

let analyzer = Analyzer::new(file_info, 15, vec![])?;
let mut state = State::new(analyzer)?;
```

To render a frame:

```rust
terminal.draw(|frame: &mut Frame| {
    binsider::tui::ui::render(&mut state, frame);
})?;
```

To handle key events:

```rust
let (sender, receiver) = sync::mpsc::channel();
let command = Command::from(&ratatui::crossterm::event::Event::Key(/* */));
state.run_command(command, sender.clone())?;
```

See the [demo example](https://github.com/orhun/binsider/blob/main/examples/demo.rs) for the full code.
