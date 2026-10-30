@@lesson m3-mut-refs Mutable References and the Borrowing Rules
@@text
## `&mut` — borrowing to change

To let a function **change** something it borrows, use a **mutable reference**, `&mut`:

```rust
fn add_world(text: &mut String) {
    text.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    add_world(&mut s);
    println!("{s}");
}
```

Three things must line up:

1. the variable is declared `mut` (`let mut s`),
2. the reference is created with `&mut s`,
3. the parameter type is `&mut String`.

With numbers, you need `*` to change the value behind the reference:

```rust
fn level_up(level: &mut u32) {
    *level += 1;
}

fn main() {
    let mut level = 1;
    level_up(&mut level);
    level_up(&mut level);
    println!("Level {level}");
}
```

## The borrowing rules

Here is the heart of Rust. At any moment, for any value, you can have **either**:

- **any number of shared (read-only) references `&T`**, **or**
- **exactly one mutable reference `&mut T`**

— but **not both at the same time**. And references must always point to valid values.

Think of a document: many people can *read* it at the same time, no problem. But if someone is *editing* it, nobody else should be reading or editing at that moment — they might see a half-finished change.

```rust
fn main() {
    let mut scores = vec![10, 20, 30];

    let first = &scores[0];   // shared borrow of scores...
    scores.push(40);          // ...but push needs a mutable borrow!
    println!("{first}");      // the shared borrow is still in use here
}
```

Error: **cannot borrow `scores` as mutable because it is also borrowed as immutable**.

This isn't Rust being picky — it's preventing a **real bug**. `push` might need to grow the vector, and growing can *move all the elements to a new, bigger place in the heap*. `first` would then point at old, freed memory. In C++ this exact bug compiles fine and causes random crashes. In Rust, it can't happen.

## Borrows end when they're last used

A borrow lasts from where it's created to where it's **last used** — not necessarily to the end of the block. So this is fine:

```rust
fn main() {
    let mut scores = vec![10, 20, 30];

    let first = scores[0];      // copies the i32 — no borrow kept
    let total: i32 = scores.iter().sum();   // a shared borrow, finished right away
    println!("first {first}, total {total}");

    scores.push(40);            // ✔ no borrows are active any more
    println!("{:?}", scores);
}
```

When the compiler complains about borrowing, these tricks usually fix it:

- **Copy** small values out instead of keeping a reference (`let first = scores[0];`).
- **Reorder** the code so you finish reading before you start changing.
- **Shorten** the borrow: do the reading in its own block or function.
- As a last resort, **clone**.

## Changing elements in a loop

To change every element of a vector, borrow it mutably in the `for`:

```rust
fn main() {
    let mut prices = vec![10.0, 25.0, 4.0];
    for price in &mut prices {
        *price *= 1.2;   // add 20%
    }
    println!("{:?}", prices);
}
```

Inside the loop, `price` is a `&mut f64`, so `*price` is the number itself.

## Why these rules are a superpower

The borrowing rules eliminate, **at compile time**, whole categories of bugs that plague other languages:

- using memory after it's been freed,
- changing a collection while looping over it,
- two parts of a program (or two threads!) changing the same data at once — "data races", some of the hardest bugs to find.

The compiler fights with you a bit at first. After a few weeks it becomes natural, and you'll start designing code that's clearer about who changes what.

@@quiz
Which of these is allowed at the same time, for the same value?
* Three `&` references
- One `&mut` and one `&` reference
- Two `&mut` references
? Many readers OR one writer — never both.

@@quiz
What's needed to change an `i32` through `r: &mut i32`?
- `r += 1;`
* `*r += 1;`
- `&r += 1;`
? `*r` is the value behind the reference.

@@exercise Double everything
Write `double_all`, which **changes** the vector it's given, doubling every number. It returns nothing.
@@starter
fn double_all(numbers: &mut Vec<i32>) {
    todo!()
}

fn main() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    println!("{:?}", v);
}
@@tests
#[test]
fn checks() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
    let mut empty: Vec<i32> = vec![];
    double_all(&mut empty);
    assert_eq!(empty, Vec::<i32>::new());
}
@@hint
`for n in numbers.iter_mut() { *n *= 2; }` — or `for n in numbers` works too, because `numbers` is already a `&mut Vec`.
@@solution
fn double_all(numbers: &mut Vec<i32>) {
    for n in numbers.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    println!("{:?}", v);
}

@@exercise Fix the borrow conflict
This program keeps a reference to the biggest score while adding a new score, which breaks the borrowing rules. Fix it **without using `.clone()`**: copy the number out instead of keeping a reference, or reorder the lines. Output:

```text
Biggest before: 30
Scores: [10, 30, 20, 50]
```
@@starter
fn main() {
    let mut scores = vec![10, 30, 20];
    let biggest = scores.iter().max().unwrap();
    scores.push(50);
    println!("Biggest before: {biggest}");
    println!("Scores: {:?}", scores);
}
@@expected
Biggest before: 30
Scores: [10, 30, 20, 50]
@@hint
`scores.iter().max()` gives an `Option<&i32>` — a reference into the vector. Copy the number out: `let biggest = *scores.iter().max().unwrap();`
@@solution
fn main() {
    let mut scores = vec![10, 30, 20];
    let biggest = *scores.iter().max().unwrap();
    scores.push(50);
    println!("Biggest before: {biggest}");
    println!("Scores: {:?}", scores);
}

@@exercise Bank transfer
Write `transfer`, which moves `amount` from one account balance to another. If the `from` account doesn't have enough money, do nothing and return `false`; otherwise do the transfer and return `true`.

Both balances are passed as `&mut u32`.
@@starter
fn transfer(from: &mut u32, to: &mut u32, amount: u32) -> bool {
    todo!()
}

fn main() {
    let mut alice = 100;
    let mut bob = 20;
    let ok = transfer(&mut alice, &mut bob, 30);
    println!("ok={ok} alice={alice} bob={bob}");
}
@@tests
#[test]
fn checks() {
    let (mut a, mut b) = (100, 20);
    assert!(transfer(&mut a, &mut b, 30));
    assert_eq!((a, b), (70, 50));
    assert!(!transfer(&mut a, &mut b, 71), "not enough money: should return false");
    assert_eq!((a, b), (70, 50), "a failed transfer must not change anything");
    assert!(transfer(&mut a, &mut b, 70), "transferring everything is allowed");
    assert_eq!((a, b), (0, 120));
}
@@hint
`if *from < amount { return false; }` then `*from -= amount;` and `*to += amount;`, then `true`.
@@solution
fn transfer(from: &mut u32, to: &mut u32, amount: u32) -> bool {
    if *from < amount {
        return false;
    }
    *from -= amount;
    *to += amount;
    true
}

fn main() {
    let mut alice = 100;
    let mut bob = 20;
    let ok = transfer(&mut alice, &mut bob, 30);
    println!("ok={ok} alice={alice} bob={bob}");
}
