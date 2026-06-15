# Commit 01 — `feat: draw λ on alternate screen`

I will use Jujutsu VCS for this, it's describe the change before you make it
works well for learning resources.

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

Always check out the docs for the crates you use as dependencies:

- [crossterm](https://docs.rs/crossterm/latest/crossterm/)

- [rand](https://docs.rs/rand/latest/rand/)

## What's happening

**Raw mode** disables the terminal's normal line-buffering and echo. Without it,
keypresses sit in a buffer until you hit Enter, and every character you type
appears on screen. A game needs neither.

- [Raw Mode](https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode)

> In the real solution, make sure you always disable raw mode, show the cursor
> again, and leave the alternate screen before exiting.” That prevents the
> classic first-terminal-app bug where the terminal is left in a weird state.

**Alternate screen** is like a second framebuffer. Your game runs on a clean
slate; when you leave (`LeaveAlternateScreen`), the user's previous terminal
content comes back exactly as it was. Without it, your output scrolls into their
shell history.

- [Struct LeaveAlternateScreen](https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html)

- To better understand **Canonical Mode vs Raw Mode**, I suggest reading
  [hecto-chapter-2](https://philippflenker.com/hecto-chapter-2/)

**`cursor::Hide`** removes the blinking cursor so it doesn't sit on top of your
λ.

## `execute!`

- [Macro execute](https://docs.rs/crossterm/latest/crossterm/macro.execute.html)

```rust
execute!(stdout(), Command1, Command2, Command3)?;
```

Writes `crossterm` commands to the given writer and flushes immediately. Each
command translates to an ANSI escape sequence.(bytes sent to the terminal that
tell it "move cursor here", "set this color", etc.)

- [crossterm docs.rs Examples](https://docs.rs/crossterm/latest/crossterm/#examples-2)

## The `?` operator

Every fallible function returns `std::io::Result<T>`. The `?` at the end means:
if this returned `Err`, return that error from the current function immediately.
In `main()` returning `std::io::Result<()>`, errors are printed and the process
exits non-zero.

## Why `stdout()` every time and not a variable?

You'll notice we call `stdout()` multiple times. Next commit we'll fix this:
call it once, store it in `let mut out = stdout()`, reuse it. For now, keep it
simple.

## Testing terminal output

You can't easily assert on what a real terminal displays, but you don't have to.
`execute!` doesn't care whether it's writing to `stdout()` or a `Vec<u8>`, it
just needs something that implements `std::io::Write`. (A vector does).

```rs
let mut buf: Vec<u8> = Vec::new();
execute!(&mut buf, Print("λ"))?;
// buf now contains the raw bytes that would have gone to the terminal
```

That's the whole trick. Swap `stdout()` for `&mut buf`, then inspect what landed
in the vector.

### Your first test

With that in mind, try writing this test yourself before reading the solution.
You need to:

1. Create an empty Vec<u8>
2. Run the draw commands against it using `execute!`
3. Assert that buf isn't empty
4. Convert buf to a string and assert it contains `'λ'`

Resources, you'll use all of these:

- [Struct Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)

- [Macro execute](https://docs.rs/crossterm/latest/crossterm/macro.execute.html)

- [Macro assert](https://doc.rust-lang.org/std/macro.assert.html)

- [from_utf8_lossy](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)

- [crossterm EnterAlternateScreen](https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html)

- [crossterm Cursor](https://docs.rs/crossterm/latest/crossterm/cursor/index.html)

- [crossterm LeaveAlternateScreen](https://docs.rs/crossterm/latest/crossterm/terminal/struct.LeaveAlternateScreen.html)

`#[test]` marks a function as a test run by `cargo test`, while `#[cfg(test)]`
conditionally compiles the module only in test builds.

```rs
#[cfg(test)]
mod tests {
    use super::*; // brings main.rs imports into scope

    #[test]
    fn your_test_name_here() {
        // your code here
    }
}
```

<details>
<summary> Solution </summary>

```rs
#[cfg(test)]
mod tests {
    use crossterm::{cursor, execute, style::{Color, Print, SetForegroundColor}, terminal};

    #[test]
    fn draw_sequence_produces_output() {
        let mut buf: Vec<u8> = Vec::new();

        execute!(
            &mut buf,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            cursor::MoveTo(10, 10),
            SetForegroundColor(Color::Cyan),
            Print("λ"),
            terminal::LeaveAlternateScreen,
            cursor::Show,
        ).unwrap();

        assert!(!buf.is_empty(), "expected ANSI bytes, got nothing");

        let out = String::from_utf8_lossy(&buf);
        assert!(out.contains('λ'), "expected λ in output");
    }
}
```

</details>
