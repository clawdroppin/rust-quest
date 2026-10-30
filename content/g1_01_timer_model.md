@@lesson g-ft1 Focus Timer 1: The Timer Model
@@text
The **Pomodoro technique** is a simple way to get work done: focus for 25 minutes, take a 5-minute break, repeat, and after four rounds take a longer break. Over the next three steps you'll build a desktop app for it:

- a big **progress ring** counting down, with Start, Pause, Restart and Skip,
- the time shown in the **window title** (visible in the taskbar), and the window **flashing** when a phase ends,
- a **"What are you working on?"** box, so every focus session is logged with its task,
- **today's minutes, a streak counter and a 7-day bar chart**, saved between runs,
- adjustable lengths in a **settings** section.

As always, we start with the **model**: the logic, with no GUI code, so it can be tested on its own.

## Thinking it through first

What *is* a Pomodoro timer, as data? Before writing code, describe it in words:

- It's always in one of three **phases**: Focus, Short break or Long break. That's an `enum`.
- Each phase has a **length**, which comes from the settings.
- It's either **running** or **paused**, and has some **time left**.
- When a phase runs out, it moves to the **next phase**. Which one depends on how many focus **rounds** have been done.

A thing that's always in one of a few states, and moves between them by fixed rules, is called a **state machine**. You draw it like this:

```text
            (rounds < 4)
  Focus ───────────────► Short break ──┐
    ▲  │                               │
    │  │ (4th round)                   │
    │  └─────────────► Long break ─────┤
    │                                  │
    └──────────────────────────────────┘
```

Many apps are state machines at heart: a game (menu → playing → game over), a download (waiting → downloading → done/failed), a traffic light. Drawing the states and arrows first makes the code almost write itself.

## Storing time the right way

You saw this in "Timers and Background Work": don't count down by subtracting a little every frame (frames don't come at exact intervals, so it drifts). Store **when** the timer was started, and calculate the rest:

```rust,norun
pub struct Timer {
    pub phase: Phase,
    pub length: Duration,          // full length of this phase
    left_when_paused: Duration,    // time left when last paused
    started: Option<Instant>,      // Some(...) while running
    pub rounds_done: u32,          // focus sessions since the last long break
}
```

- While **paused**, the time left is simply `left_when_paused`.
- While **running**, it's `left_when_paused` minus the time since `started`.
- **Pausing** saves the current time left into `left_when_paused` and sets `started` to `None`.

## Making time testable

Here's a trick professionals use. Methods that depend on the current time don't call `Instant::now()` themselves. They take `now: Instant` as a **parameter**:

```rust,norun
pub fn remaining(&self, now: Instant) -> Duration
```

In the app, we pass `Instant::now()`. But in tests we can pass *any* moment we like: `t0 + Duration::from_secs(25 * 60)` means "25 minutes later", and the test runs instantly instead of waiting 25 minutes. Look at the tests at the bottom of `timer.rs` to see how.

The same idea works for anything outside your program's control: the date, random numbers, files, the network. Pass them in, and your logic becomes easy to test.

@@project focus_timer Focus Timer, step 1: the timer model
The timer logic in `src/timer.rs`, with some methods for you to write, and `src/main.rs` with a quick test drive that runs without a window.
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
mod timer;

use std::time::Instant;
use timer::{Settings, Timer, mm_ss};

