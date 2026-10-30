@@lesson g-ft3 Focus Timer 3: Stats, Charts and Saving
@@text
A timer you use every day should show you how you're doing. In this last step you'll add:

- a **log** of every finished focus session (which day, how long, which task),
- **today's minutes**, a **streak** ("5 days in a row!") and a **7-day bar chart**,
- **saving** the log and the settings to a JSON file, so nothing is lost when you close the app.

Like before, the logic goes in its own module, `src/stats.rs`, tested on its own, and then the GUI uses it.

@@project focus_timer Focus Timer, step 3: stats and saving
Adds `src/stats.rs`: the session log (with methods for you to write), and saving. `src/main.rs` gets one new line, `mod stats;`. The starting `src/app.rs` is the finished window from step 2.
@@file Cargo.toml
[package]
name = "focus_timer"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

@@file src/main.rs
mod app;
mod stats;
mod timer;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 640.0]).with_min_inner_size([340.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Focus Timer", options, Box::new(|_cc| Ok(Box::new(app::FocusApp::new()))))
}

@@file src/app.rs
//! The window: drawing the timer, and turning clicks into calls on the model.

use crate::timer::{Phase, Settings, Timer, mm_ss};
use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, Vec2, ViewportCommand};
use std::time::{Duration, Instant};

const FOCUS_COLOR: Color32 = Color32::from_rgb(235, 100, 80);
const BREAK_COLOR: Color32 = Color32::from_rgb(90, 190, 130);

pub struct FocusApp {
    settings: Settings,
    timer: Timer,
    task: String,
    /// The window title we set last, so we only send a new one when it changes.
    title: String,
}

impl FocusApp {
    pub fn new() -> Self {
        let settings = Settings::default();
        let timer = Timer::new(&settings);
        FocusApp { settings, timer, task: String::new(), title: String::new() }
    }

    fn phase_color(&self) -> Color32 {
        if self.timer.phase == Phase::Focus { FOCUS_COLOR } else { BREAK_COLOR }
    }

    /// A phase just ran out: get the user's attention.
    fn phase_finished(&mut self, ctx: &egui::Context, _finished: Phase) {
        // Flash the taskbar button, and bring the window to the front.
        ctx.send_viewport_cmd(ViewportCommand::RequestUserAttention(egui::UserAttentionType::Informational));
        ctx.send_viewport_cmd(ViewportCommand::Focus);
    }

    /// Show the time in the window title, so it's visible in the taskbar.
    fn update_title(&mut self, ctx: &egui::Context, now: Instant) {
        let title = if self.timer.is_running() {
            format!("{} · {}", mm_ss(self.timer.remaining(now)), self.timer.phase.label())
        } else {
            "Focus Timer".to_string()
        };
        if title != self.title {
            ctx.send_viewport_cmd(ViewportCommand::Title(title.clone()));
            self.title = title;
        }
    }

    // ---------- drawing ----------

    /// The big ring: a grey circle, with a coloured arc for the time already done.
    fn ring(&self, ui: &mut egui::Ui, now: Instant) {
        let size = ui.available_width().min(260.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = size / 2.0 - 12.0;
        painter.circle_stroke(center, radius, Stroke::new(14.0, ui.visuals().faint_bg_color));

        // The arc is a line through many points on the circle, starting at the top.
        let done = self.timer.progress(now);
        let steps = (done * 100.0).ceil() as usize;
        if steps > 0 {
            let points: Vec<_> = (0..=steps)
                .map(|i| {
                    let angle = (i as f32 / 100.0).min(done) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
                    center + radius * Vec2::angled(angle)
                })
                .collect();
            painter.line(points, Stroke::new(14.0, self.phase_color()));
        }

        let text_color = ui.visuals().strong_text_color();
        painter.text(center, Align2::CENTER_CENTER, mm_ss(self.timer.remaining(now)), FontId::proportional(56.0), text_color);
        let label = format!("{} · round {}", self.timer.phase.label(), self.timer.rounds_done + 1);
        painter.text(center + Vec2::new(0.0, 42.0), Align2::CENTER_CENTER, label, FontId::proportional(16.0), self.phase_color());
    }

    fn controls(&mut self, ui: &mut egui::Ui, now: Instant) {
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 290.0).max(0.0) / 2.0); // center the row
            let big = |text: &str| egui::Button::new(RichText::new(text).size(20.0)).min_size(Vec2::new(110.0, 40.0));
            if self.timer.is_running() {
                if ui.add(big("Pause")).clicked() {
                    self.timer.pause(now);
                }
            } else if ui.add(big("Start")).clicked() {
                self.timer.start(now);
            }
            if ui.button("Restart").on_hover_text("Back to the start of this phase").clicked() {
                self.timer.restart_phase();
            }
            if ui.button("Skip").on_hover_text("Go to the next phase").clicked() {
                self.timer.advance(&self.settings, now);
            }
        });
    }

    fn settings_view(&mut self, ui: &mut egui::Ui) {
        let s = &mut self.settings;
        let mut changed = false;
        egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
            let mut row = |ui: &mut egui::Ui, label: &str, value: &mut u32, max: u32| {
                ui.label(label);
                changed |= ui.add(egui::DragValue::new(value).range(1..=max).suffix(" min")).changed();
                ui.end_row();
            };
            row(ui, "Focus", &mut s.focus_minutes, 180);
            row(ui, "Short break", &mut s.short_break_minutes, 60);
            row(ui, "Long break", &mut s.long_break_minutes, 90);
        });
        ui.horizontal(|ui| {
            ui.label("Long break every");
            changed |= ui.add(egui::DragValue::new(&mut s.rounds_before_long_break).range(1..=12)).changed();
            ui.label("rounds");
        });
        changed |= ui.checkbox(&mut s.auto_start, "Start the next phase automatically").changed();
        if changed {
            self.timer.apply_settings(&self.settings);
        }
    }
}

