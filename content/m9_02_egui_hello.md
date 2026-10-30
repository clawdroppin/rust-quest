@@lesson m9-hello Hello, egui
@@text
Time to open your first window!

## The dependency

egui apps use the **eframe** crate. In `Cargo.toml`:

```text
[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
```

This turns off eframe's default features and picks exactly what we need: the built-in fonts, and the **glow** renderer (which draws using OpenGL and compiles faster than the alternative). With `cargo add`, the same thing is:

```text
cargo add eframe --no-default-features --features default_fonts,glow
```

> **The first build takes a few minutes.** eframe pulls in many crates: windowing, graphics, fonts, text layout. Cargo compiles them all once; after that, builds take seconds. Start `cargo run` and read on while it compiles!

## The code, piece by piece

Create the project below, open it in VS Code, and run it. Then read through `main.rs` with this explanation.

**`main`** configures the window and hands control to eframe:

```rust,norun
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native("Hello egui", options, Box::new(|_cc| Ok(Box::new(HelloApp::default()))))
}
```

- `NativeOptions` holds window settings. `..Default::default()` fills in everything we didn't specify (struct update syntax!).
- `run_native(title, options, creator)` opens the window and runs the event loop until the window is closed. It never returns before that.
- The last argument is a **closure** that creates your app. eframe calls it once at startup. It returns a `Box<dyn App>`: a trait object.

**The app struct** holds all the state:

```rust,norun
#[derive(Default)]
struct HelloApp {
    name: String,
    clicks: u32,
}
```

`#[derive(Default)]` lets us create it with `HelloApp::default()`: an empty name and zero clicks.

**`ui`** is called every frame:

```rust,norun
impl eframe::App for HelloApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello, egui!");
            ...
        });
    }
}
```

- `CentralPanel` is the main area of the window. `.show(ui, |ui| { ... })` takes a **closure** that adds widgets to the panel. Most layout in egui works like this: a container gives your closure a `ui` to fill in.
- `ui.heading(...)`, `ui.label(...)`, `ui.separator()` add simple widgets.

**Widgets that change your data** take a mutable reference to it:

```rust,norun
ui.text_edit_singleline(&mut self.name);
```

When the user types, egui changes `self.name` directly. Next frame, everything that reads `self.name` shows the new value. That's why the greeting updates as you type.

**Buttons** return a `Response`, which knows what happened:

```rust,norun
if ui.button("Click me").clicked() {
    self.clicks += 1;
}
```

**Layout**: widgets go top to bottom by default. `ui.horizontal(|ui| { ... })` places its widgets side by side.

## Your turn

Make these changes, running the app after each:

1. Add a **"−1"** button next to "Click me" that lowers the count (but never below zero: `saturating_sub`). Put the two buttons in a `ui.horizontal`.
2. Add a `show_greeting: bool` field and a checkbox, `ui.checkbox(&mut self.show_greeting, "Greet me")`, that hides or shows the greeting.
3. Make the greeting bigger: `egui::RichText::new(text).size(24.0)` in `ui.label(...)`.
4. Show "even" or "odd" next to the count, in a different colour for each.

If something doesn't compile, read the error carefully: it'll usually be a borrowing or type problem you know how to fix by now.

@@project hello_egui Hello egui
Your first window: a text box, a greeting and a click counter.
@@file Cargo.toml
[package]
name = "hello_egui"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }

@@file src/main.rs
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([440.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native("Hello egui", options, Box::new(|_cc| Ok(Box::new(HelloApp::default()))))
}

/// Everything the app remembers between frames.
#[derive(Default)]
struct HelloApp {
    name: String,
    clicks: u32,
}

impl eframe::App for HelloApp {
    /// Called every frame (many times per second) to draw the whole window.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello, egui!");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });
            if !self.name.trim().is_empty() {
                ui.label(format!("Nice to meet you, {}!", self.name.trim()));
            }

            ui.separator();

            if ui.button("Click me").clicked() {
                self.clicks += 1;
            }
            ui.label(format!("The button has been clicked {} times.", self.clicks));
            if self.clicks >= 10 {
                ui.label(egui::RichText::new("That's a lot of clicking!").strong().color(egui::Color32::LIGHT_GREEN));
            }
            if self.clicks > 0 && ui.button("Reset").clicked() {
                self.clicks = 0;
            }
        });
    }
}

@@quiz
What does `ui.text_edit_singleline(&mut self.name)` do?
- Prints the name
* Draws a text box that edits `self.name` directly when the user types
- Copies the name into the widget, which keeps its own text
? egui widgets edit your data through the `&mut` reference. There's no separate copy to sync.

@@quiz
Why is the first build of an eframe project slow?
- Rust is slow at compiling GUIs
* Cargo compiles eframe and all its dependencies once; later builds reuse them
- It downloads Windows updates
? The dependencies are built once and cached in `target`.
