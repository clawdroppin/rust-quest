use crate::course::{Block, Lesson, Module, Project, Segment, load_course};
use crate::projects;
use crate::progress::Progress;
use crate::runner::{self, Check, Job, RunResult};
use eframe::egui::{self, Color32, FontId, Key, Modifiers, RichText, TextStyle};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_extras::syntax_highlighting::{CodeTheme, highlight};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

const ACCENT: Color32 = Color32::from_rgb(232, 120, 64);
const GOOD: Color32 = Color32::from_rgb(110, 205, 125);
const BAD: Color32 = Color32::from_rgb(240, 105, 95);
const WARN: Color32 = Color32::from_rgb(235, 195, 95);
const CODE_BG: Color32 = Color32::from_rgb(24, 26, 31);

#[derive(Clone, Copy, PartialEq)]
enum EditorTab {
    Exercise(usize),
    Playground,
}

/// Things the lesson view asks for; applied after drawing to keep borrows simple.
enum UiAction {
    Open(usize, usize),
    TryCode(String),
    OpenExercise(usize),
    CompleteAndNext,
}

pub struct RustQuest {
    course: Vec<Module>,
    progress: Progress,
    module: usize,
    lesson: usize,
    tab: EditorTab,
    stdin: String,
    output: Option<RunResult>,
    output_was_check: bool,
    pending: Option<(Receiver<RunResult>, Option<String>)>,
    rustc: Option<Result<(PathBuf, String), String>>,
    rustc_rx: Option<Receiver<Result<(PathBuf, String), String>>>,
    md_cache: CommonMarkCache,
    show_hint: bool,
    show_solution: bool,
    scroll_to_top: bool,
    /// Feedback from the last project action: (project folder, message).
    project_msg: Option<(String, String)>,
    /// Project folder whose "reset" button is waiting for a second click.
    confirm_reset: Option<String>,
    dirty: bool,
    last_save: Instant,
}

impl RustQuest {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        setup_fonts(ctx);
        ctx.set_theme(egui::Theme::Dark);
        ctx.all_styles_mut(|s| {
            s.text_styles.insert(TextStyle::Body, FontId::proportional(16.0));
            s.text_styles.insert(TextStyle::Button, FontId::proportional(15.0));
            s.text_styles.insert(TextStyle::Monospace, FontId::monospace(15.0));
            s.text_styles.insert(TextStyle::Heading, FontId::proportional(28.0));
            s.text_styles.insert(TextStyle::Small, FontId::proportional(12.5));
            s.spacing.item_spacing = egui::vec2(8.0, 7.0);
            s.spacing.button_padding = egui::vec2(10.0, 5.0);
            s.visuals.hyperlink_color = ACCENT;
            s.visuals.selection.bg_fill = Color32::from_rgb(120, 62, 35);
        });

        let progress = Progress::load();
        ctx.set_zoom_factor(progress.zoom.clamp(0.6, 2.5));

