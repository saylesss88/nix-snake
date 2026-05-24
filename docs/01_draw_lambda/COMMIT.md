# Commit 01 — `feat: draw λ on alternate screen`

I will use Jujutsu VCS for this, it's describe the change
before you make it works well for learning resources.

Describe the change we're going to make:

```
jj desc -m "feat: draw λ on alternate screen"
```

`cargo run` → you see a cyan λ for 2 seconds, then your terminal is restored.

## Initialize the Project

```sh
cargo new nix-snake
cargo add crossterm rand
```

- If you want to practice conventional commits, make
  `feat: draw λ on alternate screen` your second commit with your first being
  something like `chore: initialize project with dependencies`.

## What's happening

**Raw mode** disables the terminal's normal line-buffering and echo. Without it,
keypresses sit in a buffer until you hit Enter, and every character you type
appears on screen. A game needs neither.

**Alternate screen** is like a second framebuffer. Your game runs on a clean
slate; when you leave (`LeaveAlternateScreen`), the user's previous terminal
content comes back exactly as it was. Without it, your output scrolls into their
shell history.

**`cursor::Hide`** removes the blinking cursor so it doesn't sit on top of your
λ.

## `execute!`

```rust
execute!(stdout(), Command1, Command2, Command3)?;
```

Writes `crossterm` commands to the given writer and flushes immediately. Each
command translates to an ANSI escape sequence — bytes sent to the terminal that
tell it "move cursor here", "set this color", etc.

- To better understand **Canonical Mode vs Raw Mode**, I suggest reading
  [hecto-chapter-2](https://philippflenker.com/hecto-chapter-2/)

## The `?` operator

Every fallible function returns `std::io::Result<T>`. The `?` at the end means:
if this returned `Err`, return that error from the current function immediately.
In `main()` returning `std::io::Result<()>`, errors are printed and the process
exits non-zero.

## Why `stdout()` every time and not a variable?

You'll notice we call `stdout()` multiple times. Next commit we'll fix this:
call it once, store it in `let mut out = stdout()`, reuse it. For now, keep it
simple.
