# azptui

A React-flavored component model for building [ratatui](https://ratatui.rs) terminal UIs in Rust.

`azptui` lets you write TUI widgets as plain functions annotated with
`#[component]`, with hooks for state that persists across renders and for
declaring event handlers inline — similar in spirit to React function
components and hooks, but compiled, synchronous, and rendered straight to a
ratatui `Frame`.

> Status: early / provisional. APIs are still taking shape and may change.

## Example

```rust
use azptui::{on_event, use_state};
use crossterm::event::KeyCode;
use ratatui::widgets::List;

#[azptui::component]
fn counter() -> List<'static> {
    let (count, set_count) = use_state!(0);

    on_event!(
        |e| e.is_key() && e.as_key_event().unwrap().code == KeyCode::Char('+'),
        move |_| set_count(count + 1),
    );

    List::new([format!("count: {count} (press '+')")])
}
```

Components are driven by an ordinary render loop: draw, then feed the next
terminal event to the library.

```rust
ratatui::run(|terminal| {
    loop {
        terminal
            .draw(|frame| frame.render_widget(counter(), frame.area()))
            .unwrap();
        azptui::events::handle_event(crossterm::event::read().unwrap());
    }
});
```

## API

### `#[azptui::component]`

Turns a function into a component. The function may take arguments and return
any type — typically a ratatui widget, but sub-components returning plain
values (e.g. `String`) compose fine too. Inside the body the hook macros
below become available.

Each component invocation is identified by its **call site** (via
`#[track_caller]` / `std::panic::Location`). Calling the same component
function from two different places creates two independent instances, each
with its own persistent context.

### `use_state!(initial)`

Returns `(value, setter)`:

- `value` is a clone of the current state (`T: Any + Clone`), initialized to
  `initial` on the first render.
- `setter` is an `impl Fn(T)` that stores a new value and marks the component
  dirty. Move it into an `on_event!` handler to update state from events.

State identity is the `use_state!` call site, so multiple `use_state!` calls
in one component each get their own slot.

### `on_event!(filter, handler)`

Registers an event handler for this component:

- `filter: Fn(&crossterm::event::Event) -> bool` decides whether the handler
  runs.
- `handler: Fn(&Event)` performs the effect, typically calling a state setter
  it has captured.

Every handler whose filter matches a given event runs. Handlers are
re-registered on each render (keyed by call site), so closures always capture
the values of the most recent render.

### `azptui::events::handle_event(event)`

The dispatch entry point: call it with each `crossterm` event from your input
loop. It runs all registered handlers whose filters match and prunes handlers
whose components have dropped them.

## How it works

- The `#[component]` macro wraps the function body with a lifecycle pair:
  `pre_render` fetches (or creates) the component's `ComponentContext` from a
  thread-local map keyed by call-site `Location`, and `post_render` stores it
  back after the body runs.
- `ComponentContext` owns the component's state cells
  (`Rc<RefCell<dyn Any>>`), its registered event handlers, and a dirty flag
  (set by state setters; reserved for future render memoization).
- Event handlers are held by the context as `Rc`s and registered globally as
  `Weak`s, so replacing or dropping a context automatically retires its
  handlers on the next dispatch.

Everything lives in thread-locals: the component model is single-threaded by
design, matching a typical TUI event loop.

## Crates

- **`azptui`** (`lib/`) — the core library: the component model,
  per-component context/state, and event dispatch, built on top of ratatui
  and crossterm.
- **`azptui_macros`** (`lib/macros/`) — the `#[component]` attribute macro
  plus the `use_state!` and `on_event!` hook macros.
- **`azptui_demo`** (`demo/`) — a runnable example app demonstrating components and
  a `tui-logger` panel showing log output.

## Known limitations

- A component called inside a loop is a single call site, so all iterations
  share one context (no equivalent of React's `key` yet).
- Contexts are never garbage-collected: a component that stops rendering
  keeps its state and handlers alive.
- The `initial` argument to `use_state!` is evaluated on every render, even
  once state exists.
