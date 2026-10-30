@@lesson m1-style Comments, Constants and Clean Code
@@text
Code is written once, but **read many times** — by other people, and by *you*, three months later, when you've forgotten everything about it. Writing code that's easy to read is a core programming skill, not an optional extra.

## Comments

Comments are notes for humans. The compiler ignores them completely.

```rust
fn main() {
    // A line comment: everything after // until the end of the line.
    let speed = 30; // they can go at the end of a line too

    /* A block comment
       can span several lines. */

    println!("{speed}");
}
```

There are also **documentation comments**, which start with `///` and describe the item below them. Tools can turn these into a website of documentation (that's how all the official Rust docs are made):

```rust
/// Returns the area of a rectangle.
fn area(width: f64, height: f64) -> f64 {
    width * height
}

fn main() {
    println!("{}", area(2.0, 3.5));
}
```

### What makes a good comment?

Good comments explain **why**, not **what**. The code already says *what* it does.

```rust,norun
// ❌ Bad: repeats the code
// add 1 to count
count += 1;

// ✔ Good: explains the reason
// The list is numbered from 1 for the user, not from 0.
let position = index + 1;
```

If you need a comment to explain *what* code does, first try to make the code itself clearer — usually with better names or a well-named function.

Commenting out code (`// println!(...)`) is also a handy trick while experimenting or debugging.

## Constants

A **constant** is a value that never changes and is known before the program runs. Declare it with `const`, in `SCREAMING_SNAKE_CASE`, and you **must** write the type:

```rust
const MAX_PLAYERS: u32 = 4;
const GRAVITY: f64 = 9.81;
const GAME_TITLE: &str = "Crab Quest";

fn main() {
    println!("{GAME_TITLE}: up to {MAX_PLAYERS} players, gravity {GRAVITY}");
}
```

Constants are usually declared at the top of the file, outside any function, so the whole program can use them.

### No magic numbers

A **magic number** is an unexplained number in the middle of code:

```rust,norun
let total = price * 1.21;   // ❓ what is 1.21?
```

```rust,norun
const VAT_RATE: f64 = 0.21;
let total = price * (1.0 + VAT_RATE);   // ✔ self-explanatory
```

The constant explains what the number *means*, and if the tax rate ever changes, you change it in exactly one place.

## `const` vs `let`

| | `let` | `let mut` | `const` |
|---|---|---|---|
| Can change? | no | yes | no |
| Type required? | no (inferred) | no | **yes** |
| Value can be calculated while running? | yes | yes | no — must be known at compile time |
| Where | inside functions | inside functions | anywhere, usually top of file |

## Naming conventions — a summary

Rust has firm conventions, and the compiler even warns you if you break them:

| Kind of thing | Style | Example |
|---|---|---|
| variables, functions | `snake_case` | `player_name`, `fn load_level()` |
| constants | `SCREAMING_SNAKE_CASE` | `MAX_SPEED` |
| types (you'll meet these soon) | `UpperCamelCase` | `String`, `PlayerStats` |

## Formatting: let the tools do it

Rust comes with a tool called **rustfmt** that automatically formats code in the official style — indentation, spacing, line breaks. In a Cargo project you run `cargo fmt`, and VS Code (with the **rust-analyzer** extension) can do it every time you save. Nobody argues about formatting in the Rust world: everybody just uses rustfmt.

There's also **Clippy** (`cargo clippy`), which is like a very experienced programmer looking over your shoulder and suggesting improvements. You'll use both when we move to real projects in VS Code.

## Readable code checklist

- Descriptive names: `seconds_remaining`, not `sr` or `x`.
- Small functions that each do one thing, named with a verb.
- Constants instead of magic numbers.
- Consistent indentation (4 spaces).
- Comments that explain *why*.
- Blank lines to separate "paragraphs" of code that belong together.

@@quiz
Which is the best way to write a constant for the number of seconds in an hour?
- `let seconds_in_hour = 3600;`
- `const secondsInHour = 3600;`
* `const SECONDS_PER_HOUR: u32 = 3600;`
? Constants use `const`, `SCREAMING_SNAKE_CASE`, and must have a type annotation.

@@quiz
What is a good comment?
- `// multiply by 2`
* `// Each player gets two turns, so double the round count.`
- `// code`
? Good comments explain **why** something is done, which the code alone can't tell you.

@@exercise Remove the magic numbers
This program works, but it's full of magic numbers. Make it readable:

1. Create these constants at the top of the file and use them instead of the bare numbers: `HOURLY_WAGE` (`f64`, 15.5), `HOURS_PER_DAY` (`f64`, 8.0), `WORK_DAYS_PER_WEEK` (`f64`, 5.0), and `TAX_RATE` (`f64`, 0.2).
2. Give the variables `a`, `b` and `c` descriptive names.

The output must stay exactly the same:

```text
Weekly pay before tax: 620.00
Tax: 124.00
Take-home pay: 496.00
```
@@starter
fn main() {
    let a = 15.5 * 8.0 * 5.0;
    let b = a * 0.2;
    let c = a - b;
    println!("Weekly pay before tax: {:.2}", a);
    println!("Tax: {:.2}", b);
    println!("Take-home pay: {:.2}", c);
}
@@expected
Weekly pay before tax: 620.00
Tax: 124.00
Take-home pay: 496.00
@@requires
const HOURLY_WAGE: f64
const HOURS_PER_DAY: f64
const WORK_DAYS_PER_WEEK: f64
const TAX_RATE: f64
@@hint
Start with `const HOURLY_WAGE: f64 = 15.5;` above `fn main()`. Then rename `a` to something like `gross_pay` everywhere it appears.
@@solution
const HOURLY_WAGE: f64 = 15.5;
const HOURS_PER_DAY: f64 = 8.0;
const WORK_DAYS_PER_WEEK: f64 = 5.0;
const TAX_RATE: f64 = 0.2;

fn main() {
    let gross_pay = HOURLY_WAGE * HOURS_PER_DAY * WORK_DAYS_PER_WEEK;
    let tax = gross_pay * TAX_RATE;
    let take_home_pay = gross_pay - tax;

    println!("Weekly pay before tax: {:.2}", gross_pay);
    println!("Tax: {:.2}", tax);
    println!("Take-home pay: {:.2}", take_home_pay);
}
