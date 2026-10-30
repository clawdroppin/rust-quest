@@lesson m9-ship Shipping Your App, and What's Next
@@text
You've built a real application. Let's turn it into a proper program you can use every day, and share.

## The finished code

The project card below contains the complete Pocket Ledger. If your version works, keep it! If you want to compare, open **📄 This step's starting files**, or create a separate copy: the folder name is shown on the card, and you can copy the files into a new folder by hand.

## Debug vs. release builds

So far you've used `cargo run`, which makes a **debug build**: compiled quickly, with extra checks (like integer overflow detection) and debugging information, but without optimizations. For the version you actually use:

```text
cargo build --release
```

This takes longer, but the program is optimized: often 10–50× faster, and smaller. The result is in `target\release\pocket_ledger.exe`.

## No console window

Look at the first line of the finished `main.rs`:

```rust,norun
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

By default, Windows opens a black console window next to every Rust program. For a GUI app that's ugly. This **attribute** says: "in release builds (when debug assertions are off), make this a Windows GUI program, without a console". Debug builds keep the console, which is handy for `println!` and `dbg!` while developing.

- `#![...]` with `!` applies to the whole crate (it must be at the very top of `main.rs`).
- `cfg_attr(condition, attribute)` applies the attribute only when the condition is true. You've seen `cfg` before in `#[cfg(test)]`.

## Make it portable

`pocket_ledger.exe` is a **single file** with no installer and no dependencies (Rust links everything in). To install it:

1. Create a folder anywhere, e.g. `Documents\Pocket Ledger` or a USB stick.
2. Copy `target\release\pocket_ledger.exe` into it.
3. Double-click it. The data file `pocket_ledger.json` will be created next to it.
4. Right-click the `.exe` → **Show more options** → **Send to → Desktop (create shortcut)**, or pin it to Start.

Because the data lives next to the program, you can copy the whole folder to another Windows PC and keep going. To back up your data, copy `pocket_ledger.json`.

> Your debug-build data is in `target\debug\pocket_ledger.json`. Copy it next to the release `.exe` if you want to keep it.

## Sharing it

You can send `pocket_ledger.exe` to a friend. Two things to know:

- Windows SmartScreen may warn about "an unrecognized app", because the program isn't signed with a (paid) code-signing certificate. They can click **More info → Run anyway**.
- It needs OpenGL 2+ graphics support, which every Windows PC from the last 15 years has.

To share the **source code**, put the project on GitHub. People with Rust installed can build it themselves with `cargo build --release`.

## Make it yours: ideas to extend Pocket Ledger

Every one of these is doable with what you've learned. Pick one, plan it (model first, with tests), and build it. This is how you go from "followed a course" to "can program".

**Easier**

- **Change the currency**: edit `CURRENCY` in `model.rs`. Could the user choose it in the app? (Save it in the `Ledger` with `#[serde(default)]`.)
- **Light/dark theme toggle**: `ui.ctx().set_theme(egui::Theme::Light)`. Store the choice.
- **Confirm before deleting**: an `Option<u32>` field "pending delete" and a "Really delete?" button.
- **Sort the table** by amount or description by clicking the column headers.

**Medium**

- **Edit a transaction**: an "Edit" button that loads it into the form, and "Save changes" instead of "Add". (Hint: an `editing: Option<u32>` field in the app; a `Ledger::update` method with a test.)
- **CSV export**: a button that writes `export.csv` (`date,description,category,amount` per line) that Excel can open. Watch out for commas in descriptions!
- **Budgets per category**: a `HashMap<Category, i64>` in the `Ledger` (serde handles maps) and one progress bar per category.
- **Monthly overview chart**: income and expense bars side by side for the last 6 months, with the painter.

**Harder**

- **Recurring transactions**: rent and salary are the same every month. Store templates, and add them automatically when a new month starts.
- **Undo**: keep a `Vec` of previous states (the `Ledger` would need `Clone`) and an Undo button (and `Ctrl+Z`: look at `ui.input(|i| i.key_pressed(...))`).
- **Multiple accounts**: cash, bank, savings; transfers between them.

## Congratulations!

Take a moment to look back. You started with "what is a program?" and you've now:

