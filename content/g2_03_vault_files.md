@@lesson g-nv3 Note Vault 3: New, Rename and Safe Delete
@@text
So far, notes can only be created outside the app. In this last step you'll add **creating**, **renaming** and **deleting** notes. These change files on the user's disk, so this is where careful code matters most: a bug here doesn't just show something wrong, it can **lose someone's writing**. Three rules guide the design:

1. **Check before acting.** Validate the name, and make sure we won't overwrite another note.
2. **Report every failure** to the user, never ignore it, and never crash.
3. **Make destructive actions reversible.** "Delete" moves the note to a `.trash` folder instead of really deleting it.

@@project note_vault Note Vault, step 3: new, rename and delete
`src/vault.rs` gets three new methods for you to write: `create`, `rename` and `delete`. The starting `src/app.rs` is the finished window from step 2.
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
//! The window: note list on the left, the selected note on the right.

use crate::vault::{self, Vault};
use eframe::egui::{self, Color32, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::path::Path;
use std::time::{Duration, Instant};

const NOTES_DIR: &str = "notes";

#[derive(PartialEq, Clone, Copy)]
enum View {
    Edit,
    Split,
    Preview,
}

pub struct NoteApp {
    vault: Option<Vault>,
    /// Index into `vault.notes` of the open note.
    selected: Option<usize>,
    search: String,
    view: View,
    md_cache: CommonMarkCache,
    /// When the open note was last changed and not yet saved.
    unsaved_since: Option<Instant>,
    status: String,
    /// A click to apply after drawing, when nothing is borrowed any more.
    pending_select: Option<usize>,
}

impl NoteApp {
    pub fn new() -> Self {
        let (vault, status) = match Vault::open(Path::new(NOTES_DIR)) {
            Ok(v) => (Some(v), String::new()),
            Err(e) => (None, format!("Couldn't open the notes folder: {e}")),
        };
        let mut app = NoteApp {
            vault,
            selected: None,
            search: String::new(),
            view: View::Split,
            md_cache: CommonMarkCache::default(),
            unsaved_since: None,
            status,
            pending_select: None,
        };
        app.select(Some(0));
        app
    }

    /// Opens a note (saving the current one first).
    fn select(&mut self, index: Option<usize>) {
        self.save_now();
        let Some(vault) = &self.vault else { return };
        self.selected = index.filter(|&i| i < vault.notes.len());
    }

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

    // ---------- left side ----------

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let Some(vault) = &mut self.vault else { return };
        ui.add_space(6.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text("Search… (or #tag)")
                .desired_width(f32::INFINITY),
        );

        // Tag cloud: click to search for a tag, click again to clear.
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

    // ---------- right side ----------

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
        if changed && self.unsaved_since.is_none() {
            self.unsaved_since = Some(Instant::now());
        }
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let (Some(vault), Some(i)) = (&self.vault, self.selected) {
                let words = vault.notes[i].body.split_whitespace().count();
                ui.label(format!("{words} words"));
                ui.separator();
                ui.label(if self.unsaved_since.is_some() {
                    "Saving…"
                } else {
                    "Saved"
                });
                ui.separator();
            }
            if !self.status.is_empty() {
                ui.colored_label(Color32::YELLOW, &self.status);
            }
        });
    }
}

