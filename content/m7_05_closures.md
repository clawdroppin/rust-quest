@@lesson m7-closures Closures
@@text
A **closure** is a small, unnamed function you can write right where you need it, store in a variable, or pass to another function. Closures are everywhere in Rust: sorting, filtering, button handlers in a GUI, and all the iterator methods in the next lesson.

## Syntax

```rust
fn main() {
    let double = |x| x * 2;
    let add = |a: i32, b: i32| a + b;
    let greet = || println!("hi!");
    let describe = |n: i32| {
        if n > 0 { "positive" } else { "not positive" }
    };

    println!("{}", double(21));
    println!("{}", add(2, 3));
    greet();
    println!("{}", describe(-4));
}
```

- Parameters go between pipes `| |` instead of parentheses.
- The body is a single expression, or a `{ }` block for several lines.
- Types are usually **inferred**, so you rarely write them. Compare:

```rust,norun
fn  double_fn(x: i32) -> i32 { x * 2 }   // a function
let double   = |x: i32| -> i32 { x * 2 }; // a closure with every type written out
let double   = |x| x * 2;                // the same closure, types inferred
```

## Closures capture their surroundings

The big difference from functions: a closure can **use variables from the place it was created**.

```rust
fn main() {
    let tax_rate = 0.2;
    let with_tax = |price: f64| price * (1.0 + tax_rate);   // uses tax_rate from outside
    println!("{}", with_tax(100.0));

    let mut count = 0;
    let mut increment = || count += 1;   // modifies count, so the closure is `mut`
    increment();
    increment();
    println!("count = {count}");
}
```

A regular `fn` can't do that: it only sees its parameters and global constants.

## Passing closures to functions

You've already used methods that take closures. Here are some on `Vec`:

```rust
fn main() {
    let mut words = vec!["banana", "Apple", "cherry", "fig"];

    words.sort_by_key(|w| w.len());                 // sort by length
    println!("{:?}", words);

    words.sort_by_key(|w| w.to_lowercase());        // alphabetical, ignoring case
    println!("{:?}", words);

    words.retain(|w| w.len() > 3);                  // keep only long words
    println!("{:?}", words);

    let has_fig = words.iter().any(|w| *w == "fig");
    println!("{has_fig}");
}
```

`sort_by` takes a closure that compares two items, returning an `Ordering`. This is how you sort by any rule, like highest score first:

```rust
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let mut players = vec![
        Player { name: String::from("Ben"), score: 30 },
        Player { name: String::from("Ada"), score: 50 },
        Player { name: String::from("Cy"), score: 40 },
    ];
    players.sort_by(|a, b| b.score.cmp(&a.score));   // b before a: descending
    for p in &players {
        println!("{} {}", p.name, p.score);
    }
}
```

## Writing functions that take closures

A closure's type has no name you can write, so functions accept closures through a **trait bound**. There are three closure traits:

| Trait | The closure… | Can be called |
|---|---|---|
| `Fn` | only reads what it captured | many times |
| `FnMut` | changes what it captured | many times (needs `mut`) |
| `FnOnce` | consumes (moves out) what it captured | once |

For most of your own functions, `Fn` is what you want:

```rust
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn count_matching(items: &[i32], test: impl Fn(i32) -> bool) -> usize {
    let mut n = 0;
    for &item in items {
        if test(item) {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 1));
    println!("{}", apply_twice(|x| x * x, 3));

    let numbers = [3, 8, 12, 5, 20];
    let limit = 6;
    println!("{}", count_matching(&numbers, |n| n > limit));
    println!("{}", count_matching(&numbers, |n| n % 2 == 0));
}
```

`impl Fn(i32) -> bool` reads "any closure that takes an `i32` and returns a `bool`". Regular functions fit too: `count_matching(&numbers, is_prime)` works if `fn is_prime(n: i32) -> bool` exists.

## `move` closures

Normally a closure *borrows* what it captures. `move` makes it take **ownership** instead. You need this when the closure must outlive the current function, such as when returning a closure, or running it on another thread:

```rust
fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{greeting}, {name}!")
}

fn main() {
    let hello = make_greeter(String::from("Hello"));
    let ahoy = make_greeter(String::from("Ahoy"));
    println!("{}", hello("Ada"));
    println!("{}", ahoy("Ferris"));
}
```

Without `move`, the closure would borrow `greeting`, which is dropped when `make_greeter` returns. The compiler catches this and suggests adding `move`.

