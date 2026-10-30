@@lesson g-nv2 Note Vault 2: List, Editor and Autosave
@@text
Now the window: a list of notes on the left, the open note on the right, a status bar at the bottom. You'll write `src/app.rs` in four parts.

If you're continuing from step 1, press **Add 1 new file(s)** below to get `src/app.rs`, and copy `src/main.rs` from the starting files (it now opens a window). Or reset to this step's files for a clean start, with a finished `vault.rs`.

@@project note_vault Note Vault, step 2: list, editor and autosave
Adds `src/app.rs`, a bare window to build on. `src/main.rs` changes too: it opens the window.
@@file Cargo.toml
[package]
name = "note_vault"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
egui_commonmark = { version = "0.26", default-features = false, features = ["pulldown_cmark"] }

@@file src/main.rs
mod app;
mod vault;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([560.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Note Vault",
        options,
        Box::new(|_cc| Ok(Box::new(app::NoteApp::new()))),
    )
}

@@file src/app.rs
//! The window. In this step you build it up piece by piece, following the lesson.

use crate::vault::Vault;
use eframe::egui;
use std::path::Path;

const NOTES_DIR: &str = "notes";

pub struct NoteApp {
    vault: Option<Vault>,
    /// Index into `vault.notes` of the open note.
    selected: Option<usize>,
    status: String,
}

impl NoteApp {
    pub fn new() -> Self {
        let (vault, status) = match Vault::open(Path::new(NOTES_DIR)) {
            Ok(v) => (Some(v), String::new()),
            Err(e) => (None, format!("Couldn't open the notes folder: {e}")),
        };
        NoteApp { vault, selected: None, status }
    }
}

impl eframe::App for NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(&self.status);
            let Some(vault) = &self.vault else { return };
            for note in &vault.notes {
                ui.label(&note.name);
            }
            let _ = self.selected; // used from Part 1 on
        });
    }
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
    let name = name.trim();
    if name.is_empty() {
        return Err("The name can't be empty.".to_string());
    }
    if name.chars().count() > 80 {
        return Err("That name is too long (80 characters at most).".to_string());
    }
    if let Some(c) = name.chars().find(|c| r#"\/:*?"<>|"#.contains(*c)) {
        return Err(format!("A name can't contain '{c}'."));
    }
    if name.starts_with('.') {
        return Err("A name can't start with a dot.".to_string());
    }
    Ok(name.to_string())
}

/// All #tags in a text, in lowercase, without the #. A tag is a # directly followed by
/// letters, digits, - or _. So `# Heading` (with a space) isn't a tag, and neither is `##`.
pub fn tags(body: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for word in body.split_whitespace() {
        let Some(rest) = word.strip_prefix('#') else {
            continue;
        };
        let tag: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        if !tag.is_empty() {
            found.insert(tag.to_lowercase());
        }
    }
    found
}

/// Does the note match the search? Every word of the query must appear in the name or
/// the body (ignoring case). A word starting with # must be one of the note's tags.
pub fn matches(note: &Note, query: &str) -> bool {
    let name = note.name.to_lowercase();
    let body = note.body.to_lowercase();
    let note_tags = tags(&note.body);
    query.split_whitespace().all(|word| {
        let word = word.to_lowercase();
        match word.strip_prefix('#') {
            Some(tag) => note_tags.contains(tag),
            None => name.contains(&word) || body.contains(&word),
        }
    })
}

