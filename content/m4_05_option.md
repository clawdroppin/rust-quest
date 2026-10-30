@@lesson m4-option Option: Values That Might Be Missing
@@text
You've met `Option` several times already. Now that you know enums, here's the secret: **`Option` is just an enum** from the standard library:

```rust,norun
enum Option<T> {
    Some(T),
    None,
}
```

The `<T>` means it works with any type (that's a *generic*, covered in Module 7). An `Option<i32>` is either `Some(some_number)` or `None`.

## Why not just use `null`?

Many languages have a special value `null` that any variable can secretly hold, meaning "nothing here". The inventor of `null`, Tony Hoare, called it his **"billion-dollar mistake"**: forgetting to check for `null` crashes programs everywhere, every day.

Rust has no `null`. If a value might be missing, its type says so: `Option<T>`. And you **can't** use an `Option<i32>` as if it were an `i32` — the compiler forces you to handle the `None` case first:

```rust
fn main() {
    let maybe: Option<i32> = Some(5);
    let total = maybe + 1;   // ❌ error: can't add Option<i32> and i32
    println!("{total}");
}
```

## Creating Options

Return `Option` from functions that might not have an answer:

```rust
fn find_index(list: &[&str], target: &str) -> Option<usize> {
    for (i, item) in list.iter().enumerate() {
        if *item == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let fruits = ["apple", "pear", "fig"];
    println!("{:?}", find_index(&fruits, "pear"));
    println!("{:?}", find_index(&fruits, "kiwi"));
}
```

## Handling Options

**1. `match`** — handle both cases explicitly:

```rust
fn main() {
    let names = vec!["Ana", "Ben"];
    match names.get(5) {
        Some(name) => println!("Found {name}"),
        None => println!("No one at that position"),
    }
}
```

**2. `if let`** — when you only care about the `Some` case (next lesson has more).

**3. Methods that provide a fallback:**

```rust
fn main() {
    let empty: Vec<i32> = vec![];
    let scores = vec![4, 9, 2];

    println!("{}", empty.iter().max().copied().unwrap_or(0));   // a default
    println!("{}", scores.iter().max().copied().unwrap_or(0));

    let nickname: Option<&str> = None;
    println!("Hello, {}", nickname.unwrap_or("stranger"));

    println!("{}", scores.first().is_some());
    println!("{}", empty.first().is_none());
}
```

(`.copied()` turns an `Option<&i32>` into an `Option<i32>` by copying the number out.)

**4. `unwrap` and `expect`** — take the value out, **panic if it's `None`**:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let first = v.first().expect("the list should never be empty");
    println!("{first}");
}
```

Use these only when `None` would mean a bug in your program, not when it's a normal possibility. `expect` is better than `unwrap` because its message explains *why* you believed there must be a value.

**5. `map`** — transform the value inside, if there is one:

```rust
fn main() {
    let word: Option<&str> = Some("rust");
    let length = word.map(|w| w.len());          // Some(4)
    let shout = word.map(|w| w.to_uppercase());  // Some("RUST")
    println!("{:?} {:?}", length, shout);

    let nothing: Option<&str> = None;
    println!("{:?}", nothing.map(|w| w.len()));  // None
}
```

## The `?` operator with Option

Inside a function that returns `Option`, `?` means: *"if this is `None`, return `None` from the whole function right now; otherwise give me the value."*

```rust
fn first_char_of_second_word(text: &str) -> Option<char> {
    let second = text.split_whitespace().nth(1)?;   // None if there's no second word
    let c = second.chars().next()?;                  // None if it's empty
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", first_char_of_second_word("hello rusty world"));
    println!("{:?}", first_char_of_second_word("hello"));
}
```

Without `?`, you'd need two nested `match`es. You'll use `?` a lot more with `Result` in the next module.

## Option in structs

`Option` is perfect for **optional fields**:

```rust
#[derive(Debug)]
struct Contact {
    name: String,
    phone: Option<String>,
    email: Option<String>,
}

