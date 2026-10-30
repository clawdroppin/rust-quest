# Rust Quest — notes for AI assistants

Rust Quest is a portable Windows desktop app (Rust + egui) that teaches programming from zero, in Rust.
The lessons are Markdown-like files compiled into the exe. The learner's code is compiled with the
`rustc` installed on their machine. Read `README.md` for the feature overview and `USER_GUIDE.md` for
what learners see.

## Build, run, verify

```bash
cargo run                              # debug build, opens the app
cargo run -- --verify                  # every solution passes, every starter fails, examples compile
cargo run -- --verify --only m7        # only lessons whose id starts with m7 (-v prints learner feedback)
cargo run -- --verify --projects       # also `cargo test --no-run` on every project card (slow first time)
cargo clippy --all-targets             # keep this warning-free
cargo build --release                  # then copy target/release/rust_quest.exe to dist/RustQuest.exe
```

- Run `--verify` (and `--projects` if you touched a project card) after **any** content change.
  The last full run was 169 checks, 0 problems. Keep it at zero.
- The release build uses `windows_subsystem = "windows"` and prints nothing to a console, so verify
  with the debug build.
- A stale incremental build in the shared target dir sometimes ignores a content edit. If a lesson
  change doesn't show up, touch `src/course.rs`.

## Layout

| File | What it does |
|---|---|
| `src/main.rs` | window setup, and the `--verify` developer mode |
| `src/app.rs` | the whole UI: sidebar, lesson view, editor, quizzes, project cards, AI Helper window |
| `src/course.rs` | lesson file format (documented at the top), parser, `load_course()` = list of modules |
| `src/runner.rs` | finds `rustc`, compiles/runs/tests learner code on a worker thread |
| `src/projects.rs` | writes project cards to `RustQuestProjects/`, reset with backups, opens VS Code/terminals |
| `src/progress.rs` | progress JSON next to the exe (fallback `%APPDATA%`), atomic save |
| `src/ai.rs` | optional local AI client: OpenAI-compatible HTTP via `ureq`, SSE streaming on a thread |
| `content/m*_*.md` | the course, modules 0–9 |
| `content/g*_*.md` | the Guided Builds tab (`Module.guided = true`) |
| `docs/` | GitHub Pages site (`index.html`, `img/`) |
| `dist/RustQuest.exe` | the committed release build |

## Lesson files

Format reference: the doc comment at the top of `src/course.rs`. Essentials:

- A file starts with `@@lesson <id> <Title>`. Ids are `m<module>-...` for the course, `g-...` for builds.
  Progress is keyed by lesson id and exercise index, so **never rename an existing id** (learners lose progress).
- New files must be registered in `load_course()` in `src/course.rs` (`include_str!`), in teaching order.
- Exercises: `@@starter` must *fail* the check and `@@solution` must *pass* it. Use `@@expected`
  (exact output), `@@tests` (`#[test]` fns appended to the code) and/or `@@requires`.
  In-app exercises are single files with **std only**.
- ` ```rust ` blocks in `@@text` get a "Try it" button and are compiled by `--verify`. Use ` ```rust,norun `
  for fragments that don't compile on their own.
- `@@project <folder> <Title>` + `@@file <path>` sections define a Cargo project card. Later steps of the
  same project reuse the folder and list the full files for that step. The last step of a guided build
  also has a `<folder>_finished` card. Project code uses `todo!()` for learner tasks, with `cargo test`
  tests that fail until they're done; the `--projects` verify only checks that it **compiles**.

## Conventions

- **Audience: complete beginners.** Explain every new idea before using it; short sentences; say *why*,
  not just *what*. Plain, friendly English. Lessons build strictly on earlier ones: check what's
  already been taught before using a feature (e.g. closures are Module 7, threads Module 8).
- Learner-facing GUI projects must not use emoji in their UI text: egui's default fonts don't have
  them and they render as boxes. (Rust Quest itself loads a Windows emoji font in `setup_fonts`.)
- Dependencies: eframe/egui **0.37** (`egui::Panel::top/left/bottom`, `ctx.content_rect()`), egui_commonmark
  0.26, `ureq` 3 without TLS (the AI helper only talks to local `http://` servers). Rust edition 2024
  (let-chains are used).
- Keep it portable: no installer, no registry, files only next to the exe (with the existing fallbacks).
- Privacy: the app makes no network calls except to the AI server URL the learner configures.
- Code style: match the surrounding code; comments explain *why*. `cargo clippy` clean.

## Releasing

1. Bump `version` in `Cargo.toml`, `cargo build --release`, copy the exe to `dist/RustQuest.exe`.
2. Commit, push, then
   `gh release create vX.Y.Z dist/RustQuest.exe --title "Rust Quest X.Y.Z" --notes "..."`.
3. The website's download button links to `releases/latest/download/RustQuest.exe`, so it updates itself.