/// Every tag used in any note, with how many notes use it.
pub fn all_tags(notes: &[Note]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for note in notes {
        for tag in tags(&note.body) {
            *counts.entry(tag).or_insert(0) += 1;
        }
    }
    counts
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
Run it: a plain list of note names. (No notes? Create the `notes` folder with a few `.md` files, as in step 1.)

## Part 1: panels and the note list

Three panels: the status bar at the bottom, the list on the left, the note in the middle. **Order matters**: side and bottom panels must be added *before* the `CentralPanel`, which takes whatever space is left. Replace `ui()` with:

```rust,norun
fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
    egui::Panel::left("notes")
        .resizable(true)
        .default_size(260.0)
        .min_size(180.0)
        .show(ui, |ui| self.sidebar(ui));
    egui::CentralPanel::default().show(ui, |ui| self.note_view(ui));

    // Apply the click, now that nothing is borrowed any more.
    if let Some(i) = self.pending_select.take() {
        self.selected = Some(i);
    }
}
```

Add the fields `search: String` and `pending_select: Option<usize>` to `NoteApp` (with `String::new()` and `None` in `new`).

Why `pending_select`? While drawing the list, we borrow `vault.notes` to loop over it, so we can't call methods that need `&mut self` in the middle of the loop. Instead we **remember** what was clicked and act on it afterwards: the same "action" pattern as in the Shopping List and Pocket Ledger.

The sidebar: a search box, then the matching notes. Each note shows its name and a snippet of its first line:

```rust,norun
fn sidebar(&mut self, ui: &mut egui::Ui) {
    let Some(vault) = &mut self.vault else { return };
    ui.add_space(6.0);
    ui.add(
        egui::TextEdit::singleline(&mut self.search)
            .hint_text("Search… (or #tag)")
            .desired_width(f32::INFINITY),
    );
    ui.separator();

    let mut clicked = None;
    egui::ScrollArea::vertical().show(ui, |ui| {
        // "Justified": every item as wide as the list, with its text on the left.
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            let mut shown = 0;
            for (i, note) in vault.notes.iter().enumerate() {
                if !vault::matches(note, &self.search) {
                    continue;
                }
                shown += 1;
                let selected = self.selected == Some(i);
                let text = format!("{}\n{}", note.name, vault::snippet(&note.body, 40));
                if ui.selectable_label(selected, text).clicked() {
                    clicked = Some(i);
                }
            }
            if shown == 0 {
                ui.label(RichText::new("No notes match.").weak());
            }
        });
    });
    if let Some(i) = clicked {
        self.pending_select = Some(i);
    }
}
```

Your `matches` from step 1 does all the searching work: the GUI just asks it about each note. Change the imports to `use crate::vault::{self, Vault};` and `use eframe::egui::{self, Color32, RichText};`.

For now, make `note_view` show the selected note's name as a heading and its text as a label, and `status_bar` show `self.status`. Run it: click around, and try the search.

## Part 2: the editor and the preview

An enum for the three ways to show a note:

```rust,norun
#[derive(PartialEq, Clone, Copy)]
enum View {
    Edit,
    Split,
    Preview,
}
```

Add fields `view: View` (start with `View::Split`) and `md_cache: CommonMarkCache` (start with `CommonMarkCache::default()`; the Markdown viewer uses it to remember things between frames), and `use egui_commonmark::{CommonMarkCache, CommonMarkViewer};`.

Then `note_view`. If no note is selected, show a hint. Otherwise a title row with the three view buttons on the right, and below it the editor, the preview, or both side by side with `ui.columns(2, ...)`:

```rust,norun
fn note_view(&mut self, ui: &mut egui::Ui) {
    let (Some(vault), Some(i)) = (&mut self.vault, self.selected) else {
        ui.centered_and_justified(|ui| {
            ui.label(RichText::new("Pick a note on the left.").weak())
        });
        return;
    };

    // Title row
    ui.horizontal(|ui| {
        ui.heading(&vault.notes[i].name);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.selectable_value(&mut self.view, View::Preview, "Preview");
            ui.selectable_value(&mut self.view, View::Split, "Split");
            ui.selectable_value(&mut self.view, View::Edit, "Edit");
        });
    });
    ui.separator();

    let note = &mut vault.notes[i];
    let editor = |ui: &mut egui::Ui, body: &mut String| {
        egui::ScrollArea::vertical()
            .id_salt("editor")
            .show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(body)
                        .hint_text("Write here…")
                        .lock_focus(true),
                )
                .changed()
            })
            .inner
    };
    let mut changed = false;
    match self.view {
        View::Edit => changed = editor(ui, &mut note.body),
        View::Preview => {
            egui::ScrollArea::vertical()
                .id_salt("preview")
                .show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut self.md_cache, &note.body);
                });
        }
        View::Split => {
            ui.columns(2, |cols| {
                changed = editor(&mut cols[0], &mut note.body);
                egui::ScrollArea::vertical()
                    .id_salt("preview")
                    .show(&mut cols[1], |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.md_cache, &note.body);
                    });
            });
        }
    }
    // Part 3 goes here
}
```

A few details worth noticing:

- `let (Some(vault), Some(i)) = (...) else { ... };` checks **two** `Option`s at once by matching a tuple.
- The `editor` closure is used in two places, so it's written once. It returns whether the text changed: `ScrollArea::show` returns an `InnerResponse`, and `.inner` is whatever our closure returned.
- `lock_focus(true)` keeps the Tab key inside the editor (for indenting) instead of jumping to the next widget.
- `ui.columns(2, |cols| ...)` splits the space into two equal columns, `cols[0]` and `cols[1]`.

Run it, and write some Markdown in Split view: `# Heading`, `**bold**`, `- lists`, code in backticks. The preview follows as you type.

## Part 3: autosave

Edits only change the `String` in memory; nothing is saved yet. When should we save? On **every keystroke** would mean writing the file 5 times a second while typing. Common practice is a **debounce**: save once the user has paused for a moment. We remember **when** the first unsaved change happened:

```rust,norun
/// When the open note was last changed and not yet saved.
unsaved_since: Option<Instant>,
```

At the end of `note_view`, where it says Part 3:

```rust,norun
if changed && self.unsaved_since.is_none() {
    self.unsaved_since = Some(Instant::now());
}
```

Then two methods. `save_now` saves if there's anything unsaved. `take()` reads the `Option` and sets it to `None` in one go:

```rust,norun
fn save_now(&mut self) {
    if self.unsaved_since.take().is_none() {
        return;
    }
    if let (Some(vault), Some(i)) = (&mut self.vault, self.selected)
        && let Err(e) = vault.save(i)
    {
        self.status = format!("Couldn't save: {e}");
    }
}

/// Autosave: one second after the last keystroke, not on every key.
fn autosave(&mut self, ctx: &egui::Context) {
    if let Some(since) = self.unsaved_since {
        let wait = Duration::from_secs(1);
        if since.elapsed() >= wait {
            self.save_now();
        } else {
            ctx.request_repaint_after(wait - since.elapsed());
        }
    }
}
```

The `request_repaint_after` matters: if you type a word and then stop moving the mouse, egui would sleep and never get round to saving. This wakes it up exactly when the second is over.

Call `self.autosave(&ui.ctx().clone());` at the top of `ui()`.

## Part 4: never lose a word

Two more moments when we must save:

1. **Switching notes**: otherwise the change is saved to the *wrong* note's index, or lost. Write a `select` method that calls `self.save_now()` first and then changes `self.selected` (only to an index that exists: `index.filter(|&i| i < vault.notes.len())`). Use it in `ui()` instead of setting `self.selected` directly.
2. **Closing the app** in the second before autosave. eframe calls `on_exit` when the window closes. Add it to `impl eframe::App for NoteApp`:

```rust,norun
fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    self.save_now();
}
```

Finally the status bar: the word count (`body.split_whitespace().count()`), then "Saving…" while `unsaved_since` is `Some`, otherwise "Saved", then the status message in yellow if there is one. Separate them with `ui.separator()`.

Run it, edit a note, wait a second, and open the `.md` file in VS Code: your change is there. Change something in VS Code, save, and restart Note Vault: it's there too. That's the beauty of plain files.

## The tag cloud

One last thing for this step. Above the separator in `sidebar`, show every tag as a clickable label. Clicking a tag searches for it, and clicking it again clears the search:

```rust,norun
let tags = vault::all_tags(&vault.notes);
if !tags.is_empty() {
    ui.horizontal_wrapped(|ui| {
        for (tag, count) in &tags {
            let query = format!("#{tag}");
            let active = self.search.trim() == query;
            if ui
                .selectable_label(active, format!("#{tag} ({count})"))
                .clicked()
            {
                self.search = if active { String::new() } else { query };
            }
        }
    });
}
```

`horizontal_wrapped` lays the tags out like words in a paragraph, wrapping to the next line when the row is full.

@@quiz
Why does Note Vault save one second after the last change, instead of on every keystroke?
- Saving on every keystroke isn't possible in egui
* To avoid writing the file many times a second while the user types
- Because files can only be written once per second
? This is called debouncing: wait for a pause, then act once.

@@quiz
The sidebar stores the clicked note in `pending_select` instead of opening it right away. Why?
- egui doesn't allow changing state while drawing
* While looping over `vault.notes`, it's borrowed, so we can't call a `&mut self` method like `select` in the loop
- It makes the app faster
? The action pattern: remember what to do, and do it when the borrow is over.
