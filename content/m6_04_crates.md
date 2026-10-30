@@lesson m6-crates Using Crates: The Guessing Game
@@text
One of Rust's biggest strengths is its ecosystem: more than 150,000 free libraries, called **crates**, published on **crates.io**. Need random numbers, JSON, HTTP, a GUI, a database? There's a crate for that. Cargo downloads and builds them for you.

## Adding a dependency

To use a crate, add it to `[dependencies]` in `Cargo.toml`. The easiest way is from the terminal:

```text
cargo add rand
```

This adds a line like `rand = "0.10"` to `Cargo.toml`. The next time you build, Cargo downloads `rand` (and everything *it* depends on), compiles it once, and links it into your program.

## Version numbers

Crates use **semantic versioning**: `MAJOR.MINOR.PATCH`, like `0.10.3`.

- **PATCH** changes (0.10.3 → 0.10.4): bug fixes only.
- **MINOR** changes (1.2 → 1.3): new features, nothing breaks.
- **MAJOR** changes (1.x → 2.0): breaking changes; your code may need updating. For `0.x` versions, the *minor* number acts as the major one.

`rand = "0.10"` means "any 0.10.x version". Cargo picks the newest one that fits and records the exact choice in `Cargo.lock`, so every build uses the same versions.

## Finding crates and their documentation

- **crates.io** — search for crates, see how popular and maintained they are.
- **docs.rs** — the documentation for every crate, e.g. `docs.rs/rand`. Every function, with examples.
- **blessed.rs** — a curated list of the crates most Rust developers recommend.

Reading documentation is a core skill. When you use a new crate, open its docs and look for the examples first.

## Random numbers with `rand`

```rust,norun
let secret = rand::random_range(1..=100);      // a random number from 1 to 100
let coin: bool = rand::random();               // true or false
let index = rand::random_range(0..list.len()); // a random valid index
```

## The guessing game

Let's build the classic guessing game: the computer picks a secret number between 1 and 100, and you guess until you find it, with "too high" / "too low" hints.

The project below starts **without** `rand`. Your steps:

1. Create the project and open it in VS Code.
2. In VS Code's terminal, run `cargo add rand`. Look at how `Cargo.toml` changed.
3. Replace the `todo!()` in `main.rs` with the game, following the plan in the comments.
4. `cargo run` and play!

@@project guessing_game Guessing Game
The classic first Rust program, made interactive. Follow the comments in `main.rs`.
@@file Cargo.toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
@@file src/main.rs
use std::cmp::Ordering;
use std::io::{self, Write};

fn read_guess() -> Option<u32> {
    print!("Your guess: ");
    io::stdout().flush().expect("could not flush stdout");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("could not read input");
    text.trim().parse().ok()
}

fn main() {
    println!("I'm thinking of a number between 1 and 100...");

    // 1. After `cargo add rand`, pick the secret number:
    //    let secret: u32 = rand::random_range(1..=100);
    // 2. Count the guesses in a mutable variable.
    // 3. Loop:
    //    - read a guess with read_guess(); if it's None, print a message and `continue`
    //    - add 1 to the guess counter
    //    - compare the guess with the secret using `guess.cmp(&secret)`, and match on
    //      Ordering::Less / Ordering::Greater / Ordering::Equal
    //    - on Equal, print how many guesses it took and `break`
    todo!("write the game!")
}

@@text
## Comparing with `cmp` and `Ordering`

`a.cmp(&b)` compares two values and returns an `Ordering` — an enum with three variants: `Less`, `Greater` and `Equal`. It's perfect for `match`:

```rust
use std::cmp::Ordering;

fn main() {
    let guess = 40;
    let secret = 57;
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You got it!"),
    }
}
```

## A finished version

Try writing it yourself first! When you're done (or truly stuck), compare with this:

```rust,norun
fn main() {
    println!("I'm thinking of a number between 1 and 100...");
    let secret: u32 = rand::random_range(1..=100);
    let mut guesses = 0;

    loop {
        let Some(guess) = read_guess() else {
            println!("Please type a whole number.");
            continue;
        };
        guesses += 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("🎉 Correct! It took you {guesses} guesses.");
                break;
            }
        }
    }
}
```

## Make it yours

Some ideas to extend the game. Each one practises something from earlier modules:

- Let the player choose the difficulty (the maximum number) at the start.
- Limit the number of guesses, and reveal the number if they run out.
- After winning, ask "Play again? (y/n)" — wrap the whole game in an outer `loop`.
- Keep a list of previous guesses in a `Vec` and warn when the player repeats one.
- Tell them "very close!" when they're within 3.

@@quiz
What does `cargo add rand` do?
- Downloads rand into your `src` folder
* Adds `rand` as a dependency in `Cargo.toml` (Cargo downloads it on the next build)
- Installs a program called rand
? Dependencies are listed in `Cargo.toml`. Cargo fetches and builds them automatically.

@@quiz
Which version change might break your code?
- 1.4.2 → 1.4.3
- 1.4.2 → 1.5.0
* 1.4.2 → 2.0.0
? A new MAJOR version signals breaking changes.
