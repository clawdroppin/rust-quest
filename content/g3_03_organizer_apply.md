@@lesson g-fo3 File Organizer 3: Organize and Undo
@@text
The plan is ready and visible. Now the exciting (and careful) part: actually moving files, and being able to undo it.

@@project file_organizer File Organizer, step 3: organize and undo
`src/organizer.rs` gets `apply` and `undo` for you to write, plus a ready-made undo log (`RunLog`, `save_log`, `load_log`, `clear_log`). The starting `src/app.rs` is the finished preview window from step 2.
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
//! The window: pick a folder and see the plan.

use crate::organizer::{self, Category, FileInfo, Move, Taken};
use eframe::egui::{self, Color32, RichText};
use std::collections::{BTreeMap, HashSet};
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

fn downloads_folder() -> String {
    std::env::var("USERPROFILE").map(|home| format!("{home}\\Downloads")).unwrap_or_default()
}

impl OrganizerApp {
    pub fn new() -> Self {
        let mut app = OrganizerApp {
            folder: downloads_folder(),
            scanned: None,
            files: Vec::new(),
            existing: Taken::new(),
            // Everything except "Other": unknown files are often best left alone.
            enabled: Category::ALL.into_iter().filter(|&c| c != Category::Other).collect(),
            plan: Vec::new(),
            message: String::new(),
        };
        // `file_organizer <folder>` opens with that folder scanned.
        if let Some(folder) = std::env::args().nth(1) {
            app.folder = folder;
            app.scan();
        }
        app
    }

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

    // ---------- drawing ----------

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

}

impl eframe::App for OrganizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(6.0);
            self.folder_row(ui);
            self.category_row(ui);
            ui.add_space(2.0);
        });
        egui::Panel::bottom("actions").show(ui, |ui| {
            ui.add_space(6.0);
            ui.label(&self.message);
            ui.add_space(4.0);
        });
        egui::CentralPanel::default().show(ui, |ui| self.preview(ui));
    }
}

@@file src/organizer.rs
//! Planning and doing the tidy-up. No GUI code here.
//!
//! The big idea: first make a **plan** (a list of moves) without touching anything,
//! show it to the user, and only then **apply** it, keeping a log so it can be undone.

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

/// Does the moves. Never overwrites: a move whose target exists is skipped with an error.
/// Returns the moves that worked (for the undo log) and messages for those that didn't.
pub fn apply(dir: &Path, moves: &[Move]) -> (Vec<Move>, Vec<String>) {
    todo!()
}

/// Moves files back, newest move first. Returns messages for moves that couldn't be undone.
pub fn undo(dir: &Path, moves: &[Move]) -> Vec<String> {
    todo!()
}

// ---------- the undo log ----------

const LOG_FILE: &str = "last_run.json";

/// What the last run did, saved so it can be undone even after a restart.
#[derive(Serialize, Deserialize)]
pub struct RunLog {
    pub dir: PathBuf,
    pub moves: Vec<Move>,
}

pub fn save_log(log: &RunLog) -> Result<(), String> {
    let json = serde_json::to_string_pretty(log).map_err(|e| e.to_string())?;
    fs::write(LOG_FILE, json).map_err(|e| format!("Couldn't save the undo log: {e}"))
}

