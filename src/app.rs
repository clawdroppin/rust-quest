use crate::ai;
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

/// Ready-made questions for the AI helper.
#[derive(Clone, Copy)]
enum Ask {
    ExplainError,
    Hint,
    Review,
    ExplainLesson,
}

impl Ask {
    const ALL: [Ask; 4] = [Ask::ExplainError, Ask::Hint, Ask::Review, Ask::ExplainLesson];

    fn label(self) -> &'static str {
        match self {
            Ask::ExplainError => "Explain my error",
            Ask::Hint => "Give me a hint",
            Ask::Review => "Review my code",
            Ask::ExplainLesson => "Explain this lesson simply",
        }
    }

    fn question(self) -> &'static str {
        match self {
            Ask::ExplainError => "Can you explain the error or failed check I'm getting, and how I could fix it?",
            Ask::Hint => "I'm stuck. Can you give me a hint for the next step, without giving away the whole answer?",
            Ask::Review => "Can you review my code? What's good, and what could be clearer or more idiomatic?",
            Ask::ExplainLesson => "Can you explain the main idea of this lesson in simple words, with a small example?",
        }
    }
}

/// State of the AI helper window.
#[derive(Default)]
struct AiPanel {
    open: bool,
    chat: Vec<ai::Message>,
    input: String,
    stream: Option<ai::Stream>,
    models: Vec<String>,
    models_rx: Option<Receiver<Result<Vec<String>, String>>>,
    /// Result of the last connection test: (ok, message).
    status: Option<(bool, String)>,
    show_settings: bool,
}

/// Which lesson and editor tab a compile job was started from.
type JobOrigin = (usize, usize, EditorTab);