- learned the fundamentals of programming: values, variables, types, conditions, loops, functions, data structures, and how to break a problem into steps;
- learned Rust's syntax and its big ideas: ownership and borrowing, enums and pattern matching, `Option` and `Result` instead of null and exceptions, traits and generics;
- learned to organize real code: modules, crates, tests, documentation, error handling, files and JSON;
- built command-line tools, games, and a complete desktop application with a GUI.

That's a genuinely solid foundation. Rust is considered one of the harder languages to learn, and you did it.

## Where to go next

- **Keep building.** The best way to learn more is to make something you want to exist. A habit tracker, a game, a tool for your hobby, a program that renames your photos. Start small, finish it, then make it better.
- **The Rust Book** (search "The Rust Programming Language book"): the official free book. You'll find it easy going now, and it goes deeper on everything in this course.
- **Rustlings**: small exercises that run in your terminal. Great for keeping your skills sharp.
- **Exercism** (Rust track) and **Advent of Code** (every December): programming puzzles at every level.
- **Rust by Example**: short, runnable examples of every feature.
- **Explore crates** for what interests you: `ratatui` (terminal UIs), `bevy` or `macroquad` (games), `axum` (web servers), `reqwest` (talking to web APIs), `rusqlite` (databases), `clap` (command-line arguments), `chrono` (dates and times).
- **Join the community**: the official Rust forum (users.rust-lang.org) and the Rust Discord are friendly to beginners. Asking good questions is a skill too: show your code, the full error, and what you expected.

Thank you for learning with Rust Quest. Now go build something! 🦀

@@project pocket_ledger Pocket Ledger, finished
The complete budget app: transactions, filters, search, a category chart, a monthly budget, and crash-safe saving.
@@file Cargo.toml
[package]
name = "pocket_ledger"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

@@file src/main.rs
// In release builds, don't open a console window next to the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod model;
mod storage;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Pocket Ledger")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Pocket Ledger", options, Box::new(|_cc| Ok(Box::new(app::LedgerApp::load()))))
}

@@file src/app.rs
//! The window: drawing the ledger and reacting to the user.
//! All the rules live in model.rs; this file only shows things and calls methods.

use crate::model::{self, Category, Filter, Kind, Ledger};
use crate::storage;
use eframe::egui::{self, Color32, RichText};

const INCOME_COLOR: Color32 = Color32::from_rgb(110, 200, 120);
const EXPENSE_COLOR: Color32 = Color32::from_rgb(235, 110, 100);
const ACCENT: Color32 = Color32::from_rgb(110, 160, 240);

/// What the user is typing into the "new transaction" form.
struct Form {
    date: String,
    description: String,
    amount: String,
    kind: Kind,
    category: Category,
    error: Option<String>,
}

impl Form {
    fn new() -> Self {
        Form {
            date: model::today(),
            description: String::new(),
            amount: String::new(),
            kind: Kind::Expense,
            category: Category::Food,
            error: None,
        }
    }
}

pub struct LedgerApp {
    ledger: Ledger,
    form: Form,
    filter: Filter,
    /// A message for the status bar at the bottom.
    status: String,
}

impl LedgerApp {
    pub fn load() -> Self {
        let (ledger, status) = match storage::load() {
            Ok(ledger) => (ledger, format!("Your data is saved in {}", storage::data_path().display())),
            Err(e) => (Ledger::new(), e),
        };
        LedgerApp { ledger, form: Form::new(), filter: Filter::default(), status }
    }

    fn save(&mut self) {
        if let Err(e) = storage::save(&self.ledger) {
            self.status = e;
        }
    }

    /// Checks the form and, if everything is valid, adds the transaction.
    fn submit_form(&mut self) {
        let form = &mut self.form;
        let date = form.date.trim();
        let description = form.description.trim();
        if !model::is_valid_date(date) {
            form.error = Some(String::from("The date must look like 2026-10-30"));
            return;
        }
        if description.is_empty() {
            form.error = Some(String::from("Enter a description"));
            return;
        }
        let amount = match model::parse_amount(&form.amount) {
            Ok(amount) => amount,
            Err(e) => {
                form.error = Some(e);
                return;
            }
        };
        self.ledger.add(date, description, amount, form.kind, form.category);
        form.description.clear();
        form.amount.clear();
        form.error = None;
        self.save();
    }

