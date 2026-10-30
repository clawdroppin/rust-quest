# Rust Quest

A portable Windows app that teaches programming from zero, in Rust: 70 lessons in 10 modules,
ending with a complete desktop GUI app (Pocket Ledger, a personal budget tracker).

- **Run it:** `dist/RustQuest.exe`. It's a single file; copy it anywhere. Progress is saved to
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
| 8 · Real Programs | files, JSON with serde, program structure, debugging | command-line to-do app |
| 9 · GUI Capstone | event loops, egui widgets and layout, app state | Pocket Ledger budget app |

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
- `cargo build --release`, then copy `target/release/rust_quest.exe` to `dist/RustQuest.exe`.
