@@lesson m0-hello Hello, World! (Every Character Explained)
@@text
By tradition, the first program in any language prints "Hello, world!". Here it is in Rust:

```rust
fn main() {
    println!("Hello, world!");
}
```

Only three lines — but every character means something. Let's take it apart.

## `fn main() { ... }` — the main function

- `fn` is short for **function**. A function is a named block of code. (You'll write your own functions soon.)
- `main` is the function's **name**. `main` is special: it's where every Rust program **starts**. When you run the `.exe`, the computer runs the code inside `main`, top to bottom, and when it reaches the end, the program ends.
- `()` — the parentheses hold the function's *parameters* (inputs). `main` takes none, so they're empty. They still must be there.
- `{` and `}` — curly braces mark the start and end of the function's **body**: the code that belongs to it. Everything between them is "inside" `main`.

## `println!("Hello, world!");` — printing a line

- `println!` prints text and then moves to a new line ("print line").
- The `!` means `println!` is a **macro**, not a normal function. Macros are a bit like functions that write code for you. For now, just remember: `println!` needs its `!`.
- `"Hello, world!"` is a **string literal**: a piece of text written directly in your code. Text in Rust goes between **double quotes** `"`.
- `;` — the **semicolon** marks the end of an instruction (a *statement*). Think of it like the full stop at the end of a sentence.

## Indentation

The `println!` line is pushed in by 4 spaces. This is called **indentation**. The compiler doesn't actually care about it — but humans do. Indentation shows at a glance which code is *inside* which block. Always indent code inside `{ }` by 4 spaces. (In the editor, pressing `Tab` inserts 4 spaces, and pressing `Enter` keeps your indentation.)

@@quiz
Where does every Rust program start running?
- At the first line of the file
* At the beginning of the `main` function
- At the last `println!`
? Every Rust program starts at `main`. Code outside of a function doesn't "run" by itself.

@@text
## Printing several lines

Each `println!` prints one line:

```rust
fn main() {
    println!("Roses are red,");
    println!("Violets are blue,");
    println!("Rust is fast,");
    println!("And safe too.");
}
```

`println!()` with nothing inside prints an empty line.

## `print!` — no new line

`print!` (without `ln`) prints text *without* moving to a new line afterwards:

```rust
fn main() {
    print!("Hello, ");
    print!("world");
    println!("!");
    println!("Next line.");
}
```

## Special characters: escape sequences

What if you want to print a double quote? `println!("She said "hi"")` would confuse the compiler — it would think the text ends at the second `"`. The fix is an **escape sequence**: a backslash `\` followed by a character, which means "this character is special":

| Write this | To get |
|---|---|
| `\"` | a double quote `"` |
| `\\` | a single backslash `\` |
| `\n` | a new line |
| `\t` | a tab (a wide gap) |

```rust
fn main() {
    println!("She said \"hi\" to me.");
    println!("Line one\nLine two");
    println!("Name:\tFerris");
    println!("A path: C:\\Users\\Ferris");
}
```

> Ferris the crab 🦀 is Rust's unofficial mascot. Rust programmers call themselves *Rustaceans*.

@@quiz
Which line prints exactly `Say "cheese"`?
- `println!("Say "cheese"");`
* `println!("Say \"cheese\"");`
- `println!(Say "cheese");`
? Inside a string, a `"` must be written as `\"`, otherwise the string would end there.

@@exercise Introduce yourself
Write a program that prints **exactly** these three lines:

```text
Hello, world!
My name is Ferris.
I love Rust!
```

Use one `println!` per line.
@@starter
fn main() {
    println!("Hello, world!");
}
@@expected
Hello, world!
My name is Ferris.
I love Rust!
@@hint
Add two more `println!` lines inside `main`, under the first one. Don't forget the semicolons at the end of each line.
@@solution
fn main() {
    println!("Hello, world!");
    println!("My name is Ferris.");
    println!("I love Rust!");
}

@@exercise Draw a box
Print this exact picture (note the quotes in the middle line):

```text
+--------+
| "Rust" |
+--------+
```
@@starter
fn main() {
    // Your code here
}
@@expected
+--------+
| "Rust" |
+--------+
@@hint
The middle line needs escaped quotes: `println!("| \"Rust\" |");`
@@solution
fn main() {
    println!("+--------+");
    println!("| \"Rust\" |");
    println!("+--------+");
}

@@exercise One println, three lines
Print the same three lines as in the first exercise of this lesson, but this time using **only one** `println!`. Use the `\n` escape sequence to start new lines.

```text
Hello, world!
My name is Ferris.
I love Rust!
```
@@starter
fn main() {
    println!("Hello, world!");
}
@@expected
Hello, world!
My name is Ferris.
I love Rust!
@@requires
\n
@@hint
`\n` inside a string starts a new line: `println!("first\nsecond");` prints two lines.
@@solution
fn main() {
    println!("Hello, world!\nMy name is Ferris.\nI love Rust!");
}
