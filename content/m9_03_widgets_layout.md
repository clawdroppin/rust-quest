@@lesson m9-widgets Widgets and Layout
@@text
egui comes with all the widgets a typical app needs. This lesson is a guided tour, with a **Widget Gallery** project that shows every one of them working. Run it, click around, and keep it as a reference: when you need a widget later, look at how the gallery does it.

## Input widgets

Every input widget takes a `&mut` to the value it edits:

| Widget | Code | Edits a… |
|---|---|---|
| Text box | `ui.text_edit_singleline(&mut s)` | `String` |
| Text box with options | `ui.add(egui::TextEdit::singleline(&mut s).hint_text("Search..."))` | `String` |
| Multi-line text | `ui.text_edit_multiline(&mut s)` | `String` |
| Checkbox | `ui.checkbox(&mut b, "Label")` | `bool` |
| Radio button | `ui.radio_value(&mut choice, Size::Small, "Small")` | any `PartialEq` value |
| Toggle button | `ui.selectable_value(&mut choice, Page::Home, "Home")` | any `PartialEq` value |
| Slider | `ui.add(egui::Slider::new(&mut x, 0.0..=100.0))` | number |
| Drag value | `ui.add(egui::DragValue::new(&mut n).range(0..=99))` | number |
| Dropdown | `egui::ComboBox::from_id_salt("id").selected_text(...).show_ui(ui, \|ui\| { ... })` | anything |
| Colour | `ui.color_edit_button_srgba(&mut color)` | `Color32` |

`radio_value` and `selectable_value` are perfect for **enums**: they set the variable to the given variant when clicked, and draw themselves as "selected" when it already equals that variant.

Notice the two styles: simple widgets have shortcut methods (`ui.checkbox`), while widgets with options are built as a value with **builder methods** and passed to `ui.add(...)`: `egui::Slider::new(...).suffix("%").step_by(5.0)`. Each builder method takes the widget and returns it with one more setting changed. This builder pattern is common in Rust libraries; you used it with `OpenOptions` too.

## Showing things

| Widget | Code |
|---|---|
| Text | `ui.label("text")` |
| Heading | `ui.heading("Title")` |
| Styled text | `ui.label(RichText::new("text").strong().size(20.0).color(Color32::RED))` |
| Coloured text | `ui.colored_label(Color32::GREEN, "OK")` |
| Link | `ui.hyperlink_to("Rust", "https://www.rust-lang.org")` |
| Progress | `ui.add(egui::ProgressBar::new(0.75).show_percentage())` |
| Line | `ui.separator()` |
| Space | `ui.add_space(10.0)` |

## Responses

Every widget returns a `Response` describing what happened this frame:

```rust,norun
let response = ui.button("Save");
if response.clicked() { ... }
if response.hovered() { ... }

let response = ui.text_edit_singleline(&mut self.search);
if response.changed() { ... }      // the text changed this frame
if response.lost_focus() { ... }   // the user clicked elsewhere or pressed Enter

ui.button("?").on_hover_text("A tooltip");   // add a tooltip
```

Disable a widget with `ui.add_enabled(condition, widget)`. It's greyed out and can't be clicked when the condition is false.

## Layout

By default, widgets are placed **top to bottom**. To arrange them differently, open a nested area with a closure:

```rust,norun
ui.horizontal(|ui| { ... });          // left to right
ui.vertical(|ui| { ... });            // top to bottom (inside a horizontal, for example)
ui.columns(3, |cols| { cols[0].label("A"); cols[1].label("B"); cols[2].label("C"); });
ui.group(|ui| { ... });               // a frame around its contents
ui.collapsing("More", |ui| { ... });  // a section that opens and closes

egui::Grid::new("unique_id").num_columns(2).striped(true).show(ui, |ui| {
    ui.label("Name");  ui.text_edit_singleline(&mut self.name);  ui.end_row();
    ui.label("Age");   ui.add(egui::DragValue::new(&mut self.age)); ui.end_row();
});

egui::ScrollArea::vertical().show(ui, |ui| { ... });   // scroll long content
```

**Grids** are the best way to line up forms and tables. Each `end_row()` starts a new row.

## Panels: the window's structure

Panels divide the window into regions. Add the **edge panels first**, and the `CentralPanel` **last**: it takes whatever space is left.

