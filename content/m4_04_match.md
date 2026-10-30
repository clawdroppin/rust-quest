@@lesson m4-match match and Patterns
@@text
`match` is one of Rust's most loved features. It's like a supercharged `if`/`else if` chain that can look *inside* values. This lesson covers all the kinds of **patterns** you can use.

## The basics

```rust
fn main() {
    let n = 3;
    let word = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("{word}");
}
```

- Each **arm** is `pattern => expression`.
- Arms are checked **top to bottom**; the first matching one runs.
- `_` is the **wildcard**: it matches anything. It's often the last arm, to cover "everything else".
- `match` is an expression: all arms must produce the same type.
- It must be **exhaustive**: every possible value must be covered. For numbers, that basically means you need a `_` arm.

An arm can run a whole block:

```rust
fn main() {
    let lives = 0;
    match lives {
        0 => {
            println!("Game over!");
            println!("Press any key to restart.");
        }
        _ => println!("Keep going!"),
    }
}
```

## Several values, ranges, and bindings

```rust
fn describe(n: i32) -> String {
    match n {
        0 => String::from("zero"),
        1 | 2 | 3 => String::from("a few"),          // | means "or"
        4..=9 => String::from("several"),            // an inclusive range
        x if x < 0 => format!("negative ({x})"),     // binding + guard
        x => format!("lots ({x})"),                  // bind anything else to x
    }
}

fn main() {
    for n in [0, 2, 7, -4, 100] {
        println!("{n}: {}", describe(n));
    }
}
```

- `1 | 2 | 3` — any of these values.
- `4..=9` — any value in the range (works with numbers and `char`s: `'a'..='z'`).
- `x if x < 0` — a **match guard**: bind the value to `x`, but only take this arm if the extra condition is true.
- A bare name like `x` matches anything and gives it a name.

## Matching tuples

Matching several values at once makes complex decisions very readable:

```rust
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }
}

fn main() {
    for n in 1..=15 {
        print!("{} ", fizzbuzz(n));
    }
    println!();
}
```

Compare this with the `if` chain from Module 1!

## Destructuring structs and enums

Patterns can take values apart:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn where_is(p: &Point) -> String {
    match p {
        Point { x: 0, y: 0 } => String::from("at the origin"),
        Point { x: 0, y } => format!("on the y axis at {y}"),
        Point { x, y: 0 } => format!("on the x axis at {x}"),
        Point { x, y } => format!("at ({x}, {y})"),
    }
}

fn main() {
    println!("{}", where_is(&Point { x: 0, y: 5 }));
    println!("{}", where_is(&Point { x: 3, y: 4 }));
}
```

And patterns work in `let` and `for` too — you've been using them with tuples:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    let Point { x, y } = p;          // destructure a struct
    println!("{x} {y}");

    let pairs = vec![(1, 'a'), (2, 'b')];
    for (num, letter) in pairs {     // destructure tuples in a loop
        println!("{num}{letter}");
    }
}
```

## Ignoring parts

- `_` ignores one value.
- `..` ignores the rest: `Point { x, .. }` takes only `x`; `(first, ..)` takes only the first element of a tuple.

## `matches!`

To simply ask "does this value match this pattern?" and get a `bool`, use the `matches!` macro:

```rust
fn main() {
    let c = 'k';
    let is_vowel = matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
    let is_lowercase = matches!(c, 'a'..='z');
    println!("{is_vowel} {is_lowercase}");
}
```

@@quiz
What does this return for `n = 5`?

```text
match n {
    1..=4 => "low",
    5 | 6 => "mid",
    _ => "high",
}
```
- `"low"`
* `"mid"`
- `"high"`
? 5 is not in 1..=4, but it matches `5 | 6`.

@@quiz
What is a match guard?
- A pattern that matches nothing
* An extra `if` condition on an arm, like `x if x > 10 =>`
- The `_` arm
? Guards let an arm match only when an extra condition holds.

@@exercise Rock, paper, scissors
Define an enum `Hand` with `Rock`, `Paper` and `Scissors`, and an enum `Outcome` with `Win`, `Lose` and `Draw` (derive `Debug` and `PartialEq` on both).

Write `play(me: Hand, them: Hand) -> Outcome` using **one `match` on the tuple `(me, them)`**. Rock beats scissors, scissors beats paper, paper beats rock.
@@starter
// define Hand and Outcome here

fn play(me: Hand, them: Hand) -> Outcome {
    todo!()
}