impl eframe::App for FocusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let now = Instant::now();
        if let Some(finished) = self.timer.update(&self.settings, now) {
            self.phase_finished(&ctx, finished);
        }
        self.update_title(&ctx, now);
        if self.timer.is_running() {
            // Keep the countdown moving even when nothing else happens.
            ctx.request_repaint_after(Duration::from_millis(250));
        }

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    self.ring(ui, now);
                    self.controls(ui, now);
                    ui.add_space(6.0);
                    ui.add(egui::TextEdit::singleline(&mut self.task).hint_text("What are you working on?").desired_width(280.0));
                });
                ui.add_space(8.0);
                ui.separator();
                egui::CollapsingHeader::new("Settings").show(ui, |ui| self.settings_view(ui));
            });
        });
    }
}

@@file src/stats.rs
//! The history of finished focus sessions, the numbers we show about it, and saving it all.

use crate::timer::Settings;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A day, counted as days since 1 January 1970. Simple to store, compare and subtract.
pub type Day = u32;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    pub day: Day,
    pub minutes: u32,
    pub task: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Log {
    pub sessions: Vec<Session>,
}

impl Log {
    pub fn add(&mut self, day: Day, minutes: u32, task: &str) {
        todo!()
    }

    pub fn minutes_on(&self, day: Day) -> u32 {
        todo!()
    }

    /// Minutes per day for the `n` days up to and including `today`, oldest first.
    pub fn last_days(&self, today: Day, n: u32) -> Vec<(Day, u32)> {
        todo!()
    }

    /// Minutes per task on one day, sorted by task name.
    pub fn tasks_on(&self, day: Day) -> BTreeMap<&str, u32> {
        todo!()
    }

    /// How many days in a row, ending today, have at least one session.
    /// If today has none yet, the streak ending yesterday still counts.
    pub fn streak(&self, today: Day) -> u32 {
        todo!()
    }
}

/// Today's day number (in UTC; see the lesson for why that's a small simplification).
pub fn today() -> Day {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    (secs / 86_400) as Day
}

/// "Mon", "Tue", … for a day number. Day 0, 1 January 1970, was a Thursday.
pub fn weekday(day: Day) -> &'static str {
    ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][(day % 7) as usize]
}

// ---------- saving ----------

const DATA_FILE: &str = "focus_timer_data.json";

/// Everything the app saves between runs.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub settings: Settings,
    pub log: Log,
}

pub fn load() -> SaveData {
    match std::fs::read_to_string(DATA_FILE) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("Couldn't read {DATA_FILE} ({e}), starting fresh");
            SaveData::default()
        }),
        Err(_) => SaveData::default(), // first run: no file yet
    }
}

pub fn save(data: &SaveData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(DATA_FILE, json).map_err(|e| format!("Couldn't save: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_with(days: &[(Day, u32)]) -> Log {
        let mut log = Log::default();
        for &(day, minutes) in days {
            log.add(day, minutes, "work");
        }
        log
    }

    #[test]
    fn totals_per_day() {
        let log = log_with(&[(10, 25), (10, 25), (11, 50)]);
        assert_eq!(log.minutes_on(10), 50);
        assert_eq!(log.minutes_on(12), 0);
        assert_eq!(log.last_days(12, 3), vec![(10, 50), (11, 50), (12, 0)]);
    }

    #[test]
    fn totals_per_task() {
        let mut log = Log::default();
        log.add(5, 25, "essay");
        log.add(5, 25, "maths");
        log.add(5, 25, "essay");
        log.add(5, 10, "  ");
        log.add(6, 25, "essay");
        let tasks = log.tasks_on(5);
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks["essay"], 50);
        assert_eq!(tasks["(no task)"], 10);
    }

    #[test]
    fn streaks() {
        assert_eq!(log_with(&[]).streak(100), 0);
        assert_eq!(log_with(&[(98, 25), (99, 25), (100, 25)]).streak(100), 3);
        // Nothing yet today: yesterday's streak still counts.
        assert_eq!(log_with(&[(98, 25), (99, 25)]).streak(100), 2);
        // A gap breaks it.
        assert_eq!(log_with(&[(97, 25), (99, 25), (100, 25)]).streak(100), 2);
    }

    #[test]
    fn weekdays() {
        assert_eq!(weekday(0), "Thu");
        assert_eq!(weekday(4), "Mon");
    }
}

@@file src/timer.rs
//! The timer itself: phases, counting down, and what comes next. No GUI code here.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Focus => "Focus",
            Phase::ShortBreak => "Short break",
            Phase::LongBreak => "Long break",
        }
    }
}

/// What the user can change. Saved with the stats.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub focus_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    /// After this many focus sessions, the break is a long one.
    pub rounds_before_long_break: u32,
    /// Start the next phase by itself when one ends.
    pub auto_start: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { focus_minutes: 25, short_break_minutes: 5, long_break_minutes: 15, rounds_before_long_break: 4, auto_start: false }
    }
}

impl Settings {
    pub fn length_of(&self, phase: Phase) -> Duration {
        let minutes = match phase {
            Phase::Focus => self.focus_minutes,
            Phase::ShortBreak => self.short_break_minutes,
            Phase::LongBreak => self.long_break_minutes,
        };
        Duration::from_secs(minutes as u64 * 60)
    }
}

/// A countdown that can be paused. Like a stopwatch, it stores *when* it was started
/// rather than counting frames, so it never drifts.
pub struct Timer {
    pub phase: Phase,
    /// The full length of the current phase.
    pub length: Duration,
    /// Time left at the moment it was last paused (or the full length).
    left_when_paused: Duration,
    /// Set while running.
    started: Option<Instant>,
    /// Focus sessions finished since the last long break.
    pub rounds_done: u32,
}