    fn top_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("Pocket Ledger").strong().color(ACCENT));
            ui.add_space(20.0);
            let balance = self.ledger.balance();
            let color = if balance < 0 { EXPENSE_COLOR } else { INCOME_COLOR };
            ui.label("Balance:");
            ui.label(RichText::new(model::format_money(balance)).size(20.0).strong().color(color));
        });
    }

    fn form_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("New transaction");
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.form.kind, Kind::Expense, RichText::new("Expense").color(EXPENSE_COLOR));
            ui.selectable_value(&mut self.form.kind, Kind::Income, RichText::new("Income").color(INCOME_COLOR));
        });
        ui.add_space(6.0);

        let mut submit = false;
        egui::Grid::new("form").num_columns(2).spacing([8.0, 8.0]).show(ui, |ui| {
            ui.label("Date");
            ui.text_edit_singleline(&mut self.form.date);
            ui.end_row();

            ui.label("Description");
            ui.text_edit_singleline(&mut self.form.description);
            ui.end_row();

            ui.label("Amount");
            let response = ui.text_edit_singleline(&mut self.form.amount);
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submit = true;
            }
            ui.end_row();

            ui.label("Category");
            egui::ComboBox::from_id_salt("category")
                .selected_text(self.form.category.label())
                .show_ui(ui, |ui| {
                    for category in Category::ALL {
                        ui.selectable_value(&mut self.form.category, category, category.label());
                    }
                });
            ui.end_row();
        });

        ui.add_space(8.0);
        if ui.button(RichText::new("  Add  ").strong()).clicked() {
            submit = true;
        }
        if submit {
            self.submit_form();
        }
        if let Some(error) = &self.form.error {
            ui.colored_label(EXPENSE_COLOR, error);
        }

        ui.add_space(16.0);
        ui.separator();
        ui.heading("Monthly budget");
        ui.label(RichText::new("How much you plan to spend per month. 0 = no budget.").small().weak());
        let mut budget = self.ledger.monthly_budget / 100;
        let response = ui.add(egui::DragValue::new(&mut budget).range(0..=1_000_000).speed(10).prefix(model::CURRENCY));
        if response.changed() {
            self.ledger.monthly_budget = budget * 100;
        }
        if response.drag_stopped() || response.lost_focus() {
            self.save();
        }
    }

    fn summary_panel(&self, ui: &mut egui::Ui) {
        let title = match &self.filter.month {
            Some(month) => format!("Summary for {month}"),
            None => String::from("Summary (all months)"),
        };
        ui.heading(title);
        ui.add_space(6.0);

        let month_filter = Filter { month: self.filter.month.clone(), ..Filter::default() };
        let in_period = self.ledger.filtered(&month_filter);
        let (income, expenses) = model::totals(&in_period);
        egui::Grid::new("summary").num_columns(2).spacing([16.0, 4.0]).show(ui, |ui| {
            ui.label("Income");
            ui.colored_label(INCOME_COLOR, model::format_money(income));
            ui.end_row();
            ui.label("Expenses");
            ui.colored_label(EXPENSE_COLOR, model::format_money(expenses));
            ui.end_row();
            ui.label(RichText::new("Net").strong());
            let net = income - expenses;
            let color = if net < 0 { EXPENSE_COLOR } else { INCOME_COLOR };
            ui.label(RichText::new(model::format_money(net)).strong().color(color));
            ui.end_row();
        });

        // The budget only makes sense for a single month.
        if self.ledger.monthly_budget > 0 && self.filter.month.is_some() {
            ui.add_space(10.0);
            let budget = self.ledger.monthly_budget;
            let fraction = expenses as f32 / budget as f32;
            let color = if fraction > 1.0 { EXPENSE_COLOR } else if fraction > 0.8 { Color32::from_rgb(230, 190, 90) } else { INCOME_COLOR };
            ui.label("Budget used");
            ui.add(
                egui::ProgressBar::new(fraction.min(1.0))
                    .fill(color)
                    .text(format!("{} of {}", model::format_money(expenses), model::format_money(budget))),
            );
            if fraction > 1.0 {
                ui.colored_label(EXPENSE_COLOR, format!("Over budget by {}", model::format_money(expenses - budget)));
            } else {
                ui.label(format!("{} left", model::format_money(budget - expenses)));
            }
        }

        ui.add_space(14.0);
        ui.heading("Spending by category");
        ui.add_space(4.0);
        let spending = self.ledger.spending_by_category(self.filter.month.as_deref());
        category_chart(ui, &spending);
    }

    fn transactions_panel(&mut self, ui: &mut egui::Ui) {
        // --- filters ---
        ui.horizontal(|ui| {
            let month_text = self.filter.month.clone().unwrap_or_else(|| String::from("All months"));
            egui::ComboBox::from_id_salt("month").selected_text(month_text).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.filter.month, None, "All months");
                for month in self.ledger.months() {
                    ui.selectable_value(&mut self.filter.month, Some(month.clone()), month);
                }
            });

            let kind_text = match self.filter.kind {
                None => "Income & expenses",
                Some(kind) => kind.label(),
            };
            egui::ComboBox::from_id_salt("kind").selected_text(kind_text).show_ui(ui, |ui| {
                ui.selectable_value(&mut self.filter.kind, None, "Income & expenses");
                ui.selectable_value(&mut self.filter.kind, Some(Kind::Income), "Income");
                ui.selectable_value(&mut self.filter.kind, Some(Kind::Expense), "Expense");
            });

            ui.add(egui::TextEdit::singleline(&mut self.filter.search).hint_text("Search...").desired_width(160.0));
            if self.filter != Filter::default() && ui.button("Clear filters").clicked() {
                self.filter = Filter::default();
            }
        });
        ui.separator();

        if self.ledger.transactions.is_empty() {
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("No transactions yet.").size(18.0));
                ui.label("Add your first one with the form on the left.");
                ui.add_space(10.0);
                if ui.button("...or add some example data to explore").clicked() {
                    add_sample_data(&mut self.ledger);
                    self.save();
                }
            });
            return;
        }

        // --- the list ---
        let list = self.ledger.filtered(&self.filter);
        let (income, expenses) = model::totals(&list);
        ui.label(format!(
            "{} transaction(s): {} in, {} out",
            list.len(),
            model::format_money(income),
            model::format_money(expenses)
        ));
        ui.add_space(4.0);

        // We can't delete while we're looping over the list (it's borrowed),
        // so we remember what to delete and do it afterwards.
        let mut delete: Option<u32> = None;
        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            egui::Grid::new("transactions").striped(true).num_columns(5).spacing([16.0, 6.0]).show(ui, |ui| {
                ui.label(RichText::new("Date").strong());
                ui.label(RichText::new("Description").strong());
                ui.label(RichText::new("Category").strong());
                ui.label(RichText::new("Amount").strong());
                ui.label("");
                ui.end_row();

                for t in &list {
                    ui.label(&t.date);
                    ui.label(&t.description);
                    ui.label(t.category.label());
                    let color = if t.kind == Kind::Income { INCOME_COLOR } else { EXPENSE_COLOR };
                    ui.colored_label(color, model::format_money(t.signed_amount()));
                    if ui.small_button("Delete").clicked() {
                        delete = Some(t.id);
                    }
                    ui.end_row();
                }
            });
        });

        if let Some(id) = delete {
            self.ledger.remove(id);
            self.save();
        }
    }
}

