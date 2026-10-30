@@lesson g-fo2 File Organizer 2: The Preview Window
@@text
Now the window: type or pick a folder, scan it, choose the categories, and see the plan as a table. Still nothing is moved: that's step 3. Being able to preview first is what makes the app trustworthy.

If you're continuing from step 1, press **Add 1 new file(s)** to get `src/app.rs`, and copy the new `src/main.rs` from the starting files. Or reset to this step's files (which includes a finished `organizer.rs`).

@@project file_organizer File Organizer, step 2: the preview window
Adds `src/app.rs` with the app's state and a bare window. `src/main.rs` changes too: it opens the window.
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
mod app;
mod organizer;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 620.0]).with_min_inner_size([520.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("File Organizer", options, Box::new(|_cc| Ok(Box::new(app::OrganizerApp::new()))))
}

@@file src/app.rs
//! The window. In this step you build it up piece by piece, following the lesson.

use crate::organizer::{self, Category, FileInfo, Move, Taken};
use eframe::egui;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct OrganizerApp {
    /// The folder path as typed by the user.
    folder: String,
    /// The folder that `files` and `plan` belong to (set by a successful scan).
    scanned: Option<PathBuf>,
    files: Vec<FileInfo>,
    existing: Taken,
    enabled: HashSet<Category>,
    plan: Vec<Move>,
    message: String,
}

impl OrganizerApp {
    pub fn new() -> Self {
        OrganizerApp {
            folder: String::new(),
            scanned: None,
            files: Vec::new(),
            existing: Taken::new(),
            enabled: Category::ALL.into_iter().collect(),
            plan: Vec::new(),
            message: String::new(),
        }
    }
}

impl eframe::App for OrganizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("File Organizer");
            ui.text_edit_singleline(&mut self.folder);
            ui.label(&self.message);
            // Your code goes here, following the lesson.
        });
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
    let ext = match file_name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => ext.to_lowercase(),
        _ => return Category::Other, // no extension, or a name like ".gitignore"
    };
    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "heic" | "ico" => Category::Images,
        "pdf" | "doc" | "docx" | "txt" | "md" | "odt" | "rtf" | "xls" | "xlsx" | "csv" | "ppt" | "pptx" => Category::Documents,
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" => Category::Audio,
        "mp4" | "mkv" | "mov" | "avi" | "webm" | "wmv" => Category::Video,
        "zip" | "rar" | "7z" | "tar" | "gz" => Category::Archives,
        "exe" | "msi" => Category::Installers,
        "rs" | "py" | "js" | "ts" | "html" | "css" | "json" | "toml" | "c" | "cpp" | "java" => Category::Code,
        _ => Category::Other,
    }
}

/// `name`, or if that's taken, `name (2)`, `name (3)`… keeping the extension.
/// `taken` holds lowercase names, because Windows file names ignore case.
pub fn unique_name(name: &str, taken: &HashSet<String>) -> String {
    if !taken.contains(&name.to_lowercase()) {
        return name.to_string();
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem, format!(".{ext}")),
        _ => (name, String::new()),
    };
    (2..)
        .map(|n| format!("{stem} ({n}){ext}"))
        .find(|candidate| !taken.contains(&candidate.to_lowercase()))
        .unwrap()
}

/// The (lowercase) names already used in each category folder.
pub type Taken = HashMap<Category, HashSet<String>>;

/// Plans where each file goes. `existing` holds the names of the files that are already
/// in the category folders, so nothing is overwritten.
pub fn plan(files: &[FileInfo], enabled: &HashSet<Category>, existing: &Taken) -> Vec<Move> {
    // Names used so far: on disk, plus by the earlier moves of this plan.
    let mut taken = existing.clone();
    let mut moves = Vec::new();
    for file in files {
        let category = category_of(&file.name);
        if !enabled.contains(&category) {
            continue;
        }
        let names = taken.entry(category).or_default();
        let name = unique_name(&file.name, names);
        names.insert(name.to_lowercase());
        moves.push(Move { from: file.name.clone(), to: format!("{}/{name}", category.folder()) });
    }
    moves
}

/// `1.5 MB` and friends.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let mut size = bytes as f64 / 1024.0;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{size:.1} {}", UNITS[unit])
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
## The state

Look at `OrganizerApp`'s fields. Notice there are **two** folders:

```rust,norun
/// The folder path as typed by the user.
folder: String,
/// The folder that `files` and `plan` belong to (set by a successful scan).
scanned: Option<PathBuf>,
```

Why? The user can edit the text box at any time. If we used `folder` when organizing, someone could scan `Downloads`, then type another path, and press Organize: the plan made for `Downloads` would be applied to a different folder! `scanned` remembers which folder the plan is actually *for*. Keeping "what the user is typing" separate from "what we checked" is an important habit whenever input leads to an action.

## Part 1: scanning

Add these methods to `impl OrganizerApp`:

```rust,norun
fn scan(&mut self) {
    let dir = PathBuf::from(self.folder.trim());
    self.scanned = None;
    self.files.clear();
    self.plan.clear();
    if organizer::is_risky(&dir) {
        self.message = "For safety, a whole drive or your home folder can't be organized. Pick a folder inside it.".to_string();
        return;
    }
    match organizer::scan(&dir) {
        Ok(files) => {
            self.files = files;
            self.existing = organizer::existing_targets(&dir);
            self.scanned = Some(dir);
            self.replan();
            self.message = format!("Found {} files.", self.files.len());
        }
        Err(e) => self.message = format!("Couldn't read that folder: {e}"),
    }
}

fn replan(&mut self) {
    self.plan = organizer::plan(&self.files, &self.enabled, &self.existing);
}
```

