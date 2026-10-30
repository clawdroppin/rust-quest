@@lesson m3-references References and Borrowing
@@text
Moving ownership into a function and returning it is clumsy. Cloning costs time and memory. Most of the time, a function just wants to **look at** a value for a moment. For that, Rust has **references**.

## Borrowing with `&`

A **reference** lets you refer to a value **without taking ownership** of it. You create one with `&`. Creating a reference is called **borrowing**:

```rust
fn count_letters(text: &String) -> usize {
    text.len()
} // text is only a reference: nothing is dropped here

fn main() {
    let name = String::from("Ferris");
    let n = count_letters(&name);   // lend name to the function
    println!("{name} has {n} letters");  // ✔ name is still ours
}
```

```text
  name ──────────────►  "Ferris" (heap)
   ▲
   │  (borrowed: points to name, owns nothing)
  text
```

- `&String` is the type "a reference to a String".
- `&name` creates a reference to `name`.
- The function **borrows** the value and must give it back when it's done — automatically, when the reference goes out of scope. Nothing is copied and nothing is moved.

Just like in real life: if you lend a friend your book, it's still your book. They can read it, and then they give it back.

## References are read-only by default

You can't change something you've borrowed with a plain `&`:

```rust
fn add_world(text: &String) {
    text.push_str(" world");
}

fn main() {
    let s = String::from("hello");
    add_world(&s);
}
```

Error: **cannot borrow `*text` as mutable, as it is behind a `&` reference**. Read-only borrowing is the default, which is a great guarantee: when you pass `&data` to a function, you *know* it won't be changed. The next lesson shows how to borrow mutably.

## Dereferencing with `*`

A reference *points to* a value. To get to the value itself, you **dereference** it with `*`:

```rust
fn main() {
    let x = 10;
    let r = &x;          // r is a reference to x (type &i32)
    println!("{}", *r + 1);   // follow the reference, then add 1
    println!("{}", r);        // printing follows references automatically
}
```

This is the `*` you saw in Module 2, like in `*counts.entry(word).or_insert(0) += 1`. `or_insert` gives you a *reference* to the count inside the map, and `*` reaches through it to change the count itself.

Rust often dereferences for you automatically — when calling methods (`r.len()` works on a reference) and when printing. You mostly need `*` when doing maths or assigning through a reference.

## References in loops

Now Module 2's loops make sense:

```rust
fn main() {
    let names = vec![String::from("Ana"), String::from("Ben")];
    for name in &names {          // borrow each element: name is a &String
        println!("{name}");
    }
    println!("still have {} names", names.len());  // names not moved
}
```

`for x in &list` borrows the list; `for x in list` moves it.

## `&str` and `&[T]` — the best parameter types

You've been writing `&str` and `&[i32]` as parameter types since Module 2. These are references too — references to a *part* of some text or list (called **slices**, coming up in two lessons).

When a function only needs to read text, write `&str` instead of `&String`. It accepts more: both string literals and borrowed `String`s:

```rust
fn shout(text: &str) {
    println!("{}!", text.to_uppercase());
}

fn main() {
    let owned = String::from("hello");
    shout(&owned);       // &String automatically becomes &str
    shout("hi there");   // a literal is already a &str
}
```

Likewise `&[T]` accepts borrowed vectors, arrays and slices. These are the most flexible parameter types for reading.

@@quiz
What does `&` do in `print_len(&name)`?
- It copies `name`
- It moves `name` into the function
* It lends the function a reference to `name`, so `name` keeps its owner
? `&` creates a reference (a borrow). Ownership doesn't change.

@@quiz
`let r = &x;` where `x` is an `i32`. What is `*r`?
- The address of `x`
* The value of `x`
- A copy of `r`
? `*` follows (dereferences) the reference to the value it points at.

@@exercise Borrow, don't move
`describe` takes ownership of the vector, so `main` can't print the vector afterwards. Change `describe` to **borrow** the list instead (use `&[i32]`), and update the call. Output:

```text
3 numbers, sum 60
[10, 20, 30]
```
@@starter
fn describe(numbers: Vec<i32>) -> String {
    let sum: i32 = numbers.iter().sum();
    format!("{} numbers, sum {}", numbers.len(), sum)
}

fn main() {
    let numbers = vec![10, 20, 30];
    println!("{}", describe(numbers));
    println!("{:?}", numbers);
}
@@expected
3 numbers, sum 60
[10, 20, 30]
@@requires
&[i32]
@@hint
Change the parameter to `numbers: &[i32]` and call `describe(&numbers)`.
@@solution
fn describe(numbers: &[i32]) -> String {
    let sum: i32 = numbers.iter().sum();
    format!("{} numbers, sum {}", numbers.len(), sum)
}

fn main() {
    let numbers = vec![10, 20, 30];
    println!("{}", describe(&numbers));
    println!("{:?}", numbers);
}

@@exercise Count matching words
Write `count_starting_with`, which counts how many words in the list start with the given letter (case-sensitive). The function must **borrow** both the list and the letter, so `main` can still use everything afterwards.
@@starter
fn count_starting_with(words: &[String], letter: char) -> usize {
    todo!()
}

fn main() {
    let words = vec![String::from("apple"), String::from("avocado"), String::from("banana")];
    println!("{}", count_starting_with(&words, 'a'));
    println!("{:?}", words);
}
@@tests
#[test]
fn checks() {
    let words = vec![String::from("apple"), String::from("avocado"), String::from("banana")];
    assert_eq!(count_starting_with(&words, 'a'), 2);
    assert_eq!(count_starting_with(&words, 'b'), 1);
    assert_eq!(count_starting_with(&words, 'z'), 0);
    assert_eq!(count_starting_with(&[], 'a'), 0);
}
@@hint
Loop with `for word in words` (each `word` is a `&String`), and check `word.starts_with(letter)`.
@@solution
fn count_starting_with(words: &[String], letter: char) -> usize {
    let mut count = 0;
    for word in words {
        if word.starts_with(letter) {
            count += 1;
        }
    }
    count
}

fn main() {
    let words = vec![String::from("apple"), String::from("avocado"), String::from("banana")];
    println!("{}", count_starting_with(&words, 'a'));
    println!("{:?}", words);
}

@@exercise Sum through references
Write `sum_refs`, which receives a list of **references** to numbers (`&[&i32]`) and returns their sum. You'll need `*` to get the values.
@@starter
fn sum_refs(numbers: &[&i32]) -> i32 {
    todo!()
}

fn main() {
    let a = 5;
    let b = 7;
    let c = 30;
    println!("{}", sum_refs(&[&a, &b, &c]));
}
@@tests
#[test]
fn checks() {
    let (a, b, c) = (5, 7, 30);
    assert_eq!(sum_refs(&[&a, &b, &c]), 42);
    assert_eq!(sum_refs(&[]), 0);
}
@@hint
In `for n in numbers`, `n` is a `&&i32` (a reference to a reference!). `**n` gets the number. Alternatively, write `for &n in numbers`, which unpacks one level of reference in the pattern, then use `*n`.
@@solution
fn sum_refs(numbers: &[&i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += **n;
    }
    total
}

fn main() {
    let a = 5;
    let b = 7;
    let c = 30;
    println!("{}", sum_refs(&[&a, &b, &c]));
}
