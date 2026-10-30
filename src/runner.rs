//! Compiles and runs the learner's code with the locally installed `rustc`.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

const RUN_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_OUTPUT: u64 = 100_000;

#[derive(Clone)]
pub struct Job {
    pub code: String,
    pub stdin: String,
    pub check: Option<Check>,
}

#[derive(Clone)]
pub struct Check {
    pub expected: Option<String>,
    pub tests: Option<String>,
    pub requires: Vec<String>,
}

#[derive(Default)]
pub struct RunResult {
    pub compile_ok: bool,
    pub compiler_output: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    /// `Some` when this was a check: did it pass, and why not.
    pub check: Option<(bool, Vec<String>)>,
    pub internal_error: Option<String>,
}

fn command(program: &Path) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd
}

/// Find `rustc`: first on PATH, then in the default rustup location.
pub fn find_rustc() -> Result<(PathBuf, String), String> {
    let mut candidates = vec![PathBuf::from("rustc")];
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        candidates.push(Path::new(&home).join(".cargo").join("bin").join("rustc.exe"));
        candidates.push(Path::new(&home).join(".cargo").join("bin").join("rustc"));
    }
    for c in candidates {
        if let Ok(out) = command(&c).arg("--version").output()
            && out.status.success()
        {
            return Ok((c, String::from_utf8_lossy(&out.stdout).trim().to_string()));
        }
    }
    Err("rustc was not found. Install Rust from https://rustup.rs and restart the app.".into())
}

pub fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n").lines().map(str::trim_end).collect::<Vec<_>>().join("\n").trim_end().to_string()
}

fn work_dir() -> std::io::Result<PathBuf> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join("rust_quest").join(format!("{}_{}", std::process::id(), n));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

struct Compiled {
    ok: bool,
    output: String,
    exe: PathBuf,
}

fn compile(rustc: &Path, dir: &Path, source: &str, test: bool) -> std::io::Result<Compiled> {
    // Always compile a file called main.rs by relative path, so that error and panic
    // messages say `main.rs:line:col` instead of a long temp path.
    std::fs::write(dir.join("main.rs"), source)?;
    let exe = dir.join(if test { "check.exe" } else { "main.exe" });
    let mut cmd = command(rustc);
    cmd.args(["--edition=2024", "--color=never", "main.rs", "-o"]).arg(&exe).current_dir(dir);
    if test {
        cmd.args(["--test", "-A", "warnings"]);
    }
    let out = cmd.output()?;
    Ok(Compiled { ok: out.status.success(), output: String::from_utf8_lossy(&out.stderr).into_owned(), exe })
}

const TODO_MESSAGE: &str = "Your code still has a `todo!()` in it. Replace it with your own code.";

/// Turn the test harness's failure output into something a beginner can read.
fn friendly_test_failure(stdout: &str) -> String {
    let mut lines = Vec::new();
    for l in stdout
        .lines()
        .skip_while(|l| !l.starts_with("---- "))
        .take_while(|l| !l.starts_with("failures:"))
    {
        if l.trim().is_empty() || l.starts_with("note:") || l.starts_with("---- ") || l.starts_with("thread '") {
            continue;
        }
        let l = if l.contains("not yet implemented") {
            TODO_MESSAGE.to_string()
        } else if let Some(msg) = l.strip_prefix("assertion `left == right` failed: ") {
            msg.to_string()
        } else if l.starts_with("assertion `left == right` failed") {
            "A result was wrong:".to_string()
        } else if let Some(v) = l.strip_prefix("  left: ") {
            format!("  your function returned: {v}")
        } else if let Some(v) = l.strip_prefix(" right: ") {
            format!("  but it should be:       {v}")
        } else {
            l.to_string()
        };
        if !lines.contains(&l) {
            lines.push(l);
        }
    }
    lines.join("\n")
}

struct Ran {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    timed_out: bool,
}