        let (tx, rx) = channel();
        let repaint = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(runner::find_rustc());
            repaint.request_repaint();
        });

        let mut app = Self {
            course: load_course(),
            progress,
            module: 0,
            lesson: 0,
            tab: EditorTab::Playground,
            stdin: String::new(),
            output: None,
            output_was_check: false,
            pending: None,
            rustc: None,
            rustc_rx: Some(rx),
            md_cache: CommonMarkCache::default(),
            show_hint: false,
            show_solution: false,
            scroll_to_top: true,
            project_msg: None,
            confirm_reset: None,
            dirty: false,
            last_save: Instant::now(),
        };
        let (m, l) = app
            .progress
            .last_lesson
            .as_ref()
            .and_then(|id| app.find_lesson(id))
            .unwrap_or((0, 0));
        app.open_lesson(m, l);
        app
    }

    fn find_lesson(&self, id: &str) -> Option<(usize, usize)> {
        self.course.iter().enumerate().find_map(|(mi, m)| m.lessons.iter().position(|l| l.id == id).map(|li| (mi, li)))
    }

    fn current(&self) -> &Lesson {
        &self.course[self.module].lessons[self.lesson]
    }

    fn open_lesson(&mut self, m: usize, l: usize) {
        self.module = m;
        self.lesson = l;
        let has_ex = !self.current().exercises.is_empty();
        self.progress.last_lesson = Some(self.current().id.clone());
        self.dirty = true;
        self.scroll_to_top = true;
        self.select_tab(if has_ex { EditorTab::Exercise(0) } else { EditorTab::Playground });
    }

    fn select_tab(&mut self, tab: EditorTab) {
        self.tab = tab;
        self.stdin = match tab {
            EditorTab::Exercise(i) => self.current().exercises[i].stdin.clone(),
            EditorTab::Playground => String::new(),
        };
        self.output = None;
        self.show_hint = false;
        self.show_solution = false;
    }

    fn next_lesson(&self) -> Option<(usize, usize)> {
        let (m, l) = (self.module, self.lesson);
        if l + 1 < self.course[m].lessons.len() {
            return Some((m, l + 1));
        }
        (m + 1..self.course.len()).find(|&mi| !self.course[mi].lessons.is_empty()).map(|mi| (mi, 0))
    }

    fn prev_lesson(&self) -> Option<(usize, usize)> {
        let (m, l) = (self.module, self.lesson);
        if l > 0 {
            return Some((m, l - 1));
        }
        (0..m).rev().find(|&mi| !self.course[mi].lessons.is_empty()).map(|mi| (mi, self.course[mi].lessons.len() - 1))
    }

    fn total_lessons(&self) -> usize {
        self.course.iter().map(|m| m.lessons.len() + m.planned.len()).sum()
    }

    /// The editor buffer for the current tab (exercise code is created from the starter on first use).
    fn code_mut(&mut self) -> &mut String {
        match self.tab {
            EditorTab::Exercise(i) => {
                let lesson = &self.course[self.module].lessons[self.lesson];
                self.progress.code.entry(lesson.exercise_key(i)).or_insert_with(|| lesson.exercises[i].starter.clone())
            }
            EditorTab::Playground => &mut self.progress.playground,
        }
    }

    fn start_job(&mut self, ctx: &egui::Context, check: bool) {
        if self.pending.is_some() {
            return;
        }
        let rustc = match &self.rustc {
            Some(Ok((path, _))) => path.clone(),
            Some(Err(e)) => {
                self.output = Some(RunResult { internal_error: Some(e.clone()), ..Default::default() });
                return;
            }
            None => return,
        };
        let code = self.code_mut().clone();
        let (check, key) = match (check, self.tab) {
            (true, EditorTab::Exercise(i)) => {
                let ex = &self.current().exercises[i];
                (
                    Some(Check { expected: ex.expected.clone(), tests: ex.tests.clone(), requires: ex.requires.clone() }),
                    Some(self.current().exercise_key(i)),
                )
            }
            _ => (None, None),
        };
        self.output_was_check = check.is_some();
        let job = Job { code, stdin: self.stdin.clone(), check };
        let (tx, rx) = channel();
        let repaint = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(runner::execute(&rustc, &job));
            repaint.request_repaint();
        });
        self.pending = Some((rx, key));
    }

    fn poll(&mut self) {
        if let Some(rx) = &self.rustc_rx
            && let Ok(r) = rx.try_recv()
        {
            self.rustc = Some(r);
            self.rustc_rx = None;
        }
        let Some((rx, key)) = &self.pending else { return };
        let Ok(result) = rx.try_recv() else { return };
        if let (Some(key), Some((true, _))) = (key, &result.check) {
            self.progress.passed_exercises.insert(key.clone());
            let lesson = self.current();
            let all_passed = (0..lesson.exercises.len()).all(|i| self.progress.passed_exercises.contains(&lesson.exercise_key(i)));
            if all_passed {
                let id = lesson.id.clone();
                self.progress.completed_lessons.insert(id);
            }
            self.dirty = true;
        }
        self.output = Some(result);
        self.pending = None;
    }

    // ---------------------------------------------------------------- top bar

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🦀 Rust Quest").size(22.0).strong().color(ACCENT));
            ui.separator();
            let done = self.progress.completed_lessons.len();
            let total = self.total_lessons();
            ui.label(format!("{done} / {total} lessons"));
            ui.add(egui::ProgressBar::new(done as f32 / total.max(1) as f32).desired_width(180.0).fill(ACCENT));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let zoom = ui.ctx().zoom_factor();
                if ui.button("A+").on_hover_text("Bigger text (Ctrl +)").clicked() {
                    ui.ctx().set_zoom_factor((zoom + 0.1).min(2.5));
                }
                if ui.button("A−").on_hover_text("Smaller text (Ctrl −)").clicked() {
                    ui.ctx().set_zoom_factor((zoom - 0.1).max(0.6));
                }
                ui.separator();
                match &self.rustc {
                    None => {
                        ui.spinner();
                        ui.label("Looking for Rust…");
                    }
                    Some(Ok((_, v))) => {
                        ui.label(RichText::new(format!("✔ {v}")).color(GOOD).small());
                    }
                    Some(Err(e)) => {
                        ui.label(RichText::new("⚠ Rust not found").color(BAD)).on_hover_text(e);
                    }
                }
            });
        });
    }

    // ---------------------------------------------------------------- sidebar

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let mut clicked = None;
        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            ui.add_space(6.0);
            for (mi, module) in self.course.iter().enumerate() {
                let done = module.lessons.iter().filter(|l| self.progress.completed_lessons.contains(&l.id)).count();
                let total = module.lessons.len() + module.planned.len();
                let header = RichText::new(format!("{}  ({done}/{total})", module.title)).strong();
                egui::CollapsingHeader::new(header)
                    .id_salt(("module", mi))
                    .default_open(mi == self.module || mi <= 1)
                    .show(ui, |ui| {
                        ui.label(RichText::new(module.blurb).small().weak());
                        for (li, lesson) in module.lessons.iter().enumerate() {
                            let current = mi == self.module && li == self.lesson;
                            let done = self.progress.completed_lessons.contains(&lesson.id);
                            let icon = if done { "✔" } else if current { "▶" } else { "○" };
                            let mut text = RichText::new(format!("{icon}  {}", lesson.title));
                            if done {
                                text = text.color(GOOD);
                            }
                            if ui.selectable_label(current, text).clicked() {
                                clicked = Some((mi, li));
                            }
                        }
                        for title in module.planned {
                            ui.label(RichText::new(format!("🔒  {title}")).weak())
                                .on_hover_text("Coming soon — this lesson hasn't been written yet.");
                        }
                    });
            }
        });
        if let Some((m, l)) = clicked {
            self.open_lesson(m, l);
        }
    }

    // ---------------------------------------------------------------- lesson view

    fn lesson_view(&mut self, ui: &mut egui::Ui) {
        let mut actions = Vec::new();
        let mut scroll = egui::ScrollArea::vertical().id_salt("lesson").auto_shrink(false);
        if std::mem::take(&mut self.scroll_to_top) {
            scroll = scroll.vertical_scroll_offset(0.0);
        }
        let has_next = self.next_lesson().is_some();
        let prev = self.prev_lesson();
        let Self { course, progress, md_cache, module, lesson, project_msg, confirm_reset, .. } = self;
        let module_ref = &course[*module];
        let lesson_ref = &module_ref.lessons[*lesson];
        let lesson_done = progress.completed_lessons.contains(&lesson_ref.id);

        scroll.show(ui, |ui| {
            ui.set_max_width(860.0);
            ui.add_space(8.0);
            ui.label(RichText::new(module_ref.title).color(ACCENT).small());
            ui.heading(RichText::new(&lesson_ref.title).strong());
            if lesson_done {
                ui.label(RichText::new("✔ Completed").color(GOOD));
            }
            ui.add_space(8.0);

            for (bi, block) in lesson_ref.blocks.iter().enumerate() {
                ui.push_id(bi, |ui| match block {
                    Block::Text(segments) => {
                        for (si, seg) in segments.iter().enumerate() {
                            ui.push_id(si, |ui| match seg {
                                Segment::Md(md) => {
                                    CommonMarkViewer::new().show(ui, md_cache, md);
                                }
                                Segment::Code { code, runnable } => {
                                    if code_block(ui, code, *runnable) {
                                        actions.push(UiAction::TryCode(code.clone()));
                                    }
                                }
                            });
                        }
                    }
                    Block::Project(project) => project_card(ui, md_cache, project, project_msg, confirm_reset),
                    Block::Quiz(quiz) => {
                        let key = lesson_ref.quiz_key(bi);
                        let mut answer = progress.quiz_answers.get(&key).copied();
                        quiz_card(ui, md_cache, quiz, &mut answer);
                        if let Some(a) = answer
                            && progress.quiz_answers.get(&key) != Some(&a)
                        {
                            progress.quiz_answers.insert(key, a);
                        }
                    }
                });
                ui.add_space(6.0);
            }

            if !lesson_ref.exercises.is_empty() {
                ui.add_space(10.0);
                ui.separator();
                ui.label(RichText::new("🧩 Practice").size(20.0).strong());
                ui.label("Solve these in the editor on the right. Press ✔ Check when you think you've got it.");
                for (i, ex) in lesson_ref.exercises.iter().enumerate() {
                    let passed = progress.passed_exercises.contains(&lesson_ref.exercise_key(i));
                    ui.horizontal(|ui| {
                        let (icon, color) = if passed { ("✔", GOOD) } else { ("○", ui.visuals().text_color()) };
                        ui.label(RichText::new(format!("{icon}  Exercise {}: {}", i + 1, ex.title)).color(color));
                        if ui.small_button("Open").clicked() {
                            actions.push(UiAction::OpenExercise(i));
                        }
                    });
                }
            }

            for (si, seg) in lesson_ref.outro.iter().enumerate() {
                ui.push_id(("outro", si), |ui| match seg {
                    Segment::Md(md) => {
                        CommonMarkViewer::new().show(ui, md_cache, md);
                    }
                    Segment::Code { code, runnable } => {
                        if code_block(ui, code, *runnable) {
                            actions.push(UiAction::TryCode(code.clone()));
                        }
                    }
                });
            }

            ui.add_space(20.0);
            ui.separator();
            ui.horizontal(|ui| {
                if let Some((m, l)) = prev
                    && ui.button("← Previous").clicked()
                {
                    actions.push(UiAction::Open(m, l));
                }
                let label = if has_next { "Mark complete & continue →" } else { "Mark complete ✔" };
                if ui.add(egui::Button::new(RichText::new(label).strong().color(Color32::BLACK)).fill(ACCENT)).clicked() {
                    actions.push(UiAction::CompleteAndNext);
                }
            });
            if !has_next && lesson_done {
                ui.label(RichText::new("🎉 You've finished the whole course! Now go build something of your own.").color(GOOD));
            }
            ui.add_space(40.0);
        });

        for action in actions {
            match action {
                UiAction::Open(m, l) => self.open_lesson(m, l),
                UiAction::OpenExercise(i) => self.select_tab(EditorTab::Exercise(i)),
                UiAction::TryCode(code) => {
                    self.progress.playground = code;
                    self.select_tab(EditorTab::Playground);
                    self.start_job(ui.ctx(), false);
                }
                UiAction::CompleteAndNext => {
                    let id = self.current().id.clone();
                    self.progress.completed_lessons.insert(id);
                    self.dirty = true;
                    if let Some((m, l)) = self.next_lesson() {
                        self.open_lesson(m, l);
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------------- editor side

    fn editor_panel(&mut self, ui: &mut egui::Ui) {
        // Tabs
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            let lesson = self.current();
            let mut new_tab = None;
            for i in 0..lesson.exercises.len() {
                let passed = self.progress.passed_exercises.contains(&lesson.exercise_key(i));
                let label = format!("{} Exercise {}", if passed { "✔" } else { "🧩" }, i + 1);
                if ui.selectable_label(self.tab == EditorTab::Exercise(i), label).clicked() {
                    new_tab = Some(EditorTab::Exercise(i));
                }
            }
            if ui.selectable_label(self.tab == EditorTab::Playground, "🧪 Playground").on_hover_text("Free space to experiment. Examples you 'Try' open here.").clicked() {
                new_tab = Some(EditorTab::Playground);
            }
            if let Some(t) = new_tab
                && t != self.tab
            {
                self.select_tab(t);
            }
        });
        ui.separator();

        if let EditorTab::Exercise(i) = self.tab {
            egui::Panel::top("task").resizable(true).default_size(200.0).show(ui, |ui| {
                self.task_view(ui, i);
            });
        }

        egui::Panel::bottom("output").resizable(true).default_size(260.0).min_size(120.0).show(ui, |ui| {
            self.run_controls(ui);
            ui.separator();
            egui::ScrollArea::vertical().id_salt("output").auto_shrink(false).stick_to_bottom(false).show(ui, |ui| {
                self.output_view(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let id = match self.tab {
                EditorTab::Exercise(i) => egui::Id::unique(("code", self.current().exercise_key(i))),
                EditorTab::Playground => egui::Id::unique("code_playground"),
            };
            if code_editor(ui, self.code_mut(), id) {
                self.dirty = true;
            }
        });
    }

    fn task_view(&mut self, ui: &mut egui::Ui, i: usize) {
        let Self { course, md_cache, module, lesson, show_hint, show_solution, progress, .. } = self;
        let lesson = &course[*module].lessons[*lesson];
        let ex = &lesson.exercises[i];
        let mut reset = false;
        egui::ScrollArea::vertical().id_salt("task").auto_shrink(false).show(ui, |ui| {
            ui.label(RichText::new(&ex.title).strong().size(18.0).color(ACCENT));
            CommonMarkViewer::new().show(ui, md_cache, &ex.prompt);
            ui.horizontal(|ui| {
                if !ex.hint.is_empty() {
                    ui.toggle_value(show_hint, "💡 Hint");
                }
                if !ex.solution.is_empty() {
                    ui.toggle_value(show_solution, "👀 Solution");
                }
                if ui.button("↺ Reset code").on_hover_text("Throw away your code and start from the starter code again").clicked() {
                    reset = true;
                }
            });
            if *show_hint {
                egui::Frame::group(ui.style()).fill(Color32::from_rgb(45, 42, 30)).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    CommonMarkViewer::new().show(ui, md_cache, &format!("**Hint:** {}", ex.hint));
                });
            }
            if *show_solution {
                ui.label(RichText::new("Try to solve it yourself first — you learn far more from struggling a bit!").small().color(WARN));
                code_block(ui, &ex.solution, false);
            }
        });
        if reset {
            progress.code.insert(lesson.exercise_key(i), ex.starter.clone());
        }
    }

    fn run_controls(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let ready = matches!(self.rustc, Some(Ok(_))) && self.pending.is_none();
            let run = ui.add_enabled(ready, egui::Button::new(RichText::new("▶ Run").strong())).on_hover_text("Compile and run (Ctrl+Enter or F5)");
            if run.clicked() {
                self.start_job(ui.ctx(), false);
            }
            if matches!(self.tab, EditorTab::Exercise(_)) {
                let check = ui
                    .add_enabled(ready, egui::Button::new(RichText::new("✔ Check").strong().color(Color32::BLACK)).fill(GOOD))
                    .on_hover_text("Run and check if the exercise is solved (Ctrl+Shift+Enter)");
                if check.clicked() {
                    self.start_job(ui.ctx(), true);
                }
            }
            if self.pending.is_some() {
                ui.spinner();
                ui.label("Compiling…");
            }
        });
        egui::CollapsingHeader::new("⌨ Program input (stdin)").id_salt("stdin").show(ui, |ui| {
            ui.label(RichText::new("Text typed here is fed to your program when it reads input.").small().weak());
            ui.add(egui::TextEdit::multiline(&mut self.stdin).desired_rows(2).desired_width(f32::INFINITY).code_editor());
        });
    }

    fn output_view(&self, ui: &mut egui::Ui) {
        let Some(r) = &self.output else {
            ui.label(RichText::new("Press ▶ Run to compile and run your code. Output appears here.").weak());
            return;
        };
        if let Some(e) = &r.internal_error {
            ui.label(RichText::new(format!("⚠ {e}")).color(BAD));
            return;
        }
        if !r.compile_ok {
            ui.label(RichText::new("❌ Your code didn't compile").color(BAD).strong().size(17.0));
            ui.label(RichText::new("Read the FIRST error first. The line after `-->` tells you main.rs:line:column.").small().weak());
            mono(ui, &r.compiler_output, BAD);
            return;
        }
        if let Some((passed, problems)) = &r.check {
            if *passed {
                ui.label(RichText::new("🎉 Exercise passed! Great work.").color(GOOD).strong().size(18.0));
            } else {
                ui.label(RichText::new("Not quite yet:").color(WARN).strong().size(17.0));
                for p in problems {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        mono(ui, p, WARN);
                    });
                }
            }
        }
        if r.timed_out {
            ui.label(RichText::new("⏱ Stopped after 5 seconds — is there a loop that never ends?").color(WARN));
        } else if let Some(code) = r.exit_code
            && code != 0
        {
            ui.label(RichText::new(format!("💥 The program crashed (exit code {code}). Rust calls this a panic — see the message below.")).color(BAD));
        }
        if !r.compiler_output.trim().is_empty() {
            egui::CollapsingHeader::new(RichText::new("⚠ Compiler warnings (your code still ran)").color(WARN)).id_salt("warnings").show(ui, |ui| {
                mono(ui, &r.compiler_output, WARN);
            });
        }
        if !self.output_was_check || !r.stdout.is_empty() {
            ui.label(RichText::new("Output:").strong());
            if r.stdout.is_empty() {
                ui.label(RichText::new("(your program printed nothing)").weak());
            } else {
                egui::Frame::new().fill(CODE_BG).inner_margin(8.0).corner_radius(4.0).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    mono(ui, &r.stdout, Color32::from_gray(225));
                });
            }
        }
        if !r.stderr.trim().is_empty() {
            mono(ui, &r.stderr, BAD);
        }
    }
}

