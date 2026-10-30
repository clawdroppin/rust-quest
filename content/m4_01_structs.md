@@lesson m4-structs Structs: Your Own Data Types
@@text
So far you've used Rust's built-in types. Now you'll create **your own**. This is where programming starts to feel like *modeling the real world*: a game has players, a shop has products, a calendar has events. Each of those is a type you design.

## Defining a struct

A **struct** (short for *structure*) groups related values under **one name**, giving each value its own **field name**:

```rust
struct Player {
    name: String,
    health: u32,
    level: u32,
    is_online: bool,
}

fn main() {
    let hero = Player {
        name: String::from("Ferris"),
        health: 100,
        level: 3,
        is_online: true,
    };
    println!("{} is level {} with {} HP", hero.name, hero.level, hero.health);
}
```

- `struct Player { ... }` **defines** a new type. It's a blueprint — it doesn't create any player yet.
- Each field has a **name and a type**.
- `Player { name: ..., health: ..., ... }` **creates** a value (an **instance**) of that type. You must give every field a value.
- Access fields with a dot: `hero.name`.
- Struct names use `UpperCamelCase`; field names use `snake_case`.

Compare with a tuple `("Ferris", 100, 3, true)`: what's `.2` again? Named fields make code **self-explanatory**.

## Changing fields

The whole instance must be `mut` to change any field:

```rust
struct Counter {
    count: u32,
}

fn main() {
    let mut c = Counter { count: 0 };
    c.count += 1;
    c.count += 1;
    println!("{}", c.count);
}
```

## Printing structs: `#[derive(Debug)]`

`println!("{:?}", hero)` doesn't work on your own types — Rust doesn't know how to display them. But you can ask it to **automatically generate** debug printing with `#[derive(Debug)]` above the struct:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.5, y: -2.0 };
    println!("{:?}", p);
    println!("{:#?}", p);   // "pretty" debug: one field per line
}
```

You'll put `#[derive(Debug)]` on almost every struct you write. (`derive` is explained properly in Module 7.)

## Shortcuts

**Field init shorthand** — when a variable has the same name as the field:

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn make_user(name: String, age: u32) -> User {
    User { name, age }    // same as User { name: name, age: age }
}

fn main() {
    println!("{:?}", make_user(String::from("Ana"), 31));
}
```

**Struct update syntax** — copy the remaining fields from another instance:

```rust
#[derive(Debug)]
struct Settings {
    volume: u32,
    fullscreen: bool,
    language: String,
}

fn main() {
    let defaults = Settings { volume: 50, fullscreen: false, language: String::from("en") };
    let mine = Settings { volume: 80, ..defaults };
    println!("{:?}", mine);
}
```

(Careful: `..defaults` *moves* the `String` out of `defaults`, so `defaults` can't be fully used afterwards.)

## Structs and ownership

Fields usually **own** their data: use `String`, not `&str`, and `Vec<T>`, not `&[T]`. A struct that owns everything can be created, stored and passed around freely. (Structs *can* hold references, but they then need lifetime annotations — save that for later.)

Passing structs to functions follows the normal rules: by value moves, `&` borrows, `&mut` borrows to change:

```rust
#[derive(Debug)]
struct Account {
    owner: String,
    balance: i64,
}

fn print_account(a: &Account) {
    println!("{}: {}", a.owner, a.balance);
}

fn deposit(a: &mut Account, amount: i64) {
    a.balance += amount;
}

fn main() {
    let mut acc = Account { owner: String::from("Ana"), balance: 100 };
    deposit(&mut acc, 50);
    print_account(&acc);
}
```

## Tuple structs and unit structs

A **tuple struct** has a name but unnamed fields. It's useful for giving a meaningful name to a simple value, so you can't mix it up with others:

```rust
struct Meters(f64);
struct Color(u8, u8, u8);

fn main() {
    let distance = Meters(12.5);
    let red = Color(255, 0, 0);
    println!("{} m, red channel {}", distance.0, red.0);
}
```

A function taking `Meters` can't accidentally receive a plain number of seconds. (There are also **unit structs** with no fields at all, `struct Marker;` — you'll see them with traits.)

## Structs inside structs

Fields can be any type, including other structs, vectors and maps:

```rust
#[derive(Debug)]
struct Item {
    name: String,
    price: u32,
}

#[derive(Debug)]
struct Order {
    id: u32,
    items: Vec<Item>,
}

fn main() {
    let order = Order {
        id: 1,
        items: vec![
            Item { name: String::from("pen"), price: 2 },
            Item { name: String::from("book"), price: 15 },
        ],
    };
    let mut total = 0;
    for item in &order.items {
        total += item.price;
    }
    println!("Order {} costs {total}", order.id);
}
```

@@quiz
What does `#[derive(Debug)]` do?
- It makes the struct faster
* It automatically generates code so the struct can be printed with `{:?}`
- It finds bugs in the struct
? `derive` asks the compiler to write standard functionality for you. `Debug` gives you `{:?}` printing.

@@quiz
Which type should a struct field use to store a person's name?
* `String`
- `&str`
- `char`
? Fields should usually own their data. `&str` would need lifetime annotations.

@@exercise Define a Book
Define a struct `Book` with fields `title` (`String`), `author` (`String`), `pages` (`u32`) and `read` (`bool`), and derive `Debug`.

