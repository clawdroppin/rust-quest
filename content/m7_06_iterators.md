@@lesson m7-iterators Iterators
@@text
An **iterator** produces a sequence of values, one at a time. Every `for` loop you've written used one. Iterators also come with dozens of methods that, combined with closures, let you express data processing in a few clear lines.

## The idea

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let mut it = numbers.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

An iterator has one essential method, `next()`, which returns `Some(value)` until the sequence ends, then `None`. A `for` loop just calls `next()` until it gets `None`.

Three ways to get an iterator from a collection:

| Method | Gives you | The collection afterwards |
|---|---|---|
| `.iter()` | `&T` (references) | still usable |
| `.iter_mut()` | `&mut T` (can modify items) | still usable |
| `.into_iter()` | `T` (the values themselves) | moved, gone |

## Adapters: transform a sequence

**Adapters** take an iterator and return a new iterator:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    let evens: Vec<&i32> = numbers.iter().filter(|n| *n % 2 == 0).collect();
    let squares_of_odds: Vec<i32> = numbers.iter()
        .filter(|n| *n % 2 == 1)
        .map(|n| n * n)
        .collect();

    println!("{:?}", doubled);
    println!("{:?}", evens);
    println!("{:?}", squares_of_odds);
}
```

| Adapter | Does |
|---|---|
| `map(f)` | transforms each item |
| `filter(f)` | keeps items where `f` returns true |
| `filter_map(f)` | map + filter in one: keeps the `Some`s |
| `enumerate()` | pairs each item with its index: `(i, item)` |
| `zip(other)` | pairs items from two iterators |
| `rev()` | backwards |
| `take(n)` / `skip(n)` | first n items / all but the first n |
| `take_while(f)` / `skip_while(f)` | while a condition holds |
| `chain(other)` | one iterator after another |
| `flat_map(f)` | map to iterators, then flatten |
| `cloned()` / `copied()` | turns `&T` into `T` |

**Iterators are lazy**: adapters do nothing until something *consumes* the iterator. This lets Rust fuse a whole chain into a single loop, as fast as hand-written code.

## Consumers: produce a result

**Consumers** run the iterator and produce a final value:

```rust
fn main() {
    let scores = vec![72, 95, 48, 88, 61];

    let total: i32 = scores.iter().sum();
    let count = scores.iter().count();
    let best = scores.iter().max();
    let passed = scores.iter().filter(|s| **s >= 60).count();
    let any_perfect = scores.iter().any(|s| *s == 100);
    let all_positive = scores.iter().all(|s| *s > 0);
    let first_fail = scores.iter().find(|s| **s < 50);
    let position = scores.iter().position(|s| *s == 88);

    println!("{total} {count} {best:?} {passed} {any_perfect} {all_positive}");
    println!("{first_fail:?} {position:?}");
}
```

| Consumer | Returns |
|---|---|
| `collect()` | a collection: `Vec`, `String`, `HashMap`… |
| `sum()`, `product()` | the total |
| `count()` | how many items |
| `min()`, `max()` | `Option` of the smallest/largest |
| `min_by_key(f)`, `max_by_key(f)` | the item with the smallest/largest key |
| `any(f)`, `all(f)` | `bool` |
| `find(f)` | `Option` of the first match |
| `position(f)` | `Option` of the first match's index |
| `fold(start, f)` | combines all items into one value |
| `for_each(f)` | runs `f` on each item |

## About those `*` and `**`

`numbers.iter()` produces `&i32` (references). `filter` passes its closure a reference to *that*, so `&&i32`. That's why you see `*n` or `**s`. Alternatives that read better:

```rust,norun
numbers.iter().filter(|&&n| n % 2 == 0)        // destructure the references in the pattern
numbers.iter().copied().filter(|n| n % 2 == 0) // copy to plain i32 first
```

Don't worry about getting it right first time: the compiler tells you exactly when a `*` is missing.

## `collect` into different things

`collect` needs to know what to build, from a type annotation or the "turbofish" `::<>`:

```rust
use std::collections::HashMap;