impl Timer {
    pub fn new(settings: &Settings) -> Self {
        let length = settings.length_of(Phase::Focus);
        Timer { phase: Phase::Focus, length, left_when_paused: length, started: None, rounds_done: 0 }
    }

    pub fn is_running(&self) -> bool {
        self.started.is_some()
    }

    /// Time left at the moment `now`. Never below zero.
    pub fn remaining(&self, now: Instant) -> Duration {
        match self.started {
            Some(started) => self.left_when_paused.saturating_sub(now.duration_since(started)),
            None => self.left_when_paused,
        }
    }

    /// How much of the phase is done, from 0.0 to 1.0.
    pub fn progress(&self, now: Instant) -> f32 {
        if self.length.is_zero() {
            return 1.0;
        }
        1.0 - self.remaining(now).as_secs_f32() / self.length.as_secs_f32()
    }

    pub fn start(&mut self, now: Instant) {
        if self.started.is_none() {
            self.started = Some(now);
        }
    }

    pub fn pause(&mut self, now: Instant) {
        self.left_when_paused = self.remaining(now);
        self.started = None;
    }

    /// Back to the start of the current phase, paused.
    pub fn restart_phase(&mut self) {
        self.left_when_paused = self.length;
        self.started = None;
    }

    /// The phase that follows the current one.
    pub fn next_phase(&self, settings: &Settings) -> Phase {
        match self.phase {
            Phase::Focus if self.rounds_done + 1 >= settings.rounds_before_long_break => Phase::LongBreak,
            Phase::Focus => Phase::ShortBreak,
            Phase::ShortBreak | Phase::LongBreak => Phase::Focus,
        }
    }

    /// Moves on to the next phase, paused (or running, with `auto_start`).
    pub fn advance(&mut self, settings: &Settings, now: Instant) {
        let next = self.next_phase(settings);
        match self.phase {
            Phase::Focus => self.rounds_done += 1,
            Phase::LongBreak => self.rounds_done = 0,
            Phase::ShortBreak => {}
        }
        self.phase = next;
        self.length = settings.length_of(next);
        self.restart_phase();
        if settings.auto_start {
            self.start(now);
        }
    }

    /// Call this every frame. If the running phase just ran out, moves on and returns
    /// the phase that finished.
    pub fn update(&mut self, settings: &Settings, now: Instant) -> Option<Phase> {
        if self.is_running() && self.remaining(now).is_zero() {
            let finished = self.phase;
            self.advance(settings, now);
            Some(finished)
        } else {
            None
        }
    }

    /// Apply changed settings to a phase that hasn't started yet.
    pub fn apply_settings(&mut self, settings: &Settings) {
        if !self.is_running() && self.left_when_paused == self.length {
            self.length = settings.length_of(self.phase);
            self.left_when_paused = self.length;
        }
    }
}

/// `25:00`, `04:05`. Rounds up, so the display shows 00:01 until the very end.
pub fn mm_ss(d: Duration) -> String {
    let secs = d.as_millis().div_ceil(1000);
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secs(s: u64) -> Duration {
        Duration::from_secs(s)
    }

    #[test]
    fn counts_down_and_pauses() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let t0 = Instant::now();
        assert_eq!(t.remaining(t0), secs(25 * 60));
        t.start(t0);
        assert_eq!(t.remaining(t0 + secs(60)), secs(24 * 60));
        t.pause(t0 + secs(60));
        // Paused: time passing doesn't count.
        assert_eq!(t.remaining(t0 + secs(600)), secs(24 * 60));
        t.start(t0 + secs(600));
        assert_eq!(t.remaining(t0 + secs(660)), secs(23 * 60));
        assert_eq!(t.remaining(t0 + secs(9999)), Duration::ZERO);
    }

    #[test]
    fn progress_goes_from_zero_to_one() {
        let mut t = Timer::new(&Settings::default());
        let t0 = Instant::now();
        assert_eq!(t.progress(t0), 0.0);
        t.start(t0);
        assert!((t.progress(t0 + secs(25 * 30)) - 0.5).abs() < 0.001);
        assert_eq!(t.progress(t0 + secs(25 * 60)), 1.0);
    }

    #[test]
    fn long_break_after_four_rounds() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let now = Instant::now();
        let mut phases = vec![];
        for _ in 0..9 {
            t.advance(&settings, now);
            phases.push(t.phase);
        }
        use Phase::*;
        assert_eq!(phases, [ShortBreak, Focus, ShortBreak, Focus, ShortBreak, Focus, LongBreak, Focus, ShortBreak]);
    }

    #[test]
    fn update_reports_finished_phase() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let t0 = Instant::now();
        assert_eq!(t.update(&settings, t0), None); // not running
        t.start(t0);
        assert_eq!(t.update(&settings, t0 + secs(60)), None);
        assert_eq!(t.update(&settings, t0 + secs(25 * 60)), Some(Phase::Focus));
        assert_eq!(t.phase, Phase::ShortBreak);
        assert_eq!(t.length, secs(5 * 60));
        assert!(!t.is_running()); // auto_start is off
    }

    #[test]
    fn formats_time() {
        assert_eq!(mm_ss(secs(25 * 60)), "25:00");
        assert_eq!(mm_ss(secs(245)), "04:05");
        assert_eq!(mm_ss(Duration::from_millis(300)), "00:01");
        assert_eq!(mm_ss(Duration::ZERO), "00:00");
    }
}
@@text
If you're continuing with your own code, press **Add 1 new file(s)** to get `stats.rs`, and add `mod stats;` to `main.rs`.

## Storing days

