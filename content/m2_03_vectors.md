@@lesson m2-vectors Vectors: Lists That Grow
@@text
Arrays have a fixed length. But most real lists change: items get added to a shopping cart, enemies appear and disappear, messages arrive. For that, Rust has the **vector**, written `Vec`. It's the collection you'll use more than any other.

## Creating a vector

```rust
fn main() {
    let mut cart: Vec<&str> = Vec::new();   // an empty vector
    cart.push("apples");
    cart.push("bread");
    cart.push("milk");
    println!("{:?}", cart);

    let primes = vec![2, 3, 5, 7, 11];       // vec! macro: a vector with values
    println!("{:?} has {} items", primes, primes.len());
}
```

- `Vec<&str>` reads "a vector of `&str`s". The type in `< >` is the **element type**.
- `Vec::new()` creates an empty vector. Because it's empty, Rust can't guess the element type, so we wrote it. (If you push something right away, Rust figures it out.)
- `vec![...]` creates a vector with starting values — just like an array literal.
- `push` adds to the **end**. The vector must be `mut` to change.

Indexing, `len()` and `for` loops work exactly like with arrays.

## Adding and removing

```rust
fn main() {
    let mut queue = vec!["Ana", "Ben"];
    queue.push("Cleo");             // add at the end
    queue.insert(0, "Dan");         // add at index 0 (shifts the others right)
    println!("{:?}", queue);

    let removed = queue.remove(1);  // remove at index 1 (shifts the others left)
    println!("removed {removed}, now {:?}", queue);

    queue.clear();                  // remove everything
    println!("empty? {}", queue.is_empty());
}
```

## Useful vector methods

```rust
fn main() {
    let mut numbers = vec![42, 7, 19, 7, 3];
    println!("contains 19? {}", numbers.contains(&19));
    numbers.sort();
    println!("sorted: {:?}", numbers);
    numbers.dedup();               // remove neighbouring duplicates
    println!("deduped: {:?}", numbers);
    numbers.reverse();
    println!("reversed: {:?}", numbers);
    numbers.extend([100, 200]);    // add several at once
    println!("extended: {:?}", numbers);
    numbers.truncate(2);           // keep only the first 2
    println!("truncated: {:?}", numbers);
}
```

## A first look at `Option`

What should `pop()` (remove the last element) return if the vector is **empty**? There's nothing to give back! Rust's answer is a type called `Option`, which means "maybe a value":

- `Some(value)` — there is a value, and here it is.
- `None` — there is no value.

```rust
fn main() {
    let mut stack = vec![1, 2];
    println!("{:?}", stack.pop());   // Some(2)
    println!("{:?}", stack.pop());   // Some(1)
    println!("{:?}", stack.pop());   // None — it was empty
}
```

