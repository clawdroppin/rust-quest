@@lesson g-nv1 Note Vault 1: Notes, Tags and Search
@@text
In this build you'll make **Note Vault**, a notes app in the style of Obsidian or Notion, small enough to understand completely:

- every note is a **plain Markdown file** in a `notes` folder, so your notes are never locked inside the app: you can open, back up or sync them with anything,
- a **list** of notes, newest first, with a **search** box,
- **#tags** anywhere in the text, collected into a clickable **tag cloud** for filtering,
- an editor with a live **Markdown preview** (edit, split or preview),
- **autosave**, plus creating, renaming and (safely) deleting notes.

## The design

Data first, as always. A note is a name, a text and a "last changed" time; the vault is the folder plus the list of notes:

```rust,norun
pub struct Note {
    pub name: String,          // file name without ".md"; also the title
    pub body: String,
    pub modified: SystemTime,  // to sort the list, newest first
}

pub struct Vault {
    pub dir: PathBuf,
    pub notes: Vec<Note>,
}
```

Why files and not one big JSON file like the Focus Timer? For notes, plain files are a feature: users can edit them in any editor, put the folder in OneDrive or Dropbox, and they're still readable in 30 years. The price: the **file name** is the note's name, and file names have rules. On Windows these characters aren't allowed: `\ / : * ? " < > |`. So names typed by the user must be **checked** before we use them.

**Tags** aren't stored anywhere: they're **derived** from the text every time we need them. That's a good principle: store each fact once, and compute everything else from it, so things can't get out of sync. (Computing them is fast. If you had 100,000 notes, you might cache them.)

## Step 1: the text logic

In this step you write the functions that work on text: checking names, finding tags and searching. Reading and writing the files is already written for you, so you can read it as an example.

@@project note_vault Note Vault, step 1: notes, tags and search
`src/vault.rs` with the vault and some functions for you to write, and `src/main.rs` with a quick test drive in the terminal.
@@file Cargo.toml
[package]
name = "note_vault"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
egui_commonmark = { version = "0.26", default-features = false, features = ["pulldown_cmark"] }

@@file src/main.rs
mod vault;

use std::path::Path;
use vault::Vault;

/// A quick look at the vault without a window: list every note with its tags,
/// then the notes matching a search.
fn main() {
    let vault = match Vault::open(Path::new("notes")) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Couldn't open the notes folder: {e}");
            return;
        }
    };
    if vault.notes.is_empty() {
        println!("The notes folder is empty. Add a few .md files to it (see the lesson) and run again.");
        return;
    }
    for note in &vault.notes {
        let tags: Vec<String> = vault::tags(&note.body).into_iter().collect();
        println!("{}: {} | tags: {}", note.name, vault::snippet(&note.body, 40), tags.join(", "));
    }
    println!("All tags: {:?}", vault::all_tags(&vault.notes));

    let query = "#ideas";
    let found: Vec<&str> = vault.notes.iter().filter(|n| vault::matches(n, query)).map(|n| n.name.as_str()).collect();
    println!("Search for {query:?}: {found:?}");
    println!("Is \"a/b\" a valid name? {:?}", vault::valid_name("a/b"));
}

@@file src/vault.rs
//! The notes themselves: a folder of Markdown files, plus searching and #tags.
//! No GUI code here.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct Note {
    /// The file name without `.md`. Also the note's title.
    pub name: String,
    pub body: String,
    pub modified: SystemTime,
}

pub struct Vault {
    pub dir: PathBuf,
    /// Newest first.
    pub notes: Vec<Note>,
}

// ---------- text logic ----------

/// Checks a note name typed by the user, and returns it trimmed.
/// Names become file names, so some characters aren't allowed.
pub fn valid_name(name: &str) -> Result<String, String> {
    todo!()
}

/// All #tags in a text, in lowercase, without the #. A tag is a # directly followed by
/// letters, digits, - or _. So `# Heading` (with a space) isn't a tag, and neither is `##`.
pub fn tags(body: &str) -> BTreeSet<String> {
    todo!()
}

/// Does the note match the search? Every word of the query must appear in the name or
/// the body (ignoring case). A word starting with # must be one of the note's tags.
pub fn matches(note: &Note, query: &str) -> bool {
    todo!()
}

