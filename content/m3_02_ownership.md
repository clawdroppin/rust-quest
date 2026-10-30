@@lesson m3-ownership Ownership and Moves
@@text
## The three rules of ownership

1. Each value in Rust has an **owner** (a variable).
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value is **dropped** (cleaned up).

Rules 1 and 3 you saw in the last lesson. Rule 2 has a surprising consequence.

## Moves

Try this:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
}
```

Error: **borrow of moved value: `s1`**. Why?

`let s2 = s1;` copies the *handle* (pointer, length, capacity) from `s1` to `s2`. It does **not** copy the heap text. So now two handles point at the same heap memory:

```text
  s1 ─┐
      ├──►  "hello" (heap)
  s2 ─┘
```

If both were allowed to stay owners, then at the end of `main`, both would try to free the same memory — a "double free", a classic crash bug in C. Rust's solution: after `let s2 = s1;`, **`s1` is no longer valid**. Ownership has **moved** to `s2`. `s1` can't be used any more, and only `s2` will free the memory.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;          // ownership moves from s1 to s2
    println!("{s2}");     // ✔ fine
}
```

## Moving into functions

Passing a value to a function **also moves it**. The function's parameter becomes the new owner:

```rust
fn shout(text: String) {
    println!("{}!", text.to_uppercase());
} // ← text goes out of scope here, and the String is dropped

fn main() {
    let message = String::from("hello");
    shout(message);           // message moves into shout
    // shout(message);        // ❌ error: value used after move
}
```

This is exactly the error you saw in Module 2 when looping with `for x in list` and then trying to use `list` again: the loop took ownership of the list.

## Getting ownership back

Returning a value moves it **out** of the function to the caller:

```rust
fn make_greeting(name: &str) -> String {
    let greeting = format!("Hello, {name}!");
    greeting // ownership moves out to the caller
}

fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text // give it back
}

fn main() {
    let g = make_greeting("Ferris");
    let g = add_exclamation(g);   // move in, get it back
    println!("{g}");
}
```

Passing ownership in and handing it back out works, but it's clumsy. Imagine having to return every parameter just so the caller can keep using it! That's what **borrowing** (references, the `&`) fixes — two lessons from now.

## What about numbers?

You might have noticed this works fine:

```rust
fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}");   // no error!
}
```

Simple values like integers live entirely on the stack, so copying them is trivial and safe. Those types are **`Copy`**, and they're copied instead of moved. That's the next lesson.

@@quiz
After `let b = a;` where `a` is a `String`, which variable can you use?
- Both `a` and `b`
* Only `b`
- Only `a`
? Ownership moved to `b`. Using `a` afterwards is a compile error.

@@quiz
```text
fn take(v: Vec<i32>) { }

let v = vec![1, 2, 3];
take(v);
println!("{:?}", v);
```
Why doesn't this compile?
- `take` doesn't return anything
* `v` was moved into `take`, so it can't be used afterwards
- Vectors can't be printed
? Passing `v` to `take` moves ownership into the function, and the vector is dropped when `take` ends.

@@exercise Who owns the name?
This program doesn't compile: `greet` takes ownership of `name`, so `main` can't use it afterwards.

Fix it **the ownership way**: change `greet` so it hands the `String` back to the caller (return it), and in `main` take it back with `let name = greet(name);`. Output:

```text
Hello, Ferris
Ferris has 6 letters
```
@@starter
fn greet(name: String) {
    println!("Hello, {name}");
}

fn main() {
    let name = String::from("Ferris");
    let length = name.len();
    greet(name);
    println!("{name} has {length} letters");
}
@@expected
Hello, Ferris
Ferris has 6 letters
@@requires
let name = greet(name)
@@hint
Give `greet` a return type `-> String` and make `name` its last expression (no semicolon).
@@solution
fn greet(name: String) -> String {
    println!("Hello, {name}");
    name
}

fn main() {
    let name = String::from("Ferris");
    let length = name.len();
    let name = greet(name);
    println!("{name} has {length} letters");
}

@@exercise Give it back
Write `add_suffix`, which takes ownership of a `String`, adds `".rs"` to the end, and **returns it**. Then in `main`, use it so the output is:

```text
main.rs
```
@@starter
fn add_suffix(name: String) -> String {
    todo!()
}

fn main() {
    let file = String::from("main");
    println!("{file}");
}
@@expected
main.rs
@@tests
#[test]
fn checks() {
    assert_eq!(add_suffix(String::from("lib")), "lib.rs");
    assert_eq!(add_suffix(String::new()), ".rs");
}
@@hint
Make the parameter mutable (`mut name: String`), call `name.push_str(".rs");`, and return `name`. In `main`: `let file = add_suffix(file);`
@@solution
fn add_suffix(mut name: String) -> String {
    name.push_str(".rs");
    name
}

fn main() {
    let file = String::from("main");
    let file = add_suffix(file);
    println!("{file}");
}
