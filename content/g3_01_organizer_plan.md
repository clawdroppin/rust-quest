@@lesson g-fo1 File Organizer 1: Making a Plan
@@text
Everyone has one: a Downloads folder with hundreds of files in a pile. In this build you'll make an app that tidies it up:

- choose a folder, and see what's in it, sorted into **categories**: images, documents, audio, video, archives, installers, code,
- pick which categories to sort, and see a **preview** of exactly where every file will go,
- press **Organize** to move the files into folders like `Images` and `Documents`,
- and if you don't like the result, **Undo** puts everything back, even after restarting the app.

This app moves real files on your computer. That makes it genuinely useful, and it means **safety is the main design goal**. Before any code, here are the rules we'll follow:

1. **Plan, then act.** First work out a complete plan (a list of moves) without touching anything. Show it. Only move files when the user confirms.
2. **Never overwrite.** If `Images/cat.png` already exists, the new one becomes `Images/cat (2).png`.
3. **Keep a log** of everything we did, so it can be undone.
4. **Refuse dangerous folders**, like a whole drive, and leave anything we don't recognise (like hidden files) alone.
5. **Expect failures.** A file may be open in another program, or deleted in the meantime. Report it and carry on with the rest.

Rule 1 also makes the code much easier to test: making a plan is **pure logic** (lists in, a list out) that we can test without any real files. This step is all about the plan.

@@project file_organizer File Organizer, step 1: making a plan
`src/organizer.rs` with categories, scanning and planning, with some functions for you to write. `src/main.rs` creates a messy demo folder and prints the plan for it: a "dry run".
@@file Cargo.toml
[package]
name = "file_organizer"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

@@file src/main.rs
mod organizer;

use organizer::Category;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// A "dry run" in the terminal: make a messy demo folder, and print what the organizer
/// *would* do with it. Nothing is moved yet.
fn main() {
    let dir = Path::new("demo_folder");
    make_demo_folder(dir);

    let files = match organizer::scan(dir) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Couldn't read {}: {e}", dir.display());
            return;
        }
    };
    let enabled: HashSet<Category> = Category::ALL.into_iter().collect();
    let existing = organizer::existing_targets(dir);
    for m in organizer::plan(&files, &enabled, &existing) {
        let size = files.iter().find(|f| f.name == m.from).map_or(0, |f| f.size);
        println!("{:<20} -> {:<28} {}", m.from, m.to, organizer::human_size(size));
    }
}

/// Some empty files with different extensions, and one that's already "organized".
fn make_demo_folder(dir: &Path) {
    let names = ["holiday.jpg", "cat.png", "report.pdf", "budget.xlsx", "song.mp3", "movie.mp4", "setup.exe", "backup.zip", "main.rs", "README"];
    let _ = fs::create_dir_all(dir.join("Images"));
    let _ = fs::write(dir.join("Images").join("cat.png"), "an older cat");
    for name in names {
        let _ = fs::write(dir.join(name), format!("pretend this is {name}"));
    }
}

@@file src/organizer.rs
//! Planning and doing the tidy-up. No GUI code here.
//!
//! The big idea: first make a **plan** (a list of moves) without touching anything,
//! and show it to the user. (Doing the moves comes in step 3.)

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Category {
    Images,
    Documents,
    Audio,
    Video,
    Archives,
    Installers,
    Code,
    Other,
}

impl Category {
    pub const ALL: [Category; 8] = [
        Category::Images,
        Category::Documents,
        Category::Audio,
        Category::Video,
        Category::Archives,
        Category::Installers,
        Category::Code,
        Category::Other,
    ];

    /// The name of the folder files of this category go to.
    pub fn folder(self) -> &'static str {
        match self {
            Category::Images => "Images",
            Category::Documents => "Documents",
            Category::Audio => "Audio",
            Category::Video => "Video",
            Category::Archives => "Archives",
            Category::Installers => "Installers",
            Category::Code => "Code",
            Category::Other => "Other",
        }
    }
}

/// A file found in the folder.
#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
}

/// One planned (or done) move, with paths relative to the organized folder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Move {
    pub from: String,
    pub to: String,
}

// ---------- planning (pure logic, easy to test) ----------

/// Which category a file belongs to, by its extension (ignoring case).
pub fn category_of(file_name: &str) -> Category {
    todo!()
}

/// `name`, or if that's taken, `name (2)`, `name (3)`… keeping the extension.
/// `taken` holds lowercase names, because Windows file names ignore case.
pub fn unique_name(name: &str, taken: &HashSet<String>) -> String {
    todo!()
}