impl eframe::App for LedgerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(6.0);
            self.top_bar(ui);
            ui.add_space(4.0);
        });
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.label(RichText::new(&self.status).small().weak());
        });
        egui::Panel::left("form").resizable(false).default_size(280.0).show(ui, |ui| {
            ui.add_space(8.0);
            self.form_panel(ui);
        });
        egui::Panel::right("summary").resizable(false).default_size(300.0).show(ui, |ui| {
            ui.add_space(8.0);
            self.summary_panel(ui);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            self.transactions_panel(ui);
        });
    }
}

/// Draws one horizontal bar per category, scaled to the biggest one.
fn category_chart(ui: &mut egui::Ui, spending: &[(Category, i64)]) {
    let Some(&(_, biggest)) = spending.first() else {
        ui.label(RichText::new("No expenses in this period.").weak());
        return;
    };
    let label_width = 80.0;
    let amount_width = 90.0;
    for &(category, amount) in spending {
        // Reserve a strip of the panel for this row, then paint inside it.
        let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::hover());
        let painter = ui.painter();
        let text_color = ui.visuals().text_color();
        let font = egui::FontId::proportional(13.0);

        painter.text(rect.left_center(), egui::Align2::LEFT_CENTER, category.label(), font.clone(), text_color);
        painter.text(rect.right_center(), egui::Align2::RIGHT_CENTER, model::format_money(amount), font, text_color);

        let max_bar = (rect.width() - label_width - amount_width).max(10.0);
        let bar_width = max_bar * amount as f32 / biggest as f32;
        let bar = egui::Rect::from_min_size(
            egui::pos2(rect.left() + label_width, rect.top() + 4.0),
            egui::vec2(bar_width.max(2.0), rect.height() - 8.0),
        );
        painter.rect_filled(bar, 3.0, EXPENSE_COLOR);

        let share = 100.0 * amount as f32 / spending.iter().map(|(_, a)| *a).sum::<i64>() as f32;
        response.on_hover_text(format!("{}: {:.0}% of spending", category.label(), share));
    }
}

