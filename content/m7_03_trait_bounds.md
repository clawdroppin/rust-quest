@@lesson m7-bounds Trait Bounds
@@text
Generics let code work with *any* type. **Trait bounds** narrow that down to "any type **that has this capability**". This is where traits and generics come together.

## Fixing `largest`

```rust
fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    best
}

fn main() {
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", largest(&[1.5, 0.2]));
    println!("{}", largest(&['q', 'z', 'a']));
    println!("{}", largest(&["pear", "apple"]));
}
```

`T: PartialOrd` reads "T, which must implement `PartialOrd`". `PartialOrd` is the trait behind `<` and `>`, so now Rust knows `item > best` makes sense. If you try `largest` with a type that can't be compared, you get a clear compile error at the call site.

## Multiple bounds and `where`

Use `+` to require several traits:

```rust
use std::fmt::Display;

fn show_largest<T: PartialOrd + Display>(items: &[T]) {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    println!("The largest is {best}");   // needs Display
}

fn main() {
    show_largest(&[4, 8, 1]);
}
```

When bounds get long, a `where` clause keeps the signature readable. These two mean exactly the same:

```rust,norun
fn report<T: Display + Clone, U: Debug>(a: T, b: U) { ... }

fn report<T, U>(a: T, b: U)
where
    T: Display + Clone,
    U: Debug,
{
    ...
}
```

## `impl Trait` is shorthand

In the traits lesson you wrote `fn describe(shape: &impl Shape)`. That's a shorter way of writing a bound:

```rust,norun
fn describe(shape: &impl Shape) { ... }
fn describe<S: Shape>(shape: &S) { ... }      // the same thing
```

Use `impl Trait` for simple cases and the `<T: Trait>` form when you need to refer to the type more than once (for example, "two parameters of the *same* type").

`impl Trait` also works in **return** position, meaning "this returns some type with this trait, I won't say which":

```rust,norun
fn evens() -> impl Iterator<Item = u32> {
    (0..10).filter(|n| n % 2 == 0)
}
```

## Bounds on your own traits

Bounds work with any trait, including ones you write:

```rust
trait Describe {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

struct Song {
    title: String,
    seconds: u32,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("📖 {}", self.title)
    }
}

impl Describe for Song {
    fn describe(&self) -> String {
        format!("🎵 {} ({}:{:02})", self.title, self.seconds / 60, self.seconds % 60)
    }
}

fn print_all<T: Describe>(items: &[T]) {
    for (i, item) in items.iter().enumerate() {
        println!("{}. {}", i + 1, item.describe());
    }
}

fn main() {
    print_all(&[Book { title: String::from("Dune") }, Book { title: String::from("Emma") }]);
    print_all(&[Song { title: String::from("Intro"), seconds: 95 }]);
}
```

Notice `print_all(&[...])` takes a list of **one** type `T` at a time: all books, or all songs. To mix types in one list, you'd use `Vec<Box<dyn Describe>>` as in the traits lesson.

## Generics vs `dyn`: which one?

| | Generics (`T: Trait`, `impl Trait`) | Trait objects (`dyn Trait`) |
|---|---|---|
| Which method is called is decided… | at compile time | while running |
| Speed | as fast as possible | a tiny bit slower |
| Mix different types in one `Vec` | no | **yes** |

Rule of thumb: use generics by default; reach for `Box<dyn Trait>` when you need a collection of different types.

@@quiz
What does `fn largest<T: PartialOrd>(items: &[T]) -> &T` mean?
- T must be a number
* T can be any type that implements `PartialOrd` (can be compared with `<` and `>`)
- The function only works with `PartialOrd` structs you wrote
? A bound restricts which types can be used, and in exchange lets you use the trait's abilities.

@@quiz
You need to store books *and* songs in one list. Which do you use?
- `Vec<T>` where `T: Describe`
* `Vec<Box<dyn Describe>>`
- `Vec<impl Describe>`
? Generics pick one concrete type per use. Trait objects allow mixing types.

@@exercise Smallest and largest
Write a generic function `min_max<T: PartialOrd + Copy>(items: &[T]) -> Option<(T, T)>` that returns the smallest and the largest item as a tuple, or `None` if the slice is empty. Go through the list once.

(`Copy` lets you return copies of the values instead of references.)
@@starter
// write min_max here

fn main() {
    println!("{:?}", min_max(&[3, 9, -2, 7]));
    println!("{:?}", min_max(&[2.5, 1.5]));
    println!("{:?}", min_max(&['m', 'a', 'z']));
    let empty: [i32; 0] = [];
    println!("{:?}", min_max(&empty));
}
@@expected
Some((-2, 9))
Some((1.5, 2.5))
Some(('a', 'z'))
None
@@tests
#[test]
fn checks() {
    assert_eq!(min_max(&[5]), Some((5, 5)));
    assert_eq!(min_max(&[1u8, 1, 1]), Some((1, 1)));
    assert_eq!(min_max(&[10, 3, 8, 3, 12]), Some((3, 12)));
}
@@requires
fn min_max<T: PartialOrd + Copy>
@@hint
If the slice is empty, return `None`. Otherwise start with `let mut min = items[0]; let mut max = items[0];`, loop over the items (`for &item in items`), update both, and return `Some((min, max))`.
@@solution
fn min_max<T: PartialOrd + Copy>(items: &[T]) -> Option<(T, T)> {
    if items.is_empty() {
        return None;
    }
    let mut min = items[0];
    let mut max = items[0];
    for &item in items {
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    Some((min, max))
}

fn main() {
    println!("{:?}", min_max(&[3, 9, -2, 7]));
    println!("{:?}", min_max(&[2.5, 1.5]));
    println!("{:?}", min_max(&['m', 'a', 'z']));
    let empty: [i32; 0] = [];
    println!("{:?}", min_max(&empty));
}

@@exercise A labelled list
Write a generic function `numbered<T: std::fmt::Display>(items: &[T]) -> String` that returns the items as a numbered list, one per line, each line ending in `\n`:

```
1. apple
2. banana
```

An empty slice gives an empty string. It should work for anything printable: numbers, strings, and your own types that implement `Display`.
@@starter
use std::fmt;

struct Money {
    cents: u32,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

// write numbered here

fn main() {
    print!("{}", numbered(&["apple", "banana"]));
    print!("{}", numbered(&[10, 20, 30]));
    print!("{}", numbered(&[Money { cents: 1999 }, Money { cents: 5 }]));
}
@@expected
1. apple
2. banana
1. 10
2. 20
3. 30
1. $19.99
2. $0.05
@@tests
#[test]
fn checks() {
    let empty: [i32; 0] = [];
    assert_eq!(numbered(&empty), "");
    assert_eq!(numbered(&['x']), "1. x\n");
}
@@requires
fn numbered<T
Display
@@hint
`let mut out = String::new();` then `for (i, item) in items.iter().enumerate()` and `out.push_str(&format!("{}. {}\n", i + 1, item));`
@@solution
use std::fmt;

struct Money {
    cents: u32,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

fn numbered<T: fmt::Display>(items: &[T]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, item));
    }
    out
}

fn main() {
    print!("{}", numbered(&["apple", "banana"]));
    print!("{}", numbered(&[10, 20, 30]));
    print!("{}", numbered(&[Money { cents: 1999 }, Money { cents: 5 }]));
}
