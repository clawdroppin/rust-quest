@@lesson m1-functions Functions
@@text
So far all your code has lived inside `main`. Real programs are split into many **functions**: named blocks of code that each do one job. Functions are the most important tool for **organizing** code.

Why use functions?

- **Reuse** — write the code once, use it many times.
- **Naming** — `calculate_tax(price)` explains itself; ten lines of maths don't.
- **Breaking problems down** — a big task becomes several small, simple ones. This is the essence of programming.
- **Testing** — a small function is easy to check on its own (that's exactly how this app checks your exercises from now on!).

## Defining and calling a function

```rust
fn say_hello() {
    println!("Hello!");
}

fn main() {
    say_hello();
    say_hello();
    println!("Done");
}
```

- `fn say_hello() { ... }` **defines** a function — it describes what the function does, but doesn't run it.
- `say_hello();` **calls** the function — *now* its code runs. When it finishes, the program continues right after the call.

The program still starts at `main`. Functions can be defined in any order in the file, before or after `main`.

## Parameters: giving a function input

**Parameters** are variables that receive values when the function is called. In Rust, you **must** write each parameter's type:

```rust
fn greet(name: &str, times: i32) {
    println!("Hello {name}! (x{times})");
}

fn main() {
    greet("Ferris", 3);
    greet("Corro", 1);
}
```

The values you pass in a call (`"Ferris"`, `3`) are called **arguments**. They're matched to the parameters in order.

## Return values: getting a result back

A function can calculate a value and give it back to the caller. You write the **return type** after an arrow `->`:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(2, 3);
    println!("2 + 3 = {sum}");
    println!("10 + 20 = {}", add(10, 20));
}
```

Look closely: `a + b` has **no semicolon**. Remember from the operators lesson: a block's value is its last expression without a semicolon. A function body is a block, so that value is what the function **returns**.

When `main` calls `add(2, 3)`: the program jumps into `add`, sets `a = 2` and `b = 3`, evaluates `a + b` → 5, jumps back, and the whole call `add(2, 3)` is replaced by the value `5`.

@@quiz
What does this print?

```text
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let a = double(4);
    let b = double(a);
    println!("{b}");
}
```
- `8`
* `16`
- `4`
? `double(4)` is 8, so `a` is 8. Then `double(8)` is 16.

@@text
## The semicolon trap

What happens if you add a semicolon by habit?

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(1, 2));
}
```

Error: `mismatched types: expected i32, found ()`. With the `;`, `a + b;` becomes a **statement**: the value is calculated and thrown away. The function then ends without producing a value — which in Rust is the "empty" value `()`, called **unit**. The compiler even suggests: `help: remove this semicolon`.

**Statements vs expressions** — the rule of thumb:

- **Expressions** produce a value: `5`, `a + b`, `add(1, 2)`, `{ ... }` blocks.
- **Statements** do something but produce no value: `let x = 5;`, and any expression followed by `;`.

## Early `return`

You can also leave a function early with the `return` keyword. It's mostly used when you know the answer before reaching the end (you'll see this more with `if`):

```rust
fn always_ten() -> i32 {
    return 10;
}

fn main() {
    println!("{}", always_ten());
}
```

Idiomatic Rust uses the implicit "last expression" style when possible, and `return` for early exits.

## Scope: functions have their own variables

Variables created inside a function exist only inside that function. Each function is its own little world, and the only ways in and out are **parameters** and **return values**:

```rust
fn make_number() -> i32 {
    let secret = 42;
    secret
}

fn main() {
    let n = make_number();
    println!("{n}");
    // println!("{secret}"); // ❌ error: `secret` doesn't exist here
}
```

This is a good thing: you can understand a function by looking only at the function.

## Functions calling functions

```rust
fn square(x: f64) -> f64 {
    x * x
}

fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * square(radius)
}

fn main() {
    println!("{:.2}", circle_area(3.0));
}
```

Small functions combine into bigger ones. That's how all large programs are built.

## `todo!()` — a placeholder

In the exercises below, function bodies contain `todo!()`. It's a macro that means "not written yet". The code compiles, but if `todo!()` is actually run, the program panics. Replace it with your own code. (You may also see warnings about unused parameters until you use them — that's expected.)

## Naming functions

Functions use `snake_case` like variables. Name them after what they **do** — usually a verb: `calculate_total`, `print_menu`, `is_valid`, `to_celsius`.

