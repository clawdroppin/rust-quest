@@lesson m5-custom-errors Your Own Error Types
@@text
`String` errors are easy, but they have a weakness: the caller can only *read* them. What if the caller wants to react differently to different problems — retry on a network timeout, but show a message for bad input? Matching on message text is fragile. The Rust way is an **error enum**.

## An error enum

```rust
#[derive(Debug, PartialEq)]
enum WithdrawError {
    ZeroAmount,
    InsufficientFunds { balance: u32, needed: u32 },
    AccountFrozen,
}

fn withdraw(balance: u32, amount: u32, frozen: bool) -> Result<u32, WithdrawError> {
    if frozen {
        return Err(WithdrawError::AccountFrozen);
    }
    if amount == 0 {
        return Err(WithdrawError::ZeroAmount);
    }
    if amount > balance {
        return Err(WithdrawError::InsufficientFunds { balance, needed: amount });
    }
    Ok(balance - amount)
}

fn main() {
    match withdraw(50, 80, false) {
        Ok(left) => println!("{left} left"),
        Err(WithdrawError::InsufficientFunds { balance, needed }) => {
            println!("You need {} more", needed - balance)
        }
        Err(other) => println!("Failed: {:?}", other),
    }
}
```

Now each kind of failure is a **variant** — the compiler knows all possible errors, the caller can `match` on them, and variants can carry useful details.

## Displaying errors nicely: `impl Display`

`{:?}` prints `InsufficientFunds { balance: 50, needed: 80 }` — fine for programmers, ugly for users. To print an error with `{}`, implement the **`Display` trait**. Traits are explained in Module 7; for now, treat this as a recipe to copy:

```rust
use std::fmt;

#[derive(Debug)]
enum WithdrawError {
    ZeroAmount,
    InsufficientFunds { balance: u32, needed: u32 },
}

impl fmt::Display for WithdrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WithdrawError::ZeroAmount => write!(f, "the amount must be more than zero"),
            WithdrawError::InsufficientFunds { balance, needed } => {
                write!(f, "not enough money: you have {balance} but need {needed}")
            }
        }
    }
}

fn main() {
    let e = WithdrawError::InsufficientFunds { balance: 50, needed: 80 };
    println!("Error: {e}");
}
```

- `impl fmt::Display for WithdrawError` says "here's how to display this type".
- The `fmt` function receives the value (`&self`) and a formatter `f`.
- `write!(f, ...)` works like `format!`, but writes into the formatter.

This works for **any** type, not just errors. Implement `Display` on a struct and you can print it with `{}`.

## Converting errors for `?`

Say your function parses numbers (which gives a `ParseIntError`) but returns your own error type. `?` can convert automatically if you tell it how, by implementing `From`:

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    BadNumber(ParseIntError),
    OutOfRange(i64),
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

fn parse_volume(text: &str) -> Result<u8, ConfigError> {
    let n: i64 = text.trim().parse()?;        // ParseIntError → ConfigError automatically
    if !(0..=100).contains(&n) {
        return Err(ConfigError::OutOfRange(n));
    }
    Ok(n as u8)
}

