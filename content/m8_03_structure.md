@@lesson m8-structure Structuring a Program
@@text
You now know enough Rust to write real programs. The hard part is no longer *syntax* but *organization*: how to arrange code so it stays understandable as it grows from 50 to 5,000 lines. This lesson collects the patterns that work.

## Layers: model, logic, interface, storage

Almost every application has the same parts. Keep them apart:

| Layer | Responsibility | Example in a to-do app |
|---|---|---|
| **Model** | the data types | `Task`, `TaskList` |
| **Logic** | the rules, as methods on the model | add, complete, remove, "can't complete twice" |
| **Interface** | talking to the user | parse typed commands, print the list; or a GUI |
| **Storage** | saving and loading | read/write `todo.json` |

The golden rule: **the model and logic never print or read input**. They take values and return values (or `Result`s). That gives you:

- **testability** — every rule can be checked with `cargo test`;
- **replaceable interfaces** — the same model can power a command-line version *and* a GUI;
- **clarity** — when there's a bug in how the list is printed, you know where to look.

## Commands as an enum

User input is messy text. Convert it as early as possible into a **clean type**, then work with that. An enum is perfect for "one of several commands":

```rust,norun
enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
    Help,
    Quit,
}

fn parse_command(line: &str) -> Result<Command, String> { ... }
```

Then the main loop becomes a short, readable `match`, and the compiler checks that every command is handled. Add a new command to the enum and the compiler points at every place that needs updating.

## The main loop

Interactive programs share one shape:

```rust,norun
fn main() {
    let mut state = load_or_default();        // 1. set up
    loop {
        show(&state);                          // 2. show the current state
        let input = read_line();               // 3. wait for input
        match parse_command(&input) {          // 4. turn it into a command
            Ok(Command::Quit) => break,
            Ok(cmd) => {
                let message = execute(&mut state, cmd);   // 5. update the state
                println!("{message}");
                save(&state);
            }
            Err(e) => println!("{e}"),
        }
    }
}
```

**Show → input → update → repeat.** Keep this shape in mind. A GUI works the same way, except the "loop" runs 60 times per second and the input is mouse clicks.

## Command-line arguments

Programs can also receive input when they start: `todo add Buy milk`. Those words are **arguments**, available from `std::env::args()`:

```rust,norun
let args: Vec<String> = std::env::args().collect();
// args[0] is the program's own path; the real arguments start at index 1
```

With Cargo, everything after `--` is passed to your program: `cargo run -- add Buy milk`.

## Errors at the top

Lower layers return `Result`s with useful messages. The top level (`main`) decides how to show them. `main` itself can return a `Result`, so `?` works there too:

```rust,norun
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("config.txt")?;
    let port: u16 = text.trim().parse()?;
    println!("port {port}");
    Ok(())
}
```

`Box<dyn std::error::Error>` means "any kind of error" (it's a trait object!), so `?` can pass up I/O errors, parse errors, and others in the same function. If `main` returns an `Err`, the program prints it and exits with an error code. Handy for small tools; for user-facing programs, print a friendly message yourself.

## Constants and configuration

Put "magic values" in named constants at the top of a module: `const SAVE_FILE: &str = "todo.json";`, `const MAX_LIVES: u32 = 6;`. Changing them later is a one-line edit, and the name explains the meaning.

## Growing a program safely

1. Start with the **model** and its tests.
2. Add the simplest **interface** that works.
3. Add **storage**.
4. Then add features one by one, running the tests after each.

Many small, working steps beat one big leap. Commit to git after each step, so you can always go back (git is worth learning soon. VS Code has it built in: the "Source Control" tab).

@@quiz
Why shouldn't the model layer print anything?
- Printing is slow
* So it can be tested automatically and reused with a different interface (like a GUI)
- Rust doesn't allow printing in structs
? Logic that only takes and returns values can be tested and reused anywhere.

@@quiz
What's the advantage of parsing input into an enum like `Command`?
* The rest of the program works with clean, checked values, and the compiler ensures every command is handled
- Enums use less memory than strings
- It makes typing faster
? Convert messy input to a precise type once, at the edge of your program.

@@exercise Parse the commands
Write `parse_command(line: &str) -> Result<Command, String>` for a to-do app. You'll reuse this exact function in the project at the end of this module.

| Input | Result |
|---|---|
| `add Buy milk` | `Ok(Command::Add("Buy milk"))` — everything after `add`, trimmed |
| `add` (no text) | `Err` — "what should I add?" |
| `done 3` | `Ok(Command::Done(3))` |
| `remove 2` (or `rm 2`) | `Ok(Command::Remove(2))` |
| `done` / `done abc` | `Err` — a number is needed |
| `list` (or `ls`) | `Ok(Command::List)` |
| `clear` | `Ok(Command::Clear)` — remove finished tasks |
| `help` | `Ok(Command::Help)` |
| `quit` (or `exit`) | `Ok(Command::Quit)` |
| anything else | `Err` — "unknown command" |

Command words are **case-insensitive** (`ADD milk` works) and extra spaces are ignored. Empty input is an error too. The exact error messages are up to you.
@@starter
#[derive(Debug, PartialEq)]
enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
    Clear,
    Help,
    Quit,
}