/// The (lowercase) names already used in each category folder.
pub type Taken = HashMap<Category, HashSet<String>>;

/// Plans where each file goes. `existing` holds the names of the files that are already
/// in the category folders, so nothing is overwritten.
pub fn plan(files: &[FileInfo], enabled: &HashSet<Category>, existing: &Taken) -> Vec<Move> {
    todo!()
}

/// `1.5 MB` and friends.
pub fn human_size(bytes: u64) -> String {
    todo!()
}

// ---------- the file system ----------

/// The files (not folders) directly in `dir`, sorted by name. Hidden files are skipped.
pub fn scan(dir: &Path) -> io::Result<Vec<FileInfo>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        let Some(name) = entry.file_name().to_str().map(String::from) else { continue };
        if meta.is_file() && !name.starts_with('.') && !name.eq_ignore_ascii_case("desktop.ini") {
            files.push(FileInfo { name, size: meta.len() });
        }
    }
    files.sort_by_key(|f| f.name.to_lowercase());
    Ok(files)
}

/// The names of the files already in the category folders.
pub fn existing_targets(dir: &Path) -> Taken {
    let mut taken = Taken::new();
    for category in Category::ALL {
        // No folder yet? Then nothing's taken: `continue` with the next category.
        let Ok(entries) = fs::read_dir(dir.join(category.folder())) else { continue };
        let names = taken.entry(category).or_default();
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                names.insert(name.to_lowercase());
            }
        }
    }
    taken
}