/// Use Windows' own fonts when available: nicer text, and they cover the symbols and emoji
/// the lessons use (egui's built-in fonts don't).
fn setup_fonts(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};
    let dir = std::env::var_os("WINDIR").map(|w| PathBuf::from(w).join("Fonts")).unwrap_or_default();
    let mut fonts = FontDefinitions::default();
    let mut load = |name: &str, files: &[&str]| -> bool {
        let Some(bytes) = files.iter().find_map(|f| std::fs::read(dir.join(f)).ok()) else { return false };
        fonts.font_data.insert(name.to_string(), std::sync::Arc::new(FontData::from_owned(bytes)));
        true
    };
    let ui_font = load("ui", &["segoeui.ttf"]);
    let code_font = load("code", &["CascadiaMono.ttf", "consola.ttf"]);
    let symbols = load("symbols", &["seguisym.ttf"]);
    let emoji = load("emoji", &["seguiemj.ttf"]);
    for (family, primary) in [(FontFamily::Proportional, ui_font.then_some("ui")), (FontFamily::Monospace, code_font.then_some("code"))] {
        let list = fonts.families.entry(family).or_default();
        if let Some(p) = primary {
            list.insert(0, p.to_string());
        }
        if symbols {
            list.push("symbols".into());
        }
        if emoji {
            list.push("emoji".into());
        }
    }
    ctx.set_fonts(fonts);
}

