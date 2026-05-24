# Commit 05 — `feat: Snake struct with VecDeque body`

```
jj desc -m "feat: Snake struct with VecDeque body"
```

`cargo run` → steerable snake with a 3-segment tail, dies on self-collision.

## Why VecDeque?

It always helps me to actually see what I'm working with. Here's an example of a `VecDeque<(u16, u16)>`:

```rs
use std::collections::VecDeque;

let body = VecDeque::from(vec![(1, 2), (3, 6), (5, 8)]);
```

If this represents a character or snake moving across a 2D grid, you can visualize those three elements living in your
queue like this:

```text
FRONT (Head)                     BACK (Tail)
          ┌──────────────┬──────────────┬──────────────┐
Index:    │      0       │      1       │      2       │
          ├──────────────┼──────────────┼──────────────┤
Value:    │    (1, 2)    │    (3, 6)    │    (5, 8)    │
          └──────┬───────┴──────┬───────┴──────┬───────┘
                 │              │              │
                 ▼              ▼              ▼
            Grid: X=1,Y=2  Grid: X=3,Y=6  Grid: X=5,Y=8
```

When the character moves forward, you'll just do `body.push_front((new_x, new_y))` to grow the head, and `body.pop_back()` to drop the trailing piece of the tail.

If you remember from The Rust Programming Language Book, the
standard stack is (First-In, Last-Out). You pile plates on top,
and you have to take the top plate off first. You can't safely
grab a plate from the bottom without a massive disaster.

A `VecDeque` breaks that rule completely. It's a Double-Ended
Queue.

Instead of a stack of plates on a table, imagine a sleeve of
plastic cups in a dispenser at a water cooler:

- You can load a cup into the top, or load a cup into the bottom.
- You can pull a cup out of the top, or pull a cup out of the bottom.

This is also just an analogy. Because a snake can grow to 5
segments, 50 segments, etc depending on how good you are at
the game, its size is **dynamic**. In Rust, anything that
changes size dynamically must go on the heap.

Think of a `VecDeque` (short for Vector Double-Ended Queue, pronounced "deck") as a standard `Vec`, but with a superpower:
it lets you add or remove items from the **front** just as
fast as you can from the **back**.

**The Problem it Solves**

If you have a normal `Vec` and use `push` or `pop`, it's incredibly fast because Rust is just messing with the very end of the array. But if you try to use `vec.insert(0, item)` or `vec.remove(0)`, Rust has to manually shift every single other item in memory one slot to the right or left. If your `Vec` has 10,000 items, that is incredibly slow.

A `VecDeque` solves this. It gives you 4 core operations that all run in $O(1)$ time (nearly instantaneous, no matter how big it grows):

- `push_back` / `pop_back` (Just like a normal `Vec`)
- `push_front` / `pop_front` (Super fast addition/removal at the start).

Snake movement every frame:
1. Add new head position at the **front**
2. Remove old tail position from the **back**

With `Vec`, inserting at position 0 is O(n) — every element shifts right.
With `VecDeque` (a ring buffer), both ends are O(1) amortized.

For a 3-segment snake this doesn't matter. For a snake that fills the
screen it does. More importantly, `VecDeque` makes the intent clear:
this is a double-ended queue, not a random-access array.

## The move + grow trick

```rust
self.body.push_front(new_head); // add head
self.body.pop_back();           // drop tail → net length: same
```

When the snake eats food, we skip `pop_back()`. The deque grows by 1.
That's the entire grow mechanic. No special "growing" state needed.

## `set_direction` — blocking reversal

```rust
if (self.dir.0 + new_dir.0 != 0) || (self.dir.1 + new_dir.1 != 0) {
```

Opposite directions sum to (0, 0). This single check catches all four
cases (RIGHT↔LEFT, UP↔DOWN) without listing them explicitly.

## `const fn`

`set_direction` is marked `const fn` — it can be evaluated at compile
time. The compiler enforces this: the function body can only contain
operations valid at compile time. It's a way of saying "this function
has no heap allocations, no I/O, no runtime surprises."

## `expect` vs `unwrap`

Both panic if the `Option` is `None`. `expect("message")` prints your
message in the panic output. `unwrap()` prints "called unwrap on None."
Always use `expect` — it tells you *what* was None when it matters most.

## `.iter().skip(1).any()`

```rust
self.body.iter().skip(1).any(|&(x, y)| x == hx && y == hy)
```

Iterator chain: iterate all segments, skip the head (index 0),
check if any remaining segment matches the head position.
`any()` short-circuits on first match. The `|&(x, y)|` destructures
the reference that `iter()` yields — `&` dereferences, `(x, y)` unpacks.
