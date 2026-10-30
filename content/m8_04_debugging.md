@@lesson m8-debugging Debugging
@@text
Every programmer writes bugs, every day. What separates beginners from experienced developers isn't fewer bugs; it's a calm, systematic way of **finding** them. Rust's compiler already catches whole categories of bugs (null pointers, data races, use-after-free). What's left are **logic bugs**: the program runs, but does the wrong thing.

## Step 1: reproduce it

Find the exact input that causes the problem, and make it happen reliably. Even better, **write a failing test** for it:

```rust,norun
#[test]
fn average_of_two_and_three_is_two_and_a_half() {
    assert_eq!(average(&[2, 3]), 2.5);   // fails: returns 2.0
}
```

Now you have a precise, repeatable signal: when this test passes, the bug is fixed. Keep the test, and the bug can never silently come back.

## Step 2: look at the values — `dbg!`

Most bugs are a value that isn't what you think it is. `dbg!` prints an expression, its value, and the file and line, then **returns the value**, so you can wrap it around anything:

```rust
fn average(numbers: &[i32]) -> f64 {
    let total: i32 = numbers.iter().sum();
    let avg = dbg!(total / numbers.len() as i32) as f64;
    avg
}

fn main() {
    println!("{}", average(&[2, 3]));
}
```

Look at the output: `[main.rs:3:15] total / numbers.len() as i32 = 2`. The division happens on **integers** before converting to `f64`. The fix: `total as f64 / numbers.len() as f64`.

`dbg!` prints to **stderr** (the error output), so it doesn't mix with normal output. Remove it when you're done. Clippy can warn you about leftover `dbg!`s.

## Step 3: the debugger in VS Code

A **debugger** pauses your program while it runs and lets you look inside: every variable's value, step by step. For Rust in VS Code:

1. Install the **CodeLLDB** extension (Extensions tab, search "CodeLLDB").
2. Open a Cargo project. Click in the margin left of a line number to set a **breakpoint** (a red dot).
3. Above `fn main()`, rust-analyzer shows **▶ Run | Debug**. Click **Debug**. (Or press `F5` and let VS Code generate a `launch.json` for Cargo.)
4. The program stops at your breakpoint. The left panel shows all **variables**. Hover over any variable in the code to see its value.
5. Control execution with the toolbar:
   - **Continue** (`F5`): run to the next breakpoint
   - **Step Over** (`F10`): run the current line, stop at the next one
   - **Step Into** (`F11`): go *inside* the function called on this line
   - **Step Out** (`Shift+F11`): finish the current function and stop in its caller
6. Add expressions to the **Watch** panel to keep an eye on them.

Stepping through a loop line by line, watching a variable change, is the single best way to understand what your code *really* does, as opposed to what you think it does. Try it on the Hangman project: put a breakpoint in `guess` and watch `self.guessed` grow.

## Reading a panic

When a program panics, read the message carefully. It tells you **what** and **where**:

```text
thread 'main' panicked at src/game.rs:42:17:
index out of bounds: the len is 3 but the index is 3
```

To see the chain of function calls that led there, run with the `RUST_BACKTRACE` environment variable. In VS Code's PowerShell terminal:

```text
$env:RUST_BACKTRACE=1; cargo run
```

The **backtrace** lists the calls from the innermost outwards. Look for the first lines that mention *your* files; the lines in `std` or `core` are the standard library's internals.

## Strategies when you're stuck

- **Check your assumptions.** "This variable is definitely positive here." Is it? Print it.
- **Shrink the problem.** Remove code (or comment it out) until the bug disappears, then look at the last thing you removed.
- **Binary search.** If a long calculation goes wrong somewhere, print the value halfway. Wrong already? The bug is in the first half. Repeat.
- **Rubber duck.** Explain the code, line by line, out loud, to someone (or something) that knows nothing about it. You'll often hear yourself say the bug.
- **Read the error message again.** Slowly. All of it. Rust's messages usually contain the answer.
- **Take a break.** Seriously. Fresh eyes find bugs in minutes that tired eyes miss for hours.

## Common logic bugs to look for