fn parse_command(line: &str) -> Result<Command, String> {
    todo!()
}

fn main() {
    for input in ["add Buy milk", "  DONE 2 ", "rm 1", "ls", "done abc", "add", "fly away", ""] {
        println!("{:<14} => {:?}", format!("{input:?}"), parse_command(input).is_ok());
    }
    println!("{:?}", parse_command("add   Learn   Rust  "));
}
@@expected
"add Buy milk" => true
"  DONE 2 "    => true
"rm 1"         => true
"ls"           => true
"done abc"     => false
"add"          => false
"fly away"     => false
""             => false
Ok(Add("Learn   Rust"))
@@tests
#[test]
fn checks() {
    assert_eq!(parse_command("add Buy milk"), Ok(Command::Add(String::from("Buy milk"))));
    assert_eq!(parse_command("Add  Call mum "), Ok(Command::Add(String::from("Call mum"))));
    assert!(parse_command("add").is_err());
    assert!(parse_command("add    ").is_err());
    assert_eq!(parse_command("done 3"), Ok(Command::Done(3)));
    assert_eq!(parse_command("DONE 3"), Ok(Command::Done(3)));
    assert_eq!(parse_command("remove 2"), Ok(Command::Remove(2)));
    assert_eq!(parse_command("rm 2"), Ok(Command::Remove(2)));
    assert!(parse_command("done").is_err());
    assert!(parse_command("remove x").is_err());
    assert!(parse_command("done -1").is_err());
    assert_eq!(parse_command("list"), Ok(Command::List));
    assert_eq!(parse_command(" ls "), Ok(Command::List));
    assert_eq!(parse_command("clear"), Ok(Command::Clear));
    assert_eq!(parse_command("help"), Ok(Command::Help));
    assert_eq!(parse_command("quit"), Ok(Command::Quit));
    assert_eq!(parse_command("EXIT"), Ok(Command::Quit));
    assert!(parse_command("").is_err());
    assert!(parse_command("dance").is_err());
}
@@hint
Trim the line, then split it into the command word and the rest with `line.split_once(' ')`:

```
let line = line.trim();
let (word, rest) = match line.split_once(' ') {
    Some((w, r)) => (w, r.trim()),
    None => (line, ""),
};
match word.to_lowercase().as_str() {
    "add" => ...,
    "done" => ...,
    ...
}
```

For the numbers, a small helper `fn parse_id(text: &str) -> Result<u32, String>` avoids repeating yourself.
@@solution
#[derive(Debug, PartialEq)]
enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
    Clear,
    Help,
    Quit,
}

fn parse_id(text: &str) -> Result<u32, String> {
    text.parse().map_err(|_| format!("'{text}' is not a task number"))
}

fn parse_command(line: &str) -> Result<Command, String> {
    let line = line.trim();
    let (word, rest) = match line.split_once(' ') {
        Some((w, r)) => (w, r.trim()),
        None => (line, ""),
    };
    match word.to_lowercase().as_str() {
        "add" => {
            if rest.is_empty() {
                Err(String::from("what should I add? Try: add Buy milk"))
            } else {
                Ok(Command::Add(rest.to_string()))
            }
        }
        "done" => Ok(Command::Done(parse_id(rest)?)),
        "remove" | "rm" => Ok(Command::Remove(parse_id(rest)?)),
        "list" | "ls" => Ok(Command::List),
        "clear" => Ok(Command::Clear),
        "help" => Ok(Command::Help),
        "quit" | "exit" => Ok(Command::Quit),
        "" => Err(String::from("type a command, or 'help'")),
        other => Err(format!("unknown command '{other}'. Type 'help' for a list.")),
    }
}

fn main() {
    for input in ["add Buy milk", "  DONE 2 ", "rm 1", "ls", "done abc", "add", "fly away", ""] {
        println!("{:<14} => {:?}", format!("{input:?}"), parse_command(input).is_ok());
    }
    println!("{:?}", parse_command("add   Learn   Rust  "));
}

@@exercise A tiny bank, layered
Here's a complete interactive program with all the layers in one file: a **model** (`Account`), **commands** (`Command` + `parse`), **logic** (`execute`) and the **interface** (`main`'s loop). Only `execute` is missing.