pub fn load_log() -> Option<RunLog> {
    let text = fs::read_to_string(LOG_FILE).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn clear_log() {
    let _ = fs::remove_file(LOG_FILE);
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

    #[test]
    fn apply_and_undo() {
        let dir = std::env::temp_dir().join(format!("organizer_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for name in ["a.png", "b.pdf"] {
            fs::write(dir.join(name), name).unwrap();
        }
        let found = scan(&dir).unwrap();
        let moves = plan(&found, &all(), &existing_targets(&dir));
        let (done, errors) = apply(&dir, &moves);
        assert!(errors.is_empty());
        assert_eq!(done.len(), 2);
        assert!(dir.join("Images/a.png").exists());
        assert!(scan(&dir).unwrap().is_empty());

        assert!(undo(&dir, &done).is_empty());
        assert!(dir.join("a.png").exists());
        assert!(!dir.join("Images").exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
@@text
If you're continuing with your own project, copy everything from `/// Does the moves.` to the end of the undo log section, and the `apply_and_undo` test, from this step's `organizer.rs` into yours.

## Moving a file

`fs::rename(from, to)` moves a file. On the same drive it's instant, however big the file, because only the folder entry changes, not the data. Two things to know:

- The target **folder must exist** first: `fs::create_dir_all(parent)`.
- On Windows, `rename` fails if the target exists. On Linux and macOS it silently **replaces** it! Never rely on that difference: check `to.exists()` yourself first.

## Your tasks

**1. `apply(dir, moves) -> (Vec<Move>, Vec<String>)`** does every move and returns two lists: the moves that **worked** (that's what goes in the undo log) and error messages for the ones that didn't. For each move:
- build the full paths: `dir.join(&m.from)` and `dir.join(&m.to)`,
- if the target exists, push an error like `"cat.png: Images/cat.png already exists, skipped"` and `continue`. (The plan avoided this, but files can appear between planning and applying; never trust that the world stood still.)
- create the target's parent folder, then rename. Combining two `io::Result`s: `fs::create_dir_all(parent).and_then(|_| fs::rename(&from, &to))` runs the rename only if creating the folder worked. `to.parent()` returns an `Option`; a `match` covers both cases.
- on success push `m.clone()` to the done list, on failure push an error message with the file name and the error.

The important design choice: **one failure doesn't stop the rest.** If a file is open in another program, the other 200 files still get organized, and the user gets a clear list of what didn't work.

**2. `undo(dir, moves) -> Vec<String>`** moves files back and returns the errors:
- go through the moves **in reverse order** (`moves.iter().rev()`): undoing should mirror doing,
- if the original location is taken again (the user downloaded a new `cat.png` since), don't overwrite it: report and skip,
- otherwise rename from `dir.join(&m.to)` back to `dir.join(&m.from)`,
- finally, try `fs::remove_dir` on every category folder. `remove_dir` only removes **empty** folders and fails otherwise, which is exactly what we want: folders we created are cleaned up, anything with other files in it stays. Ignore its result with `let _ =`.

Run `cargo test`: `apply_and_undo` creates a real temporary folder, organizes it, checks the result, undoes it, and checks again.

## The undo log

The log is already written for you. It's a small struct saved as JSON next to the app:

```rust,norun
pub struct RunLog {
    pub dir: PathBuf,
    pub moves: Vec<Move>,
}
```

It's saved right after a run, so Undo works even after closing and reopening the app. `PathBuf` implements `Serialize`, so serde can save it directly.

## The GUI

Add `last_run: Option<RunLog>` (start it with `organizer::load_log()`) and `errors: Vec<String>` to the app. Then three methods:

```rust,norun
fn organize(&mut self) {
    let Some(dir) = self.scanned.clone() else { return };
    let (done, errors) = organizer::apply(&dir, &self.plan);
    self.message = format!("Moved {} files.", done.len());
    self.errors = errors;
    if !done.is_empty() {
        let log = RunLog { dir, moves: done };
        if let Err(e) = organizer::save_log(&log) {
            self.errors.push(e);
        }
        self.last_run = Some(log);
    }
    self.scan_again();
}

fn undo(&mut self) {
    let Some(log) = self.last_run.take() else { return };
    self.errors = organizer::undo(&log.dir, &log.moves);
    self.message = format!("Moved {} files back.", log.moves.len().saturating_sub(self.errors.len()));
    organizer::clear_log();
    self.scan_again();
}

/// Scan again after changing files, keeping the message.
fn scan_again(&mut self) {
    if self.scanned.is_some() {
        let message = std::mem::take(&mut self.message);
        self.scan();
        self.message = message;
    }
}
```

`organize` uses `self.scanned`, never the text box (remember why from step 2). After moving files, the old plan is out of date, so `scan_again` rescans. `scan` sets its own "Found N files" message, so we save ours first and put it back.

The bottom panel gets the buttons, and a collapsible list of problems:

```rust,norun
fn action_row(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let text = RichText::new(format!("Organize {} files", self.plan.len())).strong().size(16.0);
        if ui.add_enabled(!self.plan.is_empty(), egui::Button::new(text)).clicked() {
            self.organize();
        }
        if let Some(log) = &self.last_run {
            let hover = format!("Moves the {} files of the last run back, in {}", log.moves.len(), log.dir.display());
            if ui.button("Undo last run").on_hover_text(hover).clicked() {
                self.undo();
            }
        }
        ui.label(&self.message);
    });
    if !self.errors.is_empty() {
        egui::CollapsingHeader::new(RichText::new(format!("{} problem(s)", self.errors.len())).color(Color32::YELLOW))
            .show(ui, |ui| {
                for e in &self.errors {
                    ui.label(e);
                }
            });
    }
}
```

Use it in the bottom panel instead of the plain message label.

**Test on `demo_folder` first!** Organize it, look in Explorer, press Undo, look again. Then, when you trust it, try your real Downloads folder. That's how professionals test tools that change things: on throwaway data first.

## The finished app

@@project file_organizer_finished File Organizer (finished)
The complete app, for reference. Created in its own folder, `file_organizer_finished`.
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
//! The window: pick a folder, see the plan, organize, undo.

use crate::organizer::{self, Category, FileInfo, Move, RunLog, Taken};
use eframe::egui::{self, Color32, RichText};
use std::collections::{BTreeMap, HashSet};
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
    last_run: Option<RunLog>,
    message: String,
    errors: Vec<String>,
}

fn downloads_folder() -> String {
    std::env::var("USERPROFILE").map(|home| format!("{home}\\Downloads")).unwrap_or_default()
}

impl OrganizerApp {
    pub fn new() -> Self {
        let mut app = OrganizerApp {
            folder: downloads_folder(),
            scanned: None,
            files: Vec::new(),
            existing: Taken::new(),
            // Everything except "Other": unknown files are often best left alone.
            enabled: Category::ALL.into_iter().filter(|&c| c != Category::Other).collect(),
            plan: Vec::new(),
            last_run: organizer::load_log(),
            message: String::new(),
            errors: Vec::new(),
        };
        // `file_organizer <folder>` opens with that folder scanned.
        if let Some(folder) = std::env::args().nth(1) {
            app.folder = folder;
            app.scan();
        }
        app
    }

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

    fn organize(&mut self) {
        let Some(dir) = self.scanned.clone() else { return };
        let (done, errors) = organizer::apply(&dir, &self.plan);
        self.message = format!("Moved {} files.", done.len());
        self.errors = errors;
        if !done.is_empty() {
            let log = RunLog { dir, moves: done };
            if let Err(e) = organizer::save_log(&log) {
                self.errors.push(e);
            }
            self.last_run = Some(log);
        }
        self.scan_again();
    }

    fn undo(&mut self) {
        let Some(log) = self.last_run.take() else { return };
        self.errors = organizer::undo(&log.dir, &log.moves);
        self.message = format!("Moved {} files back.", log.moves.len().saturating_sub(self.errors.len()));
        organizer::clear_log();
        self.scan_again();
    }

    /// Scan again after changing files, keeping the message.
    fn scan_again(&mut self) {
        if self.scanned.is_some() {
            let message = std::mem::take(&mut self.message);
            self.scan();
            self.message = message;
        }
    }

    // ---------- drawing ----------

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

    fn action_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let text = RichText::new(format!("Organize {} files", self.plan.len())).strong().size(16.0);
            if ui.add_enabled(!self.plan.is_empty(), egui::Button::new(text)).clicked() {
                self.organize();
            }
            if let Some(log) = &self.last_run {
                let hover = format!("Moves the {} files of the last run back, in {}", log.moves.len(), log.dir.display());
                if ui.button("Undo last run").on_hover_text(hover).clicked() {
                    self.undo();
                }
            }
            ui.label(&self.message);
        });
        if !self.errors.is_empty() {
            egui::CollapsingHeader::new(RichText::new(format!("{} problem(s)", self.errors.len())).color(Color32::YELLOW))
                .show(ui, |ui| {
                    for e in &self.errors {
                        ui.label(e);
                    }
                });
        }
    }
}

impl eframe::App for OrganizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(6.0);
            self.folder_row(ui);
            self.category_row(ui);
            ui.add_space(2.0);
        });
        egui::Panel::bottom("actions").show(ui, |ui| {
            ui.add_space(6.0);
            self.action_row(ui);
            ui.add_space(4.0);
        });
        egui::CentralPanel::default().show(ui, |ui| self.preview(ui));
    }
}

