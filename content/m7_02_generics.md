@@lesson m7-generics Generics
@@text
Imagine writing a function that returns the largest number in a list. You write it for `i32`. Then you need it for `f64`. Then for `char`. Three copies of the same code, differing only in the type. **Generics** solve this: you write the code once, with a **type parameter** standing in for the real type.

You've used generic types since Module 2: `Vec<T>`, `Option<T>`, `HashMap<K, V>`, `Result<T, E>`. The `T` in `Vec<T>` means "a vector of *some type T*". `Vec<i32>` and `Vec<String>` are the same code, filled in with different types.

## Generic functions

```rust
fn first<T>(items: &[T]) -> &T {
    &items[0]
}

fn main() {
    let numbers = vec![10, 20, 30];
    let words = vec!["apple", "banana"];
    println!("{}", first(&numbers));
    println!("{}", first(&words));
}
```

- `<T>` after the function name declares a type parameter called `T` (any name works, but single capital letters are the convention).
- Inside, `T` is used like any other type.
- When you call `first(&numbers)`, Rust sees that `T` is `i32` and fills it in. The compiler generates a separate, fully optimized copy for each type you use. Generics cost nothing at runtime.

## Generic structs

```rust
#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Pair { left, right }
    }

    fn swap(self) -> Pair<T> {
        Pair { left: self.right, right: self.left }
    }
}

fn main() {
    let p = Pair::new(1, 2);
    let q = Pair::new("hello", "world");
    println!("{:?}", p.swap());
    println!("{:?}", q);
}
```

Note `impl<T> Pair<T>`: you declare `T` after `impl` so you can use it in `Pair<T>`.

Multiple type parameters are fine too:

```rust,norun
struct Labeled<L, V> {
    label: L,
    value: V,
}

let x = Labeled { label: "age", value: 42 };
let y = Labeled { label: 7, value: vec![1.5, 2.5] };
```

## Generic enums: Option and Result revealed

Here's how the standard library defines its two most famous enums. No magic, just generics:

```rust,norun
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Option<String>` is `Option` with `T = String`. `Result<f64, CalcError>` fills in both parameters.

## The catch: what can you do with a `T`?

This function **doesn't compile**:

```rust,norun
fn largest<T>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item > best {        // ❌ error: binary operation `>` cannot be applied to type `&T`
            best = item;
        }
    }
    best
}
```

Rust refuses because `T` could be *any* type, including types that can't be compared (what does it mean for one `File` to be "greater" than another?). We need to tell Rust that `T` must be comparable. That's what **trait bounds** are for, in the next lesson.

@@quiz
What does the `<T>` in `fn first<T>(items: &[T]) -> &T` declare?
- A variable named T
* A type parameter: a placeholder for a type, filled in when the function is called
- That the function returns a tuple
? Type parameters let one function work with many types.

@@quiz
Do generics make your program slower?
- Yes, every call checks the type at runtime
* No, the compiler generates a specialized copy for each concrete type used
- Only for structs
? This is called monomorphization: generic code is as fast as hand-written code for each type.

@@exercise A generic Stack
Write a generic struct `Stack<T>` that holds its items in a `Vec<T>` (field `items`), with these methods:

- `new() -> Self` — an empty stack
- `push(&mut self, item: T)` — adds an item to the top
- `pop(&mut self) -> Option<T>` — removes and returns the top item, or `None` if empty
- `peek(&self) -> Option<&T>` — the top item without removing it
- `len(&self) -> usize` and `is_empty(&self) -> bool`

A stack is "last in, first out", like a stack of plates. (Hint: `Vec` already has `push`, `pop` and `last`!)
@@starter
// write Stack<T> here

fn main() {
    let mut numbers = Stack::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{:?}", numbers.pop());
    println!("{:?}", numbers.peek());
    println!("{}", numbers.len());

    let mut words: Stack<String> = Stack::new();
    println!("{}", words.is_empty());
    words.push(String::from("hello"));
    println!("{:?}", words.pop());
    println!("{:?}", words.pop());
}
@@expected
Some(3)
Some(2)
2
true
Some("hello")
None
@@tests
#[test]
fn checks() {
    let mut s: Stack<char> = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.peek(), None);
    s.push('a');
    s.push('b');
    assert_eq!(s.peek(), Some(&'b'));
    assert_eq!(s.len(), 2);
    assert_eq!(s.pop(), Some('b'));
    assert_eq!(s.pop(), Some('a'));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}
@@requires
struct Stack<T>
impl<T> Stack<T>
@@hint
```
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    // ...
}
```
@@solution
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    let mut numbers = Stack::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{:?}", numbers.pop());
    println!("{:?}", numbers.peek());
    println!("{}", numbers.len());

    let mut words: Stack<String> = Stack::new();
    println!("{}", words.is_empty());
    words.push(String::from("hello"));
    println!("{:?}", words.pop());
    println!("{:?}", words.pop());
}

@@exercise Last element
Write a generic function `last_of<T>(items: &[T]) -> Option<&T>` that returns a reference to the last element of a slice, or `None` if it's empty. Don't use the built-in `.last()`: use `len()` and indexing.
@@starter
// write last_of here

fn main() {
    println!("{:?}", last_of(&[1, 2, 3]));
    println!("{:?}", last_of(&["x", "y"]));
    let empty: [f64; 0] = [];
    println!("{:?}", last_of(&empty));
}
@@expected
Some(3)
Some("y")
None
@@tests
#[test]
fn checks() {
    assert_eq!(last_of(&[true]), Some(&true));
    let v = vec![String::from("a"), String::from("b")];
    assert_eq!(last_of(&v), Some(&String::from("b")));
    let e: Vec<u8> = vec![];
    assert_eq!(last_of(&e), None);
}
@@requires
fn last_of<T>
@@hint
If `items.is_empty()`, return `None`. Otherwise return `Some(&items[items.len() - 1])`.
@@solution
fn last_of<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        None
    } else {
        Some(&items[items.len() - 1])
    }
}

fn main() {
    println!("{:?}", last_of(&[1, 2, 3]));
    println!("{:?}", last_of(&["x", "y"]));
    let empty: [f64; 0] = [];
    println!("{:?}", last_of(&empty));
}
