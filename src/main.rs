#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod app;
mod course;
mod progress;
mod projects;
mod runner;

fn main() -> eframe::Result {
    if std::env::args().any(|a| a == "--verify") {
        verify_course();
        return Ok(());
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Rust Quest — Learn to code in Rust")
            .with_inner_size([1440.0, 880.0])
            .with_min_inner_size([640.0, 480.0])
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("Rust Quest", options, Box::new(|cc| Ok(Box::new(app::RustQuest::new(cc)))))
}

/// Development helper (`cargo run -- --verify`): every solution must pass its own check,
/// and every starter must not (so exercises can't be passed by doing nothing).
fn verify_course() {
    let (rustc, version) = runner::find_rustc().expect("rustc not found");
    println!("Verifying with {version}");
    let mut failures = 0;
    let verbose = std::env::args().any(|a| a == "-v");
    let check_projects = std::env::args().any(|a| a == "--projects");
    let project_root = std::env::temp_dir().join("rust_quest_verify");
    // `--only m2` limits the run to lessons whose id starts with that prefix.
    let args: Vec<String> = std::env::args().collect();
    let only = args.iter().position(|a| a == "--only").and_then(|i| args.get(i + 1)).cloned().unwrap_or_default();
    for module in course::load_course() {
        for lesson in module.lessons.iter().filter(|l| l.id.starts_with(&only)) {
            // Every project step's starting files must build (tests included).
            for block in lesson.blocks.iter().filter(|_| check_projects) {
                let course::Block::Project(project) = block else { continue };
                let dir = project_root.join(format!("{}__{}", lesson.id, project.folder));
                let _ = std::fs::remove_dir_all(&dir);
                for (path, contents) in &project.files {
                    let target = dir.join(path);
                    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
                    std::fs::write(target, contents).unwrap();
                }
                let out = std::process::Command::new("cargo")
                    .args(["test", "--no-run", "--quiet"])
                    .current_dir(&dir)
                    .env("CARGO_TARGET_DIR", project_root.join("target"))
                    .output()
                    .expect("cargo failed to start");
                let ok = out.status.success();
                println!("[{}] {} project {}", if ok { "ok" } else { "FAIL" }, lesson.id, project.folder);
                if !ok {
                    failures += 1;
                    println!("{}", String::from_utf8_lossy(&out.stderr));
                }
            }
            // Runnable examples should compile, apart from ones that demonstrate errors.
            let segments = lesson.blocks.iter().filter_map(|b| match b {
                course::Block::Text(s) => Some(s),
                _ => None,
            });
            for seg in segments.flatten().chain(&lesson.outro) {
                if let course::Segment::Code { code, runnable: true } = seg {
                    let r = runner::execute(&rustc, &runner::Job { code: code.clone(), stdin: String::new(), check: None });
                    if !r.compile_ok || r.exit_code != Some(0) {
                        let first = code.lines().nth(1).unwrap_or("").trim();
                        let reason = if r.compile_ok { "panics" } else { "doesn't compile" };
                        println!("[note] {} example {reason} (intended?): {first}", lesson.id);
                    }
                }
            }
            for (i, ex) in lesson.exercises.iter().enumerate() {
                let check = runner::Check { expected: ex.expected.clone(), tests: ex.tests.clone(), requires: ex.requires.clone() };
                let run = |code: &str| {
                    let job = runner::Job { code: code.to_string(), stdin: ex.stdin.clone(), check: Some(check.clone()) };
                    runner::execute(&rustc, &job)
                };
                let sol = run(&ex.solution);
                let starter = run(&ex.starter);
                let sol_ok = matches!(sol.check, Some((true, _)));
                let starter_ok = matches!(starter.check, Some((true, _)));
                let status = if sol_ok && !starter_ok { "ok" } else { "FAIL" };
                println!("[{status}] {} #{} {}", lesson.id, i + 1, ex.title);
                if !sol_ok {
                    failures += 1;
                    println!("  solution failed: {:?}\n{}\n{}", sol.check.map(|c| c.1), sol.compiler_output, sol.stdout);
                }
                if starter_ok {
                    failures += 1;
                    println!("  starter code already passes!");
                } else if verbose && let Some((_, problems)) = &starter.check {
                    println!("  starter feedback: {}", problems.join("\n  ").replace('\n', "\n  "));
                }
            }
        }
    }
    println!("{failures} problem(s)");
}
