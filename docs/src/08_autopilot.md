# Commit 08 — `feat: autopilot mode with Mode enum`

```
git commit -m "feat: autopilot mode with Mode enum"
```

`cargo run` → starts in AUTO, watch it play. Press arrows to take over, `a` to hand back.

## `#[derive(PartialEq)]`

`#[derive(...)]` is a procedural macro that runs at compile time and
generates trait implementations for you. `PartialEq` enables `==` and `!=`.

The generated code compares enum *discriminants* (the internal integer
tag for "which variant is this"). For `Mode`, `Auto == Auto` is true,
`Auto == Manual` is false.

You can only derive `PartialEq` if all fields also implement `PartialEq`.
`Mode` has no fields, so it trivially qualifies.

## Greedy autopilot

```rust
if hx < food_x && self.dir != LEFT  { self.set_direction(RIGHT); }
else if hx > food_x ...
else if hy < food_y ...
else if hy > food_y ...
```

Moves toward food along the X axis first, then Y. This produces a
right-angle path. It works until the snake is long enough to block
its own path — watch where it dies. That failure mode is the
interesting part: the AI has no lookahead.

An improvement would be BFS (breadth-first search) to find the
shortest safe path. That's a meaningful next project.

## autopilot runs BEFORE update

```rust
if mode == Mode::Auto { snake.autopilot(food.x, food.y); }
// ...
snake.update(w, h - 1);
```

The direction change from `autopilot` must take effect before the
snake moves. If you called `autopilot` after `update`, the direction
change wouldn't show until the following frame — one step late.

## `mode == Mode::Auto`

This works because of `#[derive(PartialEq)]`. Without it, the compiler
would say "binary operation `==` cannot be applied to type `Mode`."
