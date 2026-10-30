@@lesson m9-state App State
@@text
You know the widgets. The real skill in building GUI apps is **organizing state**: what your app struct contains, and how user actions change it. Get this right, and features are easy to add. Get it wrong, and every change becomes a fight with the borrow checker.

## Two kinds of state

Look at the fields of the shopping-list app below:

```rust,norun
struct ShoppingApp {
    // the data
    items: Vec<Item>,
    // UI state
    new_name: String,
    new_quantity: u32,
    show: Show,
}
```

- **Data** is what the app is *about*: the items. It's what you'd save to a file.
- **UI state** is how the user is *currently interacting*: what's typed in the box but not added yet, which filter is selected. It's usually not saved.

Keeping them apart (in the Pocket Ledger, the data even lives in its own module) means you can test and save the data without involving the UI.

## Input buffers

A text box needs a `String` to edit, but that text isn't an item yet: it might be empty or invalid. So the app keeps a separate **buffer** (`new_name`) for the box. Only when the user presses **Add** is the buffer validated and turned into real data:

```rust,norun
fn add_item(&mut self) {
    let name = self.new_name.trim();
    if name.is_empty() {
        return;               // invalid: ignore
    }
    self.items.push(Item { name: name.to_string(), quantity: self.new_quantity, bought: false });
    self.new_name.clear();    // ready for the next one
}
```

The Pocket Ledger uses the same idea for its whole form: the amount is typed as a `String` and only becomes a number in cents after validation, with an error message if it's wrong.

## The borrowing problem, and the action pattern

Here's a problem you **will** hit. You loop over the items to draw them, and each row has a Delete button:

```rust,norun
for (index, item) in self.items.iter_mut().enumerate() {
    ui.label(&item.name);
    if ui.button("Delete").clicked() {
        self.items.remove(index);   // ❌ error: cannot borrow `self.items` as mutable more than once
    }
}
```

The compiler refuses: you can't remove things from a `Vec` while you're in the middle of looping over it. (In other languages this compiles and causes strange bugs, like skipped items or crashes.)

The fix is simple and very common in egui apps: **remember what to do, and do it after the loop**. An enum describes the possible actions:

```rust,norun
enum Action {
    Delete(usize),
    MoveUp(usize),
}

let mut action: Option<Action> = None;
for (index, item) in self.items.iter_mut().enumerate() {
    ...
    if ui.button("Delete").clicked() {
        action = Some(Action::Delete(index));
    }
}
// the loop is over: the Vec isn't borrowed any more
match action {
    Some(Action::Delete(index)) => { self.items.remove(index); }
    Some(Action::MoveUp(index)) => self.items.swap(index, index - 1),
    None => {}
}
```

Only one button can be clicked per frame, so one `Option<Action>` is enough. As a bonus, all the changes are now in one place, which makes them easy to find, log, or later make undoable.

## Derived values: compute, don't store

The app shows "3 items left to buy". Should that be a field? **No.** It's computed from the items every frame:

```rust,norun
let left = self.items.iter().filter(|i| !i.bought).count();
```

If it were a field, you'd have to remember to update it everywhere items change, and one day you'd forget. Store the minimum; compute everything else. Computers are fast, and lists in a personal app are small.

## Your turn

Run the shopping list, then extend it:

1. **Edit quantities**: replace the `x{quantity}` text with a `DragValue` in each row, so quantities can be changed directly. (Hint: `ui.add(egui::DragValue::new(&mut item.quantity).range(1..=99))`. You're already looping with `iter_mut`, so this works inside the loop!)
2. **Sort A–Z**: a button that sorts the items by name. Where should the sorting happen?
3. **A "Down" button** next to "Up": a new `Action` variant. The compiler will tell you where to handle it.
4. **Total count**: show the total number of things to buy (the sum of quantities of unbought items).

@@project shopping_list Shopping List
A small but complete GUI app: input buffers, filtering, and the action pattern.
@@file Cargo.toml
[package]
name = "shopping_list"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }

@@file src/main.rs
use eframe::egui::{self, Color32, RichText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native("Shopping List", options, Box::new(|_cc| Ok(Box::new(ShoppingApp::new()))))
}

// ---------- model ----------

