@@lesson m1-if Making Decisions: if and else
@@text
Until now, programs ran every line from top to bottom. Real programs need to **make decisions**: do one thing *if* something is true, and something else otherwise. This is called **control flow**.

## `if`

```rust
fn main() {
    let temperature = 31;

    if temperature > 30 {
        println!("It's hot! Drink water.");
    }

    println!("Have a nice day.");
}
```

- After `if` comes a **condition**: an expression that must be a `bool`.
- If it's `true`, the block `{ ... }` runs. If it's `false`, the block is skipped.
- Either way, the program then continues after the block.

Change `temperature` to 20 and run it again.

## `else`

`else` gives a block to run when the condition is **false**:

```rust
fn main() {
    let age = 15;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You can vote in {} years.", 18 - age);
    }
}
```

Exactly **one** of the two blocks runs. Never both, never neither.

## `else if` — more than two choices

```rust
fn main() {
    let score = 72;

    if score >= 90 {
        println!("Excellent!");
    } else if score >= 70 {
        println!("Good job.");
    } else if score >= 50 {
        println!("You passed.");
    } else {
        println!("Let's practise more.");
    }
}
```

The conditions are checked **in order, top to bottom**. The first one that's true wins, its block runs, and **all the rest are skipped**. That's why the order matters: `score >= 70` is only checked if `score >= 90` was false, so at that point we already know the score is below 90.

@@quiz
With `let score = 95;`, what does the program above print?
* `Excellent!`
- `Excellent!` and `Good job.` and `You passed.`
- `Good job.`
? Only the **first** true branch runs. 95 >= 90 is true, so `Excellent!` is printed and the rest is skipped.

@@text
## Conditions must be `bool`

In some languages you can write `if 1 { ... }` or `if name { ... }`. Rust doesn't allow that: the condition must be a real `bool`. This prevents confusing bugs.

```rust
fn main() {
    let lives = 3;
    // if lives { }        // ❌ error: expected `bool`, found integer
    if lives > 0 {         // ✔ say exactly what you mean
        println!("Still alive!");
    }
}
```

You can use `&&`, `||` and `!` to build conditions:

```rust
fn main() {
    let has_key = true;
    let door_locked = true;

    if door_locked && !has_key {
        println!("The door is locked.");
    } else {
        println!("You open the door.");
    }
}
```

## Nested `if`

An `if` can go inside another `if`. Indentation shows the structure:

```rust
fn main() {
    let is_weekend = true;
    let is_raining = false;

    if is_weekend {
        if is_raining {
            println!("Movie day.");
        } else {
            println!("Let's go hiking!");
        }
    } else {
        println!("Work day.");
    }
}
```

Keep nesting shallow when you can — deeply nested code gets hard to follow. Often `&&` or `else if` can flatten it.

## `if` is an expression

In Rust, `if`/`else` produces a **value**, just like a block. So you can use it on the right side of `let`:

```rust
fn main() {
    let hour = 14;
    let greeting = if hour < 12 { "Good morning" } else { "Good afternoon" };
    println!("{greeting}!");
}
```

Note: no semicolons inside the branches, because each branch's value is the result. Both branches must produce the **same type**.

This combines beautifully with functions — the `if` expression is the function's return value:

```rust
fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", bigger(3, 8));
}
```

## Early return with `if`

Sometimes it's clearest to handle special cases first and `return` right away:

```rust
fn describe(n: i32) -> &'static str {
    if n == 0 {
        return "zero";
    }
    if n < 0 {
        return "negative";
    }
    "positive"
}

fn main() {
    println!("{}", describe(-5));
    println!("{}", describe(0));
    println!("{}", describe(10));
}
```

