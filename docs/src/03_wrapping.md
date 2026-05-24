# Commit 03 — `feat: wrapping movement with Direction type`

```
jj desc -m "feat: wrapping movement with Direction type"
```

`cargo run` → λ wraps around screen edges indefinitely.

## The type conversion chain

```
u16 ──i32::from()──▶ i32 ──rem_euclid()──▶ i32 ──as u16──▶ u16
i16 ──i32::from()──▶ i32
```

`i32::from(u16)` and `i32::from(i16)` are **lossless**. The compiler
only allows `From` impls that can never lose data. If you tried
`i32::from(u64)` it wouldn't compile, because u64 doesn't fit in i32.

`as u16` at the end **is** a truncating cast. (it can silently drop bits).
We allow it only because `rem_euclid(width)` guarantees the result is
in `[0, width)`, which fits in u16. The `#[allow(clippy::...)]` is
documenting "I verified this is safe."

## `terminal::size()`

Returns `(columns, rows)` as `(u16, u16)`. Called once at startup here.
Later we'll call it every frame to handle terminal resize.

## The `loop` with no exit

This loop runs forever. (you kill it with Ctrl+C). That leaves the
terminal in raw mode (broken). Next commit we'll add proper input
handling with a clean exit. For now: if your terminal gets stuck,
run `reset`.