/// Fills an empty ledger with a few believable transactions for this month and last month.
fn add_sample_data(ledger: &mut Ledger) {
    let today = model::today();
    let this_month = &today[..7];
    let year: i32 = today[..4].parse().unwrap_or(2026);
    let month: u32 = today[5..7].parse().unwrap_or(1);
    let last_month = if month == 1 { format!("{}-12", year - 1) } else { format!("{year}-{:02}", month - 1) };

    for month in [last_month.as_str(), this_month] {
        let date = |day: u32| format!("{month}-{day:02}");
        ledger.add(&date(1), "Salary", 320_000, Kind::Income, Category::Salary);
        ledger.add(&date(1), "Rent", 110_000, Kind::Expense, Category::Housing);
        ledger.add(&date(2), "Supermarket", 8_640, Kind::Expense, Category::Food);
        ledger.add(&date(3), "Bus pass", 4_900, Kind::Expense, Category::Transport);
        ledger.add(&date(4), "Electricity", 6_215, Kind::Expense, Category::Bills);
        ledger.add(&date(5), "Pizza with friends", 2_350, Kind::Expense, Category::Fun);
        ledger.add(&date(7), "Pharmacy", 1_480, Kind::Expense, Category::Health);
    }
    ledger.add(&format!("{last_month}-12"), "New headphones", 7_999, Kind::Expense, Category::Shopping);
    ledger.add(&format!("{last_month}-20"), "Birthday money", 5_000, Kind::Income, Category::Gift);
    ledger.monthly_budget = 150_000;
}

