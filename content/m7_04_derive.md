@@lesson m7-derive Derive and the Common Traits
@@text
Some traits are so common, and their implementation so obvious, that Rust can write them for you. That's what `#[derive(...)]` does. You've been writing `#[derive(Debug)]` since Module 4; now let's see the whole family, and when to write implementations by hand.

## The derivable traits

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;                         // Copy: a is still usable
    println!("{:?}", a);               // Debug
    println!("{}", a == b);            // PartialEq
    println!("{}", a < Point { x: 1, y: 5 }); // PartialOrd: compares x, then y
    println!("{:?}", Point::default()); // Default: all zeros
}
```

| Trait | Gives you | Derived version does… |
|---|---|---|
| `Debug` | `{:?}` and `{:#?}` | prints every field |
| `Clone` | `.clone()` | clones every field |
| `Copy` | implicit copies instead of moves | (only if every field is `Copy`) |
| `PartialEq` | `==`, `!=` | equal if every field is equal |
| `Eq` | "equality is total" marker | (needed for `HashMap` keys; not for floats) |
| `PartialOrd` | `<`, `>`, `<=`, `>=` | compares fields in order, top to bottom |
| `Ord` | `.cmp()`, `.sort()`, `.max()` | same as above, total order |
| `Hash` | usable as a `HashMap`/`HashSet` key | hashes every field |
| `Default` | `Type::default()` | every field's default (0, "", false, empty Vec) |

A derive only works if every field's type also implements that trait. A struct with an `f64` field can derive `PartialEq` but not `Eq` (because `NaN != NaN`).

## Enums derive too

Enums compare by **the order the variants are written in**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Priority {
    Low,
    Medium,
    High,
}

fn main() {
    let mut tasks = vec![Priority::High, Priority::Low, Priority::Medium];
    tasks.sort();
    println!("{:?}", tasks);
    println!("{}", Priority::High > Priority::Low);
}
```

## Sorting your own structs

`sort()` needs `Ord`. If you derive it, the fields are compared in order, so **field order matters**:

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    score: u32,      // compared first
    name: String,    // only if scores are equal
}

fn main() {
    let mut players = vec![
        Player { score: 30, name: String::from("Ben") },
        Player { score: 50, name: String::from("Ada") },
        Player { score: 30, name: String::from("Al") },
    ];
    players.sort();
    for p in &players {
        println!("{} {}", p.score, p.name);
    }
}
```

Often you'll want to sort by a different rule, e.g. highest score first. That's what `sort_by` and `sort_by_key` are for, with **closures**, in the next lesson.

## `Default` and struct update syntax

`Default` pairs nicely with `..`, which fills "the rest of the fields" from another value:

```rust
#[derive(Debug, Default)]
struct Settings {
    volume: u8,
    fullscreen: bool,
    player_name: String,
    difficulty: u8,
}

fn main() {
    let s = Settings {
        volume: 80,
        player_name: String::from("Ferris"),
        ..Default::default()
    };
    println!("{:#?}", s);
}
```

## Implementing traits by hand

Derive gives the *obvious* behaviour. When you need something else, implement the trait yourself.

**`Display`** can never be derived, because only you know how your type should look to a user:

```rust,norun
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

**`Default`** by hand, when zeros aren't the right defaults:

```rust,norun
impl Default for Settings {
    fn default() -> Self {
        Settings { volume: 50, fullscreen: false, player_name: String::from("Player"), difficulty: 2 }
    }
}
```

**`From`** describes a conversion. Implementing `From<A> for B` gives you both `B::from(a)` and `a.into()`:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 1.8 + 32.0)
    }
}

fn main() {
    let boiling = Fahrenheit::from(Celsius(100.0));
    println!("{}", boiling.0);
    let body: Fahrenheit = Celsius(37.0).into();
    println!("{:.1}", body.0);
}
```

(`struct Celsius(f64);` is a **tuple struct**: a struct with unnamed fields, accessed as `.0`, `.1`. Wrapping a number in its own type like this is called the **newtype** pattern. Now the compiler won't let you mix up Celsius and Fahrenheit!)

`From` is also what makes `?` convert errors: if `From<io::Error>` is implemented for your error type, `?` converts automatically.

@@quiz
A struct has fields `name: String` and `score: f64`. Which of these can it derive?
- `Copy`
- `Eq`
* `Clone` and `PartialEq`
? `String` isn't `Copy`, and `f64` isn't `Eq`, so those derives fail. `Clone` and `PartialEq` work for both fields.

@@quiz
With `#[derive(PartialOrd, Ord, PartialEq, Eq)] enum Size { Small, Medium, Large }`, what is `Size::Large > Size::Small`?
* `true`, because variants compare in the order they are declared
- `false`
- It doesn't compile
? Derived ordering on enums follows declaration order.

@@quiz
Which trait can you **never** derive?
- `Debug`
- `Default`
* `Display`
? Display is how users see your type, so you must decide yourself.

@@exercise Sortable cards
Define an enum `Suit` with variants `Clubs, Diamonds, Hearts, Spades` (in that order) and a struct `Card` with fields `rank: u8` and `suit: Suit`.

Derive what's needed so that:

- cards can be printed with `{:?}`, compared with `==` and sorted with `.sort()`,
- sorting orders by **rank first**, then by suit.

Then implement `Display` for `Card` so it prints like `10♥`, `2♣`, `14♠`. Use the symbols ♣ ♦ ♥ ♠.
@@starter
use std::fmt;

// define Suit and Card here

fn main() {
    let mut hand = vec![
        Card { rank: 10, suit: Suit::Hearts },
        Card { rank: 2, suit: Suit::Spades },
        Card { rank: 10, suit: Suit::Clubs },
        Card { rank: 14, suit: Suit::Diamonds },
    ];
    hand.sort();
    for card in &hand {
        print!("{card} ");
    }
    println!();
    println!("{}", hand[0] == Card { rank: 2, suit: Suit::Spades });
}
@@expected
2♠ 10♣ 10♥ 14♦
true
@@tests
#[test]
fn checks() {
    assert!(Suit::Spades > Suit::Clubs);
    let a = Card { rank: 5, suit: Suit::Spades };
    let b = Card { rank: 6, suit: Suit::Clubs };
    assert!(a < b, "rank is compared first");
    assert_eq!(Card { rank: 12, suit: Suit::Diamonds }.to_string(), "12♦");
}
@@requires
derive
impl fmt::Display for Card
@@hint
`#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]` on both. Put `rank` **before** `suit` in the struct. In `fmt`, `match self.suit` to pick the symbol, then `write!(f, "{}{}", self.rank, symbol)`.
@@solution
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    rank: u8,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self.suit {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{}{}", self.rank, symbol)
    }
}

fn main() {
    let mut hand = vec![
        Card { rank: 10, suit: Suit::Hearts },
        Card { rank: 2, suit: Suit::Spades },
        Card { rank: 10, suit: Suit::Clubs },
        Card { rank: 14, suit: Suit::Diamonds },
    ];
    hand.sort();
    for card in &hand {
        print!("{card} ");
    }
    println!();
    println!("{}", hand[0] == Card { rank: 2, suit: Suit::Spades });
}

@@exercise Conversions with From
A `Duration`-like type stores a number of seconds: `struct Seconds(u64);`. Implement:

- `From<Minutes> for Seconds` (1 minute = 60 seconds), where `struct Minutes(u64);`
- `Display` for `Seconds`, printing as `m:ss` — for example `Seconds(125)` prints `2:05`.
@@starter
use std::fmt;

struct Seconds(u64);
struct Minutes(u64);

// implement From<Minutes> for Seconds

// implement Display for Seconds

fn main() {
    let s = Seconds::from(Minutes(3));
    println!("{}", s.0);
    println!("{}", Seconds(125));
    let t: Seconds = Minutes(10).into();
    println!("{t}");
}
@@expected
180
2:05
10:00
@@tests
#[test]
fn checks() {
    assert_eq!(Seconds(0).to_string(), "0:00");
    assert_eq!(Seconds(59).to_string(), "0:59");
    assert_eq!(Seconds(3601).to_string(), "60:01");
    assert_eq!(Seconds::from(Minutes(0)).0, 0);
}
@@requires
impl From<Minutes> for Seconds
@@hint
```
impl From<Minutes> for Seconds {
    fn from(m: Minutes) -> Self {
        Seconds(m.0 * 60)
    }
}
```
For Display: `write!(f, "{}:{:02}", self.0 / 60, self.0 % 60)`.
@@solution
use std::fmt;

struct Seconds(u64);
struct Minutes(u64);

impl From<Minutes> for Seconds {
    fn from(m: Minutes) -> Self {
        Seconds(m.0 * 60)
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.0 / 60, self.0 % 60)
    }
}

fn main() {
    let s = Seconds::from(Minutes(3));
    println!("{}", s.0);
    println!("{}", Seconds(125));
    let t: Seconds = Minutes(10).into();
    println!("{t}");
}
