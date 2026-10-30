@@lesson m1-numbers Numbers and Data Types
@@text
Every value in Rust has a **type**. The type tells the compiler what kind of data it is — and therefore what you can do with it. You can add two numbers, but what would it mean to add a number to `true`? The compiler uses types to catch mistakes like that before your program runs.

So far Rust has worked out types for you (this is called **type inference**). But you can also write the type yourself, after a colon:

```rust
fn main() {
    let age: i32 = 30;
    let height: f64 = 1.82;
    println!("{age} {height}");
}
```

## Integers — whole numbers

Integers are numbers without a fractional part: `-3`, `0`, `42`. Rust has several integer types, which differ in **size** (how much memory they use, and therefore how big a number they can hold) and in whether they can be **negative**:

| Type | Can be negative? | Range |
|---|---|---|
| `i8` | yes | -128 to 127 |
| `u8` | no | 0 to 255 |
| `i16` | yes | -32,768 to 32,767 |
| `u16` | no | 0 to 65,535 |
| `i32` | yes | about ±2.1 billion |
| `u32` | no | 0 to about 4.3 billion |
| `i64` | yes | about ±9.2 quintillion |
| `u64` | no | 0 to about 18 quintillion |

- `i` means **signed** (can have a minus sign), `u` means **unsigned** (zero or positive only).
- The number is how many **bits** it uses. A bit is a single 0 or 1; 8 bits make a **byte**.
- There's also `i128`/`u128` for huge numbers, and `isize`/`usize`, whose size matches your computer (64-bit on modern PCs). `usize` is used for counting things and positions in lists — you'll see it a lot.

**If you don't specify, integers are `i32`.** That's a good default.

Each type knows its limits:

```rust
fn main() {
    println!("u8 goes from {} to {}", u8::MIN, u8::MAX);
    println!("i32 goes from {} to {}", i32::MIN, i32::MAX);
    let big = 1_000_000; // underscores make big numbers readable
    println!("{big}");
}
```

## Floating-point numbers — decimals

Numbers with a decimal point are **floats**: `3.14`, `-0.5`, `2.0`. There are two types: `f32` and `f64`. **The default is `f64`**, which is more precise; use it unless you have a reason not to.

A float literal must contain a `.`: `2.0` is a float, `2` is an integer.

> **Floats aren't perfectly exact.** Computers store floats in binary, so some decimals (like 0.1) can only be approximated. Try printing `0.1 + 0.2` — you'll get `0.30000000000000004`. This is true in every programming language. It's why you should never use floats for money (use integers of cents instead).

@@quiz
You want to store a person's age. It's never negative and never above 150. Which type fits best?
- `f64`
* `u8`
- `i8`
? `u8` holds 0–255: no negatives, and plenty of room. (`i32` would also be fine — the default is rarely wrong.) `f64` would allow ages like 30.7, which doesn't make sense here.

@@text
## Integer division

Here's a classic surprise. What's `7 / 2`?

```rust
fn main() {
    println!("7 / 2 = {}", 7 / 2);
    println!("7 % 2 = {}", 7 % 2);
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
}
```

When you divide two **integers**, the result is an integer too: the fractional part is simply cut off. `7 / 2` is `3`, not `3.5`.

The `%` operator (called **remainder** or **modulo**) gives you what's left over: `7 % 2` is `1`, because 7 = 2 × 3 + **1**. It's surprisingly useful:

- `n % 2 == 0` tells you whether `n` is even.
- `total_seconds % 60` gives the seconds part of a time.

## Types don't mix automatically

Rust will **not** automatically convert between number types. This doesn't compile:

```rust
fn main() {
    let apples: i32 = 5;
    let price: f64 = 0.5;
    let total = apples * price;
    println!("{total}");
}
```

You must convert explicitly with `as`:

```rust
fn main() {
    let apples: i32 = 5;
    let price: f64 = 0.5;
    let total = apples as f64 * price;
    println!("{total}");
}
```

This strictness feels annoying at first, but it prevents subtle bugs where a value silently loses precision.

Converting a float to an integer with `as` **cuts off** the decimals (it doesn't round): `3.99 as i32` is `3`.

## Overflow

What happens if a number gets too big for its type? Say a `u8` holding 255 gets 1 added. When you're developing (as in this app), Rust **panics** — it stops the program with an error like `attempt to add with overflow`. In some other languages the number would silently wrap around to 0, causing very confusing bugs.

```rust,norun
let mut level: u8 = 255;
level = level + 1; // 💥 panic: attempt to add with overflow
```

The lesson: pick a type big enough for the values you'll store. When in doubt, `i32`, `i64` or `u64`.

@@quiz
What is `17 / 5` in Rust (both are integers)?
- `3.4`
* `3`
- `4`
? Integer division cuts off the fraction: 17 / 5 = 3.4 → `3`. And `17 % 5` would be `2`.

@@quiz
What is `20 % 6`?
- `3`
* `2`
- `0`
? 20 = 6 × 3 + **2**, so the remainder is 2.

@@exercise Minutes to hours
A movie is `135` minutes long. Using **integer division** `/` and **remainder** `%`, calculate the hours and the leftover minutes, then print:

```text
135 minutes is 2 hours and 15 minutes
```

Don't type 2 and 15 yourself — calculate them from `total_minutes`, so the program would also work for other lengths.
@@starter
fn main() {
    let total_minutes = 135;
    // calculate hours and minutes here

}
@@expected
135 minutes is 2 hours and 15 minutes
@@requires
%
@@hint
There are 60 minutes in an hour. `total_minutes / 60` gives the whole hours. `total_minutes % 60` gives what's left over.
@@solution
fn main() {
    let total_minutes = 135;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    println!("{total_minutes} minutes is {hours} hours and {minutes} minutes");
}

@@exercise A fair average
Four test scores are stored as integers. Calculate their average **as a float**, so that the result isn't cut off. The program should print:

```text
Total: 31
Average: 7.75
```

Integer division would give you `7` — convert with `as f64` first.
@@starter
fn main() {
    let a = 7;
    let b = 8;
    let c = 10;
    let d = 6;
    let total = a + b + c + d;
    let average = total / 4;
    println!("Total: {total}");
    println!("Average: {average}");
}
@@expected
Total: 31
Average: 7.75
@@requires
as f64
@@hint
`total as f64 / 4.0` — both sides of the `/` must be floats.
@@solution
fn main() {
    let a = 7;
    let b = 8;
    let c = 10;
    let d = 6;
    let total = a + b + c + d;
    let average = total as f64 / 4.0;
    println!("Total: {total}");
    println!("Average: {average}");
}