@@file src/model.rs
//! The data model: transactions, categories and the ledger, plus helpers for
//! money and dates. Nothing in here knows about the GUI.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shown in front of every amount. Change it to "€", "£", "zł "...
pub const CURRENCY: &str = "$";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kind {
    Expense,
    Income,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Expense => "Expense",
            Kind::Income => "Income",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Category {
    Food,
    Housing,
    Transport,
    Bills,
    Shopping,
    Fun,
    Health,
    Salary,
    Gift,
    Other,
}

impl Category {
    /// Every category, in the order they appear in menus.
    pub const ALL: [Category; 10] = [
        Category::Food,
        Category::Housing,
        Category::Transport,
        Category::Bills,
        Category::Shopping,
        Category::Fun,
        Category::Health,
        Category::Salary,
        Category::Gift,
        Category::Other,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Food => "Food",
            Category::Housing => "Housing",
            Category::Transport => "Transport",
            Category::Bills => "Bills",
            Category::Shopping => "Shopping",
            Category::Fun => "Fun",
            Category::Health => "Health",
            Category::Salary => "Salary",
            Category::Gift => "Gift",
            Category::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    /// "YYYY-MM-DD"
    pub date: String,
    pub description: String,
    /// In cents, always positive. `kind` says whether it's money in or out.
    pub amount: i64,
    pub kind: Kind,
    pub category: Category,
}

impl Transaction {
    /// Positive for income, negative for expenses.
    pub fn signed_amount(&self) -> i64 {
        match self.kind {
            Kind::Income => self.amount,
            Kind::Expense => -self.amount,
        }
    }

    /// The month part of the date: "2026-10-30" -> "2026-10".
    pub fn month(&self) -> &str {
        self.date.get(..7).unwrap_or("")
    }
}

/// Which transactions to show. The default shows everything.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Filter {
    /// Only this month ("YYYY-MM"), or every month if None.
    pub month: Option<String>,
    /// Only income or only expenses, or both if None.
    pub kind: Option<Kind>,
    /// Text that must appear in the description or category name (ignoring case).
    pub search: String,
}

impl Filter {
    pub fn matches(&self, t: &Transaction) -> bool {
        if let Some(month) = &self.month {
            if t.month() != month.as_str() {
                return false;
            }
        }
        if let Some(kind) = self.kind {
            if t.kind != kind {
                return false;
            }
        }
        let search = self.search.trim().to_lowercase();
        search.is_empty()
            || t.description.to_lowercase().contains(&search)
            || t.category.label().to_lowercase().contains(&search)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    pub transactions: Vec<Transaction>,
    next_id: u32,
    /// Spending limit per month, in cents. 0 means "no budget".
    #[serde(default)]
    pub monthly_budget: i64,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger::new()
    }
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { transactions: Vec::new(), next_id: 1, monthly_budget: 0 }
    }

    /// Adds a transaction and returns its new id.
    pub fn add(&mut self, date: &str, description: &str, amount: i64, kind: Kind, category: Category) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.transactions.push(Transaction {
            id,
            date: date.to_string(),
            description: description.to_string(),
            amount,
            kind,
            category,
        });
        id
    }

    /// Removes the transaction with this id. Returns false if there was none.
    pub fn remove(&mut self, id: u32) -> bool {
        let before = self.transactions.len();
        self.transactions.retain(|t| t.id != id);
        self.transactions.len() != before
    }

    /// All income minus all expenses.
    pub fn balance(&self) -> i64 {
        self.transactions.iter().map(|t| t.signed_amount()).sum()
    }

    /// Every month that has transactions, newest first.
    pub fn months(&self) -> Vec<String> {
        let mut months: Vec<String> = self.transactions.iter().map(|t| t.month().to_string()).collect();
        months.sort();
        months.dedup();
        months.reverse();
        months
    }

    /// The transactions that match the filter, newest first.
    pub fn filtered(&self, filter: &Filter) -> Vec<&Transaction> {
        let mut list: Vec<&Transaction> = self.transactions.iter().filter(|t| filter.matches(t)).collect();
        list.sort_by(|a, b| b.date.cmp(&a.date).then(b.id.cmp(&a.id)));
        list
    }

    /// Total expenses per category (in one month, or all months if None),
    /// biggest first. Categories with no expenses are left out.
    pub fn spending_by_category(&self, month: Option<&str>) -> Vec<(Category, i64)> {
        let mut totals: HashMap<Category, i64> = HashMap::new();
        for t in &self.transactions {
            if t.kind != Kind::Expense {
                continue;
            }
            if let Some(m) = month {
                if t.month() != m {
                    continue;
                }
            }
            *totals.entry(t.category).or_insert(0) += t.amount;
        }
        let mut list: Vec<(Category, i64)> = totals.into_iter().collect();
        list.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        list
    }
}

/// Adds up a list of transactions: (total income, total expenses).
pub fn totals(list: &[&Transaction]) -> (i64, i64) {
    let mut income = 0;
    let mut expenses = 0;
    for t in list {
        match t.kind {
            Kind::Income => income += t.amount,
            Kind::Expense => expenses += t.amount,
        }
    }
    (income, expenses)
}