fn main() {
    let words = vec!["apple", "bob", "kiwi"];

    let lengths: HashMap<&str, usize> = words.iter().map(|w| (*w, w.len())).collect();
    let shout: String = "hello".chars().map(|c| c.to_ascii_uppercase()).collect();
    let numbers = "4,8,15".split(',').map(|s| s.parse::<i32>()).collect::<Result<Vec<_>, _>>();

    println!("{}", lengths["kiwi"]);
    println!("{shout}");
    println!("{:?}", numbers);
}
```

That last one is neat: collecting `Result`s into `Result<Vec<_>, _>` gives `Ok(vec)` if every item was `Ok`, or the first `Err`.

## `fold`: the general-purpose consumer

`fold` carries an "accumulator" through the items. `sum`, `count`, `max`… could all be written with `fold`:

```rust
fn main() {
    let words = ["rust", "is", "fun"];
    let total_len = words.iter().fold(0, |acc, w| acc + w.len());
    let sentence = words.iter().fold(String::new(), |acc, w| {
        if acc.is_empty() { w.to_string() } else { acc + " " + w }
    });
    println!("{total_len}: {sentence}");
}
```

## Loop or iterator chain?

Both are fine! Iterator chains shine for "transform, filter, summarize" pipelines. Plain loops are often clearer when there's complex logic, early returns, or several things happening at once. Experienced Rust programmers use both.

```rust,norun
// loop version
let mut total = 0;
for order in &orders {
    if order.paid {
        total += order.amount;
    }
}

// iterator version
let total: u32 = orders.iter().filter(|o| o.paid).map(|o| o.amount).sum();
```

## Making your own iterator

Implement the `Iterator` trait: say what `Item` it produces, and write `next`. You get every adapter and consumer for free:

```rust
struct Countdown {
    current: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;
            Some(self.current + 1)
        }
    }
}