pub struct RustQuest {
    course: Vec<Module>,
    progress: Progress,
    module: usize,
    lesson: usize,
    tab: EditorTab,
    stdin: String,
    output: Option<RunResult>,
    output_was_check: bool,
    pending: Option<(Receiver<RunResult>, Option<String>, JobOrigin)>,
    /// Where we were in the other tab (course or guided builds), to return there.
    other_track: Option<(usize, usize)>,
    ai: AiPanel,
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
            other_track: None,
            ai: AiPanel::default(),
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
        let last = if app.progress.in_builds { &app.progress.last_build } else { &app.progress.last_lesson };
        let (m, l) = last.as_ref().and_then(|id| app.find_lesson(id)).unwrap_or((0, 0));
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
        let id = Some(self.current().id.clone());
        let guided = self.course[m].guided;
        if guided {
            self.progress.last_build = id;
        } else {
            self.progress.last_lesson = id;
        }
        self.progress.in_builds = guided;
        self.dirty = true;
        self.scroll_to_top = true;
        self.select_tab(if has_ex { EditorTab::Exercise(0) } else { EditorTab::Playground });
    }

    fn in_builds(&self) -> bool {
        self.course[self.module].guided
    }

    /// Switch between the course and the guided builds, returning to where the learner was.
    fn switch_track(&mut self) {
        let to_builds = !self.in_builds();
        let remembered = if to_builds { &self.progress.last_build } else { &self.progress.last_lesson };
        let dest = self
            .other_track
            .filter(|&(m, _)| self.course[m].guided == to_builds)
            .or_else(|| remembered.as_ref().and_then(|id| self.find_lesson(id)))
            .or_else(|| self.course.iter().position(|m| m.guided == to_builds && !m.lessons.is_empty()).map(|m| (m, 0)));
        if let Some((m, l)) = dest {
            self.other_track = Some((self.module, self.lesson));
            self.open_lesson(m, l);
        }
    }

    /// The code editor is always shown in the course. In the builds, where the coding
    /// happens in VS Code, it's hidden unless the learner asks for it.
    fn show_editor(&self) -> bool {
        !self.in_builds() || self.progress.show_editor_in_builds || !self.current().exercises.is_empty()
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

    /// Modules in the same tab (course or builds) as the current one.
    fn same_track(&self, mi: usize) -> bool {
        self.course[mi].guided == self.in_builds() && !self.course[mi].lessons.is_empty()
    }

    fn next_lesson(&self) -> Option<(usize, usize)> {
        let (m, l) = (self.module, self.lesson);
        if l + 1 < self.course[m].lessons.len() {
            return Some((m, l + 1));
        }
        (m + 1..self.course.len()).find(|&mi| self.same_track(mi)).map(|mi| (mi, 0))
    }

    fn prev_lesson(&self) -> Option<(usize, usize)> {
        let (m, l) = (self.module, self.lesson);
        if l > 0 {
            return Some((m, l - 1));
        }
        (0..m).rev().find(|&mi| self.same_track(mi)).map(|mi| (mi, self.course[mi].lessons.len() - 1))
    }

    /// (completed, total) lessons of the main course or of the builds.
    fn track_progress(&self, guided: bool) -> (usize, usize) {
        let lessons = self.course.iter().filter(|m| m.guided == guided).flat_map(|m| &m.lessons);
        lessons.fold((0, 0), |(done, total), l| (done + self.progress.completed_lessons.contains(&l.id) as usize, total + 1))
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
        self.pending = Some((rx, key, (self.module, self.lesson, self.tab)));
    }

    fn poll(&mut self) {
        if let Some(rx) = &self.rustc_rx
            && let Ok(r) = rx.try_recv()
        {
            self.rustc = Some(r);
            self.rustc_rx = None;
        }
        self.poll_ai();
        let Some((rx, key, origin)) = &self.pending else { return };
        let Ok(result) = rx.try_recv() else { return };
        let origin = *origin;
        if let (Some(key), Some((true, _))) = (key, &result.check) {
            self.progress.passed_exercises.insert(key.clone());
            // The learner may have moved to another lesson while it compiled, so find the
            // exercise's own lesson from its key ("lesson-id#index").
            let lesson_id = key.rsplit_once('#').map_or(key.as_str(), |(id, _)| id);
            if let Some((m, l)) = self.find_lesson(lesson_id) {
                let lesson = &self.course[m].lessons[l];
                if (0..lesson.exercises.len()).all(|i| self.progress.passed_exercises.contains(&lesson.exercise_key(i))) {
                    self.progress.completed_lessons.insert(lesson.id.clone());
                }
            }
            self.dirty = true;
        }
        // Only show the result where it was started; elsewhere it would be confusing.
        if origin == (self.module, self.lesson, self.tab) {
            self.output = Some(result);
        }
        self.pending = None;
    }

    // ---------------------------------------------------------------- AI helper

    fn poll_ai(&mut self) {
        if let Some(rx) = &self.ai.models_rx
            && let Ok(result) = rx.try_recv()
        {
            self.ai.models_rx = None;
            match result {
                Ok(models) if models.is_empty() => {
                    self.ai.status = Some((false, "Connected, but no model is loaded. Load one in LM Studio first.".into()))
                }
                Ok(models) => {
                    self.ai.status = Some((true, format!("Connected: {} model(s) available.", models.len())));
                    if !models.contains(&self.progress.ai.model) {
                        self.progress.ai.model = models[0].clone();
                        self.dirty = true;
                    }
                    self.ai.models = models;
                }
                Err(e) => self.ai.status = Some((false, e)),
            }
        }
        let Some(stream) = &self.ai.stream else { return };
        let mut finished = false;
        while let Ok(event) = stream.rx.try_recv() {
            match event {
                ai::Event::Delta(text) => match self.ai.chat.last_mut() {
                    Some(m) if m.role == "assistant" => m.content.push_str(&text),
                    _ => self.ai.chat.push(ai::Message { role: "assistant".into(), content: text, sent: None }),
                },
                ai::Event::Done => finished = true,
                ai::Event::Error(e) => {
                    self.ai.chat.push(ai::Message { role: "error".into(), content: e, sent: None });
                    finished = true;
                }
            }
        }
        if finished {
            self.ai.stream = None;
        }
    }

    /// What the AI should know about where the learner is: lesson, task, code, last result.
    fn ai_context(&self) -> String {
        let module = &self.course[self.module];
        let lesson = self.current();
        let mut ctx = format!("Context from the app (the learner doesn't see this):\nI'm on \"{}\" › \"{}\".\n", module.title, lesson.title);
        if let EditorTab::Exercise(i) = self.tab {
            let ex = &lesson.exercises[i];
            ctx.push_str(&format!("Exercise \"{}\":\n{}\n", ex.title, ai::shorten(&ex.prompt, 1500)));
        }
        if self.show_editor() {
            let key = match self.tab {
                EditorTab::Exercise(i) => self.progress.code.get(&lesson.exercise_key(i)).cloned().or_else(|| Some(lesson.exercises[i].starter.clone())),
                EditorTab::Playground => Some(self.progress.playground.clone()),
            };
            if let Some(code) = key {
                ctx.push_str(&format!("My code in the editor:\n```rust\n{}\n```\n", ai::shorten(&code, 6000)));
            }
            if let Some(r) = &self.output {
                if !r.compile_ok {
                    ctx.push_str(&format!("The compiler said:\n```text\n{}\n```\n", ai::shorten(&r.compiler_output, 3000)));
                } else {
                    if let Some((false, problems)) = &r.check {
                        ctx.push_str(&format!("The checker said:\n{}\n", ai::shorten(&problems.join("\n"), 2000)));
                    }
                    if !r.stdout.is_empty() {
                        ctx.push_str(&format!("My program printed:\n```text\n{}\n```\n", ai::shorten(&r.stdout, 1500)));
                    }
                    if !r.stderr.trim().is_empty() {
                        ctx.push_str(&format!("Error output:\n```text\n{}\n```\n", ai::shorten(&r.stderr, 1500)));
                    }
                }
            }
        }
        for block in &lesson.blocks {
            if let Block::Project(project) = block
                && projects::existing_files(project) > 0
            {
                ctx.push_str(&format!(
                    "This step has a Cargo project ({}). My files on disk right now:\n{}",
                    project.title,
                    projects::read_sources(project, 12_000)
                ));
                break;
            }
        }
        ctx
    }

    fn ai_send(&mut self, ctx: &egui::Context, question: String) {
        let question = question.trim().to_string();
        if question.is_empty() || self.ai.stream.is_some() {
            return;
        }
        let sent = self.progress.ai.include_context.then(|| format!("{}\n\nMy question: {question}", self.ai_context()));
        self.ai.chat.push(ai::Message { role: "user".into(), content: question, sent });
        let history: Vec<ai::Message> = self.ai.chat.iter().filter(|m| m.role != "error").cloned().collect();
        self.ai.stream = Some(ai::start_chat(&self.progress.ai, &history, ctx.clone()));
        self.ai.open = true;
    }

    fn ai_window(&mut self, ctx: &egui::Context) {
        let mut open = self.ai.open;
        let mut send: Option<String> = None;
        // The chat area gets a fixed share of the screen: inside a window, "all the
        // available height" would be the whole screen.
        let screen = ctx.content_rect();
        let chat_height = (screen.height() * 0.45).clamp(180.0, 560.0);
        egui::Window::new("🤖 AI Helper")
            .open(&mut open)
            .default_pos(screen.right_top() + egui::vec2(-500.0, 56.0))
            .default_width(480.0)
            .min_width(340.0)
            .show(ctx, |ui| {
                // Open while it isn't working yet: on a failed connection, or before a model was ever chosen.
                let not_set_up = match &self.ai.status {
                    Some((ok, _)) => !ok,
                    None => self.progress.ai.model.is_empty(),
                };
                let settings_open = self.ai.show_settings || not_set_up;
                egui::CollapsingHeader::new("⚙ Connection")
                    .id_salt("ai_settings")
                    .open(Some(settings_open))
                    .show(ui, |ui| self.ai_settings(ui));
                if ui.small_button(if settings_open { "Hide connection settings" } else { "Connection settings" }).clicked() {
                    self.ai.show_settings = !settings_open;
                }
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("ai_chat")
                    .max_height(chat_height)
                    .auto_shrink(false)
                    .stick_to_bottom(true)
                    .show(ui, |ui| self.ai_chat(ui));
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    for ask in Ask::ALL {
                        if ui.add_enabled(self.ai.stream.is_none(), egui::Button::new(ask.label()).small()).clicked() {
                            send = Some(ask.question().to_string());
                        }
                    }
                });
                let input_id = ai_input_id();
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.ai.input)
                        .id(input_id)
                        .desired_rows(2)
                        .desired_width(f32::INFINITY)
                        .hint_text("Ask anything… (Ctrl+Enter to send)"),
                );
                let ctrl_enter = response.has_focus() && ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::Enter));
                ui.horizontal(|ui| {
                    if self.ai.stream.is_some() {
                        ui.spinner();
                        if ui.button("■ Stop").clicked() {
                            self.ai.stream = None;
                        }
                    } else if ui.button(RichText::new("Send").strong()).clicked() || ctrl_enter {
                        send = Some(std::mem::take(&mut self.ai.input));
                    }
                    if ui.add_enabled(!self.ai.chat.is_empty() && self.ai.stream.is_none(), egui::Button::new("Clear chat")).clicked() {
                        self.ai.chat.clear();
                    }
                    ui.checkbox(&mut self.progress.ai.include_context, "Include my code & errors")
                        .on_hover_text("Sends the current lesson, your code, the last compiler/checker output and, for project steps, your project files.");
                });
            });
        self.ai.open = open;
        if let Some(question) = send {
            self.ai_send(ctx, question);
        }
    }

    fn ai_settings(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(
                "Use a free AI model running on your own computer, e.g. with LM Studio: download a model \
                 (like a Qwen Coder or Llama 3 model), then start the local server in its Developer tab. \
                 Your code is only sent to the address below.",
            )
            .small(),
        );
        ui.horizontal_wrapped(|ui| {
            for (name, url) in ai::PRESETS {
                if ui.selectable_label(self.progress.ai.base_url == url, name).clicked() {
                    self.progress.ai.base_url = url.to_string();
                    self.ai.models.clear();
                    self.ai.status = None;
                    self.dirty = true;
                }
            }
        });
        ui.horizontal(|ui| {
            ui.label("Server");
            if ui.add(egui::TextEdit::singleline(&mut self.progress.ai.base_url).desired_width(240.0)).changed() {
                self.dirty = true;
            }
        });
        ui.horizontal(|ui| {
            ui.label("Model");
            let shown = if self.progress.ai.model.is_empty() { "(first available)" } else { self.progress.ai.model.as_str() };
            egui::ComboBox::from_id_salt("ai_model").selected_text(ai::shorten(shown, 34)).width(240.0).show_ui(ui, |ui| {
                for m in &self.ai.models {
                    if ui.selectable_value(&mut self.progress.ai.model, m.clone(), m).clicked() {
                        self.dirty = true;
                    }
                }
                if self.ai.models.is_empty() {
                    ui.label("Press 'Connect' to list the models");
                }
            });
        });
        ui.horizontal(|ui| {
            if self.ai.models_rx.is_some() {
                ui.spinner();
                ui.label("Connecting…");
            } else if ui.button("🔌 Connect").clicked() {
                self.ai.models_rx = Some(ai::list_models_async(self.progress.ai.base_url.clone(), ui.ctx().clone()));
            }
            if ui.checkbox(&mut self.progress.ai.hints_only, "Hints, not full solutions").changed() {
                self.dirty = true;
            }
        });
        if let Some((ok, msg)) = &self.ai.status {
            ui.label(RichText::new(msg).color(if *ok { GOOD } else { WARN }).small());
        }
    }

    fn ai_chat(&mut self, ui: &mut egui::Ui) {
        if self.ai.chat.is_empty() {
            ui.label(
                RichText::new(
                    "Ask about the lesson, your code, an error, or your project. The helper sees what you're \
                     working on (if \"Include my code & errors\" is ticked). It can be wrong: treat it like a \
                     helpful classmate, and check what it says by running your code.",
                )
                .weak(),
            );
        }
        let streaming = self.ai.stream.is_some();
        let last = self.ai.chat.len().saturating_sub(1);
        for (i, msg) in self.ai.chat.iter().enumerate() {
            ui.push_id(("ai_msg", i), |ui| match msg.role.as_str() {
                "user" => {
                    ui.label(RichText::new("You").strong().color(ACCENT));
                    ui.label(&msg.content);
                }
                "assistant" => {
                    ui.label(RichText::new("AI helper").strong().color(Color32::from_rgb(130, 170, 255)));
                    let (thinking, answer, still_thinking) = ai::split_thinking(&msg.content);
                    if let Some(thinking) = thinking {
                        egui::CollapsingHeader::new(RichText::new(if still_thinking { "Thinking…" } else { "Thoughts" }).weak().small())
                            .id_salt("thinking")
                            .show(ui, |ui| ui.label(RichText::new(thinking).weak().small()));
                    }
                    CommonMarkViewer::new().show(ui, &mut self.md_cache, answer);
                    if !(streaming && i == last) && ui.small_button("📋 Copy").clicked() {
                        ui.ctx().copy_text(answer.to_string());
                    }
                }
                _ => {
                    ui.label(RichText::new(format!("⚠ {}", msg.content)).color(WARN));
                }
            });
            ui.add_space(8.0);
        }
        if streaming && self.ai.chat.last().is_some_and(|m| m.role == "user") {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(RichText::new("Waiting for the model… (the first answer can take a while while it loads)").weak());
            });
        }
    }

    // ---------------------------------------------------------------- top bar

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🦀 Rust Quest").size(22.0).strong().color(ACCENT));
            ui.separator();
            let builds = self.in_builds();
            let (done, total) = self.track_progress(builds);
            ui.label(format!("{done} / {total} {}", if builds { "build steps" } else { "lessons" }));
            ui.add(egui::ProgressBar::new(done as f32 / total.max(1) as f32).desired_width(140.0).fill(ACCENT));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let ai_label = if self.ai.stream.is_some() { "🤖 AI Helper …" } else { "🤖 AI Helper" };
                if ui.selectable_label(self.ai.open, ai_label).on_hover_text("Ask a local AI model (LM Studio, Ollama…) for help").clicked() {
                    self.ai.open = !self.ai.open;
                }
                if builds && self.current().exercises.is_empty() {
                    let label = if self.progress.show_editor_in_builds { "Hide editor" } else { "Show editor" };
                    if ui.button(label).on_hover_text("In the builds you code in VS Code; the in-app editor is optional").clicked() {
                        self.progress.show_editor_in_builds = !self.progress.show_editor_in_builds;
                        self.dirty = true;
                    }
                }
                ui.separator();
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
        let builds = self.in_builds();
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.selectable_label(!builds, RichText::new("📚 Course").strong()).clicked() && builds {
                self.switch_track();
            }
            if ui
                .selectable_label(builds, RichText::new("🛠 Guided Builds").strong())
                .on_hover_text("Build real, useful apps step by step, side by side with VS Code")
                .clicked()
                && !builds
            {
                self.switch_track();
            }
        });
        ui.separator();
        let builds = self.in_builds();
        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            for (mi, module) in self.course.iter().enumerate() {
                if module.guided != builds {
                    continue;
                }
                let done = module.lessons.iter().filter(|l| self.progress.completed_lessons.contains(&l.id)).count();
                let total = module.lessons.len();
                let header = RichText::new(format!("{}  ({done}/{total})", module.title)).strong();
                egui::CollapsingHeader::new(header)
                    .id_salt(("module", mi))
                    .default_open(mi == self.module || mi <= 1 || builds)
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
                let msg = if module_ref.guided {
                    "🎉 You've finished every guided build! Time to dream up an app of your own: you have all the pieces."
                } else {
                    "🎉 You've finished the whole course! Try the 🛠 Guided Builds tab next, or build something of your own."
                };
                ui.label(RichText::new(msg).color(GOOD));
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
            let ask = egui::ScrollArea::vertical()
                .id_salt("output")
                .auto_shrink(false)
                .show(ui, |ui| self.output_view(ui))
                .inner;
            if ask {
                self.ai_send(&ui.ctx().clone(), Ask::ExplainError.question().to_string());
            }
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

    /// Returns true if the learner asked the AI helper about this result.
    fn output_view(&self, ui: &mut egui::Ui) -> bool {
        let Some(r) = &self.output else {
            ui.label(RichText::new("Press ▶ Run to compile and run your code. Output appears here.").weak());
            return false;
        };
        if let Some(e) = &r.internal_error {
            ui.label(RichText::new(format!("⚠ {e}")).color(BAD));
            return false;
        }
        let ask_button = |ui: &mut egui::Ui| {
            ui.add_enabled(self.ai.stream.is_none(), egui::Button::new("🤖 Ask the AI helper about this").small())
                .on_hover_text("Needs a local AI model, see the AI Helper button at the top")
                .clicked()
        };
        let mut ask = false;
        if !r.compile_ok {
            ui.horizontal(|ui| {
                ui.label(RichText::new("❌ Your code didn't compile").color(BAD).strong().size(17.0));
                ask = ask_button(ui);
            });
            ui.label(RichText::new("Read the FIRST error first. The line after `-->` tells you main.rs:line:column.").small().weak());
            mono(ui, &r.compiler_output, BAD);
            return ask;
        }
        if let Some((passed, problems)) = &r.check {
            if *passed {
                ui.label(RichText::new("🎉 Exercise passed! Great work.").color(GOOD).strong().size(18.0));
            } else {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Not quite yet:").color(WARN).strong().size(17.0));
                    ask = ask_button(ui);
                });
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
        ask
    }
}

