@@lesson m5-calculator Project: Interactive Calculator
@@text
Let's build a real interactive program: a **calculator** that reads commands line by line, never crashes on bad input, and explains every problem clearly.

```text
Calculator (type quit to exit)
> 2 + 3
= 5
> mem * 5
= 25
> 10 / 0
error: can't divide by zero
> sqrt mem
= 5
> quit
bye!
```

Features:

- `<number> <op> <number>` with `+`, `-`, `*`, `/`
- `sqrt <number>`
- the word `mem` can be used instead of a number: it means **the last successful result**
- `quit` exits
- every kind of bad input produces a clear error message, and the program keeps going

## Design: separate the logic from the input/output

This is one of the most important design ideas in all of programming:

- **`evaluate(line, memory) -> Result<f64, CalcError>`** does the **logic**: it takes one line of text and computes the answer or an error. It doesn't read input or print anything.
- **`main`** does the **input/output**: it reads lines, calls `evaluate`, prints results, and remembers `mem`.

Why? Because `evaluate` can now be **tested** easily (just call it with strings!), and you could later put the same logic behind a completely different interface — like a GUI with buttons — without changing it. Your final GUI app in Module 9 is built exactly this way.

## The error type

Every way things can go wrong becomes a variant:

| Variant | Message |
|---|---|
| `Empty` | `nothing to calculate` |
| `BadFormat` | `use: <number> <op> <number>, or sqrt <number>` |
| `BadNumber(String)` | `not a number: <token>` |
| `UnknownOperator(String)` | `unknown operator: <op>` |
| `DivideByZero` | `can't divide by zero` |
| `NegativeSqrt` | `can't take the square root of a negative number` |

## Matching on a slice of tokens

Here's a neat trick for parsing commands. Split the line into words, collect them into a `Vec<&str>`, and `match` on `tokens.as_slice()` with **slice patterns**:

```rust,norun
match tokens.as_slice() {
    [] => ...,                  // no words at all
    ["sqrt", x] => ...,         // exactly two words, the first is "sqrt"
    [a, op, b] => ...,          // exactly three words
    _ => ...,                   // anything else
}
```

Each pattern describes the **shape** of the input. This is far clearer than lots of `if tokens.len() == ...` checks.

Inside, `x`, `a`, `op` and `b` are `&&str` (references to the elements). Functions taking `&str` accept them thanks to automatic dereferencing, and to `match` on the operator's text, write `match *op { "+" => ..., }`.

A helper `parse_number(token, memory)` handles both real numbers and the word `mem`, so `evaluate` can use `?` on it.

@@exercise 🏆 Part 1: The calculator's brain
Write the `Display` implementation for `CalcError`, plus `parse_number` and `evaluate`. The checker tests `evaluate` with many inputs. Error messages must match the table above exactly.
@@starter
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadFormat,
    BadNumber(String),
    UnknownOperator(String),
    DivideByZero,
    NegativeSqrt,
}

// impl fmt::Display for CalcError { ... }

/// Turns a token into a number. The word `mem` means the last result.
fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    todo!()
}

fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    todo!()
}

fn main() {
    println!("{:?}", evaluate("2 + 3", 0.0));
    println!("{:?}", evaluate("mem * 2", 21.0));
    println!("{:?}", evaluate("1 / 0", 0.0));
    let _ = io::stdin; // io is used in Part 2
}
@@tests
#[test]
fn arithmetic() {
    assert_eq!(evaluate("2 + 3", 0.0), Ok(5.0));
    assert_eq!(evaluate("10 - 4", 0.0), Ok(6.0));
    assert_eq!(evaluate("6 * 7", 0.0), Ok(42.0));
    assert_eq!(evaluate("9 / 2", 0.0), Ok(4.5));
    assert_eq!(evaluate("  1.5   +   1  ", 0.0), Ok(2.5), "extra spaces are fine");
    assert_eq!(evaluate("sqrt 16", 0.0), Ok(4.0));
}

#[test]
fn memory() {
    assert_eq!(evaluate("mem * 2", 21.0), Ok(42.0));
    assert_eq!(evaluate("sqrt mem", 81.0), Ok(9.0));
}

#[test]
fn errors() {
    assert_eq!(evaluate("", 0.0), Err(CalcError::Empty));
    assert_eq!(evaluate("1 / 0", 0.0), Err(CalcError::DivideByZero));
    assert_eq!(evaluate("sqrt -4", 0.0), Err(CalcError::NegativeSqrt));
    assert_eq!(evaluate("one + 1", 0.0), Err(CalcError::BadNumber(String::from("one"))));
    assert_eq!(evaluate("5 % 2", 0.0), Err(CalcError::UnknownOperator(String::from("%"))));
    assert_eq!(evaluate("hello", 0.0), Err(CalcError::BadFormat));
    assert_eq!(evaluate("1 + 2 + 3", 0.0), Err(CalcError::BadFormat));
}