@@quiz
Why does this fail to compile? `fn five() -> i32 { 5; }`
- `5` is not an `i32`
* The semicolon turns `5` into a statement, so the function returns `()` instead of an `i32`
- Functions can't return numbers
? Remove the semicolon: `fn five() -> i32 { 5 }`.

@@exercise Square it
Write the body of the function `square` so that it returns its parameter multiplied by itself. `main` is already there so you can try it with ▶ Run. When you press ✔ Check, the checker will call your function with several different numbers.
@@starter
fn square(n: i32) -> i32 {
    todo!()
}

fn main() {
    println!("5 squared is {}", square(5));
}
@@tests
#[test]
fn checks() {
    assert_eq!(square(5), 25, "square(5) should be 25");
    assert_eq!(square(0), 0, "square(0) should be 0");
    assert_eq!(square(-3), 9, "square(-3) should be 9");
    assert_eq!(square(12), 144, "square(12) should be 144");
}
@@hint
Replace `todo!()` with `n * n` — and no semicolon, because it's the return value.
@@solution
fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    println!("5 squared is {}", square(5));
}

@@exercise Temperature converter
Write `celsius_to_fahrenheit`, which converts a temperature. The formula is:

`F = C × 9 / 5 + 32`

So 0 °C → 32 °F, 100 °C → 212 °F, and -40 °C → -40 °F.
@@starter
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!()
}

fn main() {
    let c = 25.0;
    println!("{c}°C is {}°F", celsius_to_fahrenheit(c));
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    assert!(close(celsius_to_fahrenheit(0.0), 32.0), "0°C should be 32°F");
    assert!(close(celsius_to_fahrenheit(100.0), 212.0), "100°C should be 212°F");
    assert!(close(celsius_to_fahrenheit(-40.0), -40.0), "-40°C should be -40°F");
    assert!(close(celsius_to_fahrenheit(37.0), 98.6), "37°C should be 98.6°F");
}
@@hint
All numbers must be floats here: write `9.0`, `5.0` and `32.0`, not `9`, `5` and `32`.
@@solution
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let c = 25.0;
    println!("{c}°C is {}°F", celsius_to_fahrenheit(c));
}

@@exercise Name tag
Write `name_tag`, which takes a first name and a last name and returns a `String` like `"Hello, my name is Ada Lovelace"`.
@@starter
fn name_tag(first: &str, last: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", name_tag("Ada", "Lovelace"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(name_tag("Ada", "Lovelace"), "Hello, my name is Ada Lovelace");
    assert_eq!(name_tag("Grace", "Hopper"), "Hello, my name is Grace Hopper");
}
@@hint
`format!` gives you a `String`: `format!("Hello, my name is {first} {last}")`
@@solution
fn name_tag(first: &str, last: &str) -> String {
    format!("Hello, my name is {first} {last}")
}

fn main() {
    println!("{}", name_tag("Ada", "Lovelace"));
}

@@exercise Functions working together
Write **two** functions:

- `average(a: f64, b: f64, c: f64) -> f64` — the average of three numbers.
- `report(a: f64, b: f64, c: f64) -> String` — returns text like `"Average: 7.50"` (always two decimals). It must **call `average`** instead of repeating the maths.
@@starter
// Write average() here


// Write report() here


fn main() {
    println!("{}", report(6.0, 7.5, 9.0));
}
@@tests
#[test]
fn checks() {
    assert!((average(1.0, 2.0, 3.0) - 2.0).abs() < 1e-9, "average(1, 2, 3) should be 2");
    assert!((average(6.0, 7.5, 9.0) - 7.5).abs() < 1e-9, "average(6, 7.5, 9) should be 7.5");
    assert_eq!(report(6.0, 7.5, 9.0), "Average: 7.50");
    assert_eq!(report(1.0, 1.0, 2.0), "Average: 1.33");
}
@@hint
Inside `report`, write `let avg = average(a, b, c);` and then return `format!("Average: {avg:.2}")`.
@@solution
fn average(a: f64, b: f64, c: f64) -> f64 {
    (a + b + c) / 3.0
}

fn report(a: f64, b: f64, c: f64) -> String {
    let avg = average(a, b, c);
    format!("Average: {avg:.2}")
}

fn main() {
    println!("{}", report(6.0, 7.5, 9.0));
}