fn ai_input_id() -> egui::Id {
    egui::Id::unique("ai_input")
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

        let editor = self.show_editor();
        // Ctrl+Enter in the AI input sends the question instead of running the code.
        let typing_to_ai = ctx.memory(|m| m.focused()) == Some(ai_input_id());
        if editor && !typing_to_ai {
            let (run, check) = ctx.input_mut(|i| {
                let check = i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Enter);
                let run = i.consume_key(Modifiers::CTRL, Key::Enter) || i.consume_key(Modifiers::NONE, Key::F5);
                (run, check)
            });
            let ready = matches!(self.rustc, Some(Ok(_))) && self.pending.is_none();
            if ready && check && matches!(self.tab, EditorTab::Exercise(_)) {
                self.start_job(&ctx, true);
            } else if ready && (run || check) {
                self.start_job(&ctx, false);
            }
        }

        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(4.0);
            self.top_bar(ui);
            ui.add_space(2.0);
        });
        egui::Panel::left("course").resizable(true).default_size(280.0).min_size(200.0).show(ui, |ui| self.sidebar(ui));
        if editor {
            let width = ui.available_width();
            egui::Panel::right("editor").resizable(true).default_size(width * 0.42).min_size(300.0).show(ui, |ui| self.editor_panel(ui));
        }
        egui::CentralPanel::default().show(ui, |ui| self.lesson_view(ui));
        if self.ai.open {
            self.ai_window(&ctx);
        }

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