`execute(account, command) -> Result<String, String>` applies a command and returns a message for the user:

- `Deposit(n)`: add `n` to the balance → `Ok("Deposited 50. Balance: 150")`
- `Withdraw(n)`: if there's enough money, subtract it → `Ok("Withdrew 30. Balance: 120")`; otherwise change nothing and return `Err("Not enough money: balance is 120")`
- `Balance` → `Ok("Balance: 120")`
- `History` → `Ok` with all past successful deposits/withdrawals, one per line, like `+50` and `-30`; or `"No transactions yet"` when there are none.

The account keeps its `history` as a `Vec<i64>` of amounts (positive for deposits, negative for withdrawals).

The program's input is given in the stdin box below the editor.
@@starter
use std::io;

// ----- model -----
struct Account {
    balance: u64,
    history: Vec<i64>,
}

// ----- commands -----
enum Command {
    Deposit(u64),
    Withdraw(u64),
    Balance,
    History,
}

fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        ["deposit", n] => n.parse().map(Command::Deposit).map_err(|_| format!("bad amount: {n}")),
        ["withdraw", n] => n.parse().map(Command::Withdraw).map_err(|_| format!("bad amount: {n}")),
        ["balance"] => Ok(Command::Balance),
        ["history"] => Ok(Command::History),
        _ => Err(format!("unknown command: {line}")),
    }
}

// ----- logic -----
fn execute(account: &mut Account, command: Command) -> Result<String, String> {
    todo!()
}

// ----- interface -----
fn main() {
    let mut account = Account { balance: 100, history: Vec::new() };
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("> {line}");
        match parse(line).and_then(|cmd| execute(&mut account, cmd)) {
            Ok(message) => println!("{message}"),
            Err(error) => println!("Error: {error}"),
        }
    }
}
@@stdin
history
deposit 50
withdraw 30
withdraw 500
fly
balance
history
@@expected
> history
No transactions yet
> deposit 50
Deposited 50. Balance: 150
> withdraw 30
Withdrew 30. Balance: 120
> withdraw 500
Error: Not enough money: balance is 120
> fly
Error: unknown command: fly
> balance
Balance: 120
> history
+50
-30
@@tests
#[test]
fn checks() {
    let mut a = Account { balance: 10, history: Vec::new() };
    assert_eq!(execute(&mut a, Command::Withdraw(10)), Ok(String::from("Withdrew 10. Balance: 0")));
    assert!(execute(&mut a, Command::Withdraw(1)).is_err());
    assert_eq!(a.balance, 0);
    assert_eq!(a.history, vec![-10]);
}
@@hint
`match command { Command::Deposit(n) => { account.balance += n; account.history.push(n as i64); Ok(format!(...)) } ... }`

For `History`, map each amount to text with `format!("{:+}", amount)` (the `+` flag always shows the sign), collect into a `Vec<String>` and `join("\n")`.
@@solution
use std::io;

// ----- model -----
struct Account {
    balance: u64,
    history: Vec<i64>,
}

// ----- commands -----
enum Command {
    Deposit(u64),
    Withdraw(u64),
    Balance,
    History,
}

fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        ["deposit", n] => n.parse().map(Command::Deposit).map_err(|_| format!("bad amount: {n}")),
        ["withdraw", n] => n.parse().map(Command::Withdraw).map_err(|_| format!("bad amount: {n}")),
        ["balance"] => Ok(Command::Balance),
        ["history"] => Ok(Command::History),
        _ => Err(format!("unknown command: {line}")),
    }
}

// ----- logic -----
fn execute(account: &mut Account, command: Command) -> Result<String, String> {
    match command {
        Command::Deposit(n) => {
            account.balance += n;
            account.history.push(n as i64);
            Ok(format!("Deposited {n}. Balance: {}", account.balance))
        }
        Command::Withdraw(n) => {
            if n > account.balance {
                return Err(format!("Not enough money: balance is {}", account.balance));
            }
            account.balance -= n;
            account.history.push(-(n as i64));
            Ok(format!("Withdrew {n}. Balance: {}", account.balance))
        }
        Command::Balance => Ok(format!("Balance: {}", account.balance)),
        Command::History => {
            if account.history.is_empty() {
                Ok(String::from("No transactions yet"))
            } else {
                let lines: Vec<String> = account.history.iter().map(|a| format!("{a:+}")).collect();
                Ok(lines.join("\n"))
            }
        }
    }
}

// ----- interface -----
fn main() {
    let mut account = Account { balance: 100, history: Vec::new() };
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("> {line}");
        match parse(line).and_then(|cmd| execute(&mut account, cmd)) {
            Ok(message) => println!("{message}"),
            Err(error) => println!("Error: {error}"),
        }
    }
}
