//! The course: lesson file format, parser, and the list of modules.
//!
//! Lesson files live in `content/` and are compiled into the exe with `include_str!`.
//! A lesson file is split into sections by lines that start with `@@`:
//!
//! ```text
//! @@lesson <id> <Title of the lesson>
//! @@text            markdown; ```rust blocks get "Try it" buttons (```rust,norun do not)
//! @@quiz            question lines, then `- wrong`, `* correct`, `? explanation`
//! @@exercise <Title>  markdown task description
//! @@starter         code the editor starts with
//! @@expected        exact program output needed to pass
//! @@stdin           input fed to the program when checking
//! @@tests           #[test] functions appended to the code when checking
//! @@requires        one snippet per line the code must contain
//! @@hint / @@solution
//! @@text           (after an exercise) closing text shown below the practice section
//! @@project <folder> <Title>   markdown; a Cargo project card the learner can create on disk
//! @@file <path>     a file of the project above (the lesson's starting point for that file)
//! ```

pub enum Segment {
    Md(String),
    Code { code: String, runnable: bool },
}

pub struct Quiz {
    pub question: String,
    pub options: Vec<String>,
    pub correct: usize,
    pub explain: String,
}

/// A Cargo project the learner works on in VS Code.
pub struct Project {
    pub folder: String,
    pub title: String,
    pub description: String,
    /// (relative path, contents)
    pub files: Vec<(String, String)>,
}

pub enum Block {
    Text(Vec<Segment>),
    Quiz(Quiz),
    Project(Project),
}

#[derive(Default)]
pub struct Exercise {
    pub title: String,
    pub prompt: String,
    pub starter: String,
    pub expected: Option<String>,
    pub stdin: String,
    pub tests: Option<String>,
    pub requires: Vec<String>,
    pub hint: String,
    pub solution: String,
}

pub struct Lesson {
    pub id: String,
    pub title: String,
    pub blocks: Vec<Block>,
    pub exercises: Vec<Exercise>,
    /// Text written after the exercises, shown below the practice section.
    pub outro: Vec<Segment>,
}

pub struct Module {
    pub title: &'static str,
    pub blurb: &'static str,
    pub lessons: Vec<Lesson>,
    /// Lesson titles that are planned but not written yet.
    pub planned: &'static [&'static str],
}

impl Lesson {
    pub fn exercise_key(&self, idx: usize) -> String {
        format!("{}#{}", self.id, idx)
    }
    pub fn quiz_key(&self, block_idx: usize) -> String {
        format!("{}?{}", self.id, block_idx)
    }
}

/// Trim blank lines at the start and end, keep inner indentation.
fn trim_block(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let start = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(lines.len());
    let end = lines.iter().rposition(|l| !l.trim().is_empty()).map_or(start, |i| i + 1);
    lines[start..end].join("\n")
}

fn split_segments(md: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut code: Option<(String, bool)> = None;
    for line in md.lines() {
        let t = line.trim_start();
        match &mut code {
            None if t.starts_with("```rust") => {
                if !text.trim().is_empty() {
                    out.push(Segment::Md(trim_block(&text)));
                }
                text.clear();
                code = Some((String::new(), !t.contains("norun")));
            }
            None => {
                text.push_str(line);
                text.push('\n');
            }
            Some(_) if t.starts_with("```") => {
                let (c, runnable) = code.take().unwrap();
                out.push(Segment::Code { code: trim_block(&c), runnable });
            }
            Some((c, _)) => {
                c.push_str(line);
                c.push('\n');
            }
        }
    }
    if !text.trim().is_empty() {
        out.push(Segment::Md(trim_block(&text)));
    }
    out
}

fn parse_quiz(body: &str) -> Quiz {
    let mut q = Quiz { question: String::new(), options: vec![], correct: 0, explain: String::new() };
    let mut in_explain = false;
    for line in body.lines() {
        if in_explain {
            q.explain.push_str(line);
            q.explain.push('\n');
        } else if let Some(o) = line.strip_prefix("- ") {
            q.options.push(o.trim().to_string());
        } else if let Some(o) = line.strip_prefix("* ") {
            q.correct = q.options.len();
            q.options.push(o.trim().to_string());
        } else if let Some(e) = line.strip_prefix("? ") {
            in_explain = true;
            q.explain.push_str(e);
            q.explain.push('\n');
        } else {
            q.question.push_str(line);
            q.question.push('\n');
        }
    }
    q.question = trim_block(&q.question);
    q.explain = trim_block(&q.explain);
    q
}