/// Turns what the user typed into cents: "12" -> 1200, "4.5" -> 450, "0,99" -> 99.
pub fn parse_amount(text: &str) -> Result<i64, String> {
    let text = text.trim().replace(',', ".");
    if text.is_empty() {
        return Err(String::from("Enter an amount"));
    }
    let (whole, fraction) = match text.split_once('.') {
        Some((w, f)) => (w, f),
        None => (text.as_str(), ""),
    };
    let all_digits = |s: &str| s.chars().all(|c| c.is_ascii_digit());
    if !all_digits(whole) || !all_digits(fraction) || (whole.is_empty() && fraction.is_empty()) {
        return Err(format!("'{}' is not an amount", text));
    }
    if fraction.len() > 2 {
        return Err(String::from("Use at most 2 decimal places"));
    }
    let whole: i64 = if whole.is_empty() { 0 } else { whole.parse().map_err(|_| String::from("That amount is too large"))? };
    let mut cents: i64 = if fraction.is_empty() { 0 } else { fraction.parse().unwrap_or(0) };
    if fraction.len() == 1 {
        cents *= 10; // "4.5" means 50 cents, not 5
    }
    let total = whole.checked_mul(100).and_then(|w| w.checked_add(cents)).ok_or("That amount is too large")?;
    if total == 0 {
        return Err(String::from("The amount must be more than zero"));
    }
    Ok(total)
}

/// Formats cents for display: 123456 -> "$1,234.56", -305 -> "-$3.05".
pub fn format_money(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.unsigned_abs();
    let whole = (abs / 100).to_string();
    let mut grouped = String::new();
    for (i, c) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    format!("{sign}{CURRENCY}{grouped}.{:02}", abs % 100)
}

/// Checks that a date is a real calendar date written as "YYYY-MM-DD".
pub fn is_valid_date(text: &str) -> bool {
    if text.len() != 10 {
        return false;
    }
    for (i, c) in text.chars().enumerate() {
        let ok = if i == 4 || i == 7 { c == '-' } else { c.is_ascii_digit() };
        if !ok {
            return false;
        }
    }
    let year: u32 = text[0..4].parse().unwrap_or(0);
    let month: u32 = text[5..7].parse().unwrap_or(0);
    let day: u32 = text[8..10].parse().unwrap_or(0);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    day >= 1 && day <= days_in_month
}

