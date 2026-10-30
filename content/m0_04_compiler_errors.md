@@lesson m0-errors Reading Compiler Errors
@@text
You're going to see a *lot* of compiler errors while learning — and for the rest of your programming life. That's completely normal. The skill isn't avoiding errors; it's **reading them calmly and fixing them quickly**. Rust's error messages are some of the best of any language, so let's learn to read them.

## Anatomy of an error

Try running this program. It has a mistake:

```rust
fn main() {
    println!("Hello")
    println!("World");
}
```

The compiler says something like:

```text
error: expected `;`, found `println`
 --> main.rs:2:22
  |
2 |     println!("Hello")
  |                      ^ help: add `;` here
3 |     println!("World");
  |     ------- unexpected token
```

Read it piece by piece:

1. **`error: expected ;, found println`** — *what* went wrong. The compiler expected a semicolon, but found the next `println` instead.
2. **`--> main.rs:2:22`** — *where*: file `main.rs`, **line 2**, column (character) 22. The line numbers in the editor's left margin help you find it.
3. **The code snippet** — the compiler shows the line, with `^` arrows pointing at the exact spot.
4. **`help: add ; here`** — often the compiler even tells you how to fix it!

## The golden rules of errors

1. **Fix the first error first.** One mistake can cause several follow-up errors. Fixing the first one often makes the others disappear.
2. **Read the whole message.** Look for `help:` and `note:` lines — they're gold.
3. **Look at the line *before* too.** For missing semicolons or brackets, the real mistake is often at the end of the previous line.
4. **Fix one thing, then compile again.** Small steps.

## Common beginner errors

**Missing semicolon** — shown above.

**Misspelled name:**

```rust
fn main() {
    printn!("Hi");
}
```

```text
error: cannot find macro `printn` in this scope
 --> main.rs:2:5
  |
2 |     printn!("Hi");
  |     ^^^^^^ help: a macro with a similar name exists: `println`
```

**Missing closing quote** — everything after the `"` becomes part of the string, so the error may appear far away:

```rust
fn main() {
    println!("Hi);
}
```

**Unmatched braces** — every `{` needs a matching `}`:

```rust
fn main() {
    println!("Hi");
```

```text
error: this file contains an unclosed delimiter
```

## Errors vs warnings

A **warning** is not an error. Your code still compiles and runs, but the compiler thinks something looks suspicious — for example a variable you created but never used. In this app, warnings appear in a yellow section under the output. Read them: they often point at real bugs.

## Panics: errors while running

Some problems can only be detected while the program is *running* — for example dividing a number by zero that came from user input. When that happens, a Rust program **panics**: it stops immediately and prints a message explaining why. This app shows a 💥 when that happens. You'll learn more about panics later.

@@quiz
The compiler reports `--> main.rs:7:12`. What does that tell you?
- The error is error number 7
* The problem is on line 7, at character 12
- There are 7 errors and 12 warnings
? The format is `file:line:column`.

@@quiz
Your program has 5 errors. What's the best strategy?
- Fix the last error first
- Rewrite the whole program
* Fix the first error, then compile again
? Later errors are often *caused* by the first one. Fix it, recompile, and see what's left.

@@exercise Fix the bugs
This program has **three** mistakes. Use the compiler's messages to find and fix them. When it's fixed, it should print:

```text
Rust is fun.
Errors are helpful.
I can fix them!
```
@@starter
fn main() {
    println!("Rust is fun.")
    printline!("Errors are helpful.");
    println!("I can fix them!);
}
@@expected
Rust is fun.
Errors are helpful.
I can fix them!
@@hint
1. Line 2 is missing something at the end.
2. Line 3 uses a macro name that doesn't exist — compare it with line 2.
3. Line 4: every string needs a closing `"`.
@@solution
fn main() {
    println!("Rust is fun.");
    println!("Errors are helpful.");
    println!("I can fix them!");
}

@@exercise Brace yourself
This one has problems with braces and quotes. Fix it so it prints:

```text
"Quotes" need escaping.
Braces come in pairs.
```
@@starter
fn main() {
    println!(""Quotes" need escaping.");
    println!("Braces come in pairs.");
@@expected
"Quotes" need escaping.
Braces come in pairs.
@@hint
To print a `"` inside a string, write `\"`. And `main` is missing its closing `}`.
@@solution
fn main() {
    println!("\"Quotes\" need escaping.");
    println!("Braces come in pairs.");
}