pub fn parse_lesson(src: &str) -> Lesson {
    let mut sections: Vec<(String, String, String)> = Vec::new();
    for line in src.lines() {
        if let Some(rest) = line.strip_prefix("@@") {
            let (name, arg) = rest.split_once(' ').unwrap_or((rest, ""));
            sections.push((name.trim().to_string(), arg.trim().to_string(), String::new()));
        } else if let Some(last) = sections.last_mut() {
            last.2.push_str(line);
            last.2.push('\n');
        }
    }

    let mut lesson = Lesson { id: String::new(), title: String::new(), blocks: vec![], exercises: vec![], outro: vec![] };
    for (name, arg, body) in sections {
        let ex = lesson.exercises.last_mut();
        match (name.as_str(), ex) {
            ("lesson", _) => {
                let (id, title) = arg.split_once(' ').unwrap_or((&arg, ""));
                lesson.id = id.to_string();
                lesson.title = title.to_string();
            }
            ("project", _) => {
                let (folder, title) = arg.split_once(' ').unwrap_or((&arg, ""));
                lesson.blocks.push(Block::Project(Project {
                    folder: folder.to_string(),
                    title: title.to_string(),
                    description: trim_block(&body),
                    files: vec![],
                }));
            }
            ("file", _) => match lesson.blocks.last_mut() {
                Some(Block::Project(p)) => p.files.push((arg, trim_block(&body) + "\n")),
                _ => panic!("lesson {}: @@file must follow @@project", lesson.id),
            },
            ("text", Some(_)) => lesson.outro.extend(split_segments(&body)),
            ("text", None) => lesson.blocks.push(Block::Text(split_segments(&body))),
            ("quiz", _) => lesson.blocks.push(Block::Quiz(parse_quiz(&body))),
            ("exercise", _) => lesson.exercises.push(Exercise {
                title: arg,
                prompt: trim_block(&body),
                ..Default::default()
            }),
            ("starter", Some(ex)) => ex.starter = trim_block(&body),
            ("expected", Some(ex)) => ex.expected = Some(trim_block(&body)),
            ("stdin", Some(ex)) => ex.stdin = trim_block(&body),
            ("tests", Some(ex)) => ex.tests = Some(trim_block(&body)),
            ("requires", Some(ex)) => {
                ex.requires = body.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
            }
            ("hint", Some(ex)) => ex.hint = trim_block(&body),
            ("solution", Some(ex)) => ex.solution = trim_block(&body),
            (other, _) => panic!("lesson {}: unknown or misplaced section @@{}", lesson.id, other),
        }
    }
    lesson
}

macro_rules! lessons {
    ($($file:literal),* $(,)?) => {
        vec![$(parse_lesson(include_str!(concat!("../content/", $file)))),*]
    };
}

pub fn load_course() -> Vec<Module> {
    vec![
        Module {
            title: "0 · Getting Started",
            blurb: "What programs are, and your first one.",
            lessons: lessons!["m0_01_welcome.md", "m0_02_what_is_a_program.md", "m0_03_hello_world.md", "m0_04_compiler_errors.md"],
            planned: &[],
        },
        Module {
            title: "1 · Programming Fundamentals",
            blurb: "The building blocks every program is made of.",
            lessons: lessons![
                "m1_01_variables.md",
                "m1_02_numbers.md",
                "m1_03_bool_char.md",
                "m1_04_operators.md",
                "m1_05_text_formatting.md",
                "m1_06_functions.md",
                "m1_07_if_else.md",
                "m1_08_loops.md",
                "m1_09_comments_style.md",
                "m1_10_thinking.md",
            ],
            planned: &[],
        },
        Module {
            title: "2 · Collections of Data",
            blurb: "Grouping many values together.",
            lessons: lessons![
                "m2_01_tuples.md",
                "m2_02_arrays.md",
                "m2_03_vectors.md",
                "m2_04_iterating.md",
                "m2_05_strings_deep.md",
                "m2_06_hashmap.md",
                "m2_07_grids.md",
                "m2_08_project_gradebook.md",
            ],
            planned: &[],
        },
        Module {
            title: "3 · Ownership & Borrowing",
            blurb: "How Rust manages memory without a garbage collector.",
            lessons: lessons![
                "m3_01_memory.md",
                "m3_02_ownership.md",
                "m3_03_copy_clone.md",
                "m3_04_references.md",
                "m3_05_mut_references.md",
                "m3_06_slices.md",
                "m3_07_lifetimes.md",
            ],
            planned: &[],
        },
        Module {
            title: "4 · Modeling Data",
            blurb: "Structs, enums and pattern matching.",
            lessons: lessons![
                "m4_01_structs.md",
                "m4_02_methods.md",
                "m4_03_enums.md",
                "m4_04_match.md",
                "m4_05_option.md",
                "m4_06_if_let.md",
                "m4_07_project_inventory.md",
            ],
            planned: &[],
        },
        Module {
            title: "5 · Handling Errors & Input",
            blurb: "Programs that deal with the messy real world.",
            lessons: lessons![
                "m5_01_panic.md",
                "m5_02_result.md",
                "m5_03_question_mark.md",
                "m5_04_input.md",
                "m5_05_custom_errors.md",
                "m5_06_project_calculator.md",
            ],
            planned: &[],
        },
        Module {
            title: "6 · Organizing Code",
            blurb: "Cargo, modules, crates and tests.",
            lessons: lessons![
                "m6_01_cargo.md",
                "m6_02_modules.md",
                "m6_03_modules_files.md",
                "m6_04_crates.md",
                "m6_05_testing.md",
                "m6_06_docs_tools.md",
                "m6_07_project_hangman.md",
            ],
            planned: &[],
        },
        Module {
            title: "7 · Abstraction",
            blurb: "Traits, generics, closures and iterators.",
            lessons: lessons![
                "m7_01_traits.md",
                "m7_02_generics.md",
                "m7_03_trait_bounds.md",
                "m7_04_derive.md",
                "m7_05_closures.md",
                "m7_06_iterators.md",
                "m7_07_smart_pointers.md",
                "m7_08_project_movies.md",
            ],
            planned: &[],
        },
        Module {
            title: "8 · Real Programs",
            blurb: "Files, data formats and program architecture.",
            lessons: lessons![
                "m8_01_files.md",
                "m8_02_serde.md",
                "m8_03_structure.md",
                "m8_04_debugging.md",
                "m8_05_project_todo.md",
            ],
            planned: &[],
        },
        Module {
            title: "9 · GUI Capstone",
            blurb: "Build a real desktop app with a GUI.",
            lessons: lessons![
                "m9_01_how_guis_work.md",
                "m9_02_egui_hello.md",
                "m9_03_widgets_layout.md",
                "m9_04_app_state.md",
                "m9_05_ledger_model.md",
                "m9_06_ledger_gui.md",
                "m9_07_ledger_save.md",
                "m9_08_shipping.md",
            ],
            planned: &[],
        },
    ]
}
