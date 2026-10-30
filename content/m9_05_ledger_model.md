@@lesson m9-ledger1 Pocket Ledger 1: The Model
@@text
Welcome to the capstone project! Over the next four lessons you'll build **Pocket Ledger**, a personal budget app that you can genuinely use to track your money:

- record **income and expenses** with a date, description, amount and category;
- see your **balance**, and income/expenses **per month**;
- **search and filter** transactions;
- a **bar chart** of where your money goes;
- a **monthly budget** with a progress bar that turns red when you overspend;
- everything **saved automatically** to a file, so it's there next time;
- a single, portable **`.exe`** you can copy anywhere.

It uses almost every concept from the course: structs, enums, `Option`, `Result`, `match`, `Vec`, `HashMap`, ownership and borrowing, traits and derive, closures, iterators, modules, tests, crates, files, JSON, and a GUI.

We'll build it the professional way: **model first, with tests**. No window yet in this lesson.

## Design decisions

**Money is stored as whole cents in an `i64`, never as `f64`.** Floating-point numbers can't represent most decimal fractions exactly:

```rust
fn main() {
    let total = 0.1 + 0.2;
    println!("{total}");               // not quite 0.3!
    println!("{}", total == 0.3);

    let cents = 10 + 20;               // in cents: exact
    println!("{}", cents == 30);
}
```

Tiny errors like this add up in a list of transactions and make totals wrong by a cent. So `$12.50` is stored as `1250`. We only convert to a "12.50" text when showing it, and from text when the user types an amount. Real banking software does the same.

**Amounts are always positive**, and a `Kind` enum (`Income` or `Expense`) says which direction the money went. `signed_amount()` gives the `+` or `−` version for adding up.

**Dates are strings in `YYYY-MM-DD` format**, like `"2026-10-30"`. This format has a great property: sorting the strings alphabetically sorts the dates chronologically, and the first 7 characters are the month (`"2026-10"`). That's all we need, so we don't need a date crate. `is_valid_date` checks the format and that the day exists, and `today()` computes today's date from the system clock (both are written for you).

**Categories are an enum** with a constant listing all of them, `Category::ALL`, so the GUI can loop over them to fill a dropdown menu. `label()` gives each one a display name.

**A `Filter` struct describes what to show**: an optional month, an optional kind, and search text. `#[derive(Default)]` gives the "show everything" filter for free, and struct update syntax makes custom filters short:

```rust,norun
let october = Filter { month: Some(String::from("2026-10")), ..Filter::default() };
```

## The `Ledger`

```rust,norun
pub struct Ledger {
    pub transactions: Vec<Transaction>,
    next_id: u32,
}
```

Each transaction gets a permanent id (like the to-do app), so the GUI can say "delete transaction 17" even when the list is filtered or sorted differently.

## Your tasks

Create the project and run `cargo test`. Eight tests fail. Open `src/model.rs` and implement the `todo!()`s, one at a time, running `cargo test` after each. Suggested order (easiest first):

1. **`balance`** — the sum of every transaction's `signed_amount()`. One iterator chain.
2. **`remove`** — remove the transaction with that id, and return whether something was removed. (Compare the length before and after `retain`, or use `position`.)
3. **`format_money`** — `1250` → `"$12.50"`, `123456` → `"$1,234.56"`, `-305` → `"-$3.05"`. Work with the absolute value (`cents.unsigned_abs()`), split into whole units and cents with `/` and `%`, and pad the cents with `{:02}`. The thousands separators are the tricky part: build the string digit by digit and insert a `,` whenever the number of digits still to come is a multiple of 3.
4. **`Filter::matches`** — return `false` as soon as one condition fails: wrong month (if a month is set), wrong kind (if a kind is set). Then the search: an empty (or all-spaces) search matches everything; otherwise the lowercase search text must appear in the lowercase description **or** category label.
5. **`spending_by_category`** — only expenses, only in the given month if `Some`. Add up amounts per category in a `HashMap<Category, i64>` (`*map.entry(key).or_insert(0) += amount`), then turn it into a `Vec`, and sort it biggest amount first.
6. **`parse_amount`** — the user typed this, so expect anything! Read the tests to see what must be accepted (`"12"`, `"12.5"`, `"0,99"` with a comma, `".5"`) and rejected (empty, letters, negative, three decimals, zero, absurdly large). One approach: trim, replace `,` with `.`, `split_once('.')` into a whole part and a fraction part, check that both contain only digits, and that the fraction has at most two. Remember that `"4.5"` means 450 cents, not 405! Use `checked_mul` and `checked_add` to catch overflow.

When all tests pass, `cargo run` prints a small text report using your model.

> **Stuck on one?** The next lesson's starting files contain a complete `model.rs`. Peek at just the function you're stuck on, then close it and write your own.

@@project pocket_ledger Pocket Ledger, step 1: the model
The data model for the budget app, with tests. Implement the `todo!()`s in `src/model.rs` until `cargo test` passes.
@@file Cargo.toml
[package]
name = "pocket_ledger"
version = "0.1.0"
edition = "2024"

[dependencies]

@@file src/main.rs
mod model;

use model::{Category, Filter, Kind, Ledger};

// A quick text-mode test drive of the model. In the next lessons, a window replaces this.
fn main() {
    let mut ledger = Ledger::new();
    let amount = |text: &str| model::parse_amount(text).expect("valid amount");

    ledger.add("2026-10-01", "Salary", amount("3200"), Kind::Income, Category::Salary);
    ledger.add("2026-10-01", "Rent", amount("1100"), Kind::Expense, Category::Housing);
    ledger.add("2026-10-02", "Supermarket", amount("86.40"), Kind::Expense, Category::Food);
    ledger.add("2026-10-05", "Pizza with friends", amount("23,50"), Kind::Expense, Category::Fun);
    ledger.add("2026-10-07", "Bakery", amount("4.5"), Kind::Expense, Category::Food);

    println!("Today is {}\n", model::today());
    for t in ledger.filtered(&Filter::default()) {
        println!("{}  {:<20} {:>12}", t.date, t.description, model::format_money(t.signed_amount()));
    }
    println!("\nBalance: {}", model::format_money(ledger.balance()));

    println!("\nSpending by category:");
    for (category, total) in ledger.spending_by_category(None) {
        println!("  {:<10} {:>10}", category.label(), model::format_money(total));
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
        todo!()
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
        todo!()
    }

    /// All income minus all expenses.
    pub fn balance(&self) -> i64 {
        todo!()
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
        todo!()
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
    todo!()
}

/// Formats cents for display: 123456 -> "$1,234.56", -305 -> "-$3.05".
pub fn format_money(cents: i64) -> String {
    todo!()
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
Why does Pocket Ledger store money as an `i64` number of cents instead of an `f64`?
- `i64` is faster
* `f64` can't represent most decimal amounts exactly, so totals could be off by a cent
- `f64` can't be negative
? Floating-point rounding errors are unacceptable for money. Integer cents are exact.

@@quiz
Why are dates stored as `"YYYY-MM-DD"` strings?
* Alphabetical order equals date order, and the first 7 characters give the month
- Rust has no way to store dates
- It uses less memory
? This standard format (ISO 8601) is easy to sort and group. For complex date maths you'd use a crate like `chrono`.