- **Off-by-one:** `<` vs `<=`, `0..n` vs `0..=n`, `len()` vs `len() - 1`.
- **Integer division:** `7 / 2 == 3`. Convert to `f64` *before* dividing.
- **Wrong variable:** using `i` where you meant `j`, or `a.x` where you meant `b.x`.
- **Forgotten update:** a loop variable that never changes, a counter never reset.
- **Order of operations:** `a + b / 2` is `a + (b / 2)`.
- **Edge cases:** empty lists, zero, one element, the last element, negative numbers.

The exercises below each contain a bug. The tests describe the correct behaviour. Find the bug, then fix it with the smallest possible change. Use `dbg!`, and use **▶ Run** to experiment.

@@quiz
What does `dbg!(x * 2)` return?
- Nothing; it only prints
* The value of `x * 2`, after printing it with the file and line
- A string with the debug output
? `dbg!` passes the value through, so you can wrap it around any expression.

@@quiz
A loop over a list never processes the last item. What kind of bug is it most likely to be?
- A type error
* An off-by-one error, like `0..len - 1` instead of `0..len`
- A memory leak
? Boundaries are the most common place for bugs. Check your ranges and comparisons.

@@exercise Find the bug: the average
`average` should return the average of the numbers, as an `f64`, or `0.0` for an empty list. One test fails. Find out why and fix the bug.
@@starter
fn average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let total: i32 = numbers.iter().sum();
    (total / numbers.len() as i32) as f64
}

fn main() {
    println!("{}", average(&[2, 3]));
    println!("{}", average(&[1, 2, 3, 4]));
    println!("{}", average(&[]));
}
@@expected
2.5
2.5
0
@@tests
#[test]
fn checks() {
    assert_eq!(average(&[4, 4]), 4.0);
    assert_eq!(average(&[1, 2]), 1.5);
    assert_eq!(average(&[-1, -2]), -1.5);
    assert_eq!(average(&[]), 0.0);
}
@@hint
Put `dbg!(...)` around `total / numbers.len() as i32`. What type is that division done in?
@@solution
fn average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let total: i32 = numbers.iter().sum();
    total as f64 / numbers.len() as f64
}

fn main() {
    println!("{}", average(&[2, 3]));
    println!("{}", average(&[1, 2, 3, 4]));
    println!("{}", average(&[]));
}

@@exercise Find the bug: longest streak
`longest_streak` should return the length of the longest run of **equal values in a row**. For `[1, 1, 2, 2, 2, 1]` that's 3 (the three 2s). An empty list has a streak of 0.

The function is almost right, but some answers are wrong. Find the bug(s) and fix them.
@@starter
fn longest_streak(values: &[i32]) -> usize {
    let mut best = 0;
    let mut current = 1;
    for i in 1..values.len() {
        if values[i] == values[i - 1] {
            current += 1;
        } else {
            if current > best {
                best = current;
            }
            current = 1;
        }
    }
    best
}

fn main() {
    println!("{}", longest_streak(&[1, 1, 2, 2, 2, 1]));
    println!("{}", longest_streak(&[5, 7, 7, 7, 7]));
    println!("{}", longest_streak(&[9]));
    println!("{}", longest_streak(&[]));
}
@@expected
3
4
1
0
@@tests
#[test]
fn checks() {
    assert_eq!(longest_streak(&[1, 2, 3]), 1);
    assert_eq!(longest_streak(&[4, 4]), 2);
    assert_eq!(longest_streak(&[3, 3, 1, 1, 1]), 3);
    assert_eq!(longest_streak(&[]), 0);
    assert_eq!(longest_streak(&[0]), 1);
}
@@hint
Try `[5, 7, 7, 7, 7]` and `[9]` and add `dbg!(best, current)` after the loop. When the longest streak is at the **end** of the list, is `best` ever updated? Also: what should happen for an empty list?
@@solution
fn longest_streak(values: &[i32]) -> usize {
    if values.is_empty() {
        return 0;
    }
    let mut best = 0;
    let mut current = 1;
    for i in 1..values.len() {
        if values[i] == values[i - 1] {
            current += 1;
        } else {
            if current > best {
                best = current;
            }
            current = 1;
        }
    }
    if current > best {
        best = current;
    }
    best
}

fn main() {
    println!("{}", longest_streak(&[1, 1, 2, 2, 2, 1]));
    println!("{}", longest_streak(&[5, 7, 7, 7, 7]));
    println!("{}", longest_streak(&[9]));
    println!("{}", longest_streak(&[]));
}