(`&'static str` is the type of text written directly in the code — you'll understand the `'static` part much later. For now, read it as "a fixed piece of text".)

## A peek at `match`

Rust has another, very powerful way to choose between options, called `match`. Here's a sneak peek — it gets a full lesson in the module on modeling data:

```rust
fn main() {
    let day = 3;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        _ => "some other day",
    };
    println!("{name}");
}
```

@@quiz
What's wrong with this? `let x = if ready { 1 } else { "no" };`
- `if` can't be used with `let`
* The two branches have different types (`i32` and `&str`)
- There should be semicolons after `1` and `"no"`
? An `if` expression must produce one type, whichever branch runs.

@@exercise Even or odd
Write `even_or_odd`, which returns `"even"` if the number is even and `"odd"` otherwise.

Remember: a number is even when dividing it by 2 leaves no remainder.
@@starter
fn even_or_odd(n: i32) -> &'static str {
    todo!()
}

fn main() {
    println!("7 is {}", even_or_odd(7));
    println!("10 is {}", even_or_odd(10));
}
@@tests
#[test]
fn checks() {
    assert_eq!(even_or_odd(7), "odd", "7 is odd");
    assert_eq!(even_or_odd(10), "even", "10 is even");
    assert_eq!(even_or_odd(0), "even", "0 is even");
    assert_eq!(even_or_odd(-3), "odd", "-3 is odd");
}
@@hint
`n % 2 == 0` is `true` for even numbers. Use `if ... { "even" } else { "odd" }` as the function's last expression.
@@solution
fn even_or_odd(n: i32) -> &'static str {
    if n % 2 == 0 { "even" } else { "odd" }
}

fn main() {
    println!("7 is {}", even_or_odd(7));
    println!("10 is {}", even_or_odd(10));
}

@@exercise Letter grades
Write `grade`, which turns a score (0–100) into a letter:

| Score | Grade |
|---|---|
| 90 or more | `'A'` |
| 80–89 | `'B'` |
| 70–79 | `'C'` |
| 60–69 | `'D'` |
| below 60 | `'F'` |

Note the return type is `char`, so use single quotes.
@@starter
fn grade(score: u32) -> char {
    todo!()
}

fn main() {
    println!("85 -> {}", grade(85));
}
@@tests
#[test]
fn checks() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A', "90 should be an A");
    assert_eq!(grade(89), 'B', "89 should be a B");
    assert_eq!(grade(80), 'B');
    assert_eq!(grade(75), 'C');
    assert_eq!(grade(60), 'D', "60 should be a D");
    assert_eq!(grade(59), 'F', "59 should be an F");
    assert_eq!(grade(0), 'F');
}
@@hint
Use a chain: `if score >= 90 { 'A' } else if score >= 80 { 'B' } else if ...` — and check the highest grade first.
@@solution
fn grade(score: u32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

fn main() {
    println!("85 -> {}", grade(85));
}

@@exercise Biggest of three
Write `max_of_three`, which returns the largest of three numbers. Use `if`/`else` (not the `.max()` method) — this is about practising decisions.
@@starter
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    todo!()
}

fn main() {
    println!("{}", max_of_three(4, 9, 2));
}
@@tests
#[test]
fn checks() {
    assert_eq!(max_of_three(4, 9, 2), 9);
    assert_eq!(max_of_three(10, 3, 5), 10);
    assert_eq!(max_of_three(1, 2, 3), 3);
    assert_eq!(max_of_three(-5, -2, -9), -2);
    assert_eq!(max_of_three(7, 7, 7), 7);
}
@@requires
if
@@hint
`a` is the biggest if `a >= b && a >= c`. Otherwise, compare `b` and `c`.
@@solution
fn max_of_three(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}

fn main() {
    println!("{}", max_of_three(4, 9, 2));
}

@@exercise Ticket prices
A cinema charges:

- children **under 12**: 5
- seniors **65 and over**: 6
- everyone else: 10
- …but on **Tuesdays** (`is_tuesday == true`) everyone pays half of their normal price (use integer division).

Write `ticket_price`.
@@starter
fn ticket_price(age: u32, is_tuesday: bool) -> u32 {
    todo!()
}

fn main() {
    println!("Adult on Tuesday: {}", ticket_price(30, true));
}
@@tests
#[test]
fn checks() {
    assert_eq!(ticket_price(8, false), 5, "child, not Tuesday");
    assert_eq!(ticket_price(12, false), 10, "12 is not under 12");
    assert_eq!(ticket_price(30, false), 10, "adult");
    assert_eq!(ticket_price(65, false), 6, "65 is a senior");
    assert_eq!(ticket_price(30, true), 5, "adult on Tuesday");
    assert_eq!(ticket_price(8, true), 2, "child on Tuesday: 5 / 2 = 2");
    assert_eq!(ticket_price(70, true), 3, "senior on Tuesday");
}
@@hint
Break it into two steps. First work out the normal price with an `if` expression: `let price = if age < 12 { 5 } else if ... ;`. Then a second `if` decides whether to halve it.
@@solution
fn ticket_price(age: u32, is_tuesday: bool) -> u32 {
    let price = if age < 12 {
        5
    } else if age >= 65 {
        6
    } else {
        10
    };

    if is_tuesday { price / 2 } else { price }
}

fn main() {
    println!("Adult on Tuesday: {}", ticket_price(30, true));
}