@@quiz
What can a closure do that a regular `fn` can't?
- Take parameters
- Return values
* Use variables from the scope where it was created
? Closures capture their environment. Functions only see their parameters (and global items).

@@quiz
How do you sort `players` by score, highest first?
- `players.sort()`
* `players.sort_by(|a, b| b.score.cmp(&a.score))`
- `players.sort_by(|a, b| a.score > b.score)`
? `sort_by` needs an `Ordering`; comparing `b` to `a` reverses the order.

@@exercise Keep the matching items
Write a function `keep<F: Fn(i32) -> bool>(items: &[i32], test: F) -> Vec<i32>` that returns a new vector with only the items for which `test` returns `true`.

`main` already uses it with three different closures, one of which captures a variable.
@@starter
// write keep here

fn main() {
    let numbers = [4, 15, 8, 23, 42, 16];
    println!("{:?}", keep(&numbers, |n| n % 2 == 0));
    println!("{:?}", keep(&numbers, |n| n > 15));
    let low = 5;
    let high = 20;
    println!("{:?}", keep(&numbers, |n| n >= low && n <= high));
}
@@expected
[4, 8, 42, 16]
[23, 42, 16]
[15, 8, 16]
@@tests
#[test]
fn checks() {
    assert_eq!(keep(&[], |_| true), Vec::<i32>::new());
    assert_eq!(keep(&[1, 2, 3], |_| false), Vec::<i32>::new());
    assert_eq!(keep(&[1, 2, 3], |_| true), vec![1, 2, 3]);
}
@@requires
fn keep<F: Fn(i32) -> bool>
@@hint
Create `let mut result = Vec::new();`, loop with `for &item in items`, and `if test(item) { result.push(item); }`.
@@solution
fn keep<F: Fn(i32) -> bool>(items: &[i32], test: F) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in items {
        if test(item) {
            result.push(item);
        }
    }
    result
}

fn main() {
    let numbers = [4, 15, 8, 23, 42, 16];
    println!("{:?}", keep(&numbers, |n| n % 2 == 0));
    println!("{:?}", keep(&numbers, |n| n > 15));
    let low = 5;
    let high = 20;
    println!("{:?}", keep(&numbers, |n| n >= low && n <= high));
}

@@exercise Leaderboard
Sort the players with `sort_by`: **highest score first**; players with the same score are ordered **alphabetically by name**.

Then use `retain` with a closure to remove every player whose score is below `cutoff`.

(Tip: `Ordering` has a method `.then_with(|| ...)` that's used when the first comparison is `Equal`. Or use a `match`.)
@@starter
struct Player {
    name: String,
    score: u32,
}

fn leaderboard(players: &mut Vec<Player>, cutoff: u32) {
    todo!()
}

fn main() {
    let mut players = vec![
        Player { name: String::from("Cy"), score: 40 },
        Player { name: String::from("Ada"), score: 50 },
        Player { name: String::from("Bo"), score: 40 },
        Player { name: String::from("Dee"), score: 10 },
        Player { name: String::from("Al"), score: 40 },
    ];
    leaderboard(&mut players, 20);
    for p in &players {
        println!("{} {}", p.score, p.name);
    }
}
@@expected
50 Ada
40 Al
40 Bo
40 Cy
@@tests
#[test]
fn checks() {
    let mut v = vec![
        Player { name: String::from("b"), score: 1 },
        Player { name: String::from("a"), score: 1 },
        Player { name: String::from("z"), score: 9 },
    ];
    leaderboard(&mut v, 1);
    let names: Vec<&str> = v.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["z", "a", "b"]);
    leaderboard(&mut v, 100);
    assert!(v.is_empty());
}
@@requires
sort_by
retain
@@hint
```
players.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));
players.retain(|p| p.score >= cutoff);
```
@@solution
struct Player {
    name: String,
    score: u32,
}

fn leaderboard(players: &mut Vec<Player>, cutoff: u32) {
    players.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.name.cmp(&b.name)));
    players.retain(|p| p.score >= cutoff);
}

fn main() {
    let mut players = vec![
        Player { name: String::from("Cy"), score: 40 },
        Player { name: String::from("Ada"), score: 50 },
        Player { name: String::from("Bo"), score: 40 },
        Player { name: String::from("Dee"), score: 10 },
        Player { name: String::from("Al"), score: 40 },
    ];
    leaderboard(&mut players, 20);
    for p in &players {
        println!("{} {}", p.score, p.name);
    }
}
