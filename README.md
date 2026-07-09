# azptui

A React-flavored component model for building [ratatui](https://ratatui.rs) terminal UIs in Rust.

`azptui` lets you write TUI widgets as plain functions annotated with `#[component]`,
with hooks (`use_*!()`) for accessing per-component state across renders, similar in
spirit to React function components and hooks — but compiled, synchronous, and
rendered straight to a ratatui `Frame`.

> Status: early / provisional. APIs are still taking shape and may change.

## Filetree

```
.
├── Cargo.toml              # Workspace manifest (members: lib, demo)
├── lib/                     # The azptui library crate
│   ├── Cargo.toml
│   ├── macros/               # azptui_macros: proc-macro crate
│   │   └── src/lib.rs           # #[component] attribute macro + use_counter! hook
│   └── src/
│       ├── lib.rs             # Crate root, re-exports the component module and macros
│       ├── events.rs           # Event handling entry point (currently a stub)
│       └── component/          # Core component model
│           ├── mod.rs             # ComponentLocation type, re-exports lifecycle hooks
│           ├── context.rs          # ComponentContext: per-component state (e.g. counter)
│           └── lifecycle.rs        # pre/post_render: looks up/stores each
│                                   # component's context in a thread-local map,
│                                   # keyed by call-site location
└── demo/                    # Example application exercising the library
    └── src/
        ├── main.rs             # Entry point: terminal setup, render loop, tui-logger panel
        └── components/          # Demo-specific components
            ├── mod.rs              # Re-exports the `root` component
            └── root.rs             # `root`/`sub` components demonstrating use_counter!
```

## Crates

- **`azptui`** (`lib/`) — the core library: the component model, per-component
  context/state, and event handling, built on top of ratatui and crossterm.
- **`azptui_macros`** (`lib/macros/`) — the `#[component]` proc-macro that wraps a
  function body with the component lifecycle (`pre_hooks`/`post_hooks`) and asserts
  it returns a `ratatui::widgets::Widget`, plus the `use_counter!()` hook macro.
- **`azptui_demo`** (`demo/`) — a runnable example app demonstrating components and
  a `tui-logger` panel showing log output.

## Running the demo

```sh
cargo run
```