To total minutes "per day", we need a way to say which day a session happened. Real dates (with months of different lengths, leap years and time zones) are surprisingly complicated. We keep it simple: a day is a **number**, the days since 1 January 1970:

```rust,norun
pub type Day = u32;

pub fn today() -> Day {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    (secs / 86_400) as Day   // 86,400 seconds in a day
}
```

`pub type Day = u32;` is a **type alias**: `Day` is just another name for `u32`, but it makes signatures like `fn minutes_on(&self, day: Day)` say what the number means. Days as numbers are easy to compare and subtract: yesterday is `today - 1`, and "the last 7 days" is the range `today - 6 ..= today`. And since 1 January 1970 was a Thursday, the weekday is `day % 7` looked up in a list starting with Thursday (see `weekday`).

One simplification: `SystemTime` counts in **UTC** (the time zone of London in winter). If you live far from it, a session late in the evening may count for the next day. Real apps use a crate like `chrono` to handle the local time zone; adding it would be a good extra challenge.

## Your tasks in `stats.rs`

The tests at the bottom describe exactly what each method should do. Run `cargo test` and make them pass:

**1. `add(day, minutes, task)`** pushes a new `Session` to `self.sessions`. If the task is empty or only spaces, store it as `"(no task)"`. Store the task trimmed.

**2. `minutes_on(day)`**: the total minutes of all sessions on that day. An iterator chain does it in one line: `filter` the sessions on that day, `map` each to its minutes, `sum`.

**3. `last_days(today, n)`**: a list of `(day, minutes)` for the `n` days up to and including `today`, oldest first. The first day is `today - (n - 1)`; use `saturating_sub` so it can't go below zero. Then `(first..=today).map(...).collect()`.

**4. `tasks_on(day)`**: minutes per task on one day, as a `BTreeMap<&str, u32>` (sorted by task name, which is nice for showing). This is the classic "count with `entry`" pattern from the HashMap lesson:

```rust,norun
*totals.entry(s.task.as_str()).or_insert(0) += s.minutes;
```

Notice the return type borrows: `&str` keys point into the sessions' `String`s, so no text is copied.

**5. `streak(today)`**: how many days in a row, ending today, have at least one session. If there's nothing *yet* today, the streak ending yesterday still counts (you haven't broken it until the day is over). Hint: pick the starting day (today, or yesterday if today is empty), then count backwards with a `while` loop as long as `minutes_on(day) > 0`. Watch out for day 0: `0 - 1` would overflow a `u32`.

## Using it in the app

Now connect it to the window in `app.rs`. You need four changes.

**1. Load at start.** Replace the `settings` field with the save data, which holds both the settings and the log:

```rust,norun
pub struct FocusApp {
    data: SaveData,
    timer: Timer,
    task: String,
    title: String,
    status: String,   // shows a message if saving fails
}

pub fn new() -> Self {
    let data = stats::load();
    let timer = Timer::new(&data.settings);
    FocusApp { data, timer, task: String::new(), title: String::new(), status: String::new() }
}
```

Every `self.settings` in the file now becomes `self.data.settings`. The compiler will point at each one; fix them one by one. (Add `use crate::stats::{self, SaveData};` at the top. `self` in a `use` list imports the module itself, so you can write `stats::today()`.)

**2. A save helper**, which turns a failed save into a message instead of crashing:

```rust,norun
fn save(&mut self) {
    self.status = match stats::save(&self.data) {
        Ok(()) => String::new(),
        Err(e) => e,
    };
}
```

Call `self.save()` in `settings_view` when something changed, and show the status at the bottom of the window if it isn't empty: `ui.colored_label(Color32::YELLOW, &self.status);`.

**3. Log finished focus sessions** in `phase_finished` (rename `_finished` back to `finished`, now that it's used):

```rust,norun
if finished == Phase::Focus {
    let minutes = self.data.settings.focus_minutes;
    self.data.log.add(stats::today(), minutes, &self.task);
    self.save();
}
```

Note that only sessions that **ran out** are logged. Skipping a focus session doesn't count, which is fair.

**4. Show the stats.** First the numbers and the tasks:

```rust,norun
fn stats_view(&self, ui: &mut egui::Ui) {
    let today = stats::today();
    let log = &self.data.log;
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("Today: {} min", log.minutes_on(today))).strong());
        ui.separator();
        ui.label(format!("{}-day streak", log.streak(today)));
    });
    self.week_chart(ui);
    let tasks = log.tasks_on(today);
    if !tasks.is_empty() {
        ui.label(RichText::new("Today's tasks").strong());
        for (task, minutes) in tasks {
            ui.label(format!("• {task}: {minutes} min"));
        }
    }
}
```

Then the chart. It's the same idea as Pocket Ledger's: divide the width into one **slot** per day, and make each bar's height proportional to its minutes. The tallest bar uses the full height; `.max(60)` keeps a single 25-minute day from looking like a huge achievement:

```rust,norun
/// Bars for the last 7 days, drawn with the painter.
fn week_chart(&self, ui: &mut egui::Ui) {
    let today = stats::today();
    let days = self.data.log.last_days(today, 7);
    let max = days.iter().map(|&(_, m)| m).max().unwrap_or(0).max(60) as f32;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 120.0), Sense::hover());
    let painter = ui.painter_at(rect);
    let slot = rect.width() / days.len() as f32;
    for (i, &(day, minutes)) in days.iter().enumerate() {
        let x = rect.left() + slot * (i as f32 + 0.5);
        let bar_height = (rect.height() - 36.0) * minutes as f32 / max;
        let bottom = rect.bottom() - 18.0;
        let bar = egui::Rect::from_min_max(egui::pos2(x - slot * 0.3, bottom - bar_height), egui::pos2(x + slot * 0.3, bottom));
        let color = if day == today { FOCUS_COLOR } else { FOCUS_COLOR.gamma_multiply(0.5) };
        painter.rect_filled(bar, 3.0, color);
        let text_color = ui.visuals().text_color();
        painter.text(egui::pos2(x, rect.bottom()), Align2::CENTER_BOTTOM, stats::weekday(day), FontId::proportional(12.0), text_color);
        if minutes > 0 {
            painter.text(bar.center_top(), Align2::CENTER_BOTTOM, minutes.to_string(), FontId::proportional(11.0), text_color);
        }
    }
}
```

Finally, in `ui()`, show `self.stats_view(ui);` between two separators, above the settings. If the window is small, wrap the contents of the `CentralPanel` in `egui::ScrollArea::vertical().show(ui, |ui| { ... })` so nothing gets cut off.

Run a few 1-minute sessions with different tasks, close the app, and open it again: everything's still there. Have a look at `focus_timer_data.json` in the project folder, too. It's plain JSON, readable by you and by other programs.

## The finished app

Here's the complete Focus Timer, in its own folder so it doesn't touch yours. Compare it with your version, or just use it! Remember the `--release` tip from "Shipping Your App": `cargo build --release` gives you a fast, small `focus_timer.exe` in `target/release` that you can pin to your taskbar.

@@project focus_timer_finished Focus Timer (finished)
The complete app, for reference. Created in its own folder, `focus_timer_finished`.
@@file Cargo.toml
[package]
name = "focus_timer"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

@@file src/main.rs
mod app;
mod stats;
mod timer;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 640.0]).with_min_inner_size([340.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Focus Timer", options, Box::new(|_cc| Ok(Box::new(app::FocusApp::new()))))
}

@@file src/app.rs
//! The window: drawing the timer and the stats, and turning clicks into calls on the model.

use crate::stats::{self, SaveData};
use crate::timer::{Phase, Timer, mm_ss};
use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, Vec2, ViewportCommand};
use std::time::{Duration, Instant};

