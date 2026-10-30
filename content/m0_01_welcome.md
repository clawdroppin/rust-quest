@@lesson m0-welcome Welcome to Rust Quest
@@text
Welcome! This app will take you from **zero programming knowledge** all the way to building your own desktop app with a graphical interface, written in **Rust**.

You don't need any previous experience. Every idea is explained from the ground up, and you'll practise each one by writing real code that really runs on your computer.

## How this app works

The screen has three parts:

- **Left — the course map.** Modules and lessons, in order. A green ✔ means you finished it.
- **Middle — the lesson.** Explanations, examples and quick quizzes. Read this part slowly.
- **Right — the code editor.** This is where you write code. Your code is compiled by the real Rust compiler on your machine.

Under the editor are two buttons:

- **▶ Run** (shortcut: `Ctrl+Enter` or `F5`) compiles your code and runs it. Whatever your program prints shows up in the output area.
- **✔ Check** (shortcut: `Ctrl+Shift+Enter`) runs your code *and* tests whether it solves the exercise.

Your progress and your code are saved automatically in a file next to the app, so you can close it any time and continue later.

## Example code

Lessons contain example programs like the one below. Press **▶ Try it** to open it in the **Playground** tab of the editor and run it. The Playground is your free space: change things, break things, experiment. Nothing can go wrong.

```rust
fn main() {
    println!("Hello from Rust Quest!");
}
```

Don't worry about what any of that means yet — the next lessons explain every single character of it.

## Tips for learning to code

1. **Type the code yourself.** Don't copy and paste. Typing it forces your brain to notice every detail, and programming is all about details.
2. **Experiment.** After every example, change something and predict what will happen *before* you press Run. Then check if you were right.
3. **Errors are normal.** Even experienced programmers see compiler errors all day long. An error isn't failure — it's the compiler helping you.
4. **Struggle a little before using hints.** The 💡 Hint and 👀 Solution buttons exist, but the learning happens while you're figuring it out.
5. **Short, regular sessions beat marathons.** 30–60 minutes a day will get you much further than one long session a week.

@@quiz
What does the **✔ Check** button do?
- It fixes the mistakes in your code
* It runs your code and tests whether it solves the exercise
- It shows the solution
? Exactly. **▶ Run** just runs your program; **✔ Check** also verifies that it does what the exercise asks.

@@text
## Your first exercise

Every lesson ends with one or more exercises. Look at the editor on the right: there's already a program in it. Your job is to change the text between the quotes so the program prints exactly:

```text
I am learning Rust!
```

Then press **✔ Check**. When it passes, press **Mark complete & continue** at the bottom of this lesson.

@@exercise Your first change
Change the program so it prints exactly:

```text
I am learning Rust!
```

Only change the text **inside the quotes** `"..."`. Keep everything else the same. Capital letters and the `!` matter!

@@starter
fn main() {
    println!("Change me!");
}
@@expected
I am learning Rust!
@@hint
Replace the words `Change me!` with `I am learning Rust!` — keep the quote marks `"` around the text.
@@solution
fn main() {
    println!("I am learning Rust!");
}
