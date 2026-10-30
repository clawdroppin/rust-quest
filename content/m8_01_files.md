@@lesson m8-files Reading and Writing Files
@@text
So far, everything your programs knew disappeared when they ended. Real programs **remember**: settings, documents, save games, to-do lists. The simplest way to remember is a **file**.

## Writing and reading a whole file

```rust
use std::fs;

fn main() {
    fs::write("hello.txt", "Hello from Rust!\nSecond line\n").expect("could not write");

    let text = fs::read_to_string("hello.txt").expect("could not read");
    println!("The file contains {} bytes:", text.len());
    print!("{text}");

    for (i, line) in text.lines().enumerate() {
        println!("line {}: {}", i + 1, line);
    }
}
```

- `fs::write(path, contents)` creates the file (or **replaces** it if it exists) and writes the contents.
- `fs::read_to_string(path)` reads the whole file into a `String`.
- Paths are relative to the program's **current directory**. With `cargo run`, that's the project folder. (In this app, it's a temporary folder for your program.)

Both return a `Result`, because file operations can fail for many reasons: the file doesn't exist, you don't have permission, the disk is full… In real programs, handle these errors instead of using `expect`.

## Handling a missing file

The most common case: the first time your program runs, its save file doesn't exist yet. That's not an error, it just means "start empty":

```rust
use std::fs;
use std::io::ErrorKind;

fn load_text(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(String::new()),   // first run
        Err(e) => Err(format!("could not read {path}: {e}")),
    }
}

fn main() {
    println!("{:?}", load_text("does_not_exist.txt"));
}
```

The `if` after a match pattern is a **match guard**: the arm only matches when the condition is true too. `e.kind()` tells you what kind of I/O error happened.

## Adding to a file

`fs::write` replaces the whole file. To **append** (like a log), open the file with `OpenOptions` and use `writeln!`, which works like `println!` but writes into the file:

```rust
use std::fs::{self, OpenOptions};
use std::io::Write;

fn log(message: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)    // create the file if it doesn't exist
        .append(true)    // write at the end instead of replacing
        .open("app.log")?;
    writeln!(file, "{message}")?;
    Ok(())
}

fn main() {
    let _ = fs::remove_file("app.log");   // start fresh (ignore the error if it's missing)
    log("program started").unwrap();
    log("did some work").unwrap();
    print!("{}", fs::read_to_string("app.log").unwrap());
}
```

`std::io::Result<()>` is shorthand for `Result<(), std::io::Error>`, and `?` passes I/O errors up to the caller.

## Paths and folders

```rust,norun
use std::fs;
use std::path::{Path, PathBuf};

let folder = Path::new("saves");
fs::create_dir_all(folder)?;                 // create the folder (and parents) if needed
let file: PathBuf = folder.join("slot1.txt"); // saves/slot1.txt (or saves\slot1.txt on Windows)

if file.exists() { ... }

for entry in fs::read_dir(folder)? {         // list the folder's contents
    let entry = entry?;
    println!("{}", entry.path().display());
}
```

- `Path` is to `PathBuf` what `&str` is to `String`: a borrowed view vs. an owned value.
- `join` builds paths with the right separator for the operating system. Don't glue paths together with `format!`.
- `.display()` makes a path printable.

## Your own file formats

A file is just text (or bytes). **You** decide the format. A simple, common choice is one record per line, with fields separated by a character that can't appear in the data:

```text
Ada;95
Grace;88
Linus;72
```

Saving means turning your data into text (**serializing**); loading means parsing the text back (**deserializing**), which can fail if the file was edited by hand or damaged. The exercises below do exactly that. In the next lesson, you'll let a crate handle it with a standard format: JSON.

@@quiz
What does `fs::write("data.txt", text)` do if `data.txt` already exists?
- Adds `text` to the end
* Replaces the whole file with `text`
- Returns an error
? To add to the end, open the file with `OpenOptions` and `.append(true)`.

@@quiz
Your program loads `save.txt` at startup, but on the very first run the file doesn't exist. What's the best behaviour?
- Panic with an error
* Treat `ErrorKind::NotFound` as "no data yet" and start empty
- Ask the user to create the file
? A missing save file is normal on the first run. Other errors (like permission denied) should still be reported.

@@exercise A notes file
Write two functions:

- `add_note(path: &str, note: &str) -> io::Result<()>` — **appends** the note to the file as one line (creating the file if needed).
- `read_notes(path: &str) -> Vec<String>` — returns all the non-empty lines of the file. If the file doesn't exist, return an empty vector.
@@starter
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn add_note(path: &str, note: &str) -> io::Result<()> {
    todo!()
}

fn read_notes(path: &str) -> Vec<String> {
    todo!()
}

fn main() {
    let _ = fs::remove_file("notes.txt");
    println!("{} notes", read_notes("notes.txt").len());
    add_note("notes.txt", "buy milk").unwrap();
    add_note("notes.txt", "call grandma").unwrap();
    add_note("notes.txt", "learn file I/O").unwrap();
    for (i, note) in read_notes("notes.txt").iter().enumerate() {
        println!("{}. {}", i + 1, note);
    }
}
@@expected
0 notes
1. buy milk
2. call grandma
3. learn file I/O
@@tests
#[test]
fn checks() {
    let path = "check_notes.txt";
    let _ = std::fs::remove_file(path);
    assert!(read_notes(path).is_empty(), "a missing file means no notes");
    add_note(path, "one").unwrap();
    assert_eq!(read_notes(path), vec!["one"]);
    std::fs::write(path, "a\n\n  \nb\n").unwrap();
    assert_eq!(read_notes(path), vec!["a", "b"], "empty lines are skipped");
    add_note(path, "c").unwrap();
    assert_eq!(read_notes(path), vec!["a", "b", "c"]);
    let _ = std::fs::remove_file(path);
}
@@requires
OpenOptions::new()
@@hint
`add_note`: open with `OpenOptions::new().create(true).append(true).open(path)?`, then `writeln!(file, "{note}")?;` and `Ok(())`.

`read_notes`: `match fs::read_to_string(path)`; on `Err(_)` return `Vec::new()`. On `Ok(text)`: `text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).map(|l| l.to_string()).collect()`.
@@solution
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn add_note(path: &str, note: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{note}")?;
    Ok(())
}

fn read_notes(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(text) => text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let _ = fs::remove_file("notes.txt");
    println!("{} notes", read_notes("notes.txt").len());
    add_note("notes.txt", "buy milk").unwrap();
    add_note("notes.txt", "call grandma").unwrap();
    add_note("notes.txt", "learn file I/O").unwrap();
    for (i, note) in read_notes("notes.txt").iter().enumerate() {
        println!("{}. {}", i + 1, note);
    }
}

@@exercise Saving high scores
Save and load a high-score table in the format `name;score`, one entry per line.

- `save_scores(path, scores: &[(String, u32)]) -> io::Result<()>` writes the whole table (replacing the old file).
- `load_scores(path) -> Result<Vec<(String, u32)>, String>`:
  - a missing file → `Ok` with an empty list,
  - empty lines are skipped,
  - a line without `;`, or with a score that isn't a number, → `Err` with a message that includes the line number, like `"line 2: bad score 'abc'"`. (The exact wording is up to you, but it must contain `line 2`.)
@@starter
use std::fs;
use std::io::{self, ErrorKind};

fn save_scores(path: &str, scores: &[(String, u32)]) -> io::Result<()> {
    todo!()
}

fn load_scores(path: &str) -> Result<Vec<(String, u32)>, String> {
    todo!()
}

fn main() {
    let scores = vec![(String::from("Ada"), 95), (String::from("Grace"), 88)];
    save_scores("scores.txt", &scores).unwrap();
    print!("{}", fs::read_to_string("scores.txt").unwrap());

    let loaded = load_scores("scores.txt").unwrap();
    println!("{:?}", loaded);

    fs::write("broken.txt", "Ada;95\nGrace;lots\n").unwrap();
    match load_scores("broken.txt") {
        Ok(_) => println!("loaded?!"),
        Err(e) => println!("error: {e}"),
    }
}
@@tests
#[test]
fn checks() {
    let _ = std::fs::remove_file("missing_scores.txt");
    assert_eq!(load_scores("missing_scores.txt"), Ok(vec![]));

    let data = vec![(String::from("x"), 1), (String::from("y"), 22)];
    save_scores("roundtrip.txt", &data).unwrap();
    assert_eq!(load_scores("roundtrip.txt"), Ok(data));

    std::fs::write("gaps.txt", "a;1\n\nb;2\n").unwrap();
    assert_eq!(load_scores("gaps.txt").unwrap().len(), 2);

    std::fs::write("bad1.txt", "a;1\nno separator\n").unwrap();
    let err = load_scores("bad1.txt").unwrap_err();
    assert!(err.contains("line 2"), "error should mention the line number, got: {err}");

    std::fs::write("bad2.txt", "a;-5\n").unwrap();
    assert!(load_scores("bad2.txt").unwrap_err().contains("line 1"));
}
@@requires
ErrorKind::NotFound
@@hint
Saving: build a `String` with `format!("{};{}\n", name, score)` for each entry, then `fs::write(path, text)`.

Loading: first handle the read like the "missing file" example in the lesson. Then `for (i, line) in text.lines().enumerate()`: skip empty lines, use `line.split_once(';')` (it returns `Option<(&str, &str)>`), and `score.trim().parse::<u32>()`. Turn problems into `Err(format!("line {}: ...", i + 1))`.
@@solution
use std::fs;
use std::io::{self, ErrorKind};

fn save_scores(path: &str, scores: &[(String, u32)]) -> io::Result<()> {
    let mut text = String::new();
    for (name, score) in scores {
        text.push_str(&format!("{name};{score}\n"));
    }
    fs::write(path, text)
}

fn load_scores(path: &str) -> Result<Vec<(String, u32)>, String> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("could not read {path}: {e}")),
    };

    let mut scores = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((name, score)) = line.split_once(';') else {
            return Err(format!("line {}: missing ';'", i + 1));
        };
        let score: u32 = match score.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("line {}: bad score '{}'", i + 1, score.trim())),
        };
        scores.push((name.trim().to_string(), score));
    }
    Ok(scores)
}

fn main() {
    let scores = vec![(String::from("Ada"), 95), (String::from("Grace"), 88)];
    save_scores("scores.txt", &scores).unwrap();
    print!("{}", fs::read_to_string("scores.txt").unwrap());

    let loaded = load_scores("scores.txt").unwrap();
    println!("{:?}", loaded);

    fs::write("broken.txt", "Ada;95\nGrace;lots\n").unwrap();
    match load_scores("broken.txt") {
        Ok(_) => println!("loaded?!"),
        Err(e) => println!("error: {e}"),
    }
}
