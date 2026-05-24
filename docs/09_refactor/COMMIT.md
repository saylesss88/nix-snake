# Commit 09 — `refactor: extract functions and add panic hook`

```bash
jj desc -m "refactor: extract functions and add panic hook"
```

`cargo run` should behave exactly like commit 08.

This commit adds **no new gameplay**.  
The goal is to reorganize the code so `main()` reads like a script, and to make sure the terminal is restored even if the program panics.

## What you are practicing

This is a refactor about finding **good boundaries** in a growing program.

By the end of commit 08, `main()` was doing too many jobs:

- terminal setup
- input handling
- autopilot/manual mode switching
- collision handling
- rendering
- terminal cleanup

In this commit, we extract those jobs into named functions.

The point is not “make more functions.”  
The point is “make `main()` show the high-level flow of the program.”

## The shape of the refactor

After this commit, `main()` should read roughly like this:

```rust
setup_terminal()
install panic hook

create game state

while running {
    handle_input(...)
    maybe autopilot(...)
    update snake
    handle collision / reset
    handle food / score / speed
    draw_game(...)
    sleep
}

restore_terminal()
```

That is the target shape.

## What to extract

### `setup_terminal()`

Move the startup terminal code into a function.

Its job is to do terminal-specific setup:

- enable raw mode
- enter the alternate screen
- hide the cursor

This makes startup read like one named step instead of a pile of
crossterm calls.

### `restore_terminal()`

Move the terminal cleanup code into a function.

Its job is to undo setup:

- leave the alternate screen
- show the cursor
- disable raw mode

This is worth extracting because we need it in **two places**:

1. normal exit
2. panic cleanup

That is a strong signal that the behavior deserves its own function.

### `handle_input(mode, snake, running)`

Move event polling and key handling into a function.

Its job is:

- check whether a key is waiting
- quit on `q` / `Esc`
- switch to `Auto` on `a`
- switch to `Manual` on arrow keys
- update the snake direction for arrow keys

This function owns **input interpretation**.  
It does not move the snake, draw the screen, or reset the game.

### `handle_collision(snake, out)`

Move the “death flash + reset” block into a function.

Its job is:

- flash the screen red
- flush output so the flash is visible
- pause briefly
- clear/reset terminal colors
- reset the snake

Notice what it does **not** do: it does not reset score or speed.

Those still happen in `main()`, because that keeps the function focused on
the snake collision response itself.

### `draw_game(out, snake, food, mode, score, speed, term_rows)`

Move all rendering into one function.

Its job is:

- clear the screen
- draw the food
- draw the snake
- draw the status line
- flush the frame

This is a strong extraction because drawing is a separate concern from
game state updates.

## Why `queue!` here?

Inside `draw_game`, use `queue!` instead of repeated `execute!` calls.

The important difference is:

- `execute!` writes commands immediately
- `queue!` buffers commands on the writer
- `flush()` sends them at the end

That makes `draw_game` a natural “build one frame, then flush once”
function, which is cleaner and usually more efficient for terminal rendering.

## Panic hook

This commit also adds one real bug fix.

When a terminal app is in raw mode, a panic can leave the shell in a bad
state: hidden cursor, alternate screen still active, raw input behavior,
or generally “my terminal looks broken.”

To fix that, install a panic hook:

```rust
let original_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
    let _ = restore_terminal();
    original_hook(info);
}));
```

What this does:

- `take_hook()` grabs the current panic hook
- `set_hook(...)` installs a new one
- our new hook restores the terminal first
- then it calls the original hook so Rust still prints the panic message

### Why `move`?

The panic hook may run later, after the current scope is gone.

So the closure must **own** `original_hook`, not borrow it.

That is why this uses `move |info|`.

### Why ignore the `Result`?

`let _ = restore_terminal();`

Because the program is already panicking.

Cleanup is best-effort at that point. If terminal restore fails, there is
not much useful we can do, so we intentionally ignore the error.

## How to solve it

If you are doing this commit yourself, a good order is:

1. Extract `setup_terminal()`
2. Extract `restore_terminal()`
3. Extract `draw_game()`
4. Extract `handle_input()`
5. Extract `handle_collision()`
6. Add the panic hook using `restore_terminal()`

That order works well because setup/cleanup and rendering are the easiest
boundaries to recognize first.

## Refactoring rule of thumb

Extract a function when:

1. The block has one clear purpose.
2. You can name that purpose cleanly.
3. The surrounding function becomes easier to scan after extraction.

Do not extract just to shrink line count.

A refactor is good when it reduces mental load.  
It is bad when it turns one understandable block into ten tiny hops.

## What should stay the same

After this commit, the game should still behave like commit 08:

- same controls
- same autopilot behavior
- same scoring
- same snake growth
- same death/reset behavior

If behavior changes, you probably changed logic instead of just structure.
