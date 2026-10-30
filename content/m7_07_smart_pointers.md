@@lesson m7-pointers Box, Rc and RefCell
@@text
Ownership says every value has exactly one owner, and borrowing rules are checked at compile time. That covers the vast majority of programs. For the remaining cases, the standard library has **smart pointers**: types that own data and add an extra ability. This lesson introduces the three you'll meet most. You won't need them often, but you should recognize them.

## `Box<T>`: a value on the heap

`Box::new(value)` puts the value on the heap and gives you an owning pointer to it. When the box is dropped, so is the value. Two common uses:

**1. Trait objects**, which you've seen: `Vec<Box<dyn Shape>>`. Different shapes have different sizes, so they're stored on the heap behind a box of fixed size.

**2. Recursive types.** A type that contains itself would be infinitely large:

```rust,norun
enum Expr {
    Num(f64),
    Add(Expr, Expr),   // ❌ error: recursive type `Expr` has infinite size
}
```

A `Box` has a fixed size (it's just a pointer), so this works:

```rust
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

fn main() {
    // (2 + 3) * 4
    let e = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(2.0)), Box::new(Expr::Num(3.0)))),
        Box::new(Expr::Num(4.0)),
    );
    println!("{}", eval(&e));
}
```

Trees like this are how calculators, compilers and file-system browsers represent nested data. `eval` calls itself for the sub-expressions: **recursion**, a natural match for recursive data.

## `Rc<T>`: shared ownership

Sometimes a value genuinely has **several owners**, with no single one that lives the longest. For example, several playlists that contain the same song. `Rc` ("reference counted") allows this: `Rc::clone(&x)` creates another owner pointing to the **same** value (it doesn't copy the data), and the value is dropped when the last owner goes away.

```rust
use std::rc::Rc;

struct Song {
    title: String,
}

fn main() {
    let song = Rc::new(Song { title: String::from("Ferris Rag") });

    let road_trip = vec![Rc::clone(&song)];
    let workout = vec![Rc::clone(&song)];

    println!("{} / {}", road_trip[0].title, workout[0].title);
    println!("owners: {}", Rc::strong_count(&song));
}
```

`Rc` gives **shared, read-only** access: you can't mutate through it, because the other owners might be reading.

## `RefCell<T>`: borrowing checked at runtime

`RefCell` lets you mutate data through a shared reference, which is called **interior mutability**. Instead of the compiler checking the borrowing rules, `RefCell` checks them **while the program runs**:

- `.borrow()` gives read access (like `&`),
- `.borrow_mut()` gives write access (like `&mut`),
- breaking the rules (e.g. two `borrow_mut` at once) is a **panic** instead of a compile error.

## `Rc<RefCell<T>>`: shared *and* mutable

Combined, they give data with several owners that any of them can change:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Account {
    balance: i64,
}

fn main() {
    let shared = Rc::new(RefCell::new(Account { balance: 100 }));

    let alice = Rc::clone(&shared);
    let bob = Rc::clone(&shared);

    alice.borrow_mut().balance -= 30;
    bob.borrow_mut().balance += 5;

    println!("{:?}", shared.borrow());
}
```

## When to use what

| You need… | Use |
|---|---|
| A normal value with one owner | just the value (`T`) |
| A trait object, or a recursive type | `Box<T>` |
| Several owners, read-only | `Rc<T>` |
| Several owners that can change it | `Rc<RefCell<T>>` |
| The same, across threads | `Arc<Mutex<T>>` (the thread-safe versions) |

**A word of advice:** if you find yourself reaching for `Rc<RefCell<...>>` a lot, it's often a sign the design could be simpler, such as one struct owning all the data and handing out indices or IDs. That's exactly how the GUI app in Module 9 is structured: one `App` struct owns everything, and no smart pointers are needed.

@@quiz
Why can't `enum List { Item(i32, List), End }` compile?
- Enums can't contain numbers
* The type contains itself, so its size would be infinite. `Box<List>` fixes it
- `End` is a reserved word
? A `Box` is a fixed-size pointer, which breaks the infinite nesting.

@@quiz
What does `Rc::clone(&song)` do?
- Copies the whole song
* Creates another owner of the same song and increases the reference count
- Moves the song
? Rc clones are cheap: only a counter changes.

@@quiz
What happens if you call `.borrow_mut()` on a `RefCell` that's already mutably borrowed?
- A compile error
* The program panics at runtime
- It waits until the first borrow ends
? RefCell enforces the borrowing rules while the program runs.

@@exercise An expression tree
Extend the expression tree with subtraction (`Sub`) and division (`Div`), then write two functions:

- `eval(e: &Expr) -> f64` — computes the value
- `show(e: &Expr) -> String` — writes it out with parentheses around every operation: `Add(2, Mul(3, 4))` becomes `(2 + (3 * 4))`

Numbers are printed with `{}`, so `2.0` prints as `2`.
@@starter
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    // add Sub and Div
}

fn eval(e: &Expr) -> f64 {
    todo!()
}

fn show(e: &Expr) -> String {
    todo!()
}

fn num(n: f64) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

fn main() {
    // 10 - (6 / 2) * 3
    let e = Expr::Sub(num(10.0), Box::new(Expr::Mul(Box::new(Expr::Div(num(6.0), num(2.0))), num(3.0))));
    println!("{} = {}", show(&e), eval(&e));
}
@@expected
(10 - ((6 / 2) * 3)) = 1
@@tests
#[test]
fn checks() {
    assert_eq!(eval(&Expr::Num(7.5)), 7.5);
    assert_eq!(show(&Expr::Num(7.5)), "7.5");
    let e = Expr::Add(num(1.0), Box::new(Expr::Div(num(1.0), num(4.0))));
    assert_eq!(eval(&e), 1.25);
    assert_eq!(show(&e), "(1 + (1 / 4))");
}
@@requires
Sub(Box<Expr>, Box<Expr>)
Div(Box<Expr>, Box<Expr>)
@@hint
`eval`: `match e { Expr::Num(n) => *n, Expr::Add(a, b) => eval(a) + eval(b), ... }`.
`show`: `Expr::Num(n) => format!("{n}")`, `Expr::Add(a, b) => format!("({} + {})", show(a), show(b))`, and so on.
@@solution
enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn eval(e: &Expr) -> f64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
        Expr::Sub(a, b) => eval(a) - eval(b),
        Expr::Div(a, b) => eval(a) / eval(b),
    }
}

fn show(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("{n}"),
        Expr::Add(a, b) => format!("({} + {})", show(a), show(b)),
        Expr::Mul(a, b) => format!("({} * {})", show(a), show(b)),
        Expr::Sub(a, b) => format!("({} - {})", show(a), show(b)),
        Expr::Div(a, b) => format!("({} / {})", show(a), show(b)),
    }
}

fn num(n: f64) -> Box<Expr> {
    Box::new(Expr::Num(n))
}

fn main() {
    // 10 - (6 / 2) * 3
    let e = Expr::Sub(num(10.0), Box::new(Expr::Mul(Box::new(Expr::Div(num(6.0), num(2.0))), num(3.0))));
    println!("{} = {}", show(&e), eval(&e));
}
