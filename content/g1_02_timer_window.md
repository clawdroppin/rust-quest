@@lesson g-ft2 Focus Timer 2: The Window
@@text
The model works and is tested. Now you'll give it a face. In this step you write `src/app.rs` yourself, in five parts, running the app after each one so you see it grow.

If you finished step 1, keep your project: the card below only needs to **add `src/app.rs`**, and you'll copy the new `src/main.rs` from its starting files (it has two new lines: `mod app;` and the window setup). If you'd rather start clean, use **Reset project to this step's files**, which also gives you a finished `timer.rs`.

@@project focus_timer Focus Timer, step 2: the window
Adds `src/app.rs`, a bare window to build on. `src/main.rs` changes too: it now opens the window instead of the test drive.
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
//! The window. In this step you build it up piece by piece, following the lesson.

use crate::timer::{Settings, Timer, mm_ss};
use eframe::egui;
use std::time::Instant;

pub struct FocusApp {
    settings: Settings,
    timer: Timer,
    task: String,
}

impl FocusApp {
    pub fn new() -> Self {
        let settings = Settings::default();
        let timer = Timer::new(&settings);
        FocusApp { settings, timer, task: String::new() }
    }
}

impl eframe::App for FocusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(self.timer.phase.label());
            ui.label(mm_ss(self.timer.remaining(now)));
            // Part 1: Start / Pause buttons go here.

            ui.text_edit_singleline(&mut self.task);
            let _ = &self.settings; // used from Part 2 on
        });
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
Run it with `cargo run`. You get a plain window with the phase, the time and a text box. The time doesn't move, because nothing ever calls `start`. Let's fix that.

## Part 1: Start and Pause

Add a method to `impl FocusApp` that draws the buttons. Notice that it gets `now` too, so the whole frame uses one consistent moment:

```rust,norun
fn controls(&mut self, ui: &mut egui::Ui, now: Instant) {
    ui.horizontal(|ui| {
        if self.timer.is_running() {
            if ui.button("Pause").clicked() {
                self.timer.pause(now);
            }
        } else if ui.button("Start").clicked() {
            self.timer.start(now);
        }
    });
}
```

Call it in `ui()` where the comment says `// Part 1`: `self.controls(ui, now);`. Run the app and press Start.

The time changes… but only while you move the mouse! That's egui being lazy, as you saw in "Timers and Background Work". Add this near the top of `ui()`, after `let now = ...`:

```rust,norun
if self.timer.is_running() {
    // Keep the countdown moving even when nothing else happens.
    ui.ctx().request_repaint_after(Duration::from_millis(250));
}
```

and add `Duration` to the `use std::time::...` line. Four frames a second is plenty for a clock that shows seconds, and when paused, the app uses no CPU at all.

## Part 2: when a phase ends

Nothing calls `update` yet, so when the time reaches `00:00`, it just stays there. At the top of `ui()`:

```rust,norun
let ctx = ui.ctx().clone();
let now = Instant::now();
if let Some(finished) = self.timer.update(&self.settings, now) {
    self.phase_finished(&ctx, finished);
}
```