struct Item {
    name: String,
    quantity: u32,
    bought: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Show {
    All,
    ToBuy,
    Bought,
}

/// Things the user asked for while we were drawing the list.
/// We apply them after the loop, when the list isn't borrowed any more.
enum Action {
    Delete(usize),
    MoveUp(usize),
}

// ---------- app state ----------

struct ShoppingApp {
    // the data
    items: Vec<Item>,
    // UI state: what's typed in the inputs, which filter is selected
    new_name: String,
    new_quantity: u32,
    show: Show,
}

impl ShoppingApp {
    fn new() -> Self {
        let item = |name: &str, quantity| Item { name: name.to_string(), quantity, bought: false };
        ShoppingApp {
            items: vec![item("Milk", 2), item("Bread", 1), item("Apples", 6)],
            new_name: String::new(),
            new_quantity: 1,
            show: Show::All,
        }
    }

    fn add_item(&mut self) {
        let name = self.new_name.trim();
        if name.is_empty() {
            return;
        }
        // Already on the list? Increase the quantity instead of adding a duplicate.
        if let Some(existing) = self.items.iter_mut().find(|i| i.name.eq_ignore_ascii_case(name)) {
            existing.quantity += self.new_quantity;
            existing.bought = false;
        } else {
            self.items.push(Item { name: name.to_string(), quantity: self.new_quantity, bought: false });
        }
        self.new_name.clear();
        self.new_quantity = 1;
    }

    fn input_row(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let response = ui.add(egui::TextEdit::singleline(&mut self.new_name).hint_text("Add an item...").desired_width(220.0));
            ui.add(egui::DragValue::new(&mut self.new_quantity).range(1..=99).prefix("x"));
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Add").clicked() || enter {
                self.add_item();
                response.request_focus(); // keep typing without clicking the box again
            }
        });
    }

    fn list(&mut self, ui: &mut egui::Ui) {
        let mut action: Option<Action> = None;

        for (index, item) in self.items.iter_mut().enumerate() {
            let visible = match self.show {
                Show::All => true,
                Show::ToBuy => !item.bought,
                Show::Bought => item.bought,
            };
            if !visible {
                continue;
            }
            ui.horizontal(|ui| {
                ui.checkbox(&mut item.bought, "");
                let text = format!("{} x{}", item.name, item.quantity);
                if item.bought {
                    ui.label(RichText::new(text).strikethrough().weak());
                } else {
                    ui.label(text);
                }
                if index > 0 && ui.small_button("Up").clicked() {
                    action = Some(Action::MoveUp(index));
                }
                if ui.small_button("Delete").clicked() {
                    action = Some(Action::Delete(index));
                }
            });
        }

        // Now the loop is over, so we're allowed to change the Vec itself.
        match action {
            Some(Action::Delete(index)) => {
                self.items.remove(index);
            }
            Some(Action::MoveUp(index)) => self.items.swap(index, index - 1),
            None => {}
        }
    }
}

impl eframe::App for ShoppingApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Shopping List");
            self.input_row(ui);
            ui.separator();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.show, Show::All, "All");
                ui.selectable_value(&mut self.show, Show::ToBuy, "To buy");
                ui.selectable_value(&mut self.show, Show::Bought, "Bought");
            });
            ui.add_space(4.0);

            egui::ScrollArea::vertical().max_height(ui.available_height() - 40.0).show(ui, |ui| {
                self.list(ui);
            });

            ui.separator();
            let left = self.items.iter().filter(|i| !i.bought).count();
            ui.horizontal(|ui| {
                if left == 0 && !self.items.is_empty() {
                    ui.colored_label(Color32::LIGHT_GREEN, "All done!");
                } else {
                    ui.label(format!("{left} item(s) left to buy"));
                }
                let any_bought = self.items.iter().any(|i| i.bought);
                if ui.add_enabled(any_bought, egui::Button::new("Remove bought items")).clicked() {
                    self.items.retain(|i| !i.bought);
                }
            });
        });
    }
}

@@quiz
Which of these belongs in UI state rather than data?
- The list of items
* The text currently typed into the "new item" box
- Each item's quantity
? It's not an item yet: it becomes data only after the user presses Add.

@@quiz
Why does the shopping list collect an `Action` instead of deleting inside the loop?
- It's faster
* The `Vec` is borrowed by the loop, so it can't be changed until the loop ends
- egui doesn't allow buttons inside loops
? The borrow checker prevents modifying a collection while iterating over it. Defer the change until after the loop.

@@quiz
"Number of items left" is shown in the window. How should it be handled?
- Stored in a field and updated whenever items change
* Computed from the items when drawing, every frame
- Saved to a file
? Derived values should be computed, so they can never be out of date.