Several methods return `Option` because they might have nothing to give: `pop()`, `first()`, `last()`, `get(index)` (a safe version of `[index]` that doesn't panic), `iter().max()`…

There are two simple ways to deal with an `Option` for now:

```rust
fn main() {
    let scores = vec![12, 40, 31];

    // 1. if let: "if there is a value, call it x and run this block"
    if let Some(top) = scores.iter().max() {
        println!("Top score: {top}");
    } else {
        println!("No scores yet");
    }

    // 2. unwrap(): take the value out, or PANIC if it's None.
    //    Only use it when you're sure there's a value.
    let first = scores.first().unwrap();
    println!("First score: {first}");

    // get(): safe indexing
    println!("{:?}", scores.get(1));    // Some(40)
    println!("{:?}", scores.get(99));   // None (no panic!)
}
```

`Option` is one of Rust's most important ideas and gets a full lesson in Module 4. You'll see it everywhere.

@@quiz
`let mut v = vec![1, 2, 3]; v.remove(0); v.push(9);` — what is `v` now?
- `[1, 2, 3, 9]`
* `[2, 3, 9]`
- `[9, 2, 3]`
? `remove(0)` takes out the first element (1), leaving `[2, 3]`. Then `push(9)` adds 9 at the end.

@@text
## Passing lists to functions

To give a list to a function **without giving it away**, put `&` in front: you *lend* the function the list so it can look at it. The parameter type is `&[i32]`, which means "a borrowed view of some `i32`s". It accepts both vectors and arrays:

```rust
fn total(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}

fn main() {
    let v = vec![1, 2, 3];
    let a = [10, 20];
    println!("{}", total(&v));
    println!("{}", total(&a));
    println!("v is still usable: {:?}", v);
}
```

If the function needs to **change** the list, lend it *mutably* with `&mut`:

```rust
fn add_item(list: &mut Vec<String>, item: &str) {
    list.push(item.to_string());
}

fn main() {
    let mut todo = Vec::new();
    add_item(&mut todo, "learn vectors");
    add_item(&mut todo, "drink water");
    println!("{:?}", todo);
}
```

And a function can **create and return** a new vector:

```rust
fn countdown(from: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for n in (0..=from).rev() {
        result.push(n);
    }
    result
}

fn main() {
    println!("{:?}", countdown(5));
}
```

**Why the `&`?** This is about *ownership*, Rust's most famous feature, which is the whole of Module 3. For now, follow these rules of thumb:

| The function needs to… | Parameter type | Call it with |
|---|---|---|
| read a list | `&[T]` | `f(&list)` |
| change a list | `&mut Vec<T>` | `f(&mut list)` |
| make a new list | return `Vec<T>` | `let list = f();` |

(`T` stands for "whatever the element type is".)

@@exercise Average of a list
Write `average`, which returns the average of a list of numbers. If the list is **empty**, return `0.0` (otherwise you'd divide by zero!).
@@starter
fn average(numbers: &[f64]) -> f64 {
    todo!()
}

fn main() {
    let grades = vec![7.5, 8.0, 9.5];
    println!("{}", average(&grades));
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    assert!(close(average(&[7.5, 8.0, 9.5]), 25.0 / 3.0));
    assert!(close(average(&[4.0]), 4.0));
    assert!(close(average(&[]), 0.0), "an empty list should give 0.0");
    assert!(close(average(&[-2.0, 2.0]), 0.0));
}
@@hint
Check `numbers.is_empty()` first and `return 0.0;`. Then sum in a loop and divide by `numbers.len() as f64`.
@@solution
fn average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for n in numbers {
        sum += n;
    }
    sum / numbers.len() as f64
}

fn main() {
    let grades = vec![7.5, 8.0, 9.5];
    println!("{}", average(&grades));
}

@@exercise Keep the evens
Write `evens`, which returns a **new** vector containing only the even numbers from the input, in the same order.
@@starter
fn evens(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{:?}", evens(&[1, 2, 3, 4, 5, 6]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    assert_eq!(evens(&[1, 3, 5]), Vec::<i32>::new());
    assert_eq!(evens(&[0, -2, 7]), vec![0, -2]);
}
@@hint
Create `let mut result = Vec::new();`, loop over `numbers`, `push` the even ones, and return `result`. Inside `for n in numbers`, `n` is a borrowed number; write `*n` to get the number itself when pushing (the `*` is explained in Module 3).
@@solution
fn evens(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for n in numbers {
        if n % 2 == 0 {
            result.push(*n);
        }
    }
    result
}

fn main() {
    println!("{:?}", evens(&[1, 2, 3, 4, 5, 6]));
}

@@exercise Shopping list manager
Write two functions that **change** a shopping list:

- `add_unique(list, item)` adds `item` to the end, but **only if it isn't already in the list**.
- `remove_item(list, item)` removes `item` if it's in the list (and does nothing if it isn't).

Tip: `list.contains(&x)` checks for a value, and `list.iter().position(|x| x == item)` gives you `Some(index)` of the first match, or `None`.
@@starter
fn add_unique(list: &mut Vec<String>, item: &str) {
    todo!()
}

fn remove_item(list: &mut Vec<String>, item: &str) {
    todo!()
}

fn main() {
    let mut list = Vec::new();
    add_unique(&mut list, "eggs");
    add_unique(&mut list, "milk");
    add_unique(&mut list, "eggs");
    remove_item(&mut list, "milk");
    println!("{:?}", list);
}
@@tests
#[test]
fn checks() {
    let mut list = Vec::new();
    add_unique(&mut list, "eggs");
    add_unique(&mut list, "milk");
    add_unique(&mut list, "eggs");
    assert_eq!(list, vec!["eggs", "milk"], "eggs should only be added once");
    remove_item(&mut list, "milk");
    assert_eq!(list, vec!["eggs"]);
    remove_item(&mut list, "cheese");
    assert_eq!(list, vec!["eggs"], "removing a missing item should do nothing");
}
@@hint
For `add_unique`: `if !list.contains(&item.to_string()) { list.push(item.to_string()); }`.
For `remove_item`: `if let Some(i) = list.iter().position(|x| x == item) { list.remove(i); }`.
@@solution
fn add_unique(list: &mut Vec<String>, item: &str) {
    if !list.contains(&item.to_string()) {
        list.push(item.to_string());
    }
}

fn remove_item(list: &mut Vec<String>, item: &str) {
    if let Some(i) = list.iter().position(|x| x == item) {
        list.remove(i);
    }
}

fn main() {
    let mut list = Vec::new();
    add_unique(&mut list, "eggs");
    add_unique(&mut list, "milk");
    add_unique(&mut list, "eggs");
    remove_item(&mut list, "milk");
    println!("{:?}", list);
}
