# Commit 04 — `feat: keyboard steering and clean exit`

```
jj desc -m "feat: keyboard steering and clean exit"
```

`cargo run` → steer with arrow keys, `q` exits cleanly.

## Non-blocking input: `poll` before `read`

```rust
if event::poll(Duration::from_millis(0))? && let Event::Key(...) = event::read()? {
```

`event::read()` **blocks** — it sits and waits for a keypress.
`event::poll(0ms)` **does not block** — it checks and returns immediately.

The pattern: poll first, only read if something is waiting.
If nothing is queued, skip input entirely and keep the game running.

## Let chains

```rust
if event::poll(...)? && let Event::Key(KeyEvent { code, .. }) = event::read()? {
```

Two conditions AND'd together. The second uses `let` to simultaneously
check the type AND destructure it. If the event is a Resize event
instead of a Key event, the `let` pattern fails and the block is skipped.
Stable since Rust 1.64.

## `_ => {}` in match

Rust's `match` is exhaustive — you must handle every possible value.
`KeyCode` has dozens of variants (F-keys, home, end, etc.).
`_ => {}` means "do nothing for everything I didn't list."

## UP = (0, -1)

Terminal coordinate system: `(0,0)` is top-left. Y increases *downward*.
Moving up on screen = row number decreasing = delta of -1.
This is the most common source of confusion in terminal graphics.

## Clean exit matters

Previous commits used `loop {}` with no exit, leaving the terminal in
raw mode on Ctrl+C. Now `q` properly calls `disable_raw_mode()` and
`LeaveAlternateScreen` before exiting. Your shell is intact.