const FOCUS_COLOR: Color32 = Color32::from_rgb(235, 100, 80);
const BREAK_COLOR: Color32 = Color32::from_rgb(90, 190, 130);

pub struct FocusApp {
    data: SaveData,
    timer: Timer,
    task: String,
    /// The window title we set last, so we only send a new one when it changes.
    title: String,
    status: String,
}

impl FocusApp {
    pub fn new() -> Self {
        let data = stats::load();
        let timer = Timer::new(&data.settings);
        FocusApp { data, timer, task: String::new(), title: String::new(), status: String::new() }
    }

    fn phase_color(&self) -> Color32 {
        if self.timer.phase == Phase::Focus { FOCUS_COLOR } else { BREAK_COLOR }
    }

    fn save(&mut self) {
        self.status = match stats::save(&self.data) {
            Ok(()) => String::new(),
            Err(e) => e,
        };
    }

    /// A phase just ran out: log it, and get the user's attention.
    fn phase_finished(&mut self, ctx: &egui::Context, finished: Phase) {
        if finished == Phase::Focus {
            let minutes = self.data.settings.focus_minutes;
            self.data.log.add(stats::today(), minutes, &self.task);
            self.save();
        }
        // Flash the taskbar button, and bring the window to the front.
        ctx.send_viewport_cmd(ViewportCommand::RequestUserAttention(egui::UserAttentionType::Informational));
        ctx.send_viewport_cmd(ViewportCommand::Focus);
    }

    /// Show the time in the window title, so it's visible in the taskbar.
    fn update_title(&mut self, ctx: &egui::Context, now: Instant) {
        let title = if self.timer.is_running() {
            format!("{} · {}", mm_ss(self.timer.remaining(now)), self.timer.phase.label())
        } else {
            "Focus Timer".to_string()
        };
        if title != self.title {
            ctx.send_viewport_cmd(ViewportCommand::Title(title.clone()));
            self.title = title;
        }
    }

    // ---------- drawing ----------

