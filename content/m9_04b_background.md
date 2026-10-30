@@lesson m9-background Timers and Background Work
@@text
Two problems show up in almost every real GUI app, and both come from the same fact you learned in "How GUIs Work": the GUI is one loop that draws a frame, waits for something to happen, and draws again.

1. **Things that change by themselves.** A clock, a countdown, an animation. If the user doesn't move the mouse, nothing "happens", so egui doesn't redraw, and your timer looks frozen.
2. **Slow work.** Saving a big file, scanning a folder, asking a server. If you do it inside `ui()`, the next frame can't be drawn until it's finished: the window freezes, and Windows says "Not responding".

The project below solves both. Create it, run it, and play with it before reading on.

@@project background_demo Timers & Background Work
A stopwatch that keeps ticking, and a slow job running on a worker thread while the window stays responsive.
@@file Cargo.toml
[package]
name = "background_demo"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }

@@file src/main.rs
use eframe::egui::{self, Color32, RichText};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([460.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native("Timers & Background Work", options, Box::new(|_cc| Ok(Box::new(DemoApp::default()))))
}

// ---------- the stopwatch ----------

/// A stopwatch that can be paused. While running, `started` is when it was (re)started;
/// `before` is the time collected before the last pause.
#[derive(Default)]
struct Stopwatch {
    started: Option<Instant>,
    before: Duration,
}

impl Stopwatch {
    fn elapsed(&self) -> Duration {
        self.before + self.started.map_or(Duration::ZERO, |s| s.elapsed())
    }

    fn is_running(&self) -> bool {
        self.started.is_some()
    }

    fn start(&mut self) {
        if self.started.is_none() {
            self.started = Some(Instant::now());
        }
    }

    fn pause(&mut self) {
        if let Some(s) = self.started.take() {
            self.before += s.elapsed();
        }
    }

    fn reset(&mut self) {
        *self = Stopwatch::default();
    }
}

fn format_time(d: Duration) -> String {
    let ms = d.as_millis();
    format!("{:02}:{:02}.{}", ms / 60_000, ms / 1000 % 60, ms / 100 % 10)
}

// ---------- the background job ----------

/// Messages the worker thread sends to the GUI.
enum Progress {
    Working(f32),
    Finished(u64),
}

/// Counts the prime numbers below `limit`, slowly on purpose, reporting progress.
fn count_primes(limit: u64, tx: Sender<Progress>, ctx: egui::Context) {
    let mut count = 0;
    for n in 2..limit {
        if (2..).take_while(|d| d * d <= n).all(|d| n % d != 0) {
            count += 1;
        }
        if n % 20_000 == 0 {
            // If the window was closed, the receiver is gone: stop working.
            if tx.send(Progress::Working(n as f32 / limit as f32)).is_err() {
                return;
            }
            ctx.request_repaint(); // wake the GUI up so it shows the new progress
        }
    }
    let _ = tx.send(Progress::Finished(count));
    ctx.request_repaint();
}

// ---------- the app ----------

#[derive(Default)]
struct DemoApp {
    stopwatch: Stopwatch,
    limit: u64,
    job: Option<Receiver<Progress>>,
    progress: f32,
    result: Option<u64>,
    clicks: u32,
}

impl DemoApp {
    fn start_job(&mut self, ctx: &egui::Context) {
        let (tx, rx) = channel();
        let limit = self.limit;
        let ctx = ctx.clone();
        thread::spawn(move || count_primes(limit, tx, ctx));
        self.job = Some(rx);
        self.progress = 0.0;
        self.result = None;
    }

    /// Called every frame: take whatever the worker has sent so far, without waiting.
    fn check_job(&mut self) {
        let Some(rx) = &self.job else { return };
        while let Ok(msg) = rx.try_recv() {
            match msg {
                Progress::Working(p) => self.progress = p,
                Progress::Finished(count) => {
                    self.result = Some(count);
                    self.job = None;
                    return;
                }
            }
        }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.check_job();
        if self.limit == 0 {
            self.limit = 3_000_000;
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("⏱ Stopwatch");
            ui.label(RichText::new(format_time(self.stopwatch.elapsed())).size(40.0).monospace());
            ui.horizontal(|ui| {
                if self.stopwatch.is_running() {
                    if ui.button("Pause").clicked() {
                        self.stopwatch.pause();
                    }
                } else if ui.button("Start").clicked() {
                    self.stopwatch.start();
                }
                if ui.button("Reset").clicked() {
                    self.stopwatch.reset();
                }
            });
            if self.stopwatch.is_running() {
                // Nothing happens while the mouse is still, so egui wouldn't redraw.
                // Ask for a new frame soon, so the time keeps ticking.
                ui.ctx().request_repaint_after(Duration::from_millis(50));
            }

            ui.separator();
            ui.heading("🧮 Background work");
            ui.horizontal(|ui| {
                ui.label("Count primes below");
                ui.add(egui::DragValue::new(&mut self.limit).range(1_000..=20_000_000).speed(10_000));
            });
            if self.job.is_some() {
                ui.add(egui::ProgressBar::new(self.progress).show_percentage().animate(true));
            } else if ui.button("Start counting").clicked() {
                self.start_job(ui.ctx());
            }
            if let Some(count) = self.result {
                ui.label(RichText::new(format!("Found {count} primes.")).color(Color32::LIGHT_GREEN));
            }

            ui.separator();
            ui.label("The window stays responsive while the worker counts. Try it:");
            if ui.button(format!("Clicked {} times", self.clicks)).clicked() {
                self.clicks += 1;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_time() {
        assert_eq!(format_time(Duration::from_millis(61_250)), "01:01.2");
    }

    #[test]
    fn stopwatch_pauses() {
        let mut s = Stopwatch::default();
        assert_eq!(s.elapsed(), Duration::ZERO);
        s.start();
        thread::sleep(Duration::from_millis(20));
        s.pause();
        let paused = s.elapsed();
        assert!(paused >= Duration::from_millis(20));
        thread::sleep(Duration::from_millis(20));
        assert_eq!(s.elapsed(), paused);
    }
}
@@text
## Problem 1: redrawing on time

egui is **lazy**: to save battery, it only redraws when there's a reason (mouse movement, a key, a click). For a stopwatch, *time passing* is the reason, but egui can't know that. So you tell it:

```rust,norun
if self.stopwatch.is_running() {
    ui.ctx().request_repaint_after(Duration::from_millis(50));
}
```

`request_repaint_after` means "draw another frame within 50 ms, even if nothing happens". 20 frames per second is plenty for a stopwatch showing tenths of a second. A countdown in minutes could ask for one every second, or even less. Only ask while something is actually changing: when the stopwatch is paused, the app goes back to sleeping and uses no CPU at all.

(There's also `ctx.request_repaint()`, "redraw as soon as possible", for continuous animations, and for other threads that want the GUI to wake up. More on that below.)

### Storing time

Look at `Stopwatch`. It doesn't count "+1 every frame": frames don't come at exact intervals, so that would drift. Instead it remembers **when** it was started (an `Instant`), and computes the elapsed time from the clock every time it's drawn. Pausing adds the time so far to `before`. This "store the start time, compute the rest" pattern is how every reliable timer works, including the Focus Timer you'll build in the Guided Builds.

## Problem 2: slow work on a worker thread

Counting all primes below 3 million takes a few seconds. Inside `ui()`, the window would freeze for those seconds. So `start_job` hands the work to a **worker thread**, exactly like in the lesson "Time and Doing Things at Once":

```rust,norun
fn start_job(&mut self, ctx: &egui::Context) {
    let (tx, rx) = channel();
    let limit = self.limit;
    let ctx = ctx.clone();
    thread::spawn(move || count_primes(limit, tx, ctx));
    self.job = Some(rx);   // the app keeps the receiving end
}
```

1. A channel connects the worker to the app. The worker gets the sender `tx`; the app keeps the receiver in `self.job`.
2. The worker gets **copies** of everything it needs (`limit`, a clone of the egui `Context`), moved in with `move`. It never touches `self`; the app and the worker share nothing but the channel.
3. Every frame, `check_job` collects whatever messages arrived, with `try_recv`, which **never waits**:

```rust,norun
fn check_job(&mut self) {
    let Some(rx) = &self.job else { return };
    while let Ok(msg) = rx.try_recv() {
        match msg {
            Progress::Working(p) => self.progress = p,
            Progress::Finished(count) => {
                self.result = Some(count);
                self.job = None;
                return;
            }
        }
    }
}
```

The messages are an **enum**, `Progress`, so the worker can send different kinds of news: "I'm 40% done", "finished, here's the result". A real app might add `Failed(String)`.

### Waking the GUI up

There's one catch. If the user doesn't touch the mouse, egui is asleep, so nobody calls `check_job`, and the progress bar wouldn't move. That's why the worker gets a clone of the `Context`: after sending a message, it calls `ctx.request_repaint()`, which wakes the GUI so it draws a new frame and picks up the message. (An `egui::Context` is cheap to clone and safe to use from any thread.)

### Stopping

When you close the window while the worker is busy, the app (and its receiver) is dropped. The worker's next `tx.send(...)` then returns an error, and the worker stops. Always give long-running workers a way to notice that nobody's listening any more.

## The pattern, in short

| In the app | In the worker thread |
|---|---|
| create a channel, keep `rx` in the app state | owns `tx` and copies of its inputs (`move`) |
| every frame: `while let Ok(msg) = rx.try_recv()` | does the slow work |
| update the state from the messages, show progress | sends messages, then `ctx.request_repaint()` |
| clear `rx` when the job is done | ends when the work is done or `send` fails |

This pattern scales to everything slow: loading and saving files, scanning folders, network requests, talking to an AI model. Rust Quest uses it to compile your code; the guided **Local AI Chat** build uses it to stream answers word by word.

## Try it

- Start the stopwatch, then start counting: both keep working, and the click counter still responds.
- Set the limit to 20,000,000 and close the window halfway. The program exits right away: the worker noticed.
- Change `request_repaint_after(Duration::from_millis(50))` to 1000 ms. What happens to the tenths of a second?
- Move the counting into `ui()` directly (call `count_primes` there, without a thread) just once, to see the freeze for yourself. Then undo it!
- Add a **Cancel** button: setting `self.job = None` drops the receiver, so the worker stops at its next `send`.
- Add **lap times**: a `Vec<Duration>` and a "Lap" button that pushes `self.stopwatch.elapsed()`, shown in a list.

@@quiz
Your countdown only updates when you move the mouse. What's missing?
- A `thread::sleep` in `ui()`
* A call to `ctx.request_repaint_after(...)` while the countdown is running
- A bigger window
? egui only redraws when something happens. `request_repaint_after` makes "time passing" count as something.

@@quiz
Why does the worker thread get a clone of the egui `Context`?
- To draw widgets from the worker thread
* To call `request_repaint()` so the sleeping GUI wakes up and reads the new message
- Because threads can't use channels without it
? Drawing still happens only in `ui()`. The worker just pokes the GUI to say "something arrived".

@@quiz
Why does `check_job` use `try_recv` and not `recv`?
- `recv` doesn't exist on `Receiver`
- `try_recv` is faster at receiving messages
* `recv` waits for a message, which would freeze the GUI until the worker sends one
? Nothing in `ui()` may wait. `try_recv` returns at once, message or not.