#[test]
fn messages() {
    assert_eq!(CalcError::Empty.to_string(), "nothing to calculate");
    assert_eq!(CalcError::BadFormat.to_string(), "use: <number> <op> <number>, or sqrt <number>");
    assert_eq!(CalcError::BadNumber(String::from("x")).to_string(), "not a number: x");
    assert_eq!(CalcError::UnknownOperator(String::from("%")).to_string(), "unknown operator: %");
    assert_eq!(CalcError::DivideByZero.to_string(), "can't divide by zero");
    assert_eq!(CalcError::NegativeSqrt.to_string(), "can't take the square root of a negative number");
}
@@hint
`parse_number`: if the token is `"mem"`, return `Ok(memory)`; otherwise `token.parse::<f64>().map_err(|_| CalcError::BadNumber(token.to_string()))`. In `evaluate`, use the slice `match` shown above, and `?` on every `parse_number` call.
@@solution
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadFormat,
    BadNumber(String),
    UnknownOperator(String),
    DivideByZero,
    NegativeSqrt,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "nothing to calculate"),
            CalcError::BadFormat => write!(f, "use: <number> <op> <number>, or sqrt <number>"),
            CalcError::BadNumber(t) => write!(f, "not a number: {t}"),
            CalcError::UnknownOperator(op) => write!(f, "unknown operator: {op}"),
            CalcError::DivideByZero => write!(f, "can't divide by zero"),
            CalcError::NegativeSqrt => write!(f, "can't take the square root of a negative number"),
        }
    }
}

/// Turns a token into a number. The word `mem` means the last result.
fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    if token == "mem" {
        return Ok(memory);
    }
    token.parse::<f64>().map_err(|_| CalcError::BadNumber(token.to_string()))
}

fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.as_slice() {
        [] => Err(CalcError::Empty),
        ["sqrt", x] => {
            let n = parse_number(x, memory)?;
            if n < 0.0 {
                return Err(CalcError::NegativeSqrt);
            }
            Ok(n.sqrt())
        }
        [a, op, b] => {
            let x = parse_number(a, memory)?;
            let y = parse_number(b, memory)?;
            match *op {
                "+" => Ok(x + y),
                "-" => Ok(x - y),
                "*" => Ok(x * y),
                "/" => {
                    if y == 0.0 {
                        Err(CalcError::DivideByZero)
                    } else {
                        Ok(x / y)
                    }
                }
                other => Err(CalcError::UnknownOperator(other.to_string())),
            }
        }
        _ => Err(CalcError::BadFormat),
    }
}

fn main() {
    println!("{:?}", evaluate("2 + 3", 0.0));
    println!("{:?}", evaluate("mem * 2", 21.0));
    println!("{:?}", evaluate("1 / 0", 0.0));
    let _ = io::stdin; // io is used in Part 2
}

@@exercise 🏆 Part 2: The interactive program
Now write `main`: the input/output part. The starter contains a finished `evaluate` from Part 1.

`main` should:

1. print `Calculator (type quit to exit)`,
2. read lines until the input ends, **skipping empty lines**,
3. echo each line as `> line` (so you can see the input in the output),
4. stop on `quit`, after echoing it,
5. print `= result` or `error: message`, and update the memory **only on success**,
6. finally print `bye!`.

The input box contains a test session. Expected output:

```text
Calculator (type quit to exit)
> 2 + 3
= 5
> mem * 5
= 25
> 10 / 0
error: can't divide by zero
> sqrt mem
= 5
> banana + 1
error: not a number: banana
> 5 % 2
error: unknown operator: %
> hello
error: use: <number> <op> <number>, or sqrt <number>
> quit
bye!
```
@@starter
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadFormat,
    BadNumber(String),
    UnknownOperator(String),
    DivideByZero,
    NegativeSqrt,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "nothing to calculate"),
            CalcError::BadFormat => write!(f, "use: <number> <op> <number>, or sqrt <number>"),
            CalcError::BadNumber(t) => write!(f, "not a number: {t}"),
            CalcError::UnknownOperator(op) => write!(f, "unknown operator: {op}"),
            CalcError::DivideByZero => write!(f, "can't divide by zero"),
            CalcError::NegativeSqrt => write!(f, "can't take the square root of a negative number"),
        }
    }
}

/// Turns a token into a number. The word `mem` means the last result.
fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    if token == "mem" {
        return Ok(memory);
    }
    token.parse::<f64>().map_err(|_| CalcError::BadNumber(token.to_string()))
}

fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.as_slice() {
        [] => Err(CalcError::Empty),
        ["sqrt", x] => {
            let n = parse_number(x, memory)?;
            if n < 0.0 {
                return Err(CalcError::NegativeSqrt);
            }
            Ok(n.sqrt())
        }
        [a, op, b] => {
            let x = parse_number(a, memory)?;
            let y = parse_number(b, memory)?;
            match *op {
                "+" => Ok(x + y),
                "-" => Ok(x - y),
                "*" => Ok(x * y),
                "/" => {
                    if y == 0.0 {
                        Err(CalcError::DivideByZero)
                    } else {
                        Ok(x / y)
                    }
                }
                other => Err(CalcError::UnknownOperator(other.to_string())),
            }
        }
        _ => Err(CalcError::BadFormat),
    }
}

fn main() {
    println!("Calculator (type quit to exit)");
}
@@stdin
2 + 3
mem * 5
10 / 0
sqrt mem

banana + 1
5 % 2
hello
quit
1 + 1
@@expected
Calculator (type quit to exit)
> 2 + 3
= 5
> mem * 5
= 25
> 10 / 0
error: can't divide by zero
> sqrt mem
= 5
> banana + 1
error: not a number: banana
> 5 % 2
error: unknown operator: %
> hello
error: use: <number> <op> <number>, or sqrt <number>
> quit
bye!
@@requires
for line in
@@hint
`let mut memory = 0.0;` then `for line in io::stdin().lines()`. Inside: `let line = line.expect("...");`, trim it, `continue` if empty, print the echo, `break` on quit, then `match evaluate(line, memory) { Ok(r) => { println!("= {r}"); memory = r; } Err(e) => println!("error: {e}") }`.
@@solution
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
enum CalcError {
    Empty,
    BadFormat,
    BadNumber(String),
    UnknownOperator(String),
    DivideByZero,
    NegativeSqrt,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Empty => write!(f, "nothing to calculate"),
            CalcError::BadFormat => write!(f, "use: <number> <op> <number>, or sqrt <number>"),
            CalcError::BadNumber(t) => write!(f, "not a number: {t}"),
            CalcError::UnknownOperator(op) => write!(f, "unknown operator: {op}"),
            CalcError::DivideByZero => write!(f, "can't divide by zero"),
            CalcError::NegativeSqrt => write!(f, "can't take the square root of a negative number"),
        }
    }
}

/// Turns a token into a number. The word `mem` means the last result.
fn parse_number(token: &str, memory: f64) -> Result<f64, CalcError> {
    if token == "mem" {
        return Ok(memory);
    }
    token.parse::<f64>().map_err(|_| CalcError::BadNumber(token.to_string()))
}

fn evaluate(line: &str, memory: f64) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens.as_slice() {
        [] => Err(CalcError::Empty),
        ["sqrt", x] => {
            let n = parse_number(x, memory)?;
            if n < 0.0 {
                return Err(CalcError::NegativeSqrt);
            }
            Ok(n.sqrt())
        }
        [a, op, b] => {
            let x = parse_number(a, memory)?;
            let y = parse_number(b, memory)?;
            match *op {
                "+" => Ok(x + y),
                "-" => Ok(x - y),
                "*" => Ok(x * y),
                "/" => {
                    if y == 0.0 {
                        Err(CalcError::DivideByZero)
                    } else {
                        Ok(x / y)
                    }
                }
                other => Err(CalcError::UnknownOperator(other.to_string())),
            }
        }
        _ => Err(CalcError::BadFormat),
    }
}

fn main() {
    println!("Calculator (type quit to exit)");
    let mut memory = 0.0;
    for line in io::stdin().lines() {
        let line = line.expect("failed to read input");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("> {line}");
        if line == "quit" {
            break;
        }
        match evaluate(line, memory) {
            Ok(result) => {
                println!("= {result}");
                memory = result;
            }
            Err(e) => println!("error: {e}"),
        }
    }
    println!("bye!");
}

@@text
## Try it for real

Copy your finished program into the **Playground**, write your own calculations into the input box, and run it. In the next module you'll learn to create Cargo projects, so you can run programs like this one interactively in a real terminal.

## 🎉 Module 5 complete!

Your programs can now **talk to users** and **survive bad input**. You know when to panic (bugs) and when to return `Result` (expected problems), how to pass errors up with `?`, and how to design clear error types. And you've learned to separate **logic** from **input/output**, which matters for every program you'll ever write.

**Next up:** leaving the single-file world. Real projects with Cargo, multiple files, external libraries, and tests, all in VS Code.
