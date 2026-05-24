# Commit 02 — `feat: game loop, λ crawls right`

```
jj desc -m "feat: game loop, λ crawls right"
```

`cargo run` → λ slides across row 10 and the program exits.

## The game loop pattern

```
loop:
  clear screen
  update state
  draw state
  sleep
```

Every game loop in existence is a variation of this. The order matters:
clear *before* drawing so you're never reading stale state.

## `out.flush()`

`execute!` writes to an internal buffer. `flush()` forces that buffer
out to the actual terminal. Without it, you might see nothing, or worse,
see it all at once when the buffer fills. Call it once per frame, at the
end of drawing.

## Why `u16` for position?

Crossterm's `cursor::MoveTo(col, row)` takes `u16`. Terminal dimensions
are reported as `u16`. Using the same type for position avoids casts
at every draw call. You'll see why this matters when we add direction.
(Direction needs negative numbers, so it gets a different type).

## What you should notice

The λ leaves no trail. The `Clear` on each frame erases the previous
position. Try commenting out the `Clear` line and running again. You'll
see why it's there.