Notice how `scan` first **clears** the old results. If scanning fails halfway, we're left with nothing rather than a mix of old and new: no stale plan can be applied by accident.

The folder row: a text box, a "Downloads" shortcut and a Scan button (Enter in the box scans too):

```rust,norun
fn downloads_folder() -> String {
    std::env::var("USERPROFILE").map(|home| format!("{home}\\Downloads")).unwrap_or_default()
}
```

```rust,norun
fn folder_row(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Folder:");
        let response = ui.add(egui::TextEdit::singleline(&mut self.folder).desired_width(ui.available_width() - 170.0));
        let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        if ui.button("Downloads").clicked() {
            self.folder = downloads_folder();
            self.scan();
        }
        if ui.button(RichText::new("Scan").strong()).clicked() || enter {
            self.scan();
        }
    });
}
```

`downloads_folder` is a free function (outside the `impl`), because it doesn't need the app. `USERPROFILE` is the Windows environment variable for your home folder. Start the app with `folder: downloads_folder()` in `new()`, too.

Set up the panels in `ui()`: a top panel with the folder row, a bottom panel showing `self.message`, and the central panel for the preview (Part 3):

```rust,norun
egui::Panel::top("top").show(ui, |ui| {
    ui.add_space(6.0);
    self.folder_row(ui);
    ui.add_space(2.0);
});
egui::Panel::bottom("actions").show(ui, |ui| {
    ui.add_space(6.0);
    ui.label(&self.message);
    ui.add_space(4.0);
});
egui::CentralPanel::default().show(ui, |ui| self.preview(ui));
```

## Part 2: choosing categories

One checkbox per category, each with the number of files in it. The counts go in a `BTreeMap`, so they're ready to look up:

```rust,norun
fn category_row(&mut self, ui: &mut egui::Ui) {
    // How many files of each category the folder has.
    let mut counts: BTreeMap<Category, usize> = BTreeMap::new();
    for file in &self.files {
        *counts.entry(organizer::category_of(&file.name)).or_insert(0) += 1;
    }
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        ui.label("Sort into folders:");
        for category in Category::ALL {
            let count = counts.get(&category).copied().unwrap_or(0);
            let mut on = self.enabled.contains(&category);
            if ui.checkbox(&mut on, format!("{} ({count})", category.folder())).changed() {
                if on {
                    self.enabled.insert(category);
                } else {
                    self.enabled.remove(&category);
                }
                changed = true;
            }
        }
    });
    if changed {
        self.replan();
    }
}
```

A checkbox needs a `&mut bool`, but "enabled" is stored as membership in a `HashSet`. So we copy it into a local `bool`, let the checkbox change that, and update the set when it changed. That's a useful trick whenever the GUI's shape differs from your data's shape.

Call it in the top panel, under the folder row. And in `new()`, enable everything *except* `Other`: unknown files are often best left where they are.

## Part 3: the preview table

`egui::Grid` lays out rows and columns; `striped(true)` gives alternate rows a background, which makes long tables easier to read:

```rust,norun
fn preview(&self, ui: &mut egui::Ui) {
    if self.scanned.is_none() {
        ui.label(RichText::new("Choose a folder and press Scan. Nothing is changed until you press Organize.").weak());
        return;
    }
    if self.plan.is_empty() {
        ui.label(RichText::new("Nothing to do: no files of the chosen kinds here.").weak());
        return;
    }
    let sizes: BTreeMap<&str, u64> = self.files.iter().map(|f| (f.name.as_str(), f.size)).collect();
    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        egui::Grid::new("plan").striped(true).num_columns(3).show(ui, |ui| {
            ui.label(RichText::new("File").strong());
            ui.label(RichText::new("Goes to").strong());
            ui.label(RichText::new("Size").strong());
            ui.end_row();
            for m in &self.plan {
                ui.label(&m.from);
                // A renamed file (because the name was taken) is shown in yellow.
                let renamed = !m.to.ends_with(&format!("/{}", m.from));
                if renamed {
                    ui.label(RichText::new(&m.to).color(Color32::YELLOW))
                        .on_hover_text("A file with this name is already there, so this one gets a number");
                } else {
                    ui.label(&m.to);
                }
                ui.label(organizer::human_size(sizes.get(m.from.as_str()).copied().unwrap_or(0)));
                ui.end_row();
            }
        });
    });
}
```

The `sizes` map is built once per frame so each row can look up its size quickly, instead of searching the whole file list for every row.

Run it, press **Downloads**, and look at the plan for your real Downloads folder. Completely safe: nothing is moved yet. Try unticking categories, and scanning `demo_folder` from step 1 (type its full path, or the relative `demo_folder`).

## Optional: open with a folder

Programs often accept a file or folder on the command line. At the end of `new()`:

```rust,norun
// `file_organizer <folder>` opens with that folder scanned.
if let Some(folder) = std::env::args().nth(1) {
    app.folder = folder;
    app.scan();
}
```

(`new` now needs `let mut app = OrganizerApp { ... };` and `app` at the end.) Try `cargo run -- demo_folder`: everything after `--` is passed to your program.

@@quiz
Why does the app keep `scanned` separately from the `folder` text?
- To save memory
* So a plan is only ever applied to the folder it was made for, even if the text box was edited afterwards
- Because a `PathBuf` can't be edited in a text box
? Input the user is still typing shouldn't silently change what an action does.

@@quiz
The checkbox needs a `&mut bool`, but the enabled categories are a `HashSet`. What does `category_row` do?
* Copies membership into a local `bool`, lets the checkbox change it, and updates the set if it changed
- Stores a `bool` field for every category instead
- Uses `HashSet::get_mut`
? Bridging between the GUI's shape and your data's shape with a local variable is a common pattern.