```rust,norun
fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::Panel::top("toolbar").show(ui, |ui| { ... });
    egui::Panel::left("sidebar").default_size(200.0).show(ui, |ui| { ... });
    egui::Panel::right("details").show(ui, |ui| { ... });
    egui::Panel::bottom("status").show(ui, |ui| { ... });
    egui::CentralPanel::default().show(ui, |ui| { ... });
}
```

Edge panels can be made resizable with `.resizable(true)`. The strings ("toolbar", "sidebar") are **ids**: each panel needs a unique one so egui can remember its size. Grids, combo boxes and scroll areas that appear more than once need unique ids too (`from_id_salt`, `id_salt`).

## Pages with an enum

The gallery switches between pages with a simple pattern you'll use all the time: an enum field, `selectable_value` buttons to change it, and a `match` to draw the right page:

```rust,norun
enum Page { Inputs, Layout, Display }

// in the sidebar:
ui.selectable_value(&mut self.page, Page::Inputs, "Inputs");
// in the central panel:
match self.page {
    Page::Inputs => self.inputs_page(ui),
    Page::Layout => self.layout_page(ui),
    Page::Display => self.display_page(ui),
}
```

Splitting the drawing into methods (`inputs_page`, …) keeps `ui` short and readable, just like splitting any long function.

## Your turn

After exploring the gallery:

1. Add a fourth page, **About**, with your name, a short description and a link.
2. On the Inputs page, show a warning in red when `age` is under 18 and `subscribed` is true.
3. Use the `Size` choice to change the size of the text on the Display page (Small = 12, Medium = 16, Large = 22).

The **egui demo** shows every widget with its source code: search "egui demo" in your browser and open the web version. It's the best reference there is.

@@project widget_gallery Widget Gallery
A clickable reference of egui's widgets, layouts and panels.
@@file Cargo.toml
[package]
name = "widget_gallery"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }

@@file src/main.rs
use eframe::egui::{self, Color32, RichText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 620.0]),
        ..Default::default()
    };
    eframe::run_native("Widget Gallery", options, Box::new(|_cc| Ok(Box::new(Gallery::new()))))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Page {
    Inputs,
    Layout,
    Display,
}

struct Gallery {
    page: Page,
    // inputs
    name: String,
    notes: String,
    subscribed: bool,
    size: Size,
    volume: f32,
    age: u32,
    fruit: String,
    color: Color32,
    // display
    progress: f32,
    clicks: u32,
}

impl Gallery {
    fn new() -> Self {
        Gallery {
            page: Page::Inputs,
            name: String::from("Ferris"),
            notes: String::new(),
            subscribed: true,
            size: Size::Medium,
            volume: 50.0,
            age: 30,
            fruit: String::from("Apple"),
            color: Color32::from_rgb(230, 120, 60),
            progress: 0.3,
            clicks: 0,
        }
    }

    fn inputs_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Input widgets");
        ui.label("Each widget is given a `&mut` to a field of the app's struct, and changes it directly.");
        ui.add_space(8.0);