fn main() {
    println!("{:?}", parse_volume("75"));
    println!("{:?}", parse_volume("loud"));
    println!("{:?}", parse_volume("250"));
}
```

## Choosing an error type

| Situation | Good choice |
|---|---|
| Quick programs, `main`, experiments | `Box<dyn std::error::Error>` or `String` |
| A function whose caller just shows the message | `String` |
| Code where callers need to tell errors apart | your own `enum` with `Display` |

In bigger projects people often use helper crates (`thiserror`, `anyhow`) that generate this code. Now you know what they do under the hood.

@@quiz
What's the main advantage of an error enum over a `String` error?
- It's shorter to write
* Callers can `match` on the specific kind of error and react differently
- It can't fail
? Variants make each failure a distinct, checkable case.

@@exercise Password checker
Define an enum `PasswordError` with variants `TooShort(usize)` (holding the actual length), `NoDigit` and `NoUppercase`, deriving `Debug` and `PartialEq`. Implement `Display` for it with these messages:

- `TooShort(n)` → `"too short: 5 characters, need at least 8"` (with the real number)
- `NoDigit` → `"must contain a digit"`
- `NoUppercase` → `"must contain an uppercase letter"`

Then write `check_password`, which checks the rules in that order and returns the first problem.
@@starter
use std::fmt;

// define PasswordError and impl fmt::Display for it

fn check_password(pw: &str) -> Result<(), PasswordError> {
    todo!()
}

fn main() {
    for pw in ["hunter2", "longpassword", "longpassword1", "LongPassword1"] {
        match check_password(pw) {
            Ok(()) => println!("{pw}: ok"),
            Err(e) => println!("{pw}: {e}"),
        }
    }
}
@@tests
#[test]
fn checks() {
    assert_eq!(check_password("hunter2"), Err(PasswordError::TooShort(7)));
    assert_eq!(check_password("longpassword"), Err(PasswordError::NoDigit));
    assert_eq!(check_password("longpassword1"), Err(PasswordError::NoUppercase));
    assert_eq!(check_password("LongPassword1"), Ok(()));
    assert_eq!(PasswordError::TooShort(5).to_string(), "too short: 5 characters, need at least 8");
    assert_eq!(PasswordError::NoDigit.to_string(), "must contain a digit");
    assert_eq!(PasswordError::NoUppercase.to_string(), "must contain an uppercase letter");
}
@@hint
Rules: `pw.chars().count() < 8`, `!pw.chars().any(|c| c.is_ascii_digit())`, `!pw.chars().any(|c| c.is_uppercase())`. `.to_string()` works automatically once `Display` is implemented.
@@solution
use std::fmt;

#[derive(Debug, PartialEq)]
enum PasswordError {
    TooShort(usize),
    NoDigit,
    NoUppercase,
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PasswordError::TooShort(n) => write!(f, "too short: {n} characters, need at least 8"),
            PasswordError::NoDigit => write!(f, "must contain a digit"),
            PasswordError::NoUppercase => write!(f, "must contain an uppercase letter"),
        }
    }
}

fn check_password(pw: &str) -> Result<(), PasswordError> {
    let len = pw.chars().count();
    if len < 8 {
        return Err(PasswordError::TooShort(len));
    }
    if !pw.chars().any(|c| c.is_ascii_digit()) {
        return Err(PasswordError::NoDigit);
    }
    if !pw.chars().any(|c| c.is_uppercase()) {
        return Err(PasswordError::NoUppercase);
    }
    Ok(())
}

fn main() {
    for pw in ["hunter2", "longpassword", "longpassword1", "LongPassword1"] {
        match check_password(pw) {
            Ok(()) => println!("{pw}: ok"),
            Err(e) => println!("{pw}: {e}"),
        }
    }
}

@@exercise Display a struct
`Display` isn't only for errors. Implement `fmt::Display` for `Money`, which stores an amount in **cents**, so that it prints like `$12.05` (always two digits of cents). Negative amounts print as `-$3.50`.
@@starter
use std::fmt;

struct Money {
    cents: i64,
}

// impl fmt::Display for Money

fn main() {
    println!("{}", Money { cents: 1205 });
    println!("{}", Money { cents: -350 });
}
@@tests
#[test]
fn checks() {
    assert_eq!(Money { cents: 1205 }.to_string(), "$12.05");
    assert_eq!(Money { cents: 5 }.to_string(), "$0.05");
    assert_eq!(Money { cents: -350 }.to_string(), "-$3.50");
    assert_eq!(Money { cents: 0 }.to_string(), "$0.00");
}
@@hint
Work with the absolute value: `let abs = self.cents.abs();`, then dollars `abs / 100` and cents `abs % 100`. Print the sign separately: `let sign = if self.cents < 0 { "-" } else { "" };` and `write!(f, "{sign}${}.{:02}", abs / 100, abs % 100)`.
@@solution
use std::fmt;

struct Money {
    cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.abs();
        write!(f, "{sign}${}.{:02}", abs / 100, abs % 100)
    }
}

fn main() {
    println!("{}", Money { cents: 1205 });
    println!("{}", Money { cents: -350 });
}
