@@lesson m0-program What Is a Program?
@@text
Before writing code, it helps to understand what code actually *is* and what happens to it.

## Computers follow instructions

A computer's processor (the **CPU**) can only do very simple things: add two numbers, compare two numbers, copy a number from one place in memory to another, jump to a different instruction. That's about it. But it does *billions* of these per second.

A **program** is a list of instructions for the computer. Everything you've ever used — games, web browsers, Windows itself — is "just" a huge list of very simple instructions, carefully arranged.

## Source code and the compiler

The CPU only understands **machine code**: raw numbers. Writing that by hand is miserable. So instead we write **source code** in a *programming language* like Rust, which is designed to be readable by humans:

```rust,norun
fn main() {
    println!("Hello!");
}
```

Then a program called a **compiler** translates the source code into machine code. Rust's compiler is called `rustc`. The result is an **executable** — on Windows, an `.exe` file you can run.

```text
 your code (main.rs)  ──►  rustc (the compiler)  ──►  main.exe  ──►  runs on the CPU
```

That's exactly what happens when you press **▶ Run** in this app: your code is saved to a file, `rustc` compiles it into an `.exe`, and the `.exe` is run. Its output is shown to you.

> **Compiled vs interpreted languages.** Some languages (like Python or JavaScript) are usually *interpreted*: another program reads your code and performs it line by line, every time it runs. Rust is *compiled* ahead of time. That's one reason Rust programs are very fast — all the translating is done before the program ever starts.

## The compiler is strict — and that's good

The compiler checks your code very carefully before it creates the `.exe`. If something is wrong — a typo, a missing symbol, using a number where text is expected — it refuses to compile and tells you what's wrong.

Rust's compiler is famous for being **strict but helpful**. It catches many mistakes that in other languages would only show up later as crashes or weird bugs. Learning Rust means learning to have a conversation with the compiler.

## Why Rust?

- **Fast** — as fast as C and C++, the languages most games and operating systems are written in.
- **Safe** — the compiler prevents whole categories of bugs (especially memory bugs) that cause crashes and security holes in other languages.
- **Modern** — great tools (`cargo`), great error messages, and a friendly community.

Rust has a reputation for being a bit harder to *start* with than some languages. That's fine: this course goes step by step, and the habits Rust teaches you make you a better programmer in any language.

@@quiz
What does a **compiler** do?
- It runs your program line by line
* It translates your source code into machine code the CPU can run
- It checks your spelling in comments
? Right. The compiler (`rustc`) turns human-readable source code into an executable file of machine code.

@@text
## Programs run in order

The most fundamental rule of programming: **instructions run one after another, from top to bottom**, unless you tell the computer otherwise (you'll learn how later, with *if* and *loops*).

```rust
fn main() {
    println!("First");
    println!("Second");
    println!("Third");
}
```

Try it, then swap two of the lines in the Playground and run it again. The output order follows the code order. Every single time. Computers don't guess what you *meant* — they do exactly what you *wrote*.

## Input → Processing → Output

Almost every program follows the same shape:

1. **Input** — get some data (from the keyboard, a file, the mouse, the internet…)
2. **Processing** — do something with it (calculate, sort, decide…)
3. **Output** — show or save the result (print text, draw on screen, write a file…)

A calculator app: input = the buttons you press, processing = the math, output = the number on the screen. A game: input = controller, processing = physics and game rules, output = the picture and sound. Keep this shape in mind — when you design your own programs, you'll start by asking *what goes in, what comes out, and what happens in between?*

@@quiz
In what order do these instructions run?

```text
println!("A");
println!("B");
println!("C");
```
- In a random order
- C, B, A
* A, B, C
? Code runs top to bottom, one instruction after another.

@@exercise Put things in order
This program is supposed to print the steps of making tea **in the correct order**, but the lines are mixed up.

Rearrange the lines (cut and paste them with `Ctrl+X` / `Ctrl+V`) so the output is:

```text
1. Boil water
2. Put a tea bag in a cup
3. Pour the water into the cup
4. Wait 3 minutes
5. Enjoy!
```
@@starter
fn main() {
    println!("3. Pour the water into the cup");
    println!("5. Enjoy!");
    println!("1. Boil water");
    println!("4. Wait 3 minutes");
    println!("2. Put a tea bag in a cup");
}
@@expected
1. Boil water
2. Put a tea bag in a cup
3. Pour the water into the cup
4. Wait 3 minutes
5. Enjoy!
@@hint
Move the whole line `println!("1. Boil water");` to be the first line inside the `{ }`, then the line with `2.` under it, and so on.
@@solution
fn main() {
    println!("1. Boil water");
    println!("2. Put a tea bag in a cup");
    println!("3. Pour the water into the cup");
    println!("4. Wait 3 minutes");
    println!("5. Enjoy!");
}