@@file src/organizer.rs
//! Planning and doing the tidy-up. No GUI code here.
//!
//! The big idea: first make a **plan** (a list of moves) without touching anything,
//! show it to the user, and only then **apply** it, keeping a log so it can be undone.

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

/// Does the moves. Never overwrites: a move whose target exists is skipped with an error.
/// Returns the moves that worked (for the undo log) and messages for those that didn't.
pub fn apply(dir: &Path, moves: &[Move]) -> (Vec<Move>, Vec<String>) {
    let mut done = Vec::new();
    let mut errors = Vec::new();
    for m in moves {
        let from = dir.join(&m.from);
        let to = dir.join(&m.to);
        if to.exists() {
            errors.push(format!("{}: {} already exists, skipped", m.from, m.to));
            continue;
        }
        let result = match to.parent() {
            Some(parent) => fs::create_dir_all(parent).and_then(|_| fs::rename(&from, &to)),
            None => fs::rename(&from, &to),
        };
        match result {
            Ok(()) => done.push(m.clone()),
            Err(e) => errors.push(format!("{}: {e}", m.from)),
        }
    }
    (done, errors)
}

/// Moves files back, newest move first. Returns messages for moves that couldn't be undone.
pub fn undo(dir: &Path, moves: &[Move]) -> Vec<String> {
    let mut errors = Vec::new();
    for m in moves.iter().rev() {
        let back = dir.join(&m.from);
        if back.exists() {
            errors.push(format!("{}: a file with that name is back in the folder, skipped", m.from));
            continue;
        }
        if let Err(e) = fs::rename(dir.join(&m.to), back) {
            errors.push(format!("{}: {e}", m.to));
        }
    }
    // Remove category folders that are now empty (remove_dir fails if they're not: good).
    for category in Category::ALL {
        let _ = fs::remove_dir(dir.join(category.folder()));
    }
    errors
}

