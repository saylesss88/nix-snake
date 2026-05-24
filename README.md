# nix-snake — a Rust learning resource

A terminal snake game built commit-by-commit the way you'd actually write it:
get something on screen first, then iterate.

```
cargo run
```

Controls: arrow keys to play · `a` for autopilot · `q` / Esc to quit

---

## How to use this

I created an mdBook out of the "Commits" for this project in
the `docs/` folder. To open the book in your browser, cd to
the `docs/` folder and run `mdbook serve --open`.

Each Chapter in `docs/` is one commit. Read `COMMIT.md` first (the _why_), then
try to write the code yourself before looking at `main.rs`.

The progression:

| Commit | What you see when you `cargo run`        | New concepts                                     |
| ------ | ---------------------------------------- | ------------------------------------------------ |
| 01     | cyan λ appears for 2 seconds             | raw mode, alternate screen, `execute!`           |
| 02     | λ crawls across the screen               | game loop, `clear`, `flush`                      |
| 03     | λ wraps at screen edges                  | `Direction` type, `rem_euclid`, type conversions |
| 04     | arrow keys steer it, `q` exits cleanly   | `poll`, non-blocking input, let chains           |
| 05     | snake has a tail, dies on self-collision | `VecDeque`, `Snake` struct, iterator combinators |
| 06     | food spawns, eating it grows the snake   | `Food` struct, `rand`, the grow trick            |
| 07     | game speeds up as score increases        | `saturating_sub`, `try_from`, method chaining    |
| 08     | `a` key hands control to autopilot       | `Mode` enum, `#[derive(PartialEq)]`, greedy AI   |
| 09     | same as 08, but code is organized        | extract functions, `queue!`, panic hook          |

---

## Architecture notes

**Why VecDeque?** Snake movement = push head at front + pop tail from back. Both
O(1) on a `VecDeque`. `Vec::insert(0, ...)` is `O(n)`.

**Why `(i16, i16)` for direction but `(u16, u16)` for position?** Positions are
terminal coordinates — always non-negative, `crossterm` uses `u16`. Directions
are deltas, need to express `-1`. Different kinds of numbers get different
types.

**Why `rem_euclid` instead of `%`?** `-1 % 40 = -1` in Rust.
`-1_i32.rem_euclid(40) = 39`. Wrapping at screen edges requires true modulo, not
remainder.

**Why `queue!` + one `flush()` instead of `execute!` per command?** `execute!`
syscalls on every command. `queue!` buffers everything and `flush()` sends it
all at once. (one syscall per frame).

**Why a panic hook?** Raw mode + a crash = broken shell. The hook restores the
terminal before the panic message prints.

---

## Suggested exercises (after commit 09)

1. **Food spawns on snake** — `respawn()` doesn't check body positions. Fix:
   accept `&VecDeque<(u16,u16)>` and loop until you find a free cell.

2. **Greedy AI traps itself** — watch where it dies. Improve it with BFS to find
   a safe path instead of just heading toward food.

3. **Fixed start position** — snake always starts at (10,10). Start at center:
   `(term_cols / 2, term_rows / 2)`.

4. **Wall mode** — add a `Mode::Wall` where hitting the edge kills you.

5. **Frame timing drift** — `sleep(speed)` doesn't account for game logic time.
   Use `std::time::Instant` to measure and compensate.

6. **High score** — persist to `~/.local/share/nix-snake/highscore` using
   `std::fs::read_to_string` and `std::fs::write`.
