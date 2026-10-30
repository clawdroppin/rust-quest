@@lesson m1-bool-char True/False, Characters and Comparisons
@@text
## `bool` — true or false

A `bool` (short for *Boolean*, after the mathematician George Boole) has only two possible values: `true` and `false`. It's the type of every **yes/no question** a program asks: *Is the player alive? Is the file saved? Is the number even?*

```rust
fn main() {
    let is_raining = true;
    let has_umbrella: bool = false;
    println!("Raining: {is_raining}, umbrella: {has_umbrella}");
}
```

Good names for `bool` variables read like a question: `is_open`, `has_key`, `can_jump`.

## Comparisons produce `bool`s

Comparison operators compare two values and give you a `bool`:

| Operator | Meaning |
|---|---|
| `==` | equal to |
| `!=` | not equal to |
| `<` | less than |
| `>` | greater than |
| `<=` | less than or equal to |
| `>=` | greater than or equal to |

```rust
fn main() {
    let age = 17;
    let is_adult = age >= 18;
    println!("Adult? {is_adult}");
    println!("5 == 5 is {}", 5 == 5);
    println!("3 != 3 is {}", 3 != 3);
}
```

> ⚠ **`=` vs `==`**: a single `=` *stores* a value into a variable. A double `==` *asks* whether two values are equal. Mixing them up is one of the most common beginner mistakes. Luckily, Rust's compiler usually catches it.

Both sides of a comparison must be the same type: comparing an `i32` with an `f64` is a compile error.

## Combining `bool`s: AND, OR, NOT

| Operator | Name | True when... |
|---|---|---|
| `a && b` | AND | **both** `a` and `b` are true |
| `a \|\| b` | OR | **at least one** of them is true |
| `!a` | NOT | `a` is false (it flips the value) |

```rust
fn main() {
    let age = 25;
    let has_ticket = true;
    let is_vip = false;

    let can_enter = age >= 18 && has_ticket;
    let gets_free_drink = is_vip || age > 60;
    let needs_ticket = !has_ticket;

    println!("Can enter: {can_enter}");
    println!("Free drink: {gets_free_drink}");
    println!("Needs ticket: {needs_ticket}");
}
```

**Short-circuiting:** `&&` stops as soon as it sees a `false` (the answer can't be true any more), and `||` stops as soon as it sees a `true`. This will matter later when the right side does some work.

@@quiz
`let x = 7;` — what is `x > 5 && x % 2 == 0`?
- `true`
* `false`
? `x > 5` is true, but `7 % 2 == 0` is false (7 is odd). With `&&`, both must be true, so the result is `false`.

@@quiz
What is `!(3 < 5) || 2 == 2`?
* `true`
- `false`
? `3 < 5` is true, `!true` is false. `2 == 2` is true. `false || true` is `true`.

@@text
## `char` — a single character

A `char` holds exactly **one character**, written in **single quotes**:

```rust
fn main() {
    let letter = 'R';
    let digit = '7';
    let symbol = '#';
    let crab = '🦀';
    println!("{letter} {digit} {symbol} {crab}");
}
```

- `'a'` (single quotes) is a `char` — one character.
- `"a"` (double quotes) is a *string* — text, which could be any length. They are different types!

Rust's `char` can hold any Unicode character: letters from any alphabet, emoji, Chinese characters, and so on.

Characters have handy built-in abilities, called **methods**. You call a method by writing a `.` after the value:

```rust
fn main() {
    let c = 'q';
    println!("{}", c.is_alphabetic());   // is it a letter?
    println!("{}", c.is_numeric());      // is it a digit?
    println!("{}", c.is_uppercase());
    println!("{}", c.to_ascii_uppercase());
    println!("{}", '9'.to_digit(10).unwrap()); // the digit '9' as the number 9
}
```

Don't worry about `unwrap()` yet — you'll learn exactly what it does in the module on error handling.

Characters can be compared too: `'a' < 'b'` is `true`, because characters are ordered by their Unicode number.

@@quiz
Which of these is a `char`?
- `"x"`
* `'x'`
- `x`
? Single quotes make a `char`. Double quotes make a string. A bare `x` is a variable name.

@@exercise Can they ride?
A theme park ride has rules: you must be **at least 140 cm tall**, and **either** be at least 12 years old **or** be with an adult.

Create a `bool` variable `can_ride` using `height`, `age` and `with_adult`, and print it. With the given values, the output should be:

```text
Can ride: true
```

Use comparison and logical operators — don't just write `true`!
@@starter
fn main() {
    let height = 152;
    let age = 10;
    let with_adult = true;

    let can_ride = false; // replace false with the real condition
    println!("Can ride: {can_ride}");
}
@@expected
Can ride: true
@@requires
&&
||
>=
@@hint
You'll need `height >= 140`, then `&&`, then the age-or-adult part in parentheses: `(age >= 12 || with_adult)`.
@@solution
fn main() {
    let height = 152;
    let age = 10;
    let with_adult = true;

    let can_ride = height >= 140 && (age >= 12 || with_adult);
    println!("Can ride: {can_ride}");
}

@@exercise Character inspector
Given `let c = 'k';`, print the following lines by calling **methods** on `c` (not by typing the answers):

```text
Character: k
Is a letter: true
Is a digit: false
Uppercase: K
```
@@starter
fn main() {
    let c = 'k';
    println!("Character: {c}");
}
@@expected
Character: k
Is a letter: true
Is a digit: false
Uppercase: K
@@requires
is_alphabetic
is_numeric
to_ascii_uppercase
@@hint
Use `{}` placeholders and put method calls after the text: `println!("Is a letter: {}", c.is_alphabetic());`
@@solution
fn main() {
    let c = 'k';
    println!("Character: {c}");
    println!("Is a letter: {}", c.is_alphabetic());
    println!("Is a digit: {}", c.is_numeric());
    println!("Uppercase: {}", c.to_ascii_uppercase());
}
