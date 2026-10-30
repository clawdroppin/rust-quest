@@lesson m9-ledger2 Pocket Ledger 2: The Window
@@text
Your model works and is tested. Now let's give it a window: a form to add transactions on the left, and the list of transactions in the middle.

## Getting this step's files

This step adds `src/app.rs` and changes two files:

- **`Cargo.toml`** gets the `eframe` dependency;
- **`src/main.rs`** now opens a window instead of printing a report.

In the project card below, click **➕ Add new file(s)** to get `app.rs` (your own `model.rs` is not touched). Then open **📄 This step's starting files** and copy the new `Cargo.toml` and `src/main.rs` over yours. (Alternatively, **↺ Reset project** replaces everything with this step's files, including a finished `model.rs`; your old files are backed up first.)

Run `cargo run`. The first build takes a few minutes while eframe compiles. You'll see a window with a working form: add a few transactions and watch the balance at the top change. The list is plain text for now; making it great is your job in this lesson.

## How `app.rs` is organized

```rust,norun
struct Form {           // UI state: what's being typed into the form
    date: String,
    description: String,
    amount: String,     // a String! It only becomes cents after validation
    kind: Kind,
    category: Category,
    error: Option<String>,
}

pub struct LedgerApp {
    ledger: Ledger,     // the data: your model from step 1
    form: Form,
}
```

The data lives in `ledger`, the form's in-progress input in `form`. Exactly the separation from the App State lesson.

**`submit_form`** is the bridge between them. It validates the form's text fields with your model's functions (`is_valid_date`, `parse_amount`), shows an error message if something's wrong, and otherwise calls `self.ledger.add(...)` and clears the form:

```rust,norun
let amount = match model::parse_amount(&self.form.amount) {
    Ok(amount) => amount,
    Err(e) => {
        self.form.error = Some(e);   // shown in red under the Add button
        return;
    }
};
```

Notice how your careful work on `parse_amount`'s error messages pays off: they appear directly in the GUI.

**`form_panel`** draws the form in a `Grid`. A few details worth noticing:

- Expense/Income are two `selectable_value`s acting like a toggle.
- The category dropdown loops over `Category::ALL`, so adding a category to the enum adds it to the menu automatically.
- Pressing **Enter** in the amount box also submits: a text box `lost_focus()` and the Enter key was pressed this frame.
- The `submit` variable is another small use of the "remember now, act later" idea: the form is drawn first, then submitted.

**`ui`** sets up the panels: a top bar with the balance, the form on the left, and the list in the central panel.

## Task 1: a real table with Delete buttons

Replace the body of `transactions_panel` so it shows a `Grid` with a header row, colours amounts green or red, and has a **Delete** button in each row. Use the **action pattern** from the App State lesson: you can't remove from the ledger while you're looping over its transactions.

```rust,norun
fn transactions_panel(&mut self, ui: &mut egui::Ui) {
    let list = self.ledger.filtered(&Filter::default());

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
    }
}
```

Read it line by line before typing it in. Why is `delete` an `Option<u32>` (an **id**) and not an index into `list`? (Because `list` is filtered and sorted: its indexes don't match positions in `ledger.transactions`. Ids are permanent.)

Try it: add a few transactions and delete some.

## Task 2: filters

Your model already knows how to filter. The GUI just needs to let the user build a `Filter`.

1. Add a field `filter: Filter` to `LedgerApp`, and initialize it with `Filter::default()` in `new`.
2. Use it in `transactions_panel`: `self.ledger.filtered(&self.filter)`.
3. Above the table, draw a row of filter controls:

```rust,norun
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
```

Look at how naturally `Option` fits a dropdown: `None` is "All months", `Some(month)` is a specific month. And `Filter` derives `PartialEq` and `Default`, so "are any filters active?" is simply `self.filter != Filter::default()`.

4. Between the filters and the table, show a summary of what's visible, using the model's `totals` function:

```rust,norun
let (income, expenses) = model::totals(&list);
ui.label(format!("{} transaction(s): {} in, {} out", list.len(), model::format_money(income), model::format_money(expenses)));
```

## Task 3: a friendly empty state

An empty table is a confusing first impression. At the start of `transactions_panel` (after the filters), if `self.ledger.transactions.is_empty()`, show a short centred message instead, and `return`:

```rust,norun
if self.ledger.transactions.is_empty() {
    ui.add_space(30.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("No transactions yet.").size(18.0));
        ui.label("Add your first one with the form on the left.");
    });
    return;
}
```

**Bonus**: add a button under the message that fills the ledger with example data, so new users can explore the app immediately. Write a function `fn add_sample_data(ledger: &mut Ledger)` that adds a salary, rent, groceries and a few other expenses for this month (use `model::today()` and take the month with `&today[..7]`). The next step's files contain one version.

## Check your work

When you're done, your app should let you add, filter, search and delete transactions, with the balance always up to date. Compare with the next lesson's starting `app.rs` if you like. Then notice something: close the app and open it again. **Everything is gone!** Fixing that is the next lesson.

@@project pocket_ledger Pocket Ledger, step 2: the window
Adds `src/app.rs` with the window, the form and a plain list. `Cargo.toml` and `src/main.rs` change too: copy them from the starting files below, or use Reset.
@@file Cargo.toml
[package]
name = "pocket_ledger"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }

@@file src/main.rs
mod app;
mod model;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Pocket Ledger")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Pocket Ledger", options, Box::new(|_cc| Ok(Box::new(app::LedgerApp::new()))))
}

@@file src/app.rs
//! The window: drawing the ledger and reacting to the user.
//! All the rules live in model.rs; this file only shows things and calls methods.

use crate::model::{self, Category, Filter, Kind, Ledger};
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
}

impl LedgerApp {
    pub fn new() -> Self {
        LedgerApp { ledger: Ledger::new(), form: Form::new() }
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
    }

    fn transactions_panel(&mut self, ui: &mut egui::Ui) {
        // A first, plain version of the list. In this lesson you'll turn it into a
        // proper table with Delete buttons, and add filters above it.
        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
            for t in self.ledger.filtered(&Filter::default()) {
                ui.label(format!("{}   {}   {}", t.date, t.description, model::format_money(t.signed_amount())));
            }
        });
    }
}

impl eframe::App for LedgerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(6.0);
            self.top_bar(ui);
            ui.add_space(4.0);
        });
        egui::Panel::left("form").resizable(false).default_size(280.0).show(ui, |ui| {
            ui.add_space(8.0);
            self.form_panel(ui);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            self.transactions_panel(ui);
        });
    }
}

@@file src/model.rs
//! The data model: transactions, categories and the ledger, plus helpers for
//! money and dates. Nothing in here knows about the GUI.

use std::collections::HashMap;

/// Shown in front of every amount. Change it to "€", "£", "zł "...
pub const CURRENCY: &str = "$";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Ledger {
    pub transactions: Vec<Transaction>,
    next_id: u32,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger::new()
    }
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { transactions: Vec::new(), next_id: 1 }
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

@@quiz
The Delete button records `t.id` rather than the row's index in the visible list. Why?
* The visible list is filtered and sorted, so its indexes don't match positions in the ledger; ids are permanent
- Ids are smaller numbers
- egui requires ids
? Always identify data by something stable. Filtering or sorting can't change an id.

@@quiz
Why is the form's amount stored as a `String` in `Form` instead of an `i64`?
- egui text boxes can only edit numbers
* While typing, the text can be incomplete or invalid ("12." or "abc"); it's converted to cents only when submitted
- To save memory
? The text box edits a String buffer. Validation turns it into real data, or an error message.
