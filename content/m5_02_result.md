@@lesson m5-result Result: Errors You Can Handle
@@text
`Result` is how Rust handles **recoverable** errors. Like `Option`, it's just an enum:

```rust,norun
enum Result<T, E> {
    Ok(T),    // success, with a value of type T
    Err(E),   // failure, with an error of type E
}
```

`Option` says "a value or nothing". `Result` says "a value **or an explanation of what went wrong**".

## Results you've already met

`parse` returns a `Result`, because parsing can fail:

```rust
fn main() {
    let good: Result<i32, _> = "42".parse::<i32>();
    let bad: Result<i32, _> = "forty-two".parse::<i32>();
    println!("{:?}", good);
    println!("{:?}", bad);
}
```

## Handling a Result with `match`

```rust
fn main() {
    let inputs = ["12", "7x", "-3"];
    for text in inputs {
        match text.parse::<i32>() {
            Ok(n) => println!("{text} → the number {n}"),
            Err(e) => println!("{text} → error: {e}"),
        }
    }
}
```

The error value (`e`) explains what went wrong — here, `invalid digit found in string`. You can show it to the user, log it, or decide what to do next.

## Returning your own Results

Functions that can fail should return `Result`. The simplest error type to start with is a `String` message:

```rust
fn withdraw(balance: u32, amount: u32) -> Result<u32, String> {
    if amount == 0 {
        return Err(String::from("amount must be positive"));
    }
    if amount > balance {
        return Err(format!("insufficient funds: balance {balance}, wanted {amount}"));
    }
    Ok(balance - amount)
}

fn main() {
    for amount in [30, 0, 500] {
        match withdraw(100, amount) {
            Ok(left) => println!("withdrew {amount}, {left} left"),
            Err(msg) => println!("could not withdraw {amount}: {msg}"),
        }
    }
}
```

The signature `-> Result<u32, String>` is honest documentation: "this gives you a `u32`, **or** an error message — you must deal with both."

The caller **cannot forget** the error case. If you ignore a `Result` completely, the compiler warns you (`unused Result that must be used`).

## Handy Result methods

Many are the same as `Option`'s:

```rust
fn main() {
    let ok: Result<i32, String> = Ok(5);
    let err: Result<i32, String> = Err(String::from("nope"));

    println!("{}", err.clone().unwrap_or(0));          // default on error
    println!("{} {}", ok.is_ok(), err.is_err());
    println!("{:?}", ok.clone().map(|n| n * 2));        // Ok(10)
    println!("{:?}", err.clone().map_err(|e| e.len())); // Err(4): change the error
    println!("{:?}", ok.ok());                          // Result → Option: Some(5)
    println!("{:?}", err.ok());                         // None
}
```

And `unwrap()` / `expect("...")` take the value out or panic — fine in tests and quick experiments.

## Option vs Result

| Use | When |
|---|---|
| `Option<T>` | a value might be absent, and there's nothing more to say (find, first, get) |
| `Result<T, E>` | an operation might fail, and the caller should know **why** (parse, open file, validate) |

Convert between them: `option.ok_or("message")` turns `None` into `Err("message")`; `result.ok()` turns `Err` into `None`.

@@quiz
What does `"7".parse::<u8>()` return?
- `7`
* `Ok(7)`
- `Some(7)`
? `parse` returns a `Result`: `Ok(value)` on success.

@@quiz
When should a function return `Result` instead of `Option`?
- Always
* When the caller needs to know *why* it failed
- When it never fails
? `Result` carries an error value explaining the failure; `Option` just says "nothing".

@@exercise Validate a username
Write `validate_username`, which returns `Ok(())` if the username is valid, or an `Err` with the **exact** message for the first rule that's broken:

1. empty → `"username can't be empty"`
2. longer than 12 characters → `"username is too long"`
3. contains anything other than letters, digits or `_` → `"username can only contain letters, digits and _"`

`Ok(())` means "success, but there's no value to return" — `()` is the unit value.
@@starter
fn validate_username(name: &str) -> Result<(), String> {
    todo!()
}

fn main() {
    for name in ["ferris_42", "", "this_name_is_way_too_long", "bad name!"] {
        println!("{:?} → {:?}", name, validate_username(name));
    }
}
@@tests
#[test]
fn checks() {
    assert_eq!(validate_username("ferris_42"), Ok(()));
    assert_eq!(validate_username(""), Err(String::from("username can't be empty")));
    assert_eq!(validate_username("abcdefghijklm"), Err(String::from("username is too long")));
    assert_eq!(validate_username("abcdefghijkl"), Ok(()), "exactly 12 is fine");
    assert_eq!(validate_username("bad name!"), Err(String::from("username can only contain letters, digits and _")));
}
@@hint
Check each rule with an early `return Err(String::from("..."));`. For rule 3, loop over `name.chars()` and check `!(c.is_alphanumeric() || c == '_')`. At the end, `Ok(())`.
@@solution
fn validate_username(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err(String::from("username can't be empty"));
    }
    if name.chars().count() > 12 {
        return Err(String::from("username is too long"));
    }
    for c in name.chars() {
        if !(c.is_alphanumeric() || c == '_') {
            return Err(String::from("username can only contain letters, digits and _"));
        }
    }
    Ok(())
}

fn main() {
    for name in ["ferris_42", "", "this_name_is_way_too_long", "bad name!"] {
        println!("{:?} → {:?}", name, validate_username(name));
    }
}

@@exercise Parse a temperature
Write `parse_temp`, which reads text like `"21.5C"` or `"70F"` and returns the temperature **in Celsius**. Return an `Err` with a helpful message if:

- the text doesn't end in `C` or `F` → `"missing unit (C or F)"`
- the number part isn't a valid number → `"not a number: <the number part>"`

For F, convert: `C = (F - 32) × 5 / 9`.
@@starter
fn parse_temp(text: &str) -> Result<f64, String> {
    todo!()
}

fn main() {
    for t in ["21.5C", "212F", "hot", "abcC"] {
        println!("{t} → {:?}", parse_temp(t));
    }
}
@@tests
#[test]
fn checks() {
    assert_eq!(parse_temp("21.5C"), Ok(21.5));
    assert_eq!(parse_temp("212F"), Ok(100.0));
    assert_eq!(parse_temp("-40F"), Ok(-40.0));
    assert_eq!(parse_temp("hot"), Err(String::from("missing unit (C or F)")));
    assert_eq!(parse_temp("abcC"), Err(String::from("not a number: abc")));
    assert_eq!(parse_temp(""), Err(String::from("missing unit (C or F)")));
}
@@hint
`text.strip_suffix('C')` returns `Some(rest)` if the text ends with C. Try that, then `F`, else return the unit error. Then parse the rest: `match number.parse::<f64>() { Ok(n) => ..., Err(_) => Err(format!("not a number: {number}")) }`.
@@solution
fn parse_temp(text: &str) -> Result<f64, String> {
    let (number, is_fahrenheit) = if let Some(rest) = text.strip_suffix('C') {
        (rest, false)
    } else if let Some(rest) = text.strip_suffix('F') {
        (rest, true)
    } else {
        return Err(String::from("missing unit (C or F)"));
    };

    let value = match number.parse::<f64>() {
        Ok(n) => n,
        Err(_) => return Err(format!("not a number: {number}")),
    };

    if is_fahrenheit { Ok((value - 32.0) * 5.0 / 9.0) } else { Ok(value) }
}

fn main() {
    for t in ["21.5C", "212F", "hot", "abcC"] {
        println!("{t} → {:?}", parse_temp(t));
    }
}