// ---------- the undo log ----------

const LOG_FILE: &str = "last_run.json";

/// What the last run did, saved so it can be undone even after a restart.
#[derive(Serialize, Deserialize)]
pub struct RunLog {
    pub dir: PathBuf,
    pub moves: Vec<Move>,
}

pub fn save_log(log: &RunLog) -> Result<(), String> {
    let json = serde_json::to_string_pretty(log).map_err(|e| e.to_string())?;
    fs::write(LOG_FILE, json).map_err(|e| format!("Couldn't save the undo log: {e}"))
}

pub fn load_log() -> Option<RunLog> {
    let text = fs::read_to_string(LOG_FILE).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn clear_log() {
    let _ = fs::remove_file(LOG_FILE);
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

    #[test]
    fn apply_and_undo() {
        let dir = std::env::temp_dir().join(format!("organizer_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        for name in ["a.png", "b.pdf"] {
            fs::write(dir.join(name), name).unwrap();
        }
        let found = scan(&dir).unwrap();
        let moves = plan(&found, &all(), &existing_targets(&dir));
        let (done, errors) = apply(&dir, &moves);
        assert!(errors.is_empty());
        assert_eq!(done.len(), 2);
        assert!(dir.join("Images/a.png").exists());
        assert!(scan(&dir).unwrap().is_empty());

        assert!(undo(&dir, &done).is_empty());
        assert!(dir.join("a.png").exists());
        assert!(!dir.join("Images").exists());
        fs::remove_dir_all(&dir).unwrap();
    }
}
@@text
## Make it yours

- **Your own rules:** let the user edit which extensions go where (a `HashMap<String, Category>` saved as JSON), or add a category like "eBooks" (`epub`, `mobi`).
- **Sort by date:** an option to sort into `2026/2026-10` folders using `fs::metadata(...)?.modified()`.
- **Bulk renamer:** a second mode that renames files with a pattern like `Holiday {n}` → `Holiday 1.jpg`, `Holiday 2.jpg`, … with the same preview-then-apply design and undo.
- **Find duplicates:** files with the same size are *candidates*; compare their contents (`fs::read`) to be sure. Show them in groups.
- **Old files:** a filter that only organizes files older than N days, leaving recent downloads where they are.
- **Scan in the background:** scanning a folder with 100,000 files freezes the window for a moment. Move `scan` to a worker thread with a channel, as in "Timers and Background Work".

@@quiz
`apply` hits an error on the 5th of 200 files. What does it do?
- Stops and undoes the first 4 moves
- Panics
* Records the error and carries on with the rest, returning the successful moves and the error messages separately
? One locked file shouldn't block 199 others, and the user gets a clear report.

@@quiz
Why does `undo` go through the moves in reverse order?
- It's faster
* Undoing should mirror doing: if one move depended on an earlier one, the later one must be undone first
- `iter()` only works backwards on `Vec<Move>`
? Like taking off clothes: last on, first off.

@@quiz
Why is the undo log saved to a file and not just kept in memory?
* So the user can still undo after closing and reopening the app
- Because `Vec<Move>` is too large for memory
- serde requires it
? An undo that disappears when you close the window isn't much of a safety net.
