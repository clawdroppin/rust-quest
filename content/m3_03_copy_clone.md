@@lesson m3-clone Copy and Clone
@@text
## `Copy` types

Some types are so small and simple that copying them is basically free. These types are **`Copy`**: assigning or passing them makes a copy, and the original stays usable.

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let x = 21;
    let y = x;           // copy
    let z = double(x);   // copy
    println!("{x} {y} {z}");  // x is still fine
}
```

`Copy` types include:

- all integers (`i32`, `u8`, `usize`, …) and floats (`f64`, …)
- `bool` and `char`
- `&str` and other shared references (copying the reference, not the text!)
- tuples and arrays **if** everything inside them is `Copy` — e.g. `(i32, bool)`, `[f64; 3]`

**Not** `Copy`: anything that owns heap memory, like `String`, `Vec` and `HashMap`. For those, assignment **moves**.

> Rule of thumb: if a value has a fixed, small size and owns nothing on the heap, it's probably `Copy`.

## `.clone()` — an explicit deep copy

What if you really want two independent copies of a `String` or `Vec`? Call `.clone()`. This copies the heap data too:

```rust
fn main() {
    let original = vec![1, 2, 3];
    let mut copy = original.clone();
    copy.push(4);
    println!("original: {:?}", original);
    println!("copy:     {:?}", copy);
}
```

```text
  original ──►  [1, 2, 3]       (its own heap memory)
  copy     ──►  [1, 2, 3, 4]    (separate heap memory)
```

## Clone is fine — but not free

`.clone()` is the easiest way to make ownership errors go away, and **that's fine while you learn**. But it has a cost: cloning a vector with a million items copies a million items. Rust makes cloning *visible* (you have to type `.clone()`) so you always know when an expensive copy happens.

The better solution, most of the time, is to **borrow** instead of copying. That's the next lesson.

## Seeing moves, copies and clones together

```rust
fn main() {
    let a = 10;              // i32: Copy
    let b = a;               // copied
    println!("a={a} b={b}");

    let s = String::from("hi");  // String: not Copy
    let t = s.clone();           // deep copy
    let u = s;                   // move: s is no longer usable
    println!("t={t} u={u}");

    let pair = (1, 'x');     // (i32, char): Copy
    let pair2 = pair;
    println!("{:?} {:?}", pair, pair2);
}
```

@@quiz
Which of these types is **not** `Copy`?
- `u64`
- `(bool, char)`
* `Vec<u8>`
- `[i32; 4]`
? `Vec` owns heap memory, so it moves. Everything else listed is small and stack-only.

@@quiz
`let b = a.clone();` — what happens to `a`?
* Nothing — `a` is still usable, and `b` is an independent copy
- `a` is moved into `b`
- `a` becomes empty
? `clone` makes a full, separate copy and leaves the original alone.

@@exercise Two separate lists
This program wants a list of `weekdays`, and a separate list `all_days` that also includes the weekend. But it doesn't compile because `weekdays` is moved. Fix it using `.clone()` so the output is:

```text
Weekdays: ["Mon", "Tue", "Wed", "Thu", "Fri"]
All days: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
```
@@starter
fn main() {
    let weekdays = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
    let mut all_days = weekdays;
    all_days.push("Sat");
    all_days.push("Sun");
    println!("Weekdays: {:?}", weekdays);
    println!("All days: {:?}", all_days);
}
@@expected
Weekdays: ["Mon", "Tue", "Wed", "Thu", "Fri"]
All days: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
@@requires
.clone()
@@hint
`let mut all_days = weekdays.clone();`
@@solution
fn main() {
    let weekdays = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
    let mut all_days = weekdays.clone();
    all_days.push("Sat");
    all_days.push("Sun");
    println!("Weekdays: {:?}", weekdays);
    println!("All days: {:?}", all_days);
}

@@exercise Copy or move?
Read the program and predict whether each line compiles. Two lines cause errors, because they use values that were **moved**. Delete (or comment out) **only** those two lines, so the program prints:

```text
x = 5
point = (1, 2)
name2 = Ferris
list2 = [1, 2]
```
@@starter
fn main() {
    let x = 5;
    let y = x;
    let point = (1, 2);
    let point2 = point;
    let name = String::from("Ferris");
    let name2 = name;
    let list = vec![1, 2];
    let list2 = list;

    println!("x = {x}");
    println!("point = {:?}", point);
    println!("name = {name}");
    println!("name2 = {name2}");
    println!("list = {:?}", list);
    println!("list2 = {:?}", list2);
}
@@expected
x = 5
point = (1, 2)
name2 = Ferris
list2 = [1, 2]
@@hint
`i32` and `(i32, i32)` are `Copy`, so `x` and `point` are still usable. `String` and `Vec` are moved, so `name` and `list` aren't. (You might get warnings about unused `y` and `point2` — that's fine.)
@@solution
fn main() {
    let x = 5;
    let y = x;
    let point = (1, 2);
    let point2 = point;
    let name = String::from("Ferris");
    let name2 = name;
    let list = vec![1, 2];
    let list2 = list;

    println!("x = {x}");
    println!("point = {:?}", point);
    println!("name2 = {name2}");
    println!("list2 = {:?}", list2);
}