    /// The big ring: a grey circle, with a coloured arc for the time already done.
    fn ring(&self, ui: &mut egui::Ui, now: Instant) {
        let size = ui.available_width().min(260.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = size / 2.0 - 12.0;
        painter.circle_stroke(center, radius, Stroke::new(14.0, ui.visuals().faint_bg_color));

        // The arc is a line through many points on the circle, starting at the top.
        let done = self.timer.progress(now);
        let steps = (done * 100.0).ceil() as usize;
        if steps > 0 {
            let points: Vec<_> = (0..=steps)
                .map(|i| {
                    let angle = (i as f32 / 100.0).min(done) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
                    center + radius * Vec2::angled(angle)
                })
                .collect();
            painter.line(points, Stroke::new(14.0, self.phase_color()));
        }

        let text_color = ui.visuals().strong_text_color();
        painter.text(center, Align2::CENTER_CENTER, mm_ss(self.timer.remaining(now)), FontId::proportional(56.0), text_color);
        let label = format!("{} · round {}", self.timer.phase.label(), self.timer.rounds_done + 1);
        painter.text(center + Vec2::new(0.0, 42.0), Align2::CENTER_CENTER, label, FontId::proportional(16.0), self.phase_color());
    }

    fn controls(&mut self, ui: &mut egui::Ui, now: Instant) {
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 290.0).max(0.0) / 2.0); // center the row
            let big = |text: &str| egui::Button::new(RichText::new(text).size(20.0)).min_size(Vec2::new(110.0, 40.0));
            if self.timer.is_running() {
                if ui.add(big("Pause")).clicked() {
                    self.timer.pause(now);
                }
            } else if ui.add(big("Start")).clicked() {
                self.timer.start(now);
            }
            if ui.button("Restart").on_hover_text("Back to the start of this phase").clicked() {
                self.timer.restart_phase();
            }
            if ui.button("Skip").on_hover_text("Go to the next phase without logging this one").clicked() {
                self.timer.advance(&self.data.settings, now);
            }
        });
    }

    /// Bars for the last 7 days, drawn with the painter.
    fn week_chart(&self, ui: &mut egui::Ui) {
        let today = stats::today();
        let days = self.data.log.last_days(today, 7);
        let max = days.iter().map(|&(_, m)| m).max().unwrap_or(0).max(60) as f32;
        let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 120.0), Sense::hover());
        let painter = ui.painter_at(rect);
        let slot = rect.width() / days.len() as f32;
        for (i, &(day, minutes)) in days.iter().enumerate() {
            let x = rect.left() + slot * (i as f32 + 0.5);
            let bar_height = (rect.height() - 36.0) * minutes as f32 / max;
            let bottom = rect.bottom() - 18.0;
            let bar = egui::Rect::from_min_max(egui::pos2(x - slot * 0.3, bottom - bar_height), egui::pos2(x + slot * 0.3, bottom));
            let color = if day == today { FOCUS_COLOR } else { FOCUS_COLOR.gamma_multiply(0.5) };
            painter.rect_filled(bar, 3.0, color);
            let text_color = ui.visuals().text_color();
            painter.text(egui::pos2(x, rect.bottom()), Align2::CENTER_BOTTOM, stats::weekday(day), FontId::proportional(12.0), text_color);
            if minutes > 0 {
                painter.text(bar.center_top(), Align2::CENTER_BOTTOM, minutes.to_string(), FontId::proportional(11.0), text_color);
            }
        }
    }

    fn stats_view(&self, ui: &mut egui::Ui) {
        let today = stats::today();
        let log = &self.data.log;
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Today: {} min", log.minutes_on(today))).strong());
            ui.separator();
            ui.label(format!("{}-day streak", log.streak(today)));
        });
        self.week_chart(ui);
        let tasks = log.tasks_on(today);
        if !tasks.is_empty() {
            ui.label(RichText::new("Today's tasks").strong());
            for (task, minutes) in tasks {
                ui.label(format!("• {task}: {minutes} min"));
            }
        }
    }

    fn settings_view(&mut self, ui: &mut egui::Ui) {
        let s = &mut self.data.settings;
        let mut changed = false;
        egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
            let mut row = |ui: &mut egui::Ui, label: &str, value: &mut u32, max: u32| {
                ui.label(label);
                changed |= ui.add(egui::DragValue::new(value).range(1..=max).suffix(" min")).changed();
                ui.end_row();
            };
            row(ui, "Focus", &mut s.focus_minutes, 180);
            row(ui, "Short break", &mut s.short_break_minutes, 60);
            row(ui, "Long break", &mut s.long_break_minutes, 90);
        });
        ui.horizontal(|ui| {
            ui.label("Long break every");
            changed |= ui.add(egui::DragValue::new(&mut s.rounds_before_long_break).range(1..=12)).changed();
            ui.label("rounds");
        });
        changed |= ui.checkbox(&mut s.auto_start, "Start the next phase automatically").changed();
        if changed {
            self.timer.apply_settings(&self.data.settings);
            self.save();
        }
    }
}

impl eframe::App for FocusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let now = Instant::now();
        if let Some(finished) = self.timer.update(&self.data.settings, now) {
            self.phase_finished(&ctx, finished);
        }
        self.update_title(&ctx, now);
        if self.timer.is_running() {
            // Keep the countdown moving even when nothing else happens.
            ctx.request_repaint_after(Duration::from_millis(250));
        }

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    self.ring(ui, now);
                    self.controls(ui, now);
                    ui.add_space(6.0);
                    ui.add(egui::TextEdit::singleline(&mut self.task).hint_text("What are you working on?").desired_width(280.0));
                });
                ui.add_space(8.0);
                ui.separator();
                self.stats_view(ui);
                ui.separator();
                egui::CollapsingHeader::new("Settings").show(ui, |ui| self.settings_view(ui));
                if !self.status.is_empty() {
                    ui.colored_label(Color32::YELLOW, &self.status);
                }
            });
        });
    }
}

@@file src/stats.rs
//! The history of finished focus sessions, the numbers we show about it, and saving it all.

use crate::timer::Settings;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A day, counted as days since 1 January 1970. Simple to store, compare and subtract.
pub type Day = u32;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    pub day: Day,
    pub minutes: u32,
    pub task: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Log {
    pub sessions: Vec<Session>,
}

impl Log {
    pub fn add(&mut self, day: Day, minutes: u32, task: &str) {
        let task = if task.trim().is_empty() { "(no task)" } else { task.trim() };
        self.sessions.push(Session { day, minutes, task: task.to_string() });
    }

    pub fn minutes_on(&self, day: Day) -> u32 {
        self.sessions.iter().filter(|s| s.day == day).map(|s| s.minutes).sum()
    }

    /// Minutes per day for the `n` days up to and including `today`, oldest first.
    pub fn last_days(&self, today: Day, n: u32) -> Vec<(Day, u32)> {
        let first = today.saturating_sub(n - 1);
        (first..=today).map(|day| (day, self.minutes_on(day))).collect()
    }

    /// Minutes per task on one day, sorted by task name.
    pub fn tasks_on(&self, day: Day) -> BTreeMap<&str, u32> {
        let mut totals = BTreeMap::new();
        for s in self.sessions.iter().filter(|s| s.day == day) {
            *totals.entry(s.task.as_str()).or_insert(0) += s.minutes;
        }
        totals
    }

