@@lesson m6-cargo Cargo Projects in VS Code
@@text
So far, every program was a single file that this app compiled for you with `rustc`. Real Rust projects use **Cargo**, Rust's official build tool and package manager. You installed it along with Rust. From this module on, you'll work like a professional Rust developer: in **VS Code**, with **Cargo**, in a **terminal**.

## What Cargo does

- **Creates** projects with a standard layout.
- **Builds** your code (calling `rustc` for you, with the right settings).
- **Downloads and builds libraries** ("crates") your project depends on.
- **Runs** your program and your **tests**.

## A Cargo project's layout

```text
my_app/
├── Cargo.toml        ← the project's settings: name, version, dependencies
├── Cargo.lock        ← exact versions of dependencies (created automatically)
├── src/
│   └── main.rs       ← your code starts here
└── target/           ← compiled output (created automatically; never edit it)
```

`Cargo.toml` is written in **TOML**, a simple settings format:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `name`: the project's name. It's also the name of the `.exe`.
- `edition`: which edition of the Rust language to use (2024 is the newest).
- `[dependencies]`: libraries your project uses. Empty for now.

## The commands you'll use every day

Run these in a terminal, inside the project folder:

| Command | What it does |
|---|---|
| `cargo new my_app` | create a new project folder called `my_app` |
| `cargo run` | build (if needed) and run the program |
| `cargo build` | just build. The `.exe` goes in `target/debug/` |
| `cargo check` | check for errors **without** building an `.exe`. Much faster |
| `cargo test` | run the tests |
| `cargo build --release` | build an **optimised** `.exe` in `target/release/`. Slower to build, much faster to run. Use it for the final version you share |

## Setting up VS Code (one time)

1. Open VS Code, go to the **Extensions** view (`Ctrl+Shift+X`), search for **rust-analyzer** and install it. This gives you:
   - errors underlined **as you type**, without running anything,
   - autocomplete: type `my_vec.` and see every method,
   - hover over anything to see its type and documentation,
   - "go to definition" (`F12`) and much more.
2. (Optional, for debugging later) also install **CodeLLDB**.
3. Open a **terminal inside VS Code** with ``Ctrl+` `` (the key above Tab). That's where you'll type `cargo` commands.

> If **💻 Open in VS Code** below doesn't work, VS Code's `code` command isn't on your PATH. Open VS Code yourself and use **File → Open Folder…** and pick the project folder shown on the card.

## Your first Cargo project

Press **📁 Create project** on the card below. This app writes the project files into a `RustQuestProjects` folder next to the app, so you don't have to type the setup yourself. (Everything it does, `cargo new` can do too — try it in a terminal some time.)

Then:

1. Press **💻 Open in VS Code**.
2. In VS Code, open `src/main.rs` and read the code.
3. Open the terminal (``Ctrl+` ``) and type `cargo run`, then press Enter.
4. The program asks you questions **interactively** — type your answers and press Enter. Finally, a real terminal!

You can also use the **▶ cargo run** button, which opens a separate terminal window and runs the project there.

@@project hello_cargo Hello, Cargo
Your first real Cargo project: a small interactive program that asks for your name and a number. Create it, open it in VS Code, and run it with `cargo run`.
@@file Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
@@file src/main.rs
use std::io::{self, Write};

fn ask(question: &str) -> String {
    print!("{question} ");
    io::stdout().flush().expect("could not flush stdout");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("could not read input");
    answer.trim().to_string()
}

fn main() {
    println!("=== Hello, Cargo! ===");
    let name = ask("What's your name?");

    let number = loop {
        let text = ask("Pick a whole number:");
        match text.parse::<i64>() {
            Ok(n) => break n,
            Err(_) => println!("'{text}' isn't a whole number. Try again."),
        }
    };

    println!("Nice to meet you, {name}!");
    println!("{number} squared is {}.", number * number);
    if number % 2 == 0 {
        println!("{number} is even.");
    } else {
        println!("{number} is odd.");
    }
}

@@text
## Things to try in VS Code

Once it runs, explore rust-analyzer's features. They'll make you much faster:

1. **Hover** over `ask`, `answer` or `parse` to see their types and documentation.
2. Break something on purpose: delete a `;`. A red squiggle appears immediately. Hover over it to read the error.
3. Type `name.` on a new line inside `main` and look at the autocomplete list: all the methods of `String`.
4. Put the cursor on `ask` in `main` and press `F12` to jump to its definition.
5. Right-click inside the file and choose **Format Document** (`Shift+Alt+F`). That runs `rustfmt`.
6. Change the program: also ask for the person's favourite colour, and print it back. Run it again with `cargo run`.

Notice that `cargo run` the second time is much faster: Cargo only rebuilds what changed.

## Where's my .exe?

After `cargo build` (or `cargo run`), your program is at `target\debug\hello_cargo.exe`. You can double-click it, or copy it anywhere. Your first real program!

@@quiz
Which command checks your code for errors fastest, without producing an `.exe`?
- `cargo run`
- `cargo build --release`
* `cargo check`
? `cargo check` skips the slowest step (generating machine code), so it's the quickest way to see compile errors.

@@quiz
What is `Cargo.toml` for?
* The project's settings: its name, version, edition and dependencies
- Your program's code
- Compiled output
? Code goes in `src/`; compiled output goes in `target/`.