fn mono(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.add(egui::Label::new(RichText::new(text.trim_end()).monospace().color(color)).selectable(true));
}

/// A read-only highlighted code block. Returns true if "Try it" was clicked.
fn code_block(ui: &mut egui::Ui, code: &str, runnable: bool) -> bool {
    let mut tried = false;
    egui::Frame::new().fill(CODE_BG).corner_radius(6.0).inner_margin(10.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
        let job = highlight(ui.ctx(), ui.style(), &theme, code, "rs");
        ui.add(egui::Label::new(job).selectable(true));
        ui.horizontal(|ui| {
            if runnable && ui.small_button("▶ Try it").on_hover_text("Open in the Playground and run it").clicked() {
                tried = true;
            }
            if ui.small_button("📋 Copy").clicked() {
                ui.ctx().copy_text(code.to_string());
            }
        });
    });
    tried
}

fn project_card(
    ui: &mut egui::Ui,
    cache: &mut CommonMarkCache,
    project: &Project,
    msg: &mut Option<(String, String)>,
    confirm_reset: &mut Option<String>,
) {
    let dir = projects::dir_of(project);
    let existing = projects::existing_files(project);
    let total = project.files.len();
    let mut new_msg: Option<String> = None;

    egui::Frame::group(ui.style()).fill(Color32::from_rgb(32, 40, 34)).inner_margin(12.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(RichText::new(format!("🛠 Project: {}", project.title)).strong().size(18.0).color(GOOD));
        CommonMarkViewer::new().show(ui, cache, &project.description);
        ui.label(RichText::new(format!("📁 {}", dir.display())).monospace().small().weak());

        ui.horizontal_wrapped(|ui| {
            let label = if existing == 0 {
                "📁 Create project".to_string()
            } else if existing < total {
                format!("➕ Add {} new file(s)", total - existing)
            } else {
                "✔ Files ready".to_string()
            };
            if ui.add_enabled(existing < total, egui::Button::new(label)).clicked() {
                match projects::create_missing(project) {
                    Ok(n) => new_msg = Some(format!("Created {n} file(s). Your existing files were not touched.")),
                    Err(e) => new_msg = Some(format!("⚠ Couldn't create files: {e}")),
                }
            }
            let ready = existing > 0;
            if ui.add_enabled(ready, egui::Button::new("💻 Open in VS Code")).on_hover_text("Needs VS Code's `code` command (installed with VS Code by default)").clicked()
                && let Err(e) = projects::open_in_vscode(&dir)
            {
                new_msg = Some(format!("⚠ Couldn't start VS Code: {e}"));
            }
            if ui.add_enabled(ready, egui::Button::new("📂 Open folder")).clicked() {
                let _ = projects::open_folder(&dir);
            }
            for (label, command) in [("▶ cargo run", "cargo run"), ("🧪 cargo test", "cargo test")] {
                if ui.add_enabled(ready, egui::Button::new(label)).on_hover_text(format!("Runs `{command}` in a new terminal window")).clicked()
                    && let Err(e) = projects::run_in_terminal(&dir, command)
                {
                    new_msg = Some(format!("⚠ Couldn't open a terminal: {e}"));
                }
            }
        });

        egui::CollapsingHeader::new(format!("📄 This step's starting files ({total})")).id_salt("files").show(ui, |ui| {
            for (path, contents) in &project.files {
                ui.label(RichText::new(path).monospace().strong());
                code_block(ui, contents, false);
            }
            ui.horizontal(|ui| {
                let armed = confirm_reset.as_deref() == Some(project.folder.as_str());
                let text = if armed { "⚠ Click again to overwrite (a backup is kept)" } else { "↺ Reset project to this step's files" };
                if ui.add_enabled(existing > 0, egui::Button::new(text)).clicked() {
                    if armed {
                        *confirm_reset = None;
                        match projects::reset(project) {
                            Ok(backup) => new_msg = Some(format!("Reset done. Your previous files were backed up to {}", backup.display())),
                            Err(e) => new_msg = Some(format!("⚠ Reset failed: {e}")),
                        }
                    } else {
                        *confirm_reset = Some(project.folder.clone());
                    }
                }
            });
        });

        if let Some(text) = new_msg {
            *msg = Some((project.folder.clone(), text));
        }
        if let Some((folder, text)) = msg.as_ref()
            && *folder == project.folder
        {
            ui.label(RichText::new(text).color(WARN));
        }
    });
}

