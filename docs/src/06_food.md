# Commit 06 — `feat: food spawning and eating`

```
jj desc -m "feat: food spawning and eating"
```

`cargo run` → fully playable snake. Score displayed at bottom.

## The grow trick

```rust
// update() does this every frame:
push_front(new_head)
pop_back()           // ← normally drops the tail

// on eat, we push a tail duplicate BEFORE the next update():
push_back(tail_copy)

// next frame's update():
push_front(new_head)
pop_back()           // drops the duplicate, original tail survives
                     // net length: +1
```

No "growing" state flag, no special case in `update()`. The deque handles it
naturally.

## `if let` chains for the eat check

```rust
if let Some(&(hx, hy)) = snake.body.front()
    && hx == food.x
    && hy == food.y
```

`body.front()` returns `Option<&(u16, u16)>`. We destructure it with
`if let Some(&(hx, hy))`. The `&&` chains additional conditions — all must be
true for the block to run. If the body is somehow empty, the `None` case is
handled safely (block skipped).

This can be confusing at first: the & reference operator appears to be
destructuring a reference rather than creating one.

The secret to unlocking this is to recognize that & does two entirely opposite
things depending on which side of the equal sign it is on:

The Two faces of `&`

```rs
let data = &value;   // 1. EXPRESSION: Creates a reference
let &value = data;   // 2. PATTERN: Destructures a reference
```

1. On the RIGHT side (An Action/Expression)

When you use `&` in normal code, it's an **operator**. It means "Give me the
address of this thing."

```rs
let x = 5;
let y = &x;  // y is now a pointer to x (&i32)
```

2. On the LEFT side (A Blueprint / Pattern)

When you use `&` inside an `if let` or `match`, it is not an operator. It is a
structural blueprint (a pattern). You are telling Rust what the data already
looks like, so Rust can unpack it.

Think of it like an allergy warning on a food box: If a box says [Contains:
Peanut], it doesn't mean the box is adding a peanut. It means: "Hey, if you open
this up, you will find a peanut inside."

When you write:

```rs
if let Some(&(hx, hy)) = snake.body.front()
```

The data coming out of `front()` is an enum variant containing a reference:
`Some(&Tuple)`. Your pattern on the left says: "I expect a box labeled `Some`
containing a reference to a tuple. Rust, match my blueprint, peel those layers
off, copy the inner scalar values (thanks to the `Copy` trait on `u16`), and
give me the raw `hx` and `hy` variables."

## `saturating_sub`

```rust
rng.random_range(0..height.saturating_sub(1))
```

`height - 1` on a `u16` would panic (debug) or wrap to 65535 (release) if
`height` were 0. `saturating_sub` clamps to 0 instead. For any real terminal
this is unreachable, but it makes the intent explicit: "I want height minus 1,
but never negative."

## `rand::rng()` and `mut rng`

`rand::rng()` returns the thread-local RNG. Calling `.random_range()` advances
its internal state (that's how it generates different numbers each call) —
mutation. Hence `mut rng`. The `RngExt` trait (imported via `use rand::RngExt`)
provides the `.random_range()` and `.random_bool()` methods.