fn main() {
    for n in (Countdown { current: 3 }) {
        println!("{n}...");
    }
    let total: u32 = Countdown { current: 10 }.filter(|n| n % 2 == 0).sum();
    println!("liftoff! (and {total})");
}
```

@@quiz
What does `vec![1, 2, 3].iter().map(|n| n * 10)` do by itself, with nothing after it?
- Creates `[10, 20, 30]`
* Nothing yet: iterators are lazy until consumed (e.g. by `collect` or `sum`)
- Prints the numbers
? Adapters only describe the work. A consumer like `collect()` actually runs it.

@@quiz
Which gives the number of words longer than 3 letters?
- `words.iter().map(|w| w.len() > 3).sum()`
* `words.iter().filter(|w| w.len() > 3).count()`
- `words.iter().find(|w| w.len() > 3)`
? `filter` keeps the matching items; `count` counts them.

@@exercise Pipeline practice
Implement each function with an **iterator chain** (no `for` loops):

- `squares(n)` → the squares of `1..=n` as a `Vec<u32>`
- `total_length(words)` → the sum of the lengths of all words
- `initials(name)` → the first letter of each word, uppercase: `"ada king lovelace"` → `"AKL"`
- `count_above(scores, limit)` → how many scores are strictly greater than `limit`
@@starter
fn squares(n: u32) -> Vec<u32> {
    todo!()
}

fn total_length(words: &[&str]) -> usize {
    todo!()
}

fn initials(name: &str) -> String {
    todo!()
}

fn count_above(scores: &[u32], limit: u32) -> usize {
    todo!()
}

fn main() {
    println!("{:?}", squares(5));
    println!("{}", total_length(&["one", "three", "five"]));
    println!("{}", initials("ada king lovelace"));
    println!("{}", count_above(&[50, 80, 65, 90], 60));
}
@@expected
[1, 4, 9, 16, 25]
12
AKL
3
@@tests
#[test]
fn checks() {
    assert_eq!(squares(0), Vec::<u32>::new());
    assert_eq!(total_length(&[]), 0);
    assert_eq!(initials("  grace   hopper "), "GH");
    assert_eq!(initials(""), "");
    assert_eq!(count_above(&[60, 60], 60), 0);
}
@@requires
.map(
.filter(
.collect
@@hint
- `(1..=n).map(|x| x * x).collect()`
- `words.iter().map(|w| w.len()).sum()`
- `name.split_whitespace().filter_map(|w| w.chars().next()).map(|c| c.to_ascii_uppercase()).collect()`
- `scores.iter().filter(|&&s| s > limit).count()`
@@solution
fn squares(n: u32) -> Vec<u32> {
    (1..=n).map(|x| x * x).collect()
}

fn total_length(words: &[&str]) -> usize {
    words.iter().map(|w| w.len()).sum()
}

fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn count_above(scores: &[u32], limit: u32) -> usize {
    scores.iter().filter(|&&s| s > limit).count()
}

fn main() {
    println!("{:?}", squares(5));
    println!("{}", total_length(&["one", "three", "five"]));
    println!("{}", initials("ada king lovelace"));
    println!("{}", count_above(&[50, 80, 65, 90], 60));
}

@@exercise Order report
An online shop has a list of orders. Using iterator methods, write:

- `paid_total(orders)` → the sum of `amount` over **paid** orders
- `biggest_customer(orders)` → the `customer` of the order with the largest amount, or `None` if there are no orders (`max_by_key` is perfect here)
- `customers(orders)` → the names of all customers, **sorted alphabetically with no duplicates**, as a `Vec<String>`
@@starter
struct Order {
    customer: String,
    amount: u32,
    paid: bool,
}

fn paid_total(orders: &[Order]) -> u32 {
    todo!()
}

fn biggest_customer(orders: &[Order]) -> Option<String> {
    todo!()
}

fn customers(orders: &[Order]) -> Vec<String> {
    todo!()
}

fn order(customer: &str, amount: u32, paid: bool) -> Order {
    Order { customer: customer.to_string(), amount, paid }
}

fn main() {
    let orders = vec![
        order("Zoe", 120, true),
        order("Adam", 40, false),
        order("Mia", 300, true),
        order("Adam", 75, true),
    ];
    println!("{}", paid_total(&orders));
    println!("{:?}", biggest_customer(&orders));
    println!("{:?}", customers(&orders));
}
@@expected
495
Some("Mia")
["Adam", "Mia", "Zoe"]
@@tests
#[test]
fn checks() {
    assert_eq!(paid_total(&[]), 0);
    assert_eq!(biggest_customer(&[]), None);
    assert_eq!(customers(&[]), Vec::<String>::new());
    let o = vec![order("b", 5, false), order("a", 1, false), order("b", 2, false)];
    assert_eq!(paid_total(&o), 0);
    assert_eq!(customers(&o), vec!["a", "b"]);
}
@@requires
.filter(
.max_by_key(
@@hint
- `orders.iter().filter(|o| o.paid).map(|o| o.amount).sum()`
- `orders.iter().max_by_key(|o| o.amount).map(|o| o.customer.clone())`
- collect the names into a `Vec<String>`, then `names.sort();` and `names.dedup();` (dedup removes *adjacent* duplicates, which is why you sort first)
@@solution
struct Order {
    customer: String,
    amount: u32,
    paid: bool,
}

fn paid_total(orders: &[Order]) -> u32 {
    orders.iter().filter(|o| o.paid).map(|o| o.amount).sum()
}

fn biggest_customer(orders: &[Order]) -> Option<String> {
    orders.iter().max_by_key(|o| o.amount).map(|o| o.customer.clone())
}

fn customers(orders: &[Order]) -> Vec<String> {
    let mut names: Vec<String> = orders.iter().map(|o| o.customer.clone()).collect();
    names.sort();
    names.dedup();
    names
}

fn order(customer: &str, amount: u32, paid: bool) -> Order {
    Order { customer: customer.to_string(), amount, paid }
}

fn main() {
    let orders = vec![
        order("Zoe", 120, true),
        order("Adam", 40, false),
        order("Mia", 300, true),
        order("Adam", 75, true),
    ];
    println!("{}", paid_total(&orders));
    println!("{:?}", biggest_customer(&orders));
    println!("{:?}", customers(&orders));
}

@@exercise Your own iterator: Fibonacci
Implement `Iterator` for `Fibonacci` so it produces the Fibonacci sequence: `0, 1, 1, 2, 3, 5, 8, ...` (each number is the sum of the two before it). It never ends: `next` always returns `Some`.

Because it's an iterator, `main` can use `take`, `filter` and `sum` on it.
@@starter
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

// implement Iterator for Fibonacci

fn main() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", first);
    let even_sum: u64 = Fibonacci::new().take_while(|&n| n < 100).filter(|n| n % 2 == 0).sum();
    println!("{even_sum}");
}
@@expected
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
44
@@tests
#[test]
fn checks() {
    let mut f = Fibonacci::new();
    assert_eq!(f.next(), Some(0));
    assert_eq!(f.next(), Some(1));
    assert_eq!(f.nth(8), Some(55));
}
@@requires
impl Iterator for Fibonacci
type Item = u64;
@@hint
In `next`: remember `let value = self.current;`, then move forward: the new `current` is `self.next`, the new `next` is `value + self.next`. Return `Some(value)`.
@@solution
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.current;
        self.current = self.next;
        self.next += value;
        Some(value)
    }
}

fn main() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", first);
    let even_sum: u64 = Fibonacci::new().take_while(|&n| n < 100).filter(|n| n % 2 == 0).sum();
    println!("{even_sum}");
}