(Now that you have `ctx`, you can use `ctx.request_repaint_after(...)` in Part 1's code.) A finished phase should get the user's attention, since they're probably in another window. egui sends requests to the window ("viewport") with **viewport commands**:

```rust,norun
/// A phase just ran out: get the user's attention.
fn phase_finished(&mut self, ctx: &egui::Context, _finished: Phase) {
    // Flash the taskbar button, and bring the window to the front.
    ctx.send_viewport_cmd(ViewportCommand::RequestUserAttention(egui::UserAttentionType::Informational));
    ctx.send_viewport_cmd(ViewportCommand::Focus);
}
```

Update the imports at the top of the file as you go: this needs `Phase` from `crate::timer`, and `ViewportCommand` from `eframe::egui`. When a `use` is missing, the compiler's error tells you exactly what to add, and in VS Code, the 💡 quick fix (`Ctrl + .`) can add it for you.

Now add two more buttons to `controls`, after the Start/Pause button: **Restart** calls `self.timer.restart_phase()`, and **Skip** calls `self.timer.advance(&self.settings, now)`.

**Testing tip:** nobody wants to wait 25 minutes to test this. Temporarily change `focus_minutes: 25` to `1` in `Settings::default()` (in `timer.rs`), or better, build the settings in Part 5 first. Then let a phase run out and watch the taskbar.

## Part 3: the ring

Time for some drawing. The painter lets you draw shapes anywhere, as you saw with Pocket Ledger's charts. The plan:

1. Reserve a square area: `ui.allocate_exact_size(size, Sense::hover())` gives you its `Rect`.
2. Draw a full circle in a dim colour: the "track".
3. Draw an **arc** on top of it for the part that's done, in the phase's colour.
4. Write the time in the middle.

egui has no "draw an arc" function, but an arc is just a line through many points on a circle. A point at angle `a` on a circle around `center` is `center + radius * Vec2::angled(a)`. Angles are in **radians**: a full circle is `TAU` (about 6.28), and angle 0 points to the right, so we subtract a quarter turn (`FRAC_PI_2`) to start at the top, like a clock.

Add two colours at the top of the file:

```rust,norun
const FOCUS_COLOR: Color32 = Color32::from_rgb(235, 100, 80);
const BREAK_COLOR: Color32 = Color32::from_rgb(90, 190, 130);
```

and to `impl FocusApp`:

```rust,norun
fn phase_color(&self) -> Color32 {
    if self.timer.phase == Phase::Focus { FOCUS_COLOR } else { BREAK_COLOR }
}

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
```

The full circle is split into 100 steps; we draw as many as are done, and `.min(done)` makes the last point land exactly at the right spot. `ui.visuals()` gives colours from the current theme, so the ring looks right in both dark and light mode.

Now replace the heading and label in `ui()` with the ring, and centre everything:

```rust,norun
egui::CentralPanel::default().show(ui, |ui| {
    ui.vertical_centered(|ui| {
        self.ring(ui, now);
        self.controls(ui, now);
        ui.add_space(6.0);
        ui.add(egui::TextEdit::singleline(&mut self.task).hint_text("What are you working on?").desired_width(280.0));
    });
});
```

One more touch: `vertical_centered` centres single widgets, but a `horizontal` row inside it still starts at the left. Add this as the first line inside `controls`'s `ui.horizontal`, to push the buttons to the middle (290 is roughly the width of the buttons):

```rust,norun
ui.add_space((ui.available_width() - 290.0).max(0.0) / 2.0);
```

While you're there, make Start/Pause bigger than the rest. A closure that builds a button saves repeating the styling:

```rust,norun
let big = |text: &str| egui::Button::new(RichText::new(text).size(20.0)).min_size(Vec2::new(110.0, 40.0));
// ... then: ui.add(big("Pause")).clicked() instead of ui.button("Pause").clicked()
```

## Part 4: time in the title bar

When the app is behind other windows, the time is still useful in the taskbar. `ViewportCommand::Title` changes the window title. But sending a command **every frame** would be wasteful, so remember the last title and only send a new one when it changes. Add a `title: String` field to `FocusApp` (start it as `String::new()`), and:

```rust,norun
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
```

Call `self.update_title(&ctx, now);` in `ui()`, right after the `update` call.

## Part 5: settings

Finally, let the user change the lengths. A `Grid` lines up labels and inputs in neat columns, and a `DragValue` is a number you can drag or type:

```rust,norun
fn settings_view(&mut self, ui: &mut egui::Ui) {
    let s = &mut self.settings;
    let mut changed = false;
    egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
        ui.label("Focus");
        changed |= ui.add(egui::DragValue::new(&mut s.focus_minutes).range(1..=180).suffix(" min")).changed();
        ui.end_row();
        // TODO: the same for the short and long break
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
```

`changed |= ...` is short for `changed = changed | ...`: it becomes `true` if *any* of the widgets changed. When something changed, `apply_settings` updates a phase that hasn't started yet, so you see the new length straight away.

Show it under the text box, folded away so it doesn't distract:

```rust,norun
ui.separator();
egui::CollapsingHeader::new("Settings").show(ui, |ui| self.settings_view(ui));
```

**Your turn:** add the rows for the short and long break. Three rows that differ only in label, value and maximum are a good chance to try a **closure** that draws one row: `let mut row = |ui: &mut egui::Ui, label: &str, value: &mut u32, max: u32| { ... };`. (The finished version in the next step's card does exactly that, if you want to compare.)

## Done!

Set focus to 1 minute and run a full cycle: the ring fills up, the title counts down in the taskbar, the window flashes at the end, the break turns the ring green, and after four rounds you get a long break. That's a genuinely useful app already.

But close it, and it forgets everything: the settings, and how much you got done. In the next step you'll add the history, the stats and saving.

## Ideas, if you want more

- Play a sound when a phase ends. (Search crates.io for "rodio"; or keep it simple and flash the ring.)
- A "+5 min" button that adds time to the current phase. (You'd add a method to `Timer`. Write a test for it first!)
- Show the next phase under the ring: "Next: Short break".

@@quiz
Why does `update_title` remember the last title in a field?
- egui forgets the title after every frame
* To send a `ViewportCommand` only when the title actually changes, not every frame
- Because `ViewportCommand::Title` can only be used once
? Sending the same command 4 times a second would work, but it's wasteful. Remembering the last value is a common pattern.

@@quiz
`Vec2::angled(angle)` gives a point on a circle of radius 1. Why do we subtract `FRAC_PI_2` (a quarter turn) from the angle?
- To make the ring smaller
* Angle 0 points to the right; subtracting a quarter turn starts the arc at the top, like a clock
- Because radians need it
? Without it, the ring would start filling from the 3 o'clock position.