impl eframe::App for NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.autosave(&ctx);

        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("notes")
            .resizable(true)
            .default_size(260.0)
            .min_size(180.0)
            .show(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default().show(ui, |ui| self.note_view(ui));

        // Apply the click, now that nothing is borrowed any more.
        if let Some(i) = self.pending_select.take() {
            self.select(Some(i));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_now();
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

    /// Creates a new, empty note at the top of the list and returns its index (0).
    pub fn create(&mut self, name: &str) -> Result<usize, String> {
        todo!()
    }

    /// Renames a note and its file.
    pub fn rename(&mut self, index: usize, new_name: &str) -> Result<(), String> {
        todo!()
    }

    /// Moves a note's file to a `.trash` folder (never deletes it for real), and
    /// removes it from the list.
    pub fn delete(&mut self, index: usize) -> io::Result<()> {
        todo!()
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

        vault.create("First").unwrap();
        assert!(vault.create("first").is_err()); // no duplicates, whatever the case
        vault.notes[0].body.push_str("Hello #test");
        vault.save(0).unwrap();
        vault.rename(0, "Renamed").unwrap();
        assert!(dir.join("Renamed.md").exists());
        assert!(!dir.join("First.md").exists());

        let reopened = Vault::open(&dir).unwrap();
        assert_eq!(reopened.notes.len(), 1);
        assert!(reopened.notes[0].body.contains("#test"));

        vault.delete(0).unwrap();
        assert!(vault.notes.is_empty());
        assert!(!dir.join("Renamed.md").exists());
        assert_eq!(fs::read_dir(dir.join(".trash")).unwrap().count(), 1);
        fs::remove_dir_all(&dir).unwrap();
    }
}
@@text
If you're continuing with your own project, copy the three new methods (`create`, `rename`, `delete`, with their `todo!()`s) and the new `files` test from this step's `vault.rs` into yours. They also need `find` and `path_of`, which were already there.

## Your tasks in `vault.rs`

The `files` test at the bottom tries everything on a real, temporary folder (from `std::env::temp_dir()`), so run `cargo test` as you go.

**1. `create(name)`** makes a new, empty note and returns its index:
- check the name with `valid_name(name)?`. The `?` works because both return `Result<_, String>`.
- if `self.find(&name)` finds a note with that name already, return an `Err` with a friendly message. (`find` ignores upper/lower case, because Windows file names do too: `Ideas.md` and `ideas.md` are the same file!)
- the new body is `format!("# {name}\n\n")`, a heading, so the note starts with its title.
- write the file with `fs::write(self.path_of(&name), &body)`. Its error is an `io::Error`, not a `String`, so convert it: `.map_err(|e| format!("Couldn't create the file: {e}"))?`.
- `insert` the new `Note` at index 0 (it's the newest), with `modified: SystemTime::now()`, and return `Ok(0)`.

**2. `rename(index, new_name)`**:
- check the new name with `valid_name`,
- if it's the same as the old name, there's nothing to do: `return Ok(())`,
- if **another** note already has that name, return an error. Careful: renaming "ideas" to "Ideas" finds the note *itself*, which is fine. `self.find(&new_name).is_some_and(|i| i != index)` says exactly "found, and it's a different note",
- rename the file with `fs::rename(old_path, new_path)` (convert its error, as above), and only **after** that succeeded, change `name` in the list. If the rename fails, the list still matches the disk.

**3. `delete(index)`** moves the file to the trash instead of deleting it:
- the trash is `self.dir.join(".trash")`; create it with `fs::create_dir_all` (which is fine if it already exists),
- the file goes to `.trash/<name> (<timestamp>).md`, so deleting two notes with the same name over time never overwrites the first one. A timestamp is `SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_or(0, |d| d.as_secs())`,
- `fs::rename` it there, then `remove` it from `self.notes`.

Note the order in all three: **first** the disk, **then** the list, and only if the disk part worked.

## The GUI

Three additions to `app.rs`.

**New note**, in the sidebar, under the tags. A text box and an Add button (Enter works too). Add a `new_name: String` field:

```rust,norun
ui.horizontal(|ui| {
    let response = ui.add(
        egui::TextEdit::singleline(&mut self.new_name)
            .hint_text("New note name")
            .desired_width(ui.available_width() - 50.0),
    );
    let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
    if (ui.button("Add").clicked() || enter) && !self.new_name.trim().is_empty() {
        match vault.create(&self.new_name) {
            Ok(i) => {
                self.new_name.clear();
                self.search.clear();
                // The new note is inserted at 0, so the open note moved down one.
                self.selected = self.selected.map(|s| s + 1);
                self.pending_select = Some(i);
            }
            Err(e) => self.status = e,
        }
    }
});
```

That `self.selected.map(|s| s + 1)` fixes a sneaky bug. Inserting at index 0 shifts every other note down by one. The open note was, say, index 2; now it's 3. If we didn't update `selected`, then `select` would save the open note's unsaved text into **the wrong note**! Indices into a list that changes are fragile; whenever you insert or remove, think about which indices moved.

**Rename**: replace the heading in the title row with an editable text box. Add a `rename_to: String` field, and set it to the note's name in `select` (so it shows the right name when you switch notes):

```rust,norun
let response = ui.add(
    egui::TextEdit::singleline(&mut self.rename_to)
        .font(egui::TextStyle::Heading)
        .desired_width(260.0),
);
let changed = self.rename_to.trim() != vault.notes[i].name;
let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
if changed && (ui.button("Rename").clicked() || enter) {
    match vault.rename(i, &self.rename_to) {
        Ok(()) => self.status = String::new(),
        Err(e) => self.status = e,
    }
}
```

The Rename button only appears once the name has actually been changed.

**Delete, with confirmation.** A two-click button: the first click arms it ("Really delete?"), the second does it. Add `confirm_delete: bool` and `pending_delete: bool` fields, and reset `confirm_delete` to `false` in `select`. In the title row's right-to-left part, before the view buttons:

```rust,norun
let text = if self.confirm_delete { "Really delete?" } else { "Delete" };
if ui.button(RichText::new(text).color(Color32::LIGHT_RED)).clicked() {
    if self.confirm_delete {
        self.pending_delete = true;
    }
    self.confirm_delete = !self.confirm_delete;
}
ui.separator();
```

and at the end of `ui()`, after handling `pending_select`:

```rust,norun
if std::mem::take(&mut self.pending_delete)
    && let (Some(vault), Some(i)) = (&mut self.vault, self.selected)
{
    self.unsaved_since = None;
    match vault.delete(i) {
        Ok(()) => self.status = "Moved to the notes/.trash folder.".to_string(),
        Err(e) => self.status = format!("Couldn't delete: {e}"),
    }
    self.selected = None;
    self.select(Some(0));
}
```

`std::mem::take` reads the `bool` and resets it to `false` in one go (like `Option::take`). `selected` is set to `None` before `select`, because the deleted note's index now belongs to a different note.

**A welcome note.** An empty app is confusing on first start. In `new()`, if the vault has no notes, create a "Welcome" note explaining how the app works (see the finished version below for one).

## The finished app

@@project note_vault_finished Note Vault (finished)
The complete app, for reference. Created in its own folder, `note_vault_finished`.
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
//! The window: note list on the left, the selected note on the right.

use crate::vault::{self, Vault};
use eframe::egui::{self, Color32, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::path::Path;
use std::time::{Duration, Instant};

const NOTES_DIR: &str = "notes";
const WELCOME: &str = "# Welcome to Note Vault\n\nEvery note is a plain **Markdown** file in the `notes` folder.\n\n- Write on the left, see the preview on the right.\n- Add #tags anywhere, then click a tag to filter.\n- Search finds words in titles and text; `#tag` searches tags.\n\nChanges are saved automatically. #help\n";

#[derive(PartialEq, Clone, Copy)]
enum View {
    Edit,
    Split,
    Preview,
}

pub struct NoteApp {
    vault: Option<Vault>,
    /// Index into `vault.notes` of the open note.
    selected: Option<usize>,
    search: String,
    view: View,
    md_cache: CommonMarkCache,
    /// When the open note was last changed and not yet saved.
    unsaved_since: Option<Instant>,
    new_name: String,
    rename_to: String,
    confirm_delete: bool,
    status: String,
    /// Clicks to apply after drawing, when nothing is borrowed any more.
    pending_select: Option<usize>,
    pending_delete: bool,
}

impl NoteApp {
    pub fn new() -> Self {
        let (vault, status) = match Vault::open(Path::new(NOTES_DIR)) {
            Ok(v) => (Some(v), String::new()),
            Err(e) => (None, format!("Couldn't open the notes folder: {e}")),
        };
        let mut app = NoteApp {
            vault,
            selected: None,
            search: String::new(),
            view: View::Split,
            md_cache: CommonMarkCache::default(),
            unsaved_since: None,
            new_name: String::new(),
            rename_to: String::new(),
            confirm_delete: false,
            status,
            pending_select: None,
            pending_delete: false,
        };
        if let Some(vault) = &mut app.vault
            && vault.notes.is_empty()
            && vault.create("Welcome").is_ok()
        {
            vault.notes[0].body = WELCOME.to_string();
            let _ = vault.save(0);
        }
        app.select(Some(0));
        app
    }

    /// Opens a note (saving the current one first).
    fn select(&mut self, index: Option<usize>) {
        self.save_now();
        let Some(vault) = &self.vault else { return };
        self.selected = index.filter(|&i| i < vault.notes.len());
        self.rename_to = self
            .selected
            .map(|i| vault.notes[i].name.clone())
            .unwrap_or_default();
        self.confirm_delete = false;
    }

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

    // ---------- left side ----------

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let Some(vault) = &mut self.vault else { return };
        ui.add_space(6.0);
        ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .hint_text("Search… (or #tag)")
                .desired_width(f32::INFINITY),
        );

        // Tag cloud: click to search for a tag, click again to clear.
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

        // New note
        ui.horizontal(|ui| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.new_name)
                    .hint_text("New note name")
                    .desired_width(ui.available_width() - 50.0),
            );
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if (ui.button("Add").clicked() || enter) && !self.new_name.trim().is_empty() {
                match vault.create(&self.new_name) {
                    Ok(i) => {
                        self.new_name.clear();
                        self.search.clear();
                        // The new note is inserted at 0, so the open note moved down one.
                        self.selected = self.selected.map(|s| s + 1);
                        self.pending_select = Some(i);
                    }
                    Err(e) => self.status = e,
                }
            }
        });
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

    // ---------- right side ----------

    fn note_view(&mut self, ui: &mut egui::Ui) {
        let (Some(vault), Some(i)) = (&mut self.vault, self.selected) else {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Pick a note on the left, or make a new one.").weak())
            });
            return;
        };

        // Title row: rename and delete.
        ui.horizontal(|ui| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.rename_to)
                    .font(egui::TextStyle::Heading)
                    .desired_width(260.0),
            );
            let changed = self.rename_to.trim() != vault.notes[i].name;
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if changed && (ui.button("Rename").clicked() || enter) {
                match vault.rename(i, &self.rename_to) {
                    Ok(()) => self.status = String::new(),
                    Err(e) => self.status = e,
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let text = if self.confirm_delete {
                    "Really delete?"
                } else {
                    "Delete"
                };
                if ui
                    .button(RichText::new(text).color(Color32::LIGHT_RED))
                    .clicked()
                {
                    if self.confirm_delete {
                        self.pending_delete = true;
                    }
                    self.confirm_delete = !self.confirm_delete;
                }
                ui.separator();
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
        if changed && self.unsaved_since.is_none() {
            self.unsaved_since = Some(Instant::now());
        }
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let (Some(vault), Some(i)) = (&self.vault, self.selected) {
                let words = vault.notes[i].body.split_whitespace().count();
                ui.label(format!("{words} words"));
                ui.separator();
                ui.label(if self.unsaved_since.is_some() {
                    "Saving…"
                } else {
                    "Saved"
                });
                ui.separator();
            }
            if !self.status.is_empty() {
                ui.colored_label(Color32::YELLOW, &self.status);
            }
        });
    }
}

