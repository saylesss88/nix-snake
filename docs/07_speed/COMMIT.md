# Commit 07 — `feat: speed scaling with score`

```
jj desc -m "feat: speed scaling with score"
```

`cargo run` → game gets faster as you eat food, resets on death.

## Speed formula

```rust
let reduction = speed.as_millis() / 20;   // 5% of current speed
speed = speed
    .saturating_sub(Duration::from_millis(reduction))
    .max(Duration::from_millis(25));
```

Each food eaten: speed -= 5% of current speed. This is exponential decay — the
reductions get smaller as speed gets faster:

- 100ms → 95ms (-5ms)
- 95ms → 90.25ms (-4.75ms)
- ...
- Asymptotically approaches 0, floored at 25ms

## Method chaining

```rust
speed = speed.saturating_sub(...).max(...)
```

Each method returns a `Duration`. The next method is called on that returned
value. This is idiomatic Rust for transformations.(reads left to right: "take
speed, subtract, then take the max with 25ms.")

## `u128` → `u64`

`Duration::as_millis()` returns `u128`. `Duration::from_millis()` takes `u64`.
You can't pass one to the other directly.

- `value as u64` silently truncates if value > `u64::MAX` (silent bug)
- `u64::try_from(value)` returns `Result::Err` if it can't fit

For a speed value that starts at 100 and only decreases, u64 will never
overflow. `expect()` documents this reasoning: if you ever see that panic, it
means the invariant was violated.

## The game is complete

At this point you have a fully playable snake game. The next commits add polish:
autopilot mode and extracted helper functions.