    /// How many days in a row, ending today, have at least one session.
    /// If today has none yet, the streak ending yesterday still counts.
    pub fn streak(&self, today: Day) -> u32 {
        let mut day = if self.minutes_on(today) > 0 { today } else { today.saturating_sub(1) };
        let mut streak = 0;
        while self.minutes_on(day) > 0 {
            streak += 1;
            if day == 0 {
                break;
            }
            day -= 1;
        }
        streak
    }
}

/// Today's day number (in UTC; see the lesson for why that's a small simplification).
pub fn today() -> Day {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    (secs / 86_400) as Day
}

/// "Mon", "Tue", … for a day number. Day 0, 1 January 1970, was a Thursday.
pub fn weekday(day: Day) -> &'static str {
    ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][(day % 7) as usize]
}

// ---------- saving ----------

const DATA_FILE: &str = "focus_timer_data.json";

/// Everything the app saves between runs.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub settings: Settings,
    pub log: Log,
}

pub fn load() -> SaveData {
    match std::fs::read_to_string(DATA_FILE) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("Couldn't read {DATA_FILE} ({e}), starting fresh");
            SaveData::default()
        }),
        Err(_) => SaveData::default(), // first run: no file yet
    }
}

pub fn save(data: &SaveData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(DATA_FILE, json).map_err(|e| format!("Couldn't save: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_with(days: &[(Day, u32)]) -> Log {
        let mut log = Log::default();
        for &(day, minutes) in days {
            log.add(day, minutes, "work");
        }
        log
    }

    #[test]
    fn totals_per_day() {
        let log = log_with(&[(10, 25), (10, 25), (11, 50)]);
        assert_eq!(log.minutes_on(10), 50);
        assert_eq!(log.minutes_on(12), 0);
        assert_eq!(log.last_days(12, 3), vec![(10, 50), (11, 50), (12, 0)]);
    }

    #[test]
    fn totals_per_task() {
        let mut log = Log::default();
        log.add(5, 25, "essay");
        log.add(5, 25, "maths");
        log.add(5, 25, "essay");
        log.add(5, 10, "  ");
        log.add(6, 25, "essay");
        let tasks = log.tasks_on(5);
        assert_eq!(tasks.len(), 3);
        assert_eq!(tasks["essay"], 50);
        assert_eq!(tasks["(no task)"], 10);
    }

    #[test]
    fn streaks() {
        assert_eq!(log_with(&[]).streak(100), 0);
        assert_eq!(log_with(&[(98, 25), (99, 25), (100, 25)]).streak(100), 3);
        // Nothing yet today: yesterday's streak still counts.
        assert_eq!(log_with(&[(98, 25), (99, 25)]).streak(100), 2);
        // A gap breaks it.
        assert_eq!(log_with(&[(97, 25), (99, 25), (100, 25)]).streak(100), 2);
    }

    #[test]
    fn weekdays() {
        assert_eq!(weekday(0), "Thu");
        assert_eq!(weekday(4), "Mon");
    }
}

@@file src/timer.rs
//! The timer itself: phases, counting down, and what comes next. No GUI code here.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Focus,
    ShortBreak,
    LongBreak,
}

impl Phase {
    pub fn label(self) -> &'static str {
        match self {
            Phase::Focus => "Focus",
            Phase::ShortBreak => "Short break",
            Phase::LongBreak => "Long break",
        }
    }
}

/// What the user can change. Saved with the stats.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub focus_minutes: u32,
    pub short_break_minutes: u32,
    pub long_break_minutes: u32,
    /// After this many focus sessions, the break is a long one.
    pub rounds_before_long_break: u32,
    /// Start the next phase by itself when one ends.
    pub auto_start: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { focus_minutes: 25, short_break_minutes: 5, long_break_minutes: 15, rounds_before_long_break: 4, auto_start: false }
    }
}

impl Settings {
    pub fn length_of(&self, phase: Phase) -> Duration {
        let minutes = match phase {
            Phase::Focus => self.focus_minutes,
            Phase::ShortBreak => self.short_break_minutes,
            Phase::LongBreak => self.long_break_minutes,
        };
        Duration::from_secs(minutes as u64 * 60)
    }
}

/// A countdown that can be paused. Like a stopwatch, it stores *when* it was started
/// rather than counting frames, so it never drifts.
pub struct Timer {
    pub phase: Phase,
    /// The full length of the current phase.
    pub length: Duration,
    /// Time left at the moment it was last paused (or the full length).
    left_when_paused: Duration,
    /// Set while running.
    started: Option<Instant>,
    /// Focus sessions finished since the last long break.
    pub rounds_done: u32,
}

impl Timer {
    pub fn new(settings: &Settings) -> Self {
        let length = settings.length_of(Phase::Focus);
        Timer { phase: Phase::Focus, length, left_when_paused: length, started: None, rounds_done: 0 }
    }

    pub fn is_running(&self) -> bool {
        self.started.is_some()
    }

    /// Time left at the moment `now`. Never below zero.
    pub fn remaining(&self, now: Instant) -> Duration {
        match self.started {
            Some(started) => self.left_when_paused.saturating_sub(now.duration_since(started)),
            None => self.left_when_paused,
        }
    }

    /// How much of the phase is done, from 0.0 to 1.0.
    pub fn progress(&self, now: Instant) -> f32 {
        if self.length.is_zero() {
            return 1.0;
        }
        1.0 - self.remaining(now).as_secs_f32() / self.length.as_secs_f32()
    }

    pub fn start(&mut self, now: Instant) {
        if self.started.is_none() {
            self.started = Some(now);
        }
    }

    pub fn pause(&mut self, now: Instant) {
        self.left_when_paused = self.remaining(now);
        self.started = None;
    }

    /// Back to the start of the current phase, paused.
    pub fn restart_phase(&mut self) {
        self.left_when_paused = self.length;
        self.started = None;
    }