fn main() {
    let c = Contact { name: String::from("Ana"), phone: None, email: Some(String::from("ana@example.com")) };
    match &c.phone {
        Some(p) => println!("Call {} at {p}", c.name),
        None => println!("{} has no phone number", c.name),
    }
}
```

@@quiz
What's the type of `Some(3.5)`?
- `f64`
* `Option<f64>`
- `Some<f64>`
? `Some` is a variant of the `Option` enum. The value inside is an `f64`.

@@quiz
When is `.unwrap()` a reasonable choice?
- Always: it's the shortest
- Never
* When `None` would only happen because of a bug, so crashing is acceptable
? For "normal" missing values, use `match`, `if let`, `unwrap_or` or `?` instead.

@@exercise Safe division
Write `safe_divide`, which returns `None` when dividing by zero, and `Some(result)` otherwise.

Then write `average_of`, which returns the average of a list, or `None` for an empty list. Use `safe_divide` inside it.
@@starter
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

fn average_of(numbers: &[f64]) -> Option<f64> {
    todo!()
}

fn main() {
    println!("{:?}", safe_divide(10.0, 4.0));
    println!("{:?}", safe_divide(1.0, 0.0));
    println!("{:?}", average_of(&[2.0, 4.0]));
    println!("{:?}", average_of(&[]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(safe_divide(10.0, 4.0), Some(2.5));
    assert_eq!(safe_divide(1.0, 0.0), None);
    assert_eq!(average_of(&[2.0, 4.0]), Some(3.0));
    assert_eq!(average_of(&[]), None);
}
@@hint
`average_of`: sum the numbers, then return `safe_divide(sum, numbers.len() as f64)` — an empty list has length 0, so dividing by it gives `None` automatically!
@@solution
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn average_of(numbers: &[f64]) -> Option<f64> {
    let mut sum = 0.0;
    for n in numbers {
        sum += n;
    }
    safe_divide(sum, numbers.len() as f64)
}

fn main() {
    println!("{:?}", safe_divide(10.0, 4.0));
    println!("{:?}", safe_divide(1.0, 0.0));
    println!("{:?}", average_of(&[2.0, 4.0]));
    println!("{:?}", average_of(&[]));
}

@@exercise Find a user
Write `find_user`, which searches the list for a user with the given id and returns a **reference** to it (`Option<&User>`). Then write `email_of`, which returns the user's email, or `None` if the user doesn't exist **or** has no email. Use `?` in `email_of`.
@@starter
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: Option<String>,
}

fn find_user(users: &[User], id: u32) -> Option<&User> {
    todo!()
}

fn email_of(users: &[User], id: u32) -> Option<String> {
    todo!()
}

fn main() {
    let users = vec![
        User { id: 1, name: String::from("Ana"), email: Some(String::from("ana@mail.com")) },
        User { id: 2, name: String::from("Ben"), email: None },
    ];
    println!("{:?}", find_user(&users, 2).map(|u| &u.name));
    println!("{:?}", email_of(&users, 1));
    println!("{:?}", email_of(&users, 2));
}
@@tests
#[test]
fn checks() {
    let users = vec![
        User { id: 1, name: String::from("Ana"), email: Some(String::from("ana@mail.com")) },
        User { id: 2, name: String::from("Ben"), email: None },
    ];
    assert_eq!(find_user(&users, 2).map(|u| u.name.as_str()), Some("Ben"));
    assert!(find_user(&users, 3).is_none());
    assert_eq!(email_of(&users, 1), Some(String::from("ana@mail.com")));
    assert_eq!(email_of(&users, 2), None, "Ben has no email");
    assert_eq!(email_of(&users, 9), None, "user 9 doesn't exist");
}
@@requires
)?
@@hint
`find_user`: loop, `if user.id == id { return Some(user); }`, then `None`. `email_of`: `let user = find_user(users, id)?;` then `user.email.clone()` (it's already an `Option<String>`).
@@solution
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: Option<String>,
}

fn find_user(users: &[User], id: u32) -> Option<&User> {
    for user in users {
        if user.id == id {
            return Some(user);
        }
    }
    None
}

fn email_of(users: &[User], id: u32) -> Option<String> {
    let user = find_user(users, id)?;
    user.email.clone()
}

fn main() {
    let users = vec![
        User { id: 1, name: String::from("Ana"), email: Some(String::from("ana@mail.com")) },
        User { id: 2, name: String::from("Ben"), email: None },
    ];
    println!("{:?}", find_user(&users, 2).map(|u| &u.name));
    println!("{:?}", email_of(&users, 1));
    println!("{:?}", email_of(&users, 2));
}