fn quiz_card(ui: &mut egui::Ui, cache: &mut CommonMarkCache, quiz: &crate::course::Quiz, answer: &mut Option<usize>) {
    egui::Frame::group(ui.style()).fill(Color32::from_rgb(30, 36, 48)).inner_margin(12.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(RichText::new("❓ Quick check").strong().color(Color32::from_rgb(130, 170, 255)));
        CommonMarkViewer::new().show(ui, cache, &quiz.question);
        for (i, opt) in quiz.options.iter().enumerate() {
            let text = match opt.strip_prefix('`').and_then(|o| o.strip_suffix('`')) {
                Some(code) => RichText::new(code).monospace(),
                None => RichText::new(opt),
            };
            if ui.radio(*answer == Some(i), text).clicked() {
                *answer = Some(i);
            }
        }
        if let Some(a) = *answer {
            if a == quiz.correct {
                ui.label(RichText::new("✅ Correct!").color(GOOD).strong());
                if !quiz.explain.is_empty() {
                    CommonMarkViewer::new().show(ui, cache, &quiz.explain);
                }
            } else {
                ui.label(RichText::new("❌ Not quite — have another think and pick again.").color(BAD));
            }
        }
    });
}

/// Code editor with line numbers, syntax highlighting, 4-space tabs and auto-indent.
fn code_editor(ui: &mut egui::Ui, code: &mut String, id: egui::Id) -> bool {
    let theme = CodeTheme::from_memory(ui.ctx(), ui.style());
    let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _wrap: f32| {
        let job = highlight(ui.ctx(), ui.style(), &theme, buf.as_str(), "rs");
        ui.ctx().fonts_mut(|f| f.layout_job(job))
    };
    let old_len = code.chars().count();
    let mut changed = false;
    egui::ScrollArea::both().id_salt(id).auto_shrink(false).show(ui, |ui| {
        ui.horizontal_top(|ui| {
            let lines = code.split('\n').count();
            let numbers: String = (1..=lines).map(|n| format!("{n:>3}\n")).collect();
            ui.add_space(2.0);
            ui.vertical(|ui| {
                ui.label(RichText::new(numbers.trim_end()).monospace().color(Color32::from_gray(90)));
            });
            let mut out = egui::TextEdit::multiline(code)
                .id(id)
                .code_editor()
                .lock_focus(true)
                .frame(egui::Frame::NONE)
                .margin(egui::Margin::symmetric(4, 2))
                .desired_width(f32::INFINITY)
                .desired_rows(24)
                .layouter(&mut layouter)
                .show(ui);
            if !out.response.changed() {
                return;
            }
            changed = true;
            let Some(range) = out.state.cursor.char_range() else { return };
            let mut cursor = range.primary.index.0;
            if code.contains('\t') {
                let tabs_before = code.chars().take(cursor).filter(|&c| c == '\t').count();
                *code = code.replace('\t', "    ");
                cursor += tabs_before * 3;
            }
            // Auto-indent: a single newline was typed, so copy the previous line's indentation.
            if code.chars().count() == old_len + 1 && cursor > 0 && code.chars().nth(cursor - 1) == Some('\n') {
                let byte = code.char_indices().nth(cursor).map_or(code.len(), |(b, _)| b);
                let line_start = code[..byte - 1].rfind('\n').map_or(0, |i| i + 1);
                let prev = &code[line_start..byte - 1];
                let mut indent: String = prev.chars().take_while(|&c| c == ' ').collect();
                if prev.trim_end().ends_with(['{', '(', '[']) {
                    indent.push_str("    ");
                }
                code.insert_str(byte, &indent);
                cursor += indent.len();
            }
            out.state.cursor.set_char_range(Some(egui::text::CCursorRange::one(egui::text::CCursor::new(cursor))));
            out.state.store(ui.ctx(), out.response.id);
        });
    });
    changed
}