fn run_exe(exe: &Path, args: &[&str], stdin: &str) -> std::io::Result<Ran> {
    let mut child = command(exe)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(exe.parent().unwrap())
        .spawn()?;

    let mut input = child.stdin.take().unwrap();
    let stdin = stdin.to_string();
    std::thread::spawn(move || {
        let _ = input.write_all(stdin.as_bytes());
        let _ = input.write_all(b"\n");
    });
    let reader = |pipe: Box<dyn Read + Send>| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            let _ = pipe.take(MAX_OUTPUT).read_to_end(&mut buf);
            String::from_utf8_lossy(&buf).into_owned()
        })
    };
    let out_t = reader(Box::new(child.stdout.take().unwrap()));
    let err_t = reader(Box::new(child.stderr.take().unwrap()));

    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break Some(s);
        }
        if start.elapsed() > RUN_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            timed_out = true;
            break None;
        }
        std::thread::sleep(Duration::from_millis(15));
    };
    Ok(Ran {
        stdout: out_t.join().unwrap_or_default(),
        stderr: err_t.join().unwrap_or_default(),
        exit_code: status.and_then(|s| s.code()),
        timed_out,
    })
}

pub fn execute(rustc: &Path, job: &Job) -> RunResult {
    match execute_inner(rustc, job) {
        Ok(r) => r,
        Err(e) => RunResult { internal_error: Some(e.to_string()), ..Default::default() },
    }
}

fn execute_inner(rustc: &Path, job: &Job) -> std::io::Result<RunResult> {
    let dir = work_dir()?;
    let mut res = RunResult::default();

    let compiled = compile(rustc, &dir, &job.code, false)?;
    res.compile_ok = compiled.ok;
    res.compiler_output = compiled.output;
    if !compiled.ok {
        if job.check.is_some() {
            res.check = Some((false, vec!["Your code doesn't compile yet. Fix the errors first.".into()]));
        }
        let _ = std::fs::remove_dir_all(&dir);
        return Ok(res);
    }

    let ran = run_exe(&compiled.exe, &[], &job.stdin)?;
    res.stdout = ran.stdout;
    res.stderr = ran.stderr;
    res.exit_code = ran.exit_code;
    res.timed_out = ran.timed_out;

    if let Some(check) = &job.check {
        let mut problems = Vec::new();
        if res.timed_out {
            problems.push("Your program never finished (it ran for more than 5 seconds). Is there an endless loop?".into());
        } else if res.stderr.contains("not yet implemented") {
            problems.push(TODO_MESSAGE.into());
        } else if res.exit_code != Some(0) {
            problems.push("Your program crashed (panicked) instead of finishing normally.".into());
        }
        if let Some(expected) = &check.expected
            && normalize(&res.stdout) != normalize(expected)
        {
            problems.push(format!(
                "The output doesn't match yet.\n\nExpected:\n{}\n\nYour program printed:\n{}",
                normalize(expected),
                if res.stdout.trim().is_empty() { "(nothing)".into() } else { normalize(&res.stdout) }
            ));
        }
        for r in &check.requires {
            if !job.code.contains(r.as_str()) {
                problems.push(format!("This exercise wants you to use `{r}` in your code."));
            }
        }
        if let Some(tests) = &check.tests {
            let src = format!(
                "{}\n\n#[cfg(test)]\n#[allow(unused_imports)]\nmod rust_quest_checks {{\n    use super::*;\n{}\n}}\n",
                job.code, tests
            );
            let c = compile(rustc, &dir, &src, true)?;
            if !c.ok {
                problems.push(format!(
                    "Your code runs, but the checker couldn't call your functions. Did you keep the names and parameter types from the task?\n\n{}",
                    c.output
                ));
            } else {
                let t = run_exe(&c.exe, &["--test-threads=1", "-q"], "")?;
                if t.exit_code != Some(0) {
                    let details = friendly_test_failure(&t.stdout);
                    if !(details == TODO_MESSAGE && problems.iter().any(|p| p == TODO_MESSAGE)) {
                        problems.push(format!("Some checks failed:\n{details}"));
                    }
                }
            }
        }
        res.check = Some((problems.is_empty(), problems));
    }
    let _ = std::fs::remove_dir_all(&dir);
    Ok(res)
}
