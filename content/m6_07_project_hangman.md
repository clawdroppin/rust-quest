@@lesson m6-hangman Project: Hangman
@@text
Time to put Module 6 together in a real, multi-file Cargo project: **Hangman**. The computer picks a secret word and you guess letters. Every wrong guess draws another piece of the gallows. Guess the word before the drawing is finished!

You'll practise everything from this module:

- a **Cargo project** with a **crate** (`rand`) to pick random words,
- code split into **modules in separate files**, with `pub` for the public interface,
- **test-driven development**: the tests for the game logic are already written, and your job is to make them pass,
- **doc comments** describing each function.

## The design

The program is split into three files, following the rule "separate logic from input/output":

```text
src/
├── main.rs    ← the game loop: reads input, prints the board (I/O only)
├── words.rs   ← the word list, and picking a random word
└── game.rs    ← the Game struct: all the rules, no printing at all
```

Because `game.rs` never prints or reads anything, it can be tested completely with `cargo test`. And because `main.rs` only talks to `Game` through its public methods, you could later replace the text interface with a GUI without touching the rules. (Keep that in mind for Module 9!)

## The `Game` struct

```rust,norun
pub struct Game {
    word: String,          // the secret word, lowercase
    guessed: Vec<char>,    // every letter guessed so far, in order
    wrong: u32,            // how many wrong guesses
    max_wrong: u32,        // how many wrong guesses lose the game
}
```

All fields are private: `main.rs` can't cheat by peeking at `word` or resetting `wrong`. The result of a guess is an enum:

```rust,norun
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    Correct,         // the letter is in the word
    Wrong,           // it isn't; one more piece of the gallows
    AlreadyGuessed,  // you tried that letter before (no penalty)
    NotALetter,      // '7' or '?' isn't a valid guess (no penalty)
}
```

## Your steps

1. Create the project and open it in VS Code. Run `cargo add rand` in the terminal.
2. Run **`cargo test`**. Most tests fail with "not yet implemented": that's your to-do list.
3. Open `src/game.rs` and implement the methods marked `todo!()`, one at a time: `masked`, `guess`, `is_won`, `is_lost`. Run `cargo test` after each one and watch the failures disappear.
4. Open `src/words.rs` and implement `random_word` using `rand::random_range`.
5. When every test passes, `cargo run` and play!

@@project hangman Hangman
A multi-file Hangman game. The tests in `game.rs` describe exactly how the game must behave. Make them pass, then play.
@@file Cargo.toml
[package]
name = "hangman"
version = "0.1.0"
edition = "2024"

[dependencies]
@@file src/main.rs
mod game;
mod words;

use game::{Game, GuessResult};
use std::io::{self, Write};

const MAX_WRONG: u32 = 6;

/// The gallows, one picture per number of wrong guesses (0 to 6).
const PICTURES: [&str; 7] = [
    "  +---+\n      |\n      |\n      |\n     ===",
    "  +---+\n  O   |\n      |\n      |\n     ===",
    "  +---+\n  O   |\n  |   |\n      |\n     ===",
    "  +---+\n  O   |\n /|   |\n      |\n     ===",
    "  +---+\n  O   |\n /|\\  |\n      |\n     ===",
    "  +---+\n  O   |\n /|\\  |\n /    |\n     ===",
    "  +---+\n  O   |\n /|\\  |\n / \\  |\n     ===",
];

/// Asks for one line of input and returns it trimmed. Returns None at end of input.
fn ask(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().expect("could not flush stdout");
    let mut line = String::new();
    let bytes = io::stdin().read_line(&mut line).expect("could not read input");
    if bytes == 0 {
        return None;
    }
    Some(line.trim().to_string())
}

/// Plays one round. Returns true if the player won.
fn play_round() -> bool {
    let mut game = Game::new(words::random_word(), MAX_WRONG);

    while !game.is_won() && !game.is_lost() {
        println!();
        println!("{}", PICTURES[game.wrong_guesses() as usize]);
        println!();
        println!("Word:    {}", game.masked());
        println!("Guessed: {}", game.guessed_letters());
        println!("Lives:   {}", game.lives_left());

        let Some(input) = ask("Guess a letter: ") else {
            return false;
        };
        let Some(letter) = input.chars().next() else {
            continue;
        };

        match game.guess(letter) {
            GuessResult::Correct => println!("✔ Yes, '{letter}' is in the word!"),
            GuessResult::Wrong => println!("✘ No '{letter}' in the word."),
            GuessResult::AlreadyGuessed => println!("You already tried '{letter}'."),
            GuessResult::NotALetter => println!("Please type a letter from a to z."),
        }
    }

    println!();
    println!("{}", PICTURES[game.wrong_guesses() as usize]);
    if game.is_won() {
        println!("🎉 You won! The word was \"{}\".", game.word());
        true
    } else {
        println!("💀 Out of lives. The word was \"{}\".", game.word());
        false
    }
}