    /// The phase that follows the current one.
    pub fn next_phase(&self, settings: &Settings) -> Phase {
        match self.phase {
            Phase::Focus if self.rounds_done + 1 >= settings.rounds_before_long_break => Phase::LongBreak,
            Phase::Focus => Phase::ShortBreak,
            Phase::ShortBreak | Phase::LongBreak => Phase::Focus,
        }
    }

    /// Moves on to the next phase, paused (or running, with `auto_start`).
    pub fn advance(&mut self, settings: &Settings, now: Instant) {
        let next = self.next_phase(settings);
        match self.phase {
            Phase::Focus => self.rounds_done += 1,
            Phase::LongBreak => self.rounds_done = 0,
            Phase::ShortBreak => {}
        }
        self.phase = next;
        self.length = settings.length_of(next);
        self.restart_phase();
        if settings.auto_start {
            self.start(now);
        }
    }

    /// Call this every frame. If the running phase just ran out, moves on and returns
    /// the phase that finished.
    pub fn update(&mut self, settings: &Settings, now: Instant) -> Option<Phase> {
        if self.is_running() && self.remaining(now).is_zero() {
            let finished = self.phase;
            self.advance(settings, now);
            Some(finished)
        } else {
            None
        }
    }

    /// Apply changed settings to a phase that hasn't started yet.
    pub fn apply_settings(&mut self, settings: &Settings) {
        if !self.is_running() && self.left_when_paused == self.length {
            self.length = settings.length_of(self.phase);
            self.left_when_paused = self.length;
        }
    }
}

/// `25:00`, `04:05`. Rounds up, so the display shows 00:01 until the very end.
pub fn mm_ss(d: Duration) -> String {
    let secs = d.as_millis().div_ceil(1000);
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secs(s: u64) -> Duration {
        Duration::from_secs(s)
    }

    #[test]
    fn counts_down_and_pauses() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let t0 = Instant::now();
        assert_eq!(t.remaining(t0), secs(25 * 60));
        t.start(t0);
        assert_eq!(t.remaining(t0 + secs(60)), secs(24 * 60));
        t.pause(t0 + secs(60));
        // Paused: time passing doesn't count.
        assert_eq!(t.remaining(t0 + secs(600)), secs(24 * 60));
        t.start(t0 + secs(600));
        assert_eq!(t.remaining(t0 + secs(660)), secs(23 * 60));
        assert_eq!(t.remaining(t0 + secs(9999)), Duration::ZERO);
    }

    #[test]
    fn progress_goes_from_zero_to_one() {
        let mut t = Timer::new(&Settings::default());
        let t0 = Instant::now();
        assert_eq!(t.progress(t0), 0.0);
        t.start(t0);
        assert!((t.progress(t0 + secs(25 * 30)) - 0.5).abs() < 0.001);
        assert_eq!(t.progress(t0 + secs(25 * 60)), 1.0);
    }

    #[test]
    fn long_break_after_four_rounds() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let now = Instant::now();
        let mut phases = vec![];
        for _ in 0..9 {
            t.advance(&settings, now);
            phases.push(t.phase);
        }
        use Phase::*;
        assert_eq!(phases, [ShortBreak, Focus, ShortBreak, Focus, ShortBreak, Focus, LongBreak, Focus, ShortBreak]);
    }

    #[test]
    fn update_reports_finished_phase() {
        let settings = Settings::default();
        let mut t = Timer::new(&settings);
        let t0 = Instant::now();
        assert_eq!(t.update(&settings, t0), None); // not running
        t.start(t0);
        assert_eq!(t.update(&settings, t0 + secs(60)), None);
        assert_eq!(t.update(&settings, t0 + secs(25 * 60)), Some(Phase::Focus));
        assert_eq!(t.phase, Phase::ShortBreak);
        assert_eq!(t.length, secs(5 * 60));
        assert!(!t.is_running()); // auto_start is off
    }

    #[test]
    fn formats_time() {
        assert_eq!(mm_ss(secs(25 * 60)), "25:00");
        assert_eq!(mm_ss(secs(245)), "04:05");
        assert_eq!(mm_ss(Duration::from_millis(300)), "00:01");
        assert_eq!(mm_ss(Duration::ZERO), "00:00");
    }
}
@@text
## Make it yours

You've built a real productivity app. Some ideas to take it further, roughly from easy to hard:

- **Daily goal:** a setting like "4 sessions a day", and show progress towards it (maybe as dots under the ring).
- **Task suggestions:** below the text box, buttons with the last 3 different tasks, so you can pick one with a click.
- **History view:** a second "page" (an enum field like `View::Timer` / `View::History`, switched by buttons at the top) listing all sessions, grouped by day with a `BTreeMap`.
- **Export:** a button that writes the log as a CSV file (`day,minutes,task` per line) that opens in Excel.
- **Local time:** add the `chrono` crate and store real dates (`NaiveDate`) instead of day numbers. `chrono` has a `serde` feature for saving them.
- **Mini mode:** a button that shrinks the window to just the time and keeps it on top, with `ViewportCommand::InnerSize` and `ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop)`.

@@quiz
Why does `tasks_on` return a `BTreeMap` and not a `HashMap`?
- `HashMap` can't have `&str` keys
* A `BTreeMap` is sorted by key, so the tasks show up in the same, alphabetical order every time
- `BTreeMap` uses less memory
? A `HashMap`'s order is random and can change between frames, which would make the list jump around.

@@quiz
`Settings` has `#[serde(default)]`. When does that matter?
* When an older save file is missing a field that was added later: it gets its default value instead of failing to load
- It makes saving faster
- It resets the settings every time the app starts
? Without it, adding a new setting would make every existing save file unreadable.