Then write `book_summary`, which borrows a book and returns a summary like `"Dune by Frank Herbert (412 pages, read)"` or `"... (412 pages, unread)"`.
@@starter
// define the Book struct here

fn book_summary(book: &Book) -> String {
    todo!()
}

fn main() {
    let dune = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        pages: 412,
        read: true,
    };
    println!("{}", book_summary(&dune));
}
@@tests
#[test]
fn checks() {
    let b = Book { title: String::from("Dune"), author: String::from("Frank Herbert"), pages: 412, read: true };
    assert_eq!(book_summary(&b), "Dune by Frank Herbert (412 pages, read)");
    let b2 = Book { title: String::from("Emma"), author: String::from("Jane Austen"), pages: 474, read: false };
    assert_eq!(book_summary(&b2), "Emma by Jane Austen (474 pages, unread)");
    println!("{:?}", b2);
}
@@hint
`let status = if book.read { "read" } else { "unread" };` then `format!("{} by {} ({} pages, {})", book.title, book.author, book.pages, status)`.
@@solution
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    read: bool,
}

fn book_summary(book: &Book) -> String {
    let status = if book.read { "read" } else { "unread" };
    format!("{} by {} ({} pages, {})", book.title, book.author, book.pages, status)
}

fn main() {
    let dune = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        pages: 412,
        read: true,
    };
    println!("{}", book_summary(&dune));
}

@@exercise Rectangles
A `Rect` struct is given. Write three functions:

- `area(r: &Rect) -> u32`
- `can_hold(outer: &Rect, inner: &Rect) -> bool` — true if `inner` fits inside `outer` without rotating (both width and height strictly smaller)
- `scale(r: &mut Rect, factor: u32)` — multiplies width and height by `factor`
@@starter
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(r: &Rect) -> u32 {
    todo!()
}

fn can_hold(outer: &Rect, inner: &Rect) -> bool {
    todo!()
}

fn scale(r: &mut Rect, factor: u32) {
    todo!()
}

fn main() {
    let mut r = Rect { width: 3, height: 4 };
    scale(&mut r, 2);
    println!("{:?} has area {}", r, area(&r));
}
@@tests
#[test]
fn checks() {
    let big = Rect { width: 10, height: 8 };
    let small = Rect { width: 3, height: 4 };
    assert_eq!(area(&big), 80);
    assert!(can_hold(&big, &small));
    assert!(!can_hold(&small, &big));
    assert!(!can_hold(&big, &Rect { width: 10, height: 1 }), "equal width doesn't fit (strictly smaller)");
    let mut r = Rect { width: 3, height: 4 };
    scale(&mut r, 3);
    assert_eq!((r.width, r.height), (9, 12));
}
@@hint
`can_hold`: `inner.width < outer.width && inner.height < outer.height`. `scale`: `r.width *= factor;` and the same for height.
@@solution
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}

fn can_hold(outer: &Rect, inner: &Rect) -> bool {
    inner.width < outer.width && inner.height < outer.height
}

fn scale(r: &mut Rect, factor: u32) {
    r.width *= factor;
    r.height *= factor;
}

fn main() {
    let mut r = Rect { width: 3, height: 4 };
    scale(&mut r, 2);
    println!("{:?} has area {}", r, area(&r));
}

@@exercise Library catalogue
Using the `Book` struct (given), write `unread_titles`, which returns the titles of all **unread** books (in order, as new `String`s), and `total_pages`, which adds up the pages of all books.
@@starter
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
    read: bool,
}

fn unread_titles(books: &[Book]) -> Vec<String> {
    todo!()
}

fn total_pages(books: &[Book]) -> u32 {
    todo!()
}

fn main() {
    let shelf = vec![
        Book { title: String::from("Dune"), pages: 412, read: true },
        Book { title: String::from("Emma"), pages: 474, read: false },
        Book { title: String::from("Ulysses"), pages: 730, read: false },
    ];
    println!("To read: {:?}", unread_titles(&shelf));
    println!("Total pages: {}", total_pages(&shelf));
}
@@tests
#[test]
fn checks() {
    let shelf = vec![
        Book { title: String::from("Dune"), pages: 412, read: true },
        Book { title: String::from("Emma"), pages: 474, read: false },
        Book { title: String::from("Ulysses"), pages: 730, read: false },
    ];
    assert_eq!(unread_titles(&shelf), vec!["Emma", "Ulysses"]);
    assert_eq!(total_pages(&shelf), 1616);
    assert_eq!(total_pages(&[]), 0);
}
@@hint
Loop with `for book in books` (each `book` is a `&Book`). For titles, push `book.title.clone()` — you can't move the title out of a borrowed book.
@@solution
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
    read: bool,
}

fn unread_titles(books: &[Book]) -> Vec<String> {
    let mut titles = Vec::new();
    for book in books {
        if !book.read {
            titles.push(book.title.clone());
        }
    }
    titles
}

fn total_pages(books: &[Book]) -> u32 {
    let mut total = 0;
    for book in books {
        total += book.pages;
    }
    total
}

fn main() {
    let shelf = vec![
        Book { title: String::from("Dune"), pages: 412, read: true },
        Book { title: String::from("Emma"), pages: 474, read: false },
        Book { title: String::from("Ulysses"), pages: 730, read: false },
    ];
    println!("To read: {:?}", unread_titles(&shelf));
    println!("Total pages: {}", total_pages(&shelf));
}