impl eframe::App for NoteApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.autosave(&ctx);

        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::Panel::left("notes")
            .resizable(true)
            .default_size(260.0)
            .min_size(180.0)
            .show(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default().show(ui, |ui| self.note_view(ui));

        // Apply what was clicked, now that nothing is borrowed any more.
        if let Some(i) = self.pending_select.take() {
            self.select(Some(i));
        }
        if std::mem::take(&mut self.pending_delete)
            && let (Some(vault), Some(i)) = (&mut self.vault, self.selected)
        {
            self.unsaved_since = None;
            match vault.delete(i) {
                Ok(()) => self.status = "Moved to the notes/.trash folder.".to_string(),
                Err(e) => self.status = format!("Couldn't delete: {e}"),
            }
            self.selected = None;
            self.select(Some(0));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_now();
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

    /// Creates a new, empty note at the top of the list and returns its index (0).
    pub fn create(&mut self, name: &str) -> Result<usize, String> {
        let name = valid_name(name)?;
        if self.find(&name).is_some() {
            return Err(format!("There's already a note called \"{name}\"."));
        }
        let body = format!("# {name}\n\n");
        fs::write(self.path_of(&name), &body)
            .map_err(|e| format!("Couldn't create the file: {e}"))?;
        self.notes.insert(
            0,
            Note {
                name,
                body,
                modified: SystemTime::now(),
            },
        );
        Ok(0)
    }

    /// Renames a note and its file.
    pub fn rename(&mut self, index: usize, new_name: &str) -> Result<(), String> {
        let new_name = valid_name(new_name)?;
        let old_name = &self.notes[index].name;
        if *old_name == new_name {
            return Ok(());
        }
        // A different note with that name? (Same note, different capitals, is fine.)
        if self.find(&new_name).is_some_and(|i| i != index) {
            return Err(format!("There's already a note called \"{new_name}\"."));
        }
        fs::rename(self.path_of(old_name), self.path_of(&new_name))
            .map_err(|e| format!("Couldn't rename: {e}"))?;
        self.notes[index].name = new_name;
        Ok(())
    }

    /// Moves a note's file to a `.trash` folder (never deletes it for real), and
    /// removes it from the list.
    pub fn delete(&mut self, index: usize) -> io::Result<()> {
        let trash = self.dir.join(".trash");
        fs::create_dir_all(&trash)?;
        let name = &self.notes[index].name;
        let stamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        fs::rename(
            self.path_of(name),
            trash.join(format!("{name} ({stamp}).md")),
        )?;
        self.notes.remove(index);
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

        vault.create("First").unwrap();
        assert!(vault.create("first").is_err()); // no duplicates, whatever the case
        vault.notes[0].body.push_str("Hello #test");
        vault.save(0).unwrap();
        vault.rename(0, "Renamed").unwrap();
        assert!(dir.join("Renamed.md").exists());
        assert!(!dir.join("First.md").exists());

        let reopened = Vault::open(&dir).unwrap();
        assert_eq!(reopened.notes.len(), 1);
        assert!(reopened.notes[0].body.contains("#test"));

        vault.delete(0).unwrap();
        assert!(vault.notes.is_empty());
        assert!(!dir.join("Renamed.md").exists());
        assert_eq!(fs::read_dir(dir.join(".trash")).unwrap().count(), 1);
        fs::remove_dir_all(&dir).unwrap();
    }
}
@@text
## Make it yours

- **Pin notes:** keep some notes at the top of the list. Where would you store which ones are pinned? (Ideas: a `pinned.json` in the folder, or a line like `pinned: true` at the top of the note.)
- **Restore from trash:** a "Trash" view listing `.trash`, with a Restore button.
- **Links between notes:** treat `[[Note name]]` as a link. In the list, show which notes link to the open one ("backlinks").
- **Sort options:** newest, oldest, A–Z, using an enum and `sort_by_key`.
- **Choose the folder:** a settings field for the notes folder, saved in a small JSON file, so your vault can live in OneDrive.
- **Daily note:** a button that opens (or creates) a note named after today's date, for a journal.

@@quiz
Why does `delete` move the file to a `.trash` folder instead of deleting it with `fs::remove_file`?
- `remove_file` doesn't work on Windows
* So a mistake can be undone: a user's writing should never be lost by one wrong click
- To save disk space
? Destructive actions should be reversible whenever possible.

@@quiz
After creating a note at index 0, the code does `self.selected = self.selected.map(|s| s + 1)`. What would go wrong without it?
- The new note wouldn't appear
- The app would crash
* The open note's index would now point at a different note, so unsaved text could be saved into the wrong note
? Inserting at the front shifts every index by one.