/// Folders we refuse to organize: a whole drive, or the user's home folder itself.
pub fn is_risky(dir: &Path) -> bool {
    let home = std::env::var_os("USERPROFILE").map(PathBuf::from);
    dir.parent().is_none() || home.is_some_and(|h| h == dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(names: &[&str]) -> Vec<FileInfo> {
        names.iter().map(|n| FileInfo { name: n.to_string(), size: 1 }).collect()
    }

    fn all() -> HashSet<Category> {
        Category::ALL.into_iter().collect()
    }

    #[test]
    fn categories() {
        assert_eq!(category_of("Holiday.JPG"), Category::Images);
        assert_eq!(category_of("report.final.pdf"), Category::Documents);
        assert_eq!(category_of("setup.exe"), Category::Installers);
        assert_eq!(category_of("README"), Category::Other);
        assert_eq!(category_of(".gitignore"), Category::Other);
        assert_eq!(category_of("weird.xyz"), Category::Other);
    }

    #[test]
    fn unique_names() {
        let taken: HashSet<String> = ["cat.png", "cat (2).png", "notes"].iter().map(|s| s.to_string()).collect();
        assert_eq!(unique_name("dog.png", &taken), "dog.png");
        assert_eq!(unique_name("Cat.PNG", &taken), "Cat (3).PNG");
        assert_eq!(unique_name("notes", &taken), "notes (2)");
    }

    #[test]
    fn plans_moves() {
        let mut existing = Taken::new();
        existing.entry(Category::Images).or_default().insert("cat.png".to_string());
        let moves = plan(&files(&["cat.png", "a.pdf", "b.mp3", "c.PNG", "x"]), &all(), &existing);
        let to: Vec<&str> = moves.iter().map(|m| m.to.as_str()).collect();
        assert_eq!(to, ["Images/cat (2).png", "Documents/a.pdf", "Audio/b.mp3", "Images/c.PNG", "Other/x"]);
    }

    #[test]
    fn plan_avoids_clashes_within_itself() {
        // Two files that only differ in case would end up with the same name.
        let moves = plan(&files(&["A.png", "a.png"]), &all(), &Taken::new());
        assert_eq!(moves[1].to, "Images/a (2).png");
    }

    #[test]
    fn plan_skips_disabled_categories() {
        let only_images: HashSet<Category> = [Category::Images].into_iter().collect();
        let moves = plan(&files(&["a.png", "b.pdf"]), &only_images, &Taken::new());
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn sizes() {
        assert_eq!(human_size(500), "500 B");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(5 * 1024 * 1024), "5.0 MB");
    }

}
@@text
## The data

```rust,norun
pub enum Category { Images, Documents, Audio, Video, Archives, Installers, Code, Other }

pub struct FileInfo {
    pub name: String,
    pub size: u64,
}

/// One planned move, with paths relative to the organized folder.
pub struct Move {
    pub from: String,   // "cat.png"
    pub to: String,     // "Images/cat (2).png"
}
```

`Category` derives a lot of traits: `Hash` and `Eq` so it can go in a `HashSet` (which categories are enabled) or be a `HashMap` key, `PartialOrd`/`Ord` so it can be a `BTreeMap` key (for sorted counts in the GUI), and `Copy` because it's tiny. `Category::ALL` lists every variant, so we can loop over them, which Rust enums can't do by themselves. `Move` derives `Serialize` and `Deserialize`, because we'll save moves in the undo log in step 3.

Paths in a `Move` are **relative** to the organized folder, which makes plans easy to read and to test: no `C:\Users\...` everywhere.

## Already written: talking to the file system

Read `scan` and `existing_targets`. They're the only functions in this step that touch the disk:

- `scan(dir)` lists the **files** directly in a folder (not subfolders: `meta.is_file()`), skipping hidden ones (starting with `.`) and Windows' `desktop.ini`, sorted by name.
- `existing_targets(dir)` collects the names of the files already in each category folder (from an earlier run, perhaps), as a `HashMap<Category, HashSet<String>>`. It has a type alias, `Taken`, because that type is long. Names are stored **lowercase**, because Windows file names ignore case: `Cat.png` and `cat.png` are the same file.
- `is_risky(dir)` says no to a drive's root (`dir.parent()` is `None` for `C:\`) and to your home folder itself. Organizing `C:\Users\You` would move things like `NTUSER.DAT`... let's not.

## Your tasks

Run `cargo test`: six tests fail. In this order:

**1. `category_of(file_name)`**: the category, from the extension, ignoring case.
- `file_name.rsplit_once('.')` splits at the **last** dot: `"report.final.pdf"` gives `("report.final", "pdf")`. It returns `None` if there's no dot at all.
- A name like `.gitignore` gives an empty first part: that's a hidden file with no extension, so `Other`. A match arm with a guard handles it: `Some((stem, ext)) if !stem.is_empty() => ext.to_lowercase()`.
- Then `match ext.as_str()` with arms like `"jpg" | "jpeg" | "png" | "gif" => Category::Images`. Look at the test for the extensions it expects, and add more you know!

**2. `unique_name(name, taken)`**: returns `name` if it isn't in `taken` (compare in lowercase!), otherwise `name (2)`, `name (3)`, … keeping the extension: `cat.png` becomes `cat (2).png`, and `notes` (no extension) becomes `notes (2)`.
- Split into stem and extension like above (the extension *with* its dot, or an empty string).
- `(2..)` is an endless range: 2, 3, 4, … Use `.map(...)` to build each candidate name and `.find(...)` to get the first one that isn't taken. Since there are only finitely many taken names, `find` always finds one, so `.unwrap()` is safe here.

**3. `plan(files, enabled, existing)`**: a `Move` for every file whose category is enabled. The trick is to make sure **no two files get the same name**, neither with files already on disk (`existing`) nor with each other:
- Start with `let mut taken = existing.clone();`.
- For each file: find its category, skip it if not enabled (`continue`).
- Get that category's set of taken names with `taken.entry(category).or_default()`, find a `unique_name`, and **insert** the new name (lowercase) into the set, so the next file can't take it too.
- Push a `Move { from: file.name.clone(), to: format!("{}/{name}", category.folder()) }`.

The test `plan_avoids_clashes_within_itself` checks the tricky case: two files `A.png` and `a.png` can't both exist in one folder on Windows… but on other systems they can, so the plan must still give them different names.

**4. `human_size(bytes)`**: `500 B`, `1.5 KB`, `5.0 MB`. Below 1024, show bytes. Otherwise divide by 1024 (as `f64`) until the number is below 1024 or you run out of units, and format with one decimal (`{size:.1}`).

Then `cargo run`: the dry run creates `demo_folder` in the project and prints the plan. `cat.png` should become `Images/cat (2).png`, because the demo folder already has an `Images/cat.png`.

@@quiz
Why does the organizer make a complete plan before moving anything?
- Moving files is faster in a batch
* So the user can check exactly what will happen first, and the planning logic can be tested without touching real files
- Because `fs::rename` needs a plan
? "Plan, then act" separates the risky part (moving) from the thinking part (deciding where things go).

@@quiz
`taken` stores file names in lowercase. Why?
- Lowercase uses less memory
* Windows treats `Cat.png` and `cat.png` as the same file, so the comparison must ignore case
- `HashSet` can't store capital letters
? Otherwise the plan might move `Cat.png` onto an existing `cat.png`.
