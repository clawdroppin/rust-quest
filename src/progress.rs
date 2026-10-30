//! Learner progress, saved as JSON next to the exe so the app stays portable.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const FILE_NAME: &str = "rust_quest_progress.json";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    pub completed_lessons: BTreeSet<String>,
    pub passed_exercises: BTreeSet<String>,
    /// The learner's code for each exercise, keyed by `lesson#index`.
    pub code: BTreeMap<String, String>,
    pub quiz_answers: BTreeMap<String, usize>,
    pub playground: String,
    pub last_lesson: Option<String>,
    pub zoom: f32,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            completed_lessons: BTreeSet::new(),
            passed_exercises: BTreeSet::new(),
            code: BTreeMap::new(),
            quiz_answers: BTreeMap::new(),
            playground: "fn main() {\n    println!(\"Playground: try anything here!\");\n}".into(),
            last_lesson: None,
            zoom: 1.0,
        }
    }
}

/// Next to the exe if that folder is writable, otherwise in %APPDATA%.
fn candidate_paths() -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        v.push(dir.join(FILE_NAME));
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        v.push(PathBuf::from(appdata).join("RustQuest").join(FILE_NAME));
    }
    v
}

impl Progress {
    pub fn load() -> Self {
        candidate_paths()
            .iter()
            .find_map(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let Ok(json) = serde_json::to_string_pretty(self) else { return };
        for p in candidate_paths() {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            if std::fs::write(&p, &json).is_ok() {
                return;
            }
        }
    }
}
