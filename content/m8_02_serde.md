@@lesson m8-json Saving Data as JSON with serde
@@text
Writing your own file format works for simple data, but gets painful fast: nested lists, optional fields, text that contains your separator character… The standard solution is to use a well-known format and a library that converts your types to and from it automatically.

## JSON

**JSON** (JavaScript Object Notation) is the most common data format in the world. Web APIs, config files and save files use it everywhere. It's plain text that people can read:

```json
{
  "name": "Ferris",
  "age": 7,
  "languages": ["Rust", "English"],
  "address": { "city": "Crabtown", "zip": null },
  "premium": true
}
```

JSON has objects `{ "key": value }`, arrays `[ ... ]`, strings, numbers, `true`/`false` and `null`. That maps neatly onto Rust: structs ↔ objects, `Vec` ↔ arrays, `Option` ↔ value or `null`.

## serde: serialize + deserialize

**serde** is Rust's serialization framework, one of the most downloaded crates of all. It's split in two:

- `serde` itself, which provides the `Serialize` and `Deserialize` traits and their **derive** macros;
- a **format** crate, like `serde_json`, `toml` or `csv`, which does the actual reading and writing.

Add both to a Cargo project:

```text
cargo add serde --features derive
cargo add serde_json
```

`--features derive` switches on an optional part of the crate: the `#[derive(Serialize, Deserialize)]` macros. In `Cargo.toml` it looks like this:

```text
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Deriving and converting

```rust,norun
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    favorite: bool,
}

let c = Contact { name: "Ada".into(), phone: "555-0100".into(), favorite: true };

// Rust value → JSON text
let json: String = serde_json::to_string_pretty(&c).unwrap();

// JSON text → Rust value (the type annotation tells serde what to build)
let back: Contact = serde_json::from_str(&json).unwrap();
```

That's it. serde writes all the conversion code at compile time. It works with nested structs, `Vec`s, `HashMap`s, `Option`s and enums, as long as every type inside also derives `Serialize`/`Deserialize`.

`from_str` returns a `Result`, because the text may not be valid JSON, or may not match your type (a missing field, a string where a number should be…). The error message says exactly where: `missing field 'phone' at line 4 column 1`.

## The save/load pattern

Combine `serde_json` with what you learned about files:

```rust,norun
fn save(path: &str, book: &AddressBook) -> Result<(), String> {
    let json = serde_json::to_string_pretty(book).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("could not write {path}: {e}"))
}

fn load(path: &str) -> Result<AddressBook, String> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| format!("{path} is damaged: {e}")),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(AddressBook::default()),
        Err(e) => Err(format!("could not read {path}: {e}")),
    }
}
```

You'll use this exact pattern in the to-do app and in the GUI capstone.

## Useful attributes

serde's behaviour can be tuned with attributes:

```rust,norun
#[derive(Serialize, Deserialize)]
struct Settings {
    volume: u8,

    #[serde(default)]                 // missing in the file? use Default (0, "", false…)
    dark_mode: bool,

    #[serde(rename = "playerName")]   // the JSON key has a different name
    player_name: String,

    #[serde(skip)]                    // never saved; Default when loading
    unsaved_changes: bool,
}
```

`#[serde(default)]` is very important for **evolving** a program: when version 2 of your app adds a new field, old save files that don't have it still load.

Enums are saved by variant name: `Priority::High` becomes `"High"`. Enum variants with data become objects: `Shape::Circle { r: 2.0 }` becomes `{"Circle":{"r":2.0}}`.

## Try it

The project below is a tiny address book that saves to `contacts.json`. Run it a few times. Each run adds a contact and the file grows. Open `contacts.json` in VS Code to see what serde wrote, then try the experiments listed in `main.rs`.

@@project json_basics JSON Basics
A tiny address book that loads and saves itself as JSON with serde.
@@file Cargo.toml
[package]
name = "json_basics"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
@@file src/main.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

const FILE: &str = "contacts.json";

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Group {
    Family,
    Friends,
    Work,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    group: Group,
    email: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct AddressBook {
    contacts: Vec<Contact>,
    #[serde(default)]
    times_opened: u32,
}

fn load(path: &str) -> Result<AddressBook, String> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| format!("{path} is damaged: {e}")),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(AddressBook::default()),
        Err(e) => Err(format!("could not read {path}: {e}")),
    }
}

fn save(path: &str, book: &AddressBook) -> Result<(), String> {
    let json = serde_json::to_string_pretty(book).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("could not write {path}: {e}"))
}

fn main() {
    let mut book = match load(FILE) {
        Ok(book) => book,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        }
    };

    book.times_opened += 1;
    let n = book.contacts.len() + 1;
    book.contacts.push(Contact {
        name: format!("Contact {n}"),
        phone: format!("555-01{n:02}"),
        group: if n % 2 == 0 { Group::Friends } else { Group::Work },
        email: if n % 3 == 0 { Some(format!("contact{n}@example.com")) } else { None },
    });

    println!("Opened {} time(s). Contacts:", book.times_opened);
    for c in &book.contacts {
        let email = c.email.as_deref().unwrap_or("-");
        println!("  {:<12} {:<10} {:<8} {}", c.name, c.phone, format!("{:?}", c.group), email);
    }

    if let Err(e) = save(FILE, &book) {
        eprintln!("Error: {e}");
    }

    // Experiments:
    // 1. Run this 3-4 times, then open contacts.json in VS Code.
    // 2. Edit a name in contacts.json by hand, save, and run again.
    // 3. Break the JSON (delete a quote or a comma) and run. Read the error message!
    // 4. Add a new field `birthday: Option<String>` to Contact. Run. Why does it still load?
    //    Then add `notes: String` instead. What happens, and how does #[serde(default)] fix it?
}

@@text
## About experiment 4

`Option` fields are allowed to be missing from JSON: serde fills in `None`. But a plain `String` field that's missing is an error (`missing field 'notes'`), because serde can't invent a value. Adding `#[serde(default)]` above the field tells serde to use `String::default()` (an empty string) instead. That's how you keep old save files working as your program grows.

@@quiz
What does `#[derive(Serialize, Deserialize)]` give you?
- A JSON file
* Code generated at compile time that converts your type to and from formats like JSON
- A database
? serde derives the conversion code. A format crate like `serde_json` uses it to read and write text.

@@quiz
Your app adds a new field `theme: String` to its settings. Users' old settings files don't have it. How do you keep them loading?
- Delete the old files
* Put `#[serde(default)]` on the new field (or make it an `Option`)
- Use `#[serde(skip)]`
? `default` fills in missing fields. `skip` would mean the field is never saved at all.

@@quiz
Why does `serde_json::from_str` return a `Result`?
- Because JSON is slow
* Because the text might not be valid JSON, or might not match your type
- It doesn't; it always succeeds
? Data from files can be damaged or edited by hand, so loading must handle errors.
