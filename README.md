# Rust Quest

A portable Windows app that teaches programming from zero, in Rust: 73 lessons in 10 modules,
ending with a complete desktop GUI app (Pocket Ledger, a personal budget tracker), plus a
**Guided Builds** tab with four bigger apps built step by step, side by side with VS Code.

- **Website:** https://clawdroppin.github.io/rust-quest/ · **User guide:** [USER_GUIDE.md](USER_GUIDE.md)
- **Run it:** download `RustQuest.exe` from [Releases](https://github.com/clawdroppin/rust-quest/releases/latest) (or `dist/RustQuest.exe`). It's a single file; copy it anywhere. Progress is saved to
  `rust_quest_progress.json` next to the exe.
- **Needs:** Rust installed (`rustc` on PATH or in `~/.cargo/bin`). The app compiles your code with it.
  VS Code (with rust-analyzer) is used from Module 6 on, for Cargo projects.

## The course

| Module | Topics | Projects |
|---|---|---|
| 0 · Getting Started | what programs are, hello world, compiler errors | |
| 1 · Programming Fundamentals | variables, types, operators, strings, functions, if/else, loops, style | |
| 2 · Collections of Data | tuples, arrays, Vec, iterating, strings, HashMap, grids | grade book |
| 3 · Ownership & Borrowing | memory, ownership, Copy/Clone, references, slices, lifetimes | |
| 4 · Modeling Data | structs, methods, enums, match, Option, if let | game shop inventory |
| 5 · Handling Errors & Input | panic, Result, `?`, reading input, custom errors | calculator |
| 6 · Organizing Code | Cargo, modules, files, crates, tests, docs, clippy | guessing game, Hangman |
| 7 · Abstraction | traits, generics, bounds, derive, closures, iterators, Box/Rc/RefCell | movie library |
| 8 · Real Programs | files, JSON with serde, program structure, debugging, HashSet/BTreeMap/VecDeque, time, threads and channels | command-line to-do app |
| 9 · GUI Capstone | event loops, egui widgets and layout, app state, timers and background work | Pocket Ledger budget app |

## Guided Builds

A separate tab (top of the sidebar). Each build has three steps with a project card per step
(starting files, with `todo!()`s and tests) and a card with the finished app. The in-app editor is
hidden there by default, since the coding happens in VS Code.

| Build | What it is | Practises |
|---|---|---|
| 🍅 Focus Timer | Pomodoro timer: progress ring, task log, streak, weekly chart | state machines, `Instant`, painter, JSON |
| 📝 Note Vault | Markdown notes as files: search, #tags, live preview, autosave | files, sets/maps, `egui_commonmark` |
| 🗂 File Organizer | sorts a folder into subfolders, with preview and undo | `std::fs`, safe file handling |
| 💬 Local AI Chat | streaming chat client for LM Studio / Ollama | HTTP + JSON (`ureq`), worker threads |

## AI helper (optional)

The **🤖 AI Helper** button connects to a model running locally through the OpenAI-compatible API
(LM Studio, Ollama, llama.cpp server, Jan). It can include the current lesson, the learner's code,
the last compiler/checker output and, for project steps, the project's files. "Hints, not full
solutions" is on by default. Nothing is sent anywhere except to the configured server address.

In-app exercises (std only) are compiled and checked inside the app. From Module 6 on, lessons
also contain **project cards**: Cargo projects written to `RustQuestProjects/<name>` next to the exe,
opened in VS Code and run with `cargo run` / `cargo test`.

## Development

- Lessons live in `content/*.md` (format documented at the top of `src/course.rs`) and are
  compiled into the exe. Register new lesson files in `load_course()` in `src/course.rs`.
- `cargo run -- --verify` checks that every exercise solution passes and every starter fails,
  and that runnable examples compile (`-v` also prints the feedback a learner would see,
  `--only m7` limits it to lessons whose id starts with `m7`).
- `cargo run -- --verify --projects` also builds every project card's files with `cargo test --no-run`
  (in `%TEMP%\rust_quest_verify`, with a shared target dir; the first run downloads and builds eframe).
- `src/ai.rs` is the AI helper's client (`ureq`, plain HTTP, streaming on a worker thread).
- `cargo build --release`, then copy `target/release/rust_quest.exe` to `dist/RustQuest.exe`.
