@@lesson m2-tuples Tuples: Grouping a Few Values
@@text
So far every variable held **one** value. But data often comes in groups: a point on a map has an *x* and a *y*; a player has a name and a score. This module is all about **collections**: ways to keep many values together. We start with the simplest one.

## What is a tuple?

A **tuple** groups a fixed number of values together. The values can have **different types**. You write it with parentheses and commas:

```rust
fn main() {
    let point = (3, 7);
    let player = ("Ferris", 42, true);
    println!("{:?}", point);
    println!("{:?}", player);
}
```

Tuples are printed with `{:?}` (the debug format) — `{}` doesn't work for them.

The **type** of a tuple lists the types of its parts: `point` is `(i32, i32)`, and `player` is `(&str, i32, bool)`.

## Getting values out

**By position**, with `.0`, `.1`, `.2`… Positions start at **0** (more on why soon):

```rust
fn main() {
    let player = ("Ferris", 42, true);
    println!("Name: {}", player.0);
    println!("Score: {}", player.1);
    println!("Online: {}", player.2);
}
```

**By destructuring** — unpacking all the parts into separate variables at once. This is usually clearer:

```rust
fn main() {
    let player = ("Ferris", 42, true);
    let (name, score, online) = player;
    println!("{name} has {score} points (online: {online})");
}
```

Use `_` for parts you don't need: `let (name, _, _) = player;`

A mutable tuple can have its parts changed:

```rust
fn main() {
    let mut position = (0, 0);
    position.0 += 5;
    position.1 -= 2;
    println!("{:?}", position);
}
```

## Returning several values from a function

Functions can only return **one** value — but that value can be a tuple! This is the most common use of tuples:

```rust
fn min_and_max(a: i32, b: i32, c: i32) -> (i32, i32) {
    let smallest = a.min(b).min(c);
    let largest = a.max(b).max(c);
    (smallest, largest)
}

fn main() {
    let (lo, hi) = min_and_max(8, 3, 11);
    println!("from {lo} to {hi}");
}
```

## A neat trick: swapping

```rust
fn main() {
    let a = 1;
    let b = 2;
    let (a, b) = (b, a);
    println!("a = {a}, b = {b}");
}
```

## The empty tuple `()`

Remember **unit**, `()`, the "nothing" value that functions return when they don't return anything? It's simply a tuple with zero parts. Now you know where the strange name comes from.

## When to use tuples

Tuples are great for **small, short-lived** groups — like returning two values from a function. For anything more important (like a player with ten properties), you'll soon learn **structs**, which give each part a *name*. `player.name` is much clearer than `player.0`.

@@quiz
`let t = (5, "five", 5.0);` — what is `t.1`?
- `5`
* `"five"`
- `5.0`
? Positions start at 0: `t.0` is `5`, `t.1` is `"five"`, `t.2` is `5.0`.

@@quiz
What's the type of `(10, 'x')`?
- `(i32)`
* `(i32, char)`
- `tuple`
? A tuple type lists each part's type in order.

@@exercise Divide with remainder
Write `divide`, which returns **both** the result of integer division and the remainder as a tuple. For example, `divide(17, 5)` returns `(3, 2)`.
@@starter
fn divide(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

fn main() {
    let (quotient, remainder) = divide(17, 5);
    println!("17 / 5 = {quotient} remainder {remainder}");
}
@@tests
#[test]
fn checks() {
    assert_eq!(divide(17, 5), (3, 2));
    assert_eq!(divide(10, 2), (5, 0));
    assert_eq!(divide(3, 7), (0, 3));
}
@@hint
Return a tuple as the last expression: `(a / b, a % b)`
@@solution
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, remainder) = divide(17, 5);
    println!("17 / 5 = {quotient} remainder {remainder}");
}

@@exercise Distance between points
A point is a tuple `(f64, f64)` of x and y. Write `distance`, which returns the straight-line distance between two points, using Pythagoras: √(dx² + dy²), where dx and dy are the differences in x and y.

Use **destructuring** to unpack the points.
@@starter
fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    todo!()
}

fn main() {
    println!("{}", distance((0.0, 0.0), (3.0, 4.0)));
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    assert!(close(distance((0.0, 0.0), (3.0, 4.0)), 5.0), "(0,0) to (3,4) should be 5");
    assert!(close(distance((1.0, 1.0), (1.0, 1.0)), 0.0), "same point should be 0");
    assert!(close(distance((-1.0, -1.0), (2.0, 3.0)), 5.0), "(-1,-1) to (2,3) should be 5");
}
@@requires
let (
@@hint
`let (x1, y1) = a;` and `let (x2, y2) = b;` — then `let dx = x2 - x1;` and so on.
@@solution
fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    println!("{}", distance((0.0, 0.0), (3.0, 4.0)));
}

@@exercise Seconds to clock
Write `to_clock`, which turns a number of seconds into `(hours, minutes, seconds)`. For example 3725 seconds is 1 hour, 2 minutes and 5 seconds → `(1, 2, 5)`.
@@starter
fn to_clock(total_seconds: u32) -> (u32, u32, u32) {
    todo!()
}

fn main() {
    let (h, m, s) = to_clock(3725);
    println!("{h:02}:{m:02}:{s:02}");
}
@@tests
#[test]
fn checks() {
    assert_eq!(to_clock(3725), (1, 2, 5));
    assert_eq!(to_clock(59), (0, 0, 59));
    assert_eq!(to_clock(3600), (1, 0, 0));
    assert_eq!(to_clock(86399), (23, 59, 59));
}
@@hint
Hours: `total_seconds / 3600`. What's left after the hours: `total_seconds % 3600`. Divide that by 60 for minutes, and use `% 60` for seconds.
@@solution
fn to_clock(total_seconds: u32) -> (u32, u32, u32) {
    let hours = total_seconds / 3600;
    let minutes = total_seconds % 3600 / 60;
    let seconds = total_seconds % 60;
    (hours, minutes, seconds)
}

fn main() {
    let (h, m, s) = to_clock(3725);
    println!("{h:02}:{m:02}:{s:02}");
}