/// Every tag used in any note, with how many notes use it.
pub fn all_tags(notes: &[Note]) -> BTreeMap<String, usize> {
    todo!()
}

/// The first line with some text, without Markdown heading marks, for the note list.
pub fn snippet(body: &str, max_chars: usize) -> String {
    let line = body
        .lines()
        .map(|l| l.trim_start_matches('#').trim())
        .find(|l| !l.is_empty())
        .unwrap_or("");
    if line.chars().count() > max_chars {
        let cut: String = line.chars().take(max_chars).collect();
        format!("{cut}…")
    } else {
        line.to_string()
    }
}

// ---------- files ----------

impl Vault {
    /// Opens (or creates) a folder of notes and reads every `.md` file in it.
    pub fn open(dir: &Path) -> io::Result<Vault> {
        fs::create_dir_all(dir)?;
        let mut notes = Vec::new();
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && let Some(name) = path.file_stem().and_then(|s| s.to_str())
            {
                let body = fs::read_to_string(&path)?;
                let modified = fs::metadata(&path)?.modified()?;
                notes.push(Note {
                    name: name.to_string(),
                    body,
                    modified,
                });
            }
        }
        let mut vault = Vault {
            dir: dir.to_path_buf(),
            notes,
        };
        vault.sort();
        Ok(vault)
    }

    pub fn sort(&mut self) {
        self.notes
            .sort_by_key(|note| std::cmp::Reverse(note.modified));
    }

    fn path_of(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{name}.md"))
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.notes
            .iter()
            .position(|n| n.name.eq_ignore_ascii_case(name))
    }

    /// Writes one note to its file.
    pub fn save(&mut self, index: usize) -> io::Result<()> {
        let note = &self.notes[index];
        fs::write(self.path_of(&note.name), &note.body)?;
        self.notes[index].modified = SystemTime::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(name: &str, body: &str) -> Note {
        Note {
            name: name.to_string(),
            body: body.to_string(),
            modified: SystemTime::now(),
        }
    }

    #[test]
    fn checks_names() {
        assert_eq!(
            valid_name("  Shopping list "),
            Ok("Shopping list".to_string())
        );
        assert!(valid_name("   ").is_err());
        assert!(valid_name("a/b").is_err());
        assert!(valid_name("what?").is_err());
        assert!(valid_name(".hidden").is_err());
        assert!(valid_name(&"x".repeat(81)).is_err());
    }

    #[test]
    fn finds_tags() {
        let body = "# My Title\nIdeas for #rust and #GUI-apps, maybe #rust again.\n## Sub heading\n#todo: call Sam. Not a tag: C# or #.";
        let found: Vec<String> = tags(body).into_iter().collect();
        assert_eq!(found, ["gui-apps", "rust", "todo"]);
    }

    #[test]
    fn searches() {
        let n = note(
            "Trip to Rome",
            "Pack the #travel adapter.\nBook the Colosseum",
        );
        assert!(matches(&n, ""));
        assert!(matches(&n, "rome"));
        assert!(matches(&n, "colosseum PACK"));
        assert!(matches(&n, "#travel"));
        assert!(!matches(&n, "#rome"));
        assert!(!matches(&n, "rome paris"));
    }

    #[test]
    fn counts_tags() {
        let notes = [note("a", "#x #y"), note("b", "#x"), note("c", "nothing")];
        let counts = all_tags(&notes);
        assert_eq!(counts["x"], 2);
        assert_eq!(counts["y"], 1);
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn snippets() {
        assert_eq!(snippet("\n\n# Hello there\nmore", 50), "Hello there");
        assert_eq!(snippet("abcdef", 3), "abc…");
        assert_eq!(snippet("", 3), "");
    }

    #[test]
    fn files() {
        let dir = std::env::temp_dir().join(format!("note_vault_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let mut vault = Vault::open(&dir).unwrap();
        assert!(vault.notes.is_empty());
        fs::write(dir.join("First.md"), "Hello").unwrap();
        let mut vault2 = Vault::open(&dir).unwrap();
        assert_eq!(vault2.notes.len(), 1);
        vault2.notes[0].body.push_str(" #test");
        vault2.save(0).unwrap();
        vault = Vault::open(&dir).unwrap();
        assert_eq!(vault.notes[0].body, "Hello #test");
        fs::remove_dir_all(&dir).unwrap();
    }
}
@@text
Notice `Cargo.toml`: besides `eframe`, it uses **`egui_commonmark`**, a crate that renders Markdown in egui (it's what Rust Quest uses for its lessons). We'll use it in step 2. The first build downloads and compiles it, which takes a minute.

## Read `Vault::open` first

```rust,norun
pub fn open(dir: &Path) -> io::Result<Vault> {
    fs::create_dir_all(dir)?;
    let mut notes = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "md")
            && let Some(name) = path.file_stem().and_then(|s| s.to_str())
        {
            let body = fs::read_to_string(&path)?;
            let modified = fs::metadata(&path)?.modified()?;
            notes.push(Note { name: name.to_string(), body, modified });
        }
    }
    ...
}
```

- `fs::read_dir` lists a folder. Each entry can fail (hence `entry?`), and gives a `PathBuf`.
- `path.extension()` is the part after the last dot; `file_stem()` the name without it. Both return `Option<&OsStr>`: file names on Windows aren't always valid UTF-8, so `to_str()` returns an `Option` too.
- `fs::metadata(&path)?.modified()?` is the file's "last changed" time.
- Every `?` passes an error up to the caller, who decides what to do (the GUI will show a message).

`sort` then uses `sort_by_key(|note| std::cmp::Reverse(note.modified))`: wrapping a key in `Reverse` sorts **descending**, newest first.

## Your tasks

Run `cargo test`. Four tests fail. Make them pass, one function at a time:

**1. `valid_name(name)`** returns the trimmed name, or an `Err` with a message the user will read. Reject:
- an empty name (after trimming),
- more than 80 characters (`name.chars().count()`, not `len()`, which counts bytes),
- any of the characters `\ / : * ? " < > |`. A neat way: put them in a string and ask `"...".contains(c)` for each char of the name. A **raw string** `r#"\/:*?"<>|"#` lets you write the backslash and the quote without escaping them. For a helpful message, use `find` to get the first bad character, and say which it was.
- a name starting with `.` (hidden files on some systems).

**2. `tags(body)`** returns all tags, lowercase, without the `#`, as a `BTreeSet<String>` (sorted, no duplicates). The rules, from the test:
- go through `body.split_whitespace()`,
- a word is a tag if it **starts with `#`** (`strip_prefix('#')` gives you the rest, or `None`),
- the tag is the following characters while they're letters, digits, `-` or `_`. `chars().take_while(...)` collects exactly that, so `#todo:` gives `todo`, and `#rust,` gives `rust`,
- skip it if that's empty (so `#` alone, `##` and `# Heading` aren't tags).

**3. `matches(note, query)`**: does a note match the search? Every **word** of the query must match (so "rome trip" finds notes containing both words), ignoring upper/lower case:
- a word starting with `#` must be one of the note's `tags`,
- any other word must appear in the name or the body.

An empty query matches everything, and `Iterator::all` already returns `true` for no words at all. Convert the name and body `to_lowercase()` **once**, before looping over the words.

**4. `all_tags(notes)`**: every tag in any note, with the number of notes using it: a `BTreeMap<String, usize>` filled with the `entry(...).or_insert(0)` pattern.

## Try it

Create a folder `notes` in the project folder (next to `Cargo.toml`), with two or three `.md` files, for example `Ideas.md`:

```text
# App ideas
A habit tracker #ideas #gui
A recipe book #ideas
```

Then `cargo run`. The test drive lists your notes with their tags, all tags with their counts, and searches for `#ideas`.

@@quiz
Why are the tags found from the text every time, instead of being stored in a separate list?
- Storing them would need a database
* So they can never get out of sync with the text: each fact is stored once, everything else is computed
- `BTreeSet` can't be saved
? If tags were stored separately, editing the text would have to update the list too, and any bug there would make them disagree.

@@quiz
Why does `valid_name` use `name.chars().count()` and not `name.len()`?
- `len()` doesn't exist on `&str`
* `len()` counts bytes, and letters like é or emoji take more than one byte
- `chars().count()` is faster
? A limit on what the user sees should count characters, not bytes.