/// Today's date as "YYYY-MM-DD" (in UTC, which is close enough for a budget app).
pub fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_from_days((seconds / 86_400) as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Converts a number of days since 1970-01-01 into (year, month, day).
/// This is Howard Hinnant's well-known algorithm; you don't need to understand the maths!
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Ledger {
        let mut l = Ledger::new();
        l.add("2026-09-30", "Rent", 80_000, Kind::Expense, Category::Housing);
        l.add("2026-10-01", "Salary", 250_000, Kind::Income, Category::Salary);
        l.add("2026-10-02", "Groceries", 5_430, Kind::Expense, Category::Food);
        l.add("2026-10-05", "Cinema", 1_200, Kind::Expense, Category::Fun);
        l.add("2026-10-07", "Pizza", 2_000, Kind::Expense, Category::Food);
        l
    }

    #[test]
    fn parses_amounts() {
        assert_eq!(parse_amount("12"), Ok(1200));
        assert_eq!(parse_amount("12.5"), Ok(1250));
        assert_eq!(parse_amount("12.05"), Ok(1205));
        assert_eq!(parse_amount(" 0,99 "), Ok(99));
        assert_eq!(parse_amount(".5"), Ok(50));
        assert_eq!(parse_amount("7."), Ok(700));
    }

    #[test]
    fn rejects_bad_amounts() {
        for bad in ["", "   ", "abc", "-5", "1.234", "1.2.3", "0", "0.00", ".", "12e3", "99999999999999999999"] {
            assert!(parse_amount(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn formats_money() {
        assert_eq!(format_money(0), "$0.00");
        assert_eq!(format_money(5), "$0.05");
        assert_eq!(format_money(1250), "$12.50");
        assert_eq!(format_money(123_456), "$1,234.56");
        assert_eq!(format_money(100_000_000), "$1,000,000.00");
        assert_eq!(format_money(-305), "-$3.05");
    }

    #[test]
    fn validates_dates() {
        assert!(is_valid_date("2026-10-30"));
        assert!(is_valid_date("2024-02-29"));
        assert!(!is_valid_date("2026-02-29"));
        assert!(!is_valid_date("2026-13-01"));
        assert!(!is_valid_date("2026-04-31"));
        assert!(!is_valid_date("2026-1-5"));
        assert!(!is_valid_date("hello"));
        assert!(is_valid_date(&today()));
    }

    #[test]
    fn balance_adds_income_and_subtracts_expenses() {
        assert_eq!(Ledger::new().balance(), 0);
        assert_eq!(sample().balance(), 250_000 - 80_000 - 5_430 - 1_200 - 2_000);
    }

    #[test]
    fn remove_deletes_by_id() {
        let mut l = sample();
        assert!(l.remove(3));
        assert!(!l.remove(3));
        assert_eq!(l.transactions.len(), 4);
        assert_eq!(l.add("2026-10-08", "Bus", 300, Kind::Expense, Category::Transport), 6, "ids are never reused");
    }

    #[test]
    fn filters_by_month_kind_and_text() {
        let l = sample();
        let october = Filter { month: Some(String::from("2026-10")), ..Filter::default() };
        assert_eq!(l.filtered(&october).len(), 4);
        let expenses = Filter { kind: Some(Kind::Expense), ..Filter::default() };
        assert_eq!(l.filtered(&expenses).len(), 4);
        let food = Filter { search: String::from("FOOD"), ..Filter::default() };
        assert_eq!(l.filtered(&food).len(), 2, "search also matches the category name");
        let pizza = Filter { search: String::from("piz"), ..Filter::default() };
        assert_eq!(l.filtered(&pizza)[0].description, "Pizza");
        let all = l.filtered(&Filter::default());
        assert_eq!(all[0].description, "Pizza", "newest first");
        assert_eq!(all[4].description, "Rent");
    }

    #[test]
    fn lists_months_newest_first() {
        assert_eq!(sample().months(), ["2026-10", "2026-09"]);
    }

    #[test]
    fn spending_by_category_is_sorted_biggest_first() {
        let l = sample();
        assert_eq!(
            l.spending_by_category(Some("2026-10")),
            vec![(Category::Food, 7_430), (Category::Fun, 1_200)]
        );
        assert_eq!(l.spending_by_category(None)[0], (Category::Housing, 80_000));
        assert!(l.spending_by_category(Some("2025-01")).is_empty());
    }

    #[test]
    fn totals_split_income_and_expenses() {
        let l = sample();
        let all = l.filtered(&Filter::default());
        assert_eq!(totals(&all), (250_000, 88_630));
    }
}

@@file src/storage.rs
//! Saving and loading the ledger as a JSON file next to the program.

use crate::model::Ledger;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

const FILE_NAME: &str = "pocket_ledger.json";

/// The data file lives next to the .exe, so the app is portable: copy the
/// folder to a USB stick and your data comes along.
/// (While developing, the .exe is in target\debug, so the file is there too.)
pub fn data_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join(FILE_NAME)))
        .unwrap_or_else(|| PathBuf::from(FILE_NAME))
}

/// Loads the ledger. A missing file just means "first run": an empty ledger.
/// A damaged file is renamed (so it's never overwritten) and reported as an error.
pub fn load() -> Result<Ledger, String> {
    let path = data_path();
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Ledger::new()),
        Err(e) => return Err(format!("Could not read {}: {e}", path.display())),
    };
    serde_json::from_str(&text).map_err(|e| {
        let backup = path.with_extension("damaged.json");
        let _ = fs::rename(&path, &backup);
        format!("The data file was damaged ({e}). It was renamed to {}.", backup.display())
    })
}

/// Saves the ledger. It writes a temporary file first and then renames it, so
/// a crash halfway through saving can never leave you with a half-written file.
pub fn save(ledger: &Ledger) -> Result<(), String> {
    let path = data_path();
    let json = serde_json::to_string_pretty(ledger).map_err(|e| e.to_string())?;
    let temp = path.with_extension("json.tmp");
    fs::write(&temp, json).map_err(|e| format!("Could not save: {e}"))?;
    fs::rename(&temp, &path).map_err(|e| format!("Could not save: {e}"))
}

@@quiz
What does `cargo build --release` do differently from `cargo build`?
- It uploads your program to crates.io
* It compiles with optimizations, producing a much faster program (more slowly)
- It removes all your tests
? Release builds take longer to compile but run much faster. The `.exe` goes in `target\release`.

@@quiz
What does `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` do?
- Makes the app only run on Windows
* Hides the console window in release builds, while keeping it in debug builds
- Enables debugging
? It applies `windows_subsystem = "windows"` only when debug assertions are off, which is the case in release builds.