fn main() {
    println!("=== HANGMAN ===");
    let mut wins = 0;
    let mut rounds = 0;

    loop {
        rounds += 1;
        if play_round() {
            wins += 1;
        }
        println!("Score: {wins} won out of {rounds}");

        match ask("Play again? (y/n): ") {
            Some(answer) if answer.eq_ignore_ascii_case("y") => continue,
            _ => break,
        }
    }
    println!("Thanks for playing!");
}
@@file src/words.rs
//! The list of secret words.

/// All the words the game can pick from. Lowercase letters only.
pub const WORDS: [&str; 20] = [
    "rust", "ferris", "compiler", "borrow", "variable", "function", "module", "crate", "vector",
    "string", "integer", "closure", "iterator", "trait", "pattern", "keyboard", "program",
    "library", "terminal", "computer",
];

/// Picks a random word from `WORDS`.
pub fn random_word() -> &'static str {
    // Use rand::random_range to pick a random index from 0..WORDS.len()
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_are_lowercase_letters() {
        for word in WORDS {
            assert!(!word.is_empty());
            assert!(word.chars().all(|c| c.is_ascii_lowercase()), "bad word: {word}");
        }
    }

    #[test]
    fn random_word_comes_from_the_list() {
        for _ in 0..50 {
            assert!(WORDS.contains(&random_word()));
        }
    }
}
@@file src/game.rs
//! The rules of Hangman. Nothing in here prints or reads input.

/// What happened when the player guessed a letter.
#[derive(Debug, PartialEq)]
pub enum GuessResult {
    /// The letter is in the word.
    Correct,
    /// The letter is not in the word: it costs a life.
    Wrong,
    /// The letter was guessed before. No penalty.
    AlreadyGuessed,
    /// The guess wasn't a letter a-z. No penalty.
    NotALetter,
}

/// One game of Hangman.
pub struct Game {
    word: String,
    guessed: Vec<char>,
    wrong: u32,
    max_wrong: u32,
}

impl Game {
    /// Starts a new game with the given secret word (any case) and number of lives.
    pub fn new(word: &str, max_wrong: u32) -> Game {
        Game {
            word: word.to_lowercase(),
            guessed: Vec::new(),
            wrong: 0,
            max_wrong,
        }
    }

    /// The secret word. Only show this when the game is over!
    pub fn word(&self) -> &str {
        &self.word
    }

    /// How many wrong guesses have been made.
    pub fn wrong_guesses(&self) -> u32 {
        self.wrong
    }

    /// How many more wrong guesses the player can make before losing.
    pub fn lives_left(&self) -> u32 {
        self.max_wrong - self.wrong
    }

    /// The guessed letters so far, separated by spaces, e.g. "e t a".
    pub fn guessed_letters(&self) -> String {
        let letters: Vec<String> = self.guessed.iter().map(|c| c.to_string()).collect();
        letters.join(" ")
    }

    /// The word with unguessed letters hidden, letters separated by spaces.
    /// For "rust" after guessing 'u' and 't': "_ u _ t".
    pub fn masked(&self) -> String {
        todo!()
    }

    /// Makes a guess. Uppercase letters count as lowercase.
    /// See `GuessResult` for the possible outcomes.
    pub fn guess(&mut self, letter: char) -> GuessResult {
        todo!()
    }

    /// True when every letter of the word has been guessed.
    pub fn is_won(&self) -> bool {
        todo!()
    }

