@@lesson m5-panic When Things Go Wrong: panic!
@@text
Real programs deal with a messy world: users type nonsense, files go missing, the network drops. Good programs **expect problems and handle them gracefully**. This module is about doing that well.

Rust divides errors into two kinds:

| Kind | Example | Rust's tool |
|---|---|---|
| **Unrecoverable** — a bug; the program is in a state that should be impossible | an index past the end of a list, a broken assumption | `panic!` |
| **Recoverable** — something that can reasonably happen and should be handled | the user typed "abc" instead of a number; a file doesn't exist | `Result` (next lesson) |

## `panic!`

A **panic** stops the program immediately with an error message. You can trigger one yourself:

```rust
fn main() {
    println!("Starting...");
    panic!("Something went terribly wrong!");
    println!("This line never runs");
}
```

The message tells you *where* it happened (`main.rs:3:5`) and *why*. You've already seen automatic panics:

```rust,norun
let v = vec![1, 2, 3];
v[10];                  // index out of bounds
let n: Option<i32> = None;
n.unwrap();             // called `Option::unwrap()` on a `None` value
let x: u8 = 255;
x + 1;                  // attempt to add with overflow (in debug builds)
```

## Should I panic?

Panic when continuing would be **wrong or impossible**, and the problem is a **bug**, not a normal situation:

- ✔ An internal rule of your program was broken ("this list can never be empty here").
- ✔ In quick experiments, examples, and tests.
- ❌ The user typed something invalid — that's normal; tell them and ask again.
- ❌ A file is missing — that's normal; show a helpful message.

A panic in a GUI app closes the whole window with no explanation — a terrible experience for users. So in real programs, most problems should be handled with `Result`.

## `expect`: unwrap with a reason

When you *are* sure a value exists, `expect` documents why — and if you're wrong, the panic message says exactly what assumption failed:

```rust
fn main() {
    let config = vec![("volume", 80)];
    let first = config.first().expect("config always has at least one entry");
    println!("{:?}", first);
}
```

## Defensive checks: `assert!`

`assert!(condition, "message")` panics if the condition is false. Use it to check your own assumptions while developing:

```rust
fn average(values: &[f64]) -> f64 {
    assert!(!values.is_empty(), "average() needs at least one value");
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

fn main() {
    println!("{}", average(&[1.0, 2.0, 3.0]));
    println!("{}", average(&[]));
}
```

`assert_eq!(a, b)` checks that two values are equal — that's what the exercise checker has been using all along!

@@quiz
A user types their age as "twenty". What should your program do?
- `panic!`, because the input is wrong
* Show a friendly message and ask again (handle it as a recoverable error)
- Ignore it silently
? Bad user input is a normal situation, not a bug. Handle it with `Result`.

@@quiz
What's the advantage of `expect("...")` over `unwrap()`?
- It never panics
* If it panics, the message explains which assumption was wrong
- It's faster
? Both panic on `None`/`Err`; `expect` gives a meaningful message.

@@exercise Guard the input
Write `percentage(part, whole)`, which returns `part / whole * 100`. It should **panic with a clear message** if `whole` is 0 — use `assert!` with the message `"whole must not be zero"`.

(This is a case where the caller is a programmer, and passing 0 is a bug in *their* code.)
@@starter
fn percentage(part: f64, whole: f64) -> f64 {
    todo!()
}

fn main() {
    println!("{}%", percentage(1.0, 4.0));
}
@@tests
#[test]
fn works() {
    assert_eq!(percentage(1.0, 4.0), 25.0);
    assert_eq!(percentage(3.0, 3.0), 100.0);
}

#[test]
#[should_panic(expected = "whole must not be zero")]
fn panics_on_zero() {
    percentage(1.0, 0.0);
}
@@requires
assert!
@@hint
`assert!(whole != 0.0, "whole must not be zero");` then `part / whole * 100.0`.
@@solution
fn percentage(part: f64, whole: f64) -> f64 {
    assert!(whole != 0.0, "whole must not be zero");
    part / whole * 100.0
}

fn main() {
    println!("{}%", percentage(1.0, 4.0));
}