        // A Grid lines things up in columns. Call end_row() after each row.
        egui::Grid::new("inputs").num_columns(2).spacing([12.0, 10.0]).striped(true).show(ui, |ui| {
            ui.label("Text (single line)");
            ui.text_edit_singleline(&mut self.name);
            ui.end_row();

            ui.label("Text with a hint");
            ui.add(egui::TextEdit::singleline(&mut self.notes).hint_text("Type something..."));
            ui.end_row();

            ui.label("Checkbox");
            ui.checkbox(&mut self.subscribed, "Subscribe to the newsletter");
            ui.end_row();

            ui.label("Radio buttons");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.size, Size::Small, "Small");
                ui.radio_value(&mut self.size, Size::Medium, "Medium");
                ui.radio_value(&mut self.size, Size::Large, "Large");
            });
            ui.end_row();

            ui.label("Selectable buttons");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.size, Size::Small, "S");
                ui.selectable_value(&mut self.size, Size::Medium, "M");
                ui.selectable_value(&mut self.size, Size::Large, "L");
            });
            ui.end_row();

            ui.label("Slider");
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).suffix("%"));
            ui.end_row();

            ui.label("Drag value");
            ui.add(egui::DragValue::new(&mut self.age).range(0..=120).suffix(" years"));
            ui.end_row();

            ui.label("Combo box");
            egui::ComboBox::from_id_salt("fruit").selected_text(&self.fruit).show_ui(ui, |ui| {
                for fruit in ["Apple", "Banana", "Cherry", "Kiwi"] {
                    ui.selectable_value(&mut self.fruit, fruit.to_string(), fruit);
                }
            });
            ui.end_row();

            ui.label("Colour picker");
            ui.color_edit_button_srgba(&mut self.color);
            ui.end_row();
        });

        ui.add_space(10.0);
        ui.label(RichText::new("The current state of the struct:").strong());
        ui.label(format!(
            "name={:?}, subscribed={}, size={:?}, volume={:.0}, age={}, fruit={}",
            self.name, self.subscribed, self.size, self.volume, self.age, self.fruit
        ));
        ui.label(RichText::new("This text uses the colour you picked.").color(self.color).size(18.0));
    }

    fn layout_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Layout");
        ui.add_space(8.0);

        ui.label("ui.horizontal puts widgets side by side:");
        ui.horizontal(|ui| {
            for i in 1..=4 {
                let _ = ui.button(format!("Button {i}"));
            }
        });
        ui.add_space(8.0);

        ui.label("ui.columns splits the space into equal columns:");
        ui.columns(3, |columns| {
            columns[0].label("Left column");
            columns[1].label("Middle column");
            columns[2].label("Right column");
        });
        ui.separator();

        ui.collapsing("A collapsing section (click me)", |ui| {
            ui.label("Hidden until you open it. Great for advanced options.");
        });
        ui.separator();

        ui.label("A group draws a frame around its contents:");
        ui.group(|ui| {
            ui.label("Inside a group");
            ui.label("Still inside");
        });
        ui.separator();

        ui.label("A ScrollArea makes long content scrollable:");
        egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
            for i in 1..=50 {
                ui.label(format!("Line {i}"));
            }
        });
    }

    fn display_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Showing information");
        ui.add_space(8.0);

        ui.label("A normal label.");
        ui.label(RichText::new("Strong text").strong());
        ui.label(RichText::new("Weak (dim) text").weak());
        ui.label(RichText::new("Big coloured text").size(22.0).color(Color32::LIGHT_BLUE));
        ui.label(RichText::new("let code = \"monospace\";").monospace());
        ui.colored_label(Color32::from_rgb(235, 110, 100), "colored_label is a shortcut");
        ui.hyperlink_to("A link to the Rust website", "https://www.rust-lang.org");
        ui.separator();

        ui.add(egui::ProgressBar::new(self.progress).show_percentage());
        ui.horizontal(|ui| {
            if ui.button("Less").clicked() {
                self.progress = (self.progress - 0.1).max(0.0);
            }
            if ui.button("More").clicked() {
                self.progress = (self.progress + 0.1).min(1.0);
            }
        });
        ui.separator();

        let response = ui.button(format!("Hover over me ({} clicks)", self.clicks)).on_hover_text("This is a tooltip!");
        if response.clicked() {
            self.clicks += 1;
        }
        // add_enabled greys out a widget when the condition is false.
        if ui.add_enabled(self.clicks > 0, egui::Button::new("Reset clicks")).clicked() {
            self.clicks = 0;
        }
    }
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Panels: first the ones at the edges, the CentralPanel last (it takes the rest).
        egui::Panel::top("title").show(ui, |ui| {
            ui.heading("Widget Gallery");
        });
        egui::Panel::left("menu").resizable(false).default_size(150.0).show(ui, |ui| {
            ui.add_space(8.0);
            ui.selectable_value(&mut self.page, Page::Inputs, "Inputs");
            ui.selectable_value(&mut self.page, Page::Layout, "Layout");
            ui.selectable_value(&mut self.page, Page::Display, "Display");
        });
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| match self.page {
                Page::Inputs => self.inputs_page(ui),
                Page::Layout => self.layout_page(ui),
                Page::Display => self.display_page(ui),
            });
        });
    }
}

@@quiz
In which order should you add panels?
* Edge panels (top, left, right, bottom) first, then the `CentralPanel` last
- `CentralPanel` first
- The order doesn't matter
? The central panel takes whatever space the edge panels leave.

@@quiz
What's the best widget for choosing one variant of an enum from three options shown side by side?
- A slider
* `radio_value` or `selectable_value` buttons
- A text box
? They set the variable to a variant when clicked and show which one is selected.

@@quiz
What does `ui.add_enabled(false, egui::Button::new("Save"))` do?
- Hides the button
* Shows the button greyed out so it can't be clicked
- Clicks the button
? Disabled widgets are visible but inactive.