impl eframe::App for RustQuest {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll();

        let (run, check) = ctx.input_mut(|i| {
            let check = i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Enter);
            let run = i.consume_key(Modifiers::CTRL, Key::Enter) || i.consume_key(Modifiers::NONE, Key::F5);
            (run, check)
        });
        if check && matches!(self.tab, EditorTab::Exercise(_)) {
            self.start_job(&ctx, true);
        } else if run || check {
            self.start_job(&ctx, false);
        }

        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(4.0);
            self.top_bar(ui);
            ui.add_space(2.0);
        });
        egui::Panel::left("course").resizable(true).default_size(280.0).min_size(200.0).show(ui, |ui| self.sidebar(ui));
        let width = ui.available_width();
        egui::Panel::right("editor").resizable(true).default_size(width * 0.42).min_size(340.0).show(ui, |ui| self.editor_panel(ui));
        egui::CentralPanel::default().show(ui, |ui| self.lesson_view(ui));

        let zoom = ctx.zoom_factor();
        if (zoom - self.progress.zoom).abs() > 0.001 {
            self.progress.zoom = zoom;
            self.dirty = true;
        }
        if self.dirty && self.last_save.elapsed() > Duration::from_secs(2) {
            self.progress.save();
            self.dirty = false;
            self.last_save = Instant::now();
        } else if self.dirty {
            ctx.request_repaint_after(Duration::from_secs(2));
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.progress.save();
    }
}