/// A quick test drive of the timer, without a window and without waiting 25 minutes:
/// the timer takes "now" as an argument, so we can pretend time has passed.
fn main() {
    let settings = Settings::default();
    let mut timer = Timer::new(&settings);
    let mut now = Instant::now();

    println!("Starting: {} for {}", timer.phase.label(), mm_ss(timer.remaining(now)));
    timer.start(now);
    for _ in 0..8 {
        now += timer.remaining(now); // jump to the moment this phase ends
        if let Some(finished) = timer.update(&settings, now) {
            println!("{} finished. Next: {} for {}", finished.label(), timer.phase.label(), mm_ss(timer.length));
        }
        timer.start(now);
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
        todo!()
    }

    /// How much of the phase is done, from 0.0 to 1.0.
    pub fn progress(&self, now: Instant) -> f32 {
        todo!()
    }

    pub fn start(&mut self, now: Instant) {
        if self.started.is_none() {
            self.started = Some(now);
        }
    }

    pub fn pause(&mut self, now: Instant) {
        todo!()
    }

    /// Back to the start of the current phase, paused.
    pub fn restart_phase(&mut self) {
        self.left_when_paused = self.length;
        self.started = None;
    }

    /// The phase that follows the current one.
    pub fn next_phase(&self, settings: &Settings) -> Phase {
        todo!()
    }

    /// Moves on to the next phase, paused (or running, with `auto_start`).
    pub fn advance(&mut self, settings: &Settings, now: Instant) {
        todo!()
    }

    /// Call this every frame. If the running phase just ran out, moves on and returns
    /// the phase that finished.
    pub fn update(&mut self, settings: &Settings, now: Instant) -> Option<Phase> {
        todo!()
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
    todo!()
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
## What's there already

Create the project, open it in VS Code, and read through `src/timer.rs` from top to bottom. Already written for you:

- `Phase`, with `label()` for showing it.
- `Settings`, with `#[derive(Serialize, Deserialize)]` because we'll save it in step 3, and `#[serde(default)]`, so that a saved file from an older version with a missing field still loads (the missing field gets its default value). `length_of(phase)` converts minutes into a `Duration`.
- `Timer::new`, `is_running`, `start`, `restart_phase` and `apply_settings`.
- Tests for everything.

Run `cargo test` now. Five tests fail with "not yet implemented": your job is to make them pass. You'll also see warnings about unused variables; they disappear as you fill in the methods.

## Your tasks

Work through them in this order, running `cargo test` after each one.

**1. `mm_ss(d)`** formats a `Duration` like `04:05`. You wrote almost the same function in the lesson "Time and Doing Things at Once". One difference: it should **round up** to whole seconds, so the display shows `00:01` until the time is really over, not `00:00` for the last second. Hint: `d.as_millis().div_ceil(1000)` is the number of seconds, rounded up.

**2. `remaining(now)`**: time left at the moment `now`.
- If `started` is `None` (paused): `left_when_paused`.
- If it's `Some(started)`: `left_when_paused` minus `now.duration_since(started)`. Use `saturating_sub` instead of `-`, because subtracting a larger `Duration` from a smaller one panics; `saturating_sub` stops at zero instead.
- A `match self.started { ... }` handles both cases neatly.

**3. `progress(now)`**: how much of the phase is done, from `0.0` to `1.0`. That's `1.0 - remaining / length`, with both converted with `.as_secs_f32()`. Guard against dividing by zero: if `self.length.is_zero()`, return `1.0`.

**4. `pause(now)`**: remember the time left (use `self.remaining(now)`!), then set `started` to `None`.

**5. `next_phase(settings)`** returns the phase that comes after the current one, following the state machine above:
- after a break (short or long), always `Focus`;
- after `Focus`: if `self.rounds_done + 1 >= settings.rounds_before_long_break`, a `LongBreak`, otherwise a `ShortBreak`. (The `+ 1` counts the round that's finishing right now.)

A `match self.phase` with a **match guard** (`Phase::Focus if ... =>`) expresses this nicely.

**6. `advance(settings, now)`** moves to the next phase:
- work out the next phase (call `next_phase`!),
- update `rounds_done`: +1 when a Focus ends, back to 0 when a Long break ends,
- set `phase`, set `length` from the settings, and call `restart_phase()`,
- if `settings.auto_start`, call `self.start(now)`.

**7. `update(settings, now)`** is called every frame by the GUI. If the timer is running *and* no time is left, it should `advance` and return `Some(the phase that just finished)`. Otherwise `None`. Careful: save the finished phase *before* advancing.

When all tests pass, run `cargo run`. The test drive in `main.rs` jumps from the end of each phase to the next, and should print the whole cycle:

```text
Starting: Focus for 25:00
Focus finished. Next: Short break for 05:00
Short break finished. Next: Focus for 25:00
...
Focus finished. Next: Long break for 15:00
```

Stuck? The **🤖 AI Helper** can see your `timer.rs` when "Include my code & errors" is on: ask it about the failing test. And the next step's project card contains a complete `timer.rs` you can compare with.

@@quiz
Why do `remaining`, `pause` and `update` take `now: Instant` as a parameter, instead of calling `Instant::now()` inside?
- It's faster
* So tests can pretend any amount of time has passed, instantly
- Because `Instant::now()` can't be called inside a method
? Passing in the time makes the logic deterministic and testable. The app just passes `Instant::now()`.

@@quiz
What does `Duration::from_secs(3).saturating_sub(Duration::from_secs(5))` return?
- It panics
- -2 seconds
* `Duration::ZERO`
? A `Duration` can't be negative. `saturating_sub` stops at zero instead of panicking like `-` would.

@@quiz
The timer is a "state machine". Which part of the code describes its arrows (the moves between states)?
- `Phase::label`
* `next_phase`
- `Settings::default`
? `next_phase` decides, for each state, which state comes next.
