@@lesson m3-lifetimes Lifetimes: The Basics
@@text
The final rule of borrowing: **a reference must never outlive the value it points to.** A reference to something that no longer exists is called a **dangling reference**, and in other languages it causes crashes and security holes. In Rust, it's a compile error.

## A dangling reference

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    } // x is dropped here...
    println!("{r}"); // ...but r still points to it!
}
```

Error: **`x` does not live long enough**. The compiler tracks how long every value lives — its **lifetime** — and checks every reference against it.

The same mistake inside a function:

```rust
fn make_text() -> &String {
    let s = String::from("hello");
    &s
} // s is dropped here, so the reference would dangle

fn main() {
    println!("{}", make_text());
}
```

The fix is simple: **return the owned value**, so ownership moves out to the caller:

```rust
fn make_text() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    println!("{}", make_text());
}
```

> **Rule of thumb:** a function can return a reference only if it points into one of its **inputs** (like `first_word` in the last lesson) or to something that lives for the whole program. If the function *creates* the value, return it owned.

## Lifetime annotations

Usually the compiler works out lifetimes on its own. But consider:

```rust
fn longer(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    println!("{}", longer("apple", "fig"));
}
```

Error: **missing lifetime specifier**. The result points into either `a` or `b` — the compiler can't tell which, so it doesn't know how long the result is allowed to live. You tell it with a **lifetime annotation**:

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let first = String::from("apple");
    let result;
    {
        let second = String::from("fig");
        result = longer(&first, &second);
        println!("{result}");
    }
}
```

- `'a` (said "tick a") is a **name for a lifetime**. It's declared in `<'a>` after the function name.
- `&'a str` means "a reference that lives at least as long as `'a`".
- The signature says: *the result lives as long as the **shorter-lived** of `a` and `b`*.

Annotations **don't change** how long anything lives. They only *describe* the relationships between references, so the compiler can check that callers use the result safely.

## When do you need annotations?

Rarely, at this stage. The compiler applies simple rules ("lifetime elision") so most functions need none:

- One reference parameter → the result borrows from it. (`fn first_word(s: &str) -> &str` needs no annotation.)
- A method taking `&self` → the result borrows from `self` (you'll see methods in the next module).

You need `'a` when a function returns a reference and has **several** reference parameters it could come from.

## `'static`

One special lifetime has a name already: `'static`, meaning "lives for the entire program". String literals are `&'static str`, because their text is stored inside the executable. You saw this in Module 1, in `fn even_or_odd(n: i32) -> &'static str`: the function returns literals like `"even"`, which always exist.

## Ownership summary

You now know the complete ownership system:

1. Every value has **one owner**; when the owner goes out of scope, the value is **dropped**.
2. Assigning or passing a value **moves** it (unless it's `Copy`); `.clone()` makes a deep copy.
3. You can **borrow** with references: many `&` **or** one `&mut` at a time.
4. References can **never outlive** what they point to.

When the compiler complains, it's one of these four rules. Read the error, find which rule, and fix it with the toolbox: borrow instead of move, reorder, copy small values out, return owned values, or clone.

@@quiz
Why is `fn make() -> &String { let s = String::from("x"); &s }` rejected?
- Functions can't return references
* `s` is dropped when the function ends, so the reference would dangle
- `String` can't be borrowed
? Return the `String` itself instead.

@@quiz
In `fn pick<'a>(x: &'a str, y: &'a str) -> &'a str`, what does `'a` do?
- It makes the strings live longer
* It tells the compiler the result is only valid while both inputs are valid
- It copies the strings
? Lifetime annotations describe relationships; they never change how long values live.

@@exercise Fix the dangling reference
`make_label` tries to return a reference to a `String` it created inside itself. Fix it so it returns an owned `String`. Output:

```text
Item #7
```
@@starter
fn make_label(n: u32) -> &String {
    let label = format!("Item #{n}");
    &label
}

fn main() {
    println!("{}", make_label(7));
}
@@expected
Item #7
@@hint
Change the return type to `String` and return `label` itself.
@@solution
fn make_label(n: u32) -> String {
    let label = format!("Item #{n}");
    label
}

fn main() {
    println!("{}", make_label(7));
}

@@exercise Longest line
Write `longest_line`, which returns the longest line in the text, as a slice **borrowed from the input** (no copying). On a tie, return the first one. For empty text, return `""`.

Since there's only one reference parameter, you don't need any lifetime annotations!
@@starter
fn longest_line(text: &str) -> &str {
    todo!()
}

fn main() {
    let poem = "Roses are red\nViolets are blue\nRust is great";
    println!("{}", longest_line(poem));
}
@@tests
#[test]
fn checks() {
    assert_eq!(longest_line("Roses are red\nViolets are blue\nRust is great"), "Violets are blue");
    assert_eq!(longest_line("ab\ncd"), "ab", "tie: the first one wins");
    assert_eq!(longest_line(""), "");
}
@@hint
`let mut best = "";` then `for line in text.lines() { if line.len() > best.len() { best = line; } }`.
@@solution
fn longest_line(text: &str) -> &str {
    let mut best = "";
    for line in text.lines() {
        if line.len() > best.len() {
            best = line;
        }
    }
    best
}

fn main() {
    let poem = "Roses are red\nViolets are blue\nRust is great";
    println!("{}", longest_line(poem));
}

@@exercise Pick the cheaper
Write `cheaper`, which takes two product names and their prices, and returns the **name** of the cheaper product (the first one if the prices are equal). The returned `&str` comes from one of the two name parameters, so you'll need a lifetime annotation `'a`.

The starter's signature is missing the annotations — add them.
@@starter
fn cheaper(name_a: &str, price_a: f64, name_b: &str, price_b: f64) -> &str {
    todo!()
}

fn main() {
    println!("{}", cheaper("apple", 0.5, "banana", 0.3));
}
@@tests
#[test]
fn checks() {
    assert_eq!(cheaper("apple", 0.5, "banana", 0.3), "banana");
    assert_eq!(cheaper("apple", 0.2, "banana", 0.3), "apple");
    assert_eq!(cheaper("apple", 1.0, "banana", 1.0), "apple", "equal prices: return the first");
}
@@requires
<'a>
@@hint
`fn cheaper<'a>(name_a: &'a str, price_a: f64, name_b: &'a str, price_b: f64) -> &'a str` — the prices are plain numbers, so they need no annotation.
@@solution
fn cheaper<'a>(name_a: &'a str, price_a: f64, name_b: &'a str, price_b: f64) -> &'a str {
    if price_a <= price_b { name_a } else { name_b }
}

fn main() {
    println!("{}", cheaper("apple", 0.5, "banana", 0.3));
}

@@text
## 🎉 Module 3 complete!

You now understand the part of Rust that most people find hardest. Don't worry if it doesn't feel completely natural yet; it takes practice, and you'll get plenty. From here on, compiler errors about borrowing should look like puzzles you know how to solve rather than mysteries.

**Next up:** structs and enums, which let you create your own types and model real-world data properly.
