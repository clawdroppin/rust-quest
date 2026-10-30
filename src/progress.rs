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
    /// The last step opened in the Guided Builds tab.
    pub last_build: Option<String>,
    /// Whether the Guided Builds tab was open, to reopen it on the next start.
    pub in_builds: bool,
    pub zoom: f32,
    pub show_editor_in_builds: bool,
    pub ai: crate::ai::AiSettings,
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
            last_build: None,
            in_builds: false,
            zoom: 1.0,
            show_editor_in_builds: false,
            ai: crate::ai::AiSettings::default(),
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
        for path in candidate_paths() {
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            match serde_json::from_str(&text) {
                Ok(progress) => return progress,
                Err(_) => {
                    // Never silently overwrite a damaged progress file: keep a copy.
                    let _ = std::fs::copy(&path, path.with_extension("damaged.json"));
                    break;
                }
            }
        }
        Self::default()
    }

    /// Writes to a temporary file and renames it, so a crash mid-save can't corrupt progress.
    pub fn save(&self) {
        let Ok(json) = serde_json::to_string_pretty(self) else { return };
        for p in candidate_paths() {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let temp = p.with_extension("json.tmp");
            if std::fs::write(&temp, &json).is_ok() && std::fs::rename(&temp, &p).is_ok() {
                return;
            }
            let _ = std::fs::remove_file(&temp);
        }
    }
}