fn main() {
    println!("{:?}", play(Hand::Rock, Hand::Scissors));
}
@@tests
#[test]
fn checks() {
    use Hand::*;
    assert_eq!(play(Rock, Scissors), Outcome::Win);
    assert_eq!(play(Scissors, Paper), Outcome::Win);
    assert_eq!(play(Paper, Rock), Outcome::Win);
    assert_eq!(play(Rock, Paper), Outcome::Lose);
    assert_eq!(play(Paper, Scissors), Outcome::Lose);
    assert_eq!(play(Scissors, Rock), Outcome::Lose);
    assert_eq!(play(Paper, Paper), Outcome::Draw);
}
@@requires
match (me, them)
@@hint
List the three winning combinations as arms returning `Outcome::Win`. For draws you can't write `(x, x)` in a pattern, but you can use a guard: `(a, b) if a == b => Outcome::Draw` (needs `PartialEq` on `Hand`). Everything else (`_`) is a loss.
@@solution
#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn play(me: Hand, them: Hand) -> Outcome {
    match (me, them) {
        (Hand::Rock, Hand::Scissors) | (Hand::Scissors, Hand::Paper) | (Hand::Paper, Hand::Rock) => Outcome::Win,
        (a, b) if a == b => Outcome::Draw,
        _ => Outcome::Lose,
    }
}

fn main() {
    println!("{:?}", play(Hand::Rock, Hand::Scissors));
}

@@exercise Character classes
Write `classify`, which describes a character using a single `match` with ranges:

- `'a'..='z'` → `"lowercase"`
- `'A'..='Z'` → `"uppercase"`
- `'0'..='9'` → `"digit"`
- `' '`, `'\t'` or `'\n'` → `"whitespace"`
- anything else → `"symbol"`
@@starter
fn classify(c: char) -> &'static str {
    todo!()
}

fn main() {
    for c in ['q', 'Q', '7', ' ', '#'] {
        println!("{c:?} is {}", classify(c));
    }
}
@@tests
#[test]
fn checks() {
    assert_eq!(classify('q'), "lowercase");
    assert_eq!(classify('Q'), "uppercase");
    assert_eq!(classify('7'), "digit");
    assert_eq!(classify(' '), "whitespace");
    assert_eq!(classify('\t'), "whitespace");
    assert_eq!(classify('#'), "symbol");
    assert_eq!(classify('é'), "symbol");
}
@@requires
match
..=
@@hint
`match c { 'a'..='z' => "lowercase", ... ' ' | '\t' | '\n' => "whitespace", _ => "symbol" }`
@@solution
fn classify(c: char) -> &'static str {
    match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' => "whitespace",
        _ => "symbol",
    }
}

fn main() {
    for c in ['q', 'Q', '7', ' ', '#'] {
        println!("{c:?} is {}", classify(c));
    }
}

@@exercise Shipping cost
Write `shipping_cost`, which works out the cost of shipping a parcel with one `match` on the tuple `(zone, weight_kg)`. Use ranges and guards:

| Zone | Weight | Cost |
|---|---|---|
| `"local"` | up to 5 kg | 3 |
| `"local"` | more than 5 kg | 6 |
| `"national"` | up to 5 kg | 8 |
| `"national"` | more than 5 kg | 15 |
| `"international"` | any | 20 + 4 per kg |
| anything else | | 0 (unknown zone) |
@@starter
fn shipping_cost(zone: &str, weight_kg: u32) -> u32 {
    todo!()
}

fn main() {
    println!("{}", shipping_cost("international", 3));
}
@@tests
#[test]
fn checks() {
    assert_eq!(shipping_cost("local", 5), 3);
    assert_eq!(shipping_cost("local", 6), 6);
    assert_eq!(shipping_cost("national", 0), 8);
    assert_eq!(shipping_cost("national", 20), 15);
    assert_eq!(shipping_cost("international", 3), 32);
    assert_eq!(shipping_cost("mars", 1), 0);
}
@@requires
match
@@hint
Arms like `("local", 0..=5) => 3,` and `("local", _) => 6,`. For international: `("international", w) => 20 + 4 * w,`.
@@solution
fn shipping_cost(zone: &str, weight_kg: u32) -> u32 {
    match (zone, weight_kg) {
        ("local", 0..=5) => 3,
        ("local", _) => 6,
        ("national", 0..=5) => 8,
        ("national", _) => 15,
        ("international", w) => 20 + 4 * w,
        _ => 0,
    }
}

fn main() {
    println!("{}", shipping_cost("international", 3));
}