    /// True when the player has run out of lives.
    pub fn is_lost(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_hides_every_letter() {
        let game = Game::new("rust", 6);
        assert_eq!(game.masked(), "_ _ _ _");
        assert_eq!(game.lives_left(), 6);
    }

    #[test]
    fn correct_guess_reveals_letters() {
        let mut game = Game::new("rust", 6);
        assert_eq!(game.guess('u'), GuessResult::Correct);
        assert_eq!(game.masked(), "_ u _ _");
        assert_eq!(game.lives_left(), 6);
    }

    #[test]
    fn correct_guess_reveals_every_copy() {
        let mut game = Game::new("letter", 6);
        game.guess('t');
        assert_eq!(game.masked(), "_ _ t t _ _");
    }

    #[test]
    fn wrong_guess_costs_a_life() {
        let mut game = Game::new("rust", 6);
        assert_eq!(game.guess('z'), GuessResult::Wrong);
        assert_eq!(game.wrong_guesses(), 1);
        assert_eq!(game.lives_left(), 5);
    }

    #[test]
    fn repeated_guess_is_free() {
        let mut game = Game::new("rust", 6);
        game.guess('z');
        assert_eq!(game.guess('z'), GuessResult::AlreadyGuessed);
        game.guess('r');
        assert_eq!(game.guess('r'), GuessResult::AlreadyGuessed);
        assert_eq!(game.wrong_guesses(), 1);
    }

    #[test]
    fn uppercase_counts_as_lowercase() {
        let mut game = Game::new("Rust", 6);
        assert_eq!(game.guess('R'), GuessResult::Correct);
        assert_eq!(game.guess('r'), GuessResult::AlreadyGuessed);
        assert_eq!(game.masked(), "r _ _ _");
    }

    #[test]
    fn non_letters_are_rejected() {
        let mut game = Game::new("rust", 6);
        assert_eq!(game.guess('7'), GuessResult::NotALetter);
        assert_eq!(game.guess('?'), GuessResult::NotALetter);
        assert_eq!(game.wrong_guesses(), 0);
        assert_eq!(game.guessed_letters(), "");
    }

    #[test]
    fn guessing_every_letter_wins() {
        let mut game = Game::new("noon", 6);
        assert!(!game.is_won());
        game.guess('n');
        game.guess('o');
        assert!(game.is_won());
        assert!(!game.is_lost());
    }

    #[test]
    fn running_out_of_lives_loses() {
        let mut game = Game::new("a", 2);
        game.guess('x');
        assert!(!game.is_lost());
        game.guess('y');
        assert!(game.is_lost());
        assert!(!game.is_won());
        assert_eq!(game.lives_left(), 0);
    }

    #[test]
    fn guessed_letters_are_listed_in_order() {
        let mut game = Game::new("rust", 6);
        game.guess('e');
        game.guess('t');
        game.guess('a');
        assert_eq!(game.guessed_letters(), "e t a");
    }
}

@@text
## Hints, if you get stuck

**`masked`**: go through `self.word.chars()`. For each letter, push either the letter or `'_'` into a `Vec<String>` (or build the string directly), then join with spaces. `self.guessed.contains(&c)` tells you whether a letter was guessed.

**`guess`**, in this order:

1. Convert the letter: `let letter = letter.to_ascii_lowercase();`
2. If `!letter.is_ascii_lowercase()`, return `NotALetter`.
3. If it's already in `self.guessed`, return `AlreadyGuessed`.
4. Push it into `self.guessed`.
5. If `self.word.contains(letter)`, return `Correct`; otherwise add 1 to `self.wrong` and return `Wrong`.

**`is_won`**: every character of the word is in `guessed`. A `for` loop that returns `false` early works; so does `self.word.chars().all(|c| self.guessed.contains(&c))` — you'll learn exactly how `all` works in the next module.

**`is_lost`**: compare `self.wrong` with `self.max_wrong`.

**`random_word`**: `WORDS[rand::random_range(0..WORDS.len())]`.

## A possible solution for `guess`

Try it yourself first! Here's one way to write the hardest method:

```rust,norun
pub fn guess(&mut self, letter: char) -> GuessResult {
    let letter = letter.to_ascii_lowercase();
    if !letter.is_ascii_lowercase() {
        return GuessResult::NotALetter;
    }
    if self.guessed.contains(&letter) {
        return GuessResult::AlreadyGuessed;
    }
    self.guessed.push(letter);
    if self.word.contains(letter) {
        GuessResult::Correct
    } else {
        self.wrong += 1;
        GuessResult::Wrong
    }
}
```

## Finish like a pro

When everything works:

1. Run `cargo clippy` and fix anything it suggests.
2. Run `cargo fmt`.
3. Run `cargo doc --open` and look at the documentation generated from your `///` comments.

## Make it yours

- Add categories (animals, countries, programming…) and let the player choose one: a `match` in `words.rs` returning different lists.
- Load the word list from a text file (you'll learn file reading in Module 8).
- Add a "hint" command that reveals one letter but costs a life. Write the test first!
- Difficulty levels: easy (8 lives), normal (6), hard (4).

@@quiz
Why doesn't `game.rs` print anything?
* So the rules can be tested automatically and reused with a different interface (like a GUI)
- Because modules aren't allowed to print
- To make the program faster
? Separating logic from I/O makes the logic testable and reusable.

@@quiz
`Game`'s fields are all private. What does that protect against?
- Nothing, it's just style
* Code outside the module changing the state in ways that break the rules (like resetting `wrong` or reading `word`)
- The compiler optimizing the struct away
? The only way to change a `Game` is through its methods, which always follow the rules.
