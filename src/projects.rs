//! Cargo projects that lessons create on disk, for work in VS Code and a real terminal.

use crate::course::Project;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROOT_NAME: &str = "RustQuestProjects";

/// Next to the exe if that folder is writable, otherwise in Documents.
/// Worked out once: project cards ask for it every frame.
pub fn root() -> PathBuf {
    static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    ROOT.get_or_init(find_root).clone()
}

fn find_root() -> PathBuf {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let root = dir.join(ROOT_NAME);
        let probe = root.join(".write_test");
        if std::fs::create_dir_all(&root).is_ok() && std::fs::write(&probe, b"ok").is_ok() {
            let _ = std::fs::remove_file(probe);
            return root;
        }
    }
    let home = std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_default();
    home.join("Documents").join(ROOT_NAME)
}

pub fn dir_of(project: &Project) -> PathBuf {
    root().join(&project.folder)
}

/// How many of the project's files already exist on disk.
pub fn existing_files(project: &Project) -> usize {
    let dir = dir_of(project);
    project.files.iter().filter(|(path, _)| dir.join(path).exists()).count()
}

/// Write the files that don't exist yet. Never touches the learner's existing work.
pub fn create_missing(project: &Project) -> std::io::Result<usize> {
    let dir = dir_of(project);
    let mut written = 0;
    for (path, contents) in &project.files {
        let target = dir.join(path);
        if !target.exists() {
            write_file(&target, contents)?;
            written += 1;
        }
    }
    Ok(written)
}

/// Overwrite every file with this lesson's version, backing up the old ones first.
pub fn reset(project: &Project) -> std::io::Result<PathBuf> {
    let dir = dir_of(project);
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let backup = root().join("_backups").join(format!("{}_{stamp}", project.folder));
    for (path, contents) in &project.files {
        let target = dir.join(path);
        if target.exists() {
            let saved = backup.join(path);
            if let Some(parent) = saved.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&target, saved)?;
        }
        write_file(&target, contents)?;
    }
    Ok(backup)
}

/// The project's source files as they are on disk now (Cargo.toml and src/*.rs), for the
/// AI helper. Stops after roughly `max_chars` so small local models aren't overwhelmed.
pub fn read_sources(project: &Project, max_chars: usize) -> String {
    let dir = dir_of(project);
    let mut paths = vec![dir.join("Cargo.toml")];
    if let Ok(entries) = std::fs::read_dir(dir.join("src")) {
        let mut rs: Vec<PathBuf> = entries.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "rs")).collect();
        rs.sort();
        paths.extend(rs);
    }
    let mut out = String::new();
    for path in paths {
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let name = path.strip_prefix(&dir).unwrap_or(&path).display().to_string().replace('\\', "/");
        let lang = if name.ends_with(".rs") { "rust" } else { "toml" };
        if out.len() + text.len() > max_chars {
            out.push_str(&format!("\n({name} and any further files were left out: too long)\n"));
            break;
        }
        out.push_str(&format!("\n{name}:\n```{lang}\n{}\n```\n", text.trim_end()));
    }
    out
}

fn write_file(target: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(target, contents)
}

/// PATH with ~/.cargo/bin added, in case cargo isn't on the system PATH.
fn path_with_cargo() -> std::ffi::OsString {
    let mut paths: Vec<PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    if let Some(home) = std::env::var_os("USERPROFILE") {
        paths.push(Path::new(&home).join(".cargo").join("bin"));
    }
    std::env::join_paths(paths).unwrap_or_default()
}

pub fn open_in_vscode(dir: &Path) -> std::io::Result<()> {
    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "code", "."]).current_dir(dir);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd.spawn().map(|_| ())
}

pub fn open_folder(dir: &Path) -> std::io::Result<()> {
    Command::new("explorer").arg(dir).spawn().map(|_| ())
}

/// Run a command (like `cargo run`) in a new terminal window that stays open afterwards.
pub fn run_in_terminal(dir: &Path, command_line: &str) -> std::io::Result<()> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/K").arg(command_line).current_dir(dir).env("PATH", path_with_cargo());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0000_0010); // CREATE_NEW_CONSOLE
    }
    cmd.spawn().map(|_| ())
}
