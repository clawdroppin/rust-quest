@@lesson m5-input Reading User Input
@@text
Until now, all data was written directly into the code. Real programs **interact**: they ask questions and react to answers. This lesson shows how to read text the user types.

## Standard input

Every program has a **standard input** (stdin) — usually the keyboard in a terminal — and a **standard output** (stdout), where `println!` writes. To read a line from stdin:

```rust
use std::io;

fn main() {
    println!("What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");
    println!("Hello, {}!", name.trim());
}
```

Step by step:

1. `use std::io;` brings in the input/output module.
2. `String::new()` creates an empty, growable `String` to hold the input.
3. `io::stdin().read_line(&mut name)` waits for the user to type a line and press Enter, then **appends** it to `name` — that's why it needs `&mut`.
4. `read_line` returns a `Result` (reading can fail, e.g. if input is closed), so we handle it — here with `expect`.
5. The text includes the **newline** from pressing Enter, so we `trim()` it.

## ⌨ Input in this app

When you run a program here, it can't pause for you to type. Instead, open **⌨ Program input (stdin)** under the editor and type the input **in advance**, one answer per line. When the program reads a line, it gets the next one.

For exercises, the input is already filled in for you. Try the example above with ▶ Try it: type a name into the input box and run it again.

To run a program **interactively** in a real terminal, you'll use Cargo projects, starting next module.

## Reading numbers

Input is always text. To get a number, `parse` it — and since the user might type anything, handle the error:

```rust
use std::io;

fn main() {
    println!("How old are you?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");

    match input.trim().parse::<u32>() {
        Ok(age) => println!("Next year you'll be {}", age + 1),
        Err(_) => println!("'{}' isn't a valid age.", input.trim()),
    }
}
```

## A reusable helper

Reading a line is so common that it's worth a small function:

```rust
use std::io;

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read input");
    s.trim().to_string()
}

fn main() {
    println!("First number:");
    let a: f64 = read_line().parse().unwrap_or(0.0);
    println!("Second number:");
    let b: f64 = read_line().parse().unwrap_or(0.0);
    println!("{a} + {b} = {}", a + b);
}
```

## Asking again until the input is valid

The classic pattern for robust input: **loop until you get something valid**.

```rust
use std::io;

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("failed to read input");
    s.trim().to_string()
}

fn ask_number(question: &str) -> i32 {
    loop {
        println!("{question}");
        let text = read_line();
        match text.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("  '{text}' is not a whole number, try again."),
        }
    }
}

fn main() {
    let n = ask_number("Pick a number:");
    println!("You picked {n}");
}
```

Put `abc`, `4.5` and `7` on three lines in the input box and run it: it rejects the first two and accepts `7`.

> ⚠ When input runs out (the end of the input box, or Ctrl+Z in a Windows terminal), `read_line` returns `Ok(0)` and leaves the string empty. A loop that keeps asking would then ask forever — this app stops it after 5 seconds. Real programs check for this: `if io::stdin().read_line(&mut s)? == 0 { /* no more input */ }`.

## Reading all lines

To process input line by line until it ends:

```rust
use std::io;

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.expect("failed to read line");
        if let Ok(n) = line.trim().parse::<i32>() {
            total += n;
        }
    }
    println!("Sum of all numbers: {total}");
}
```

## Prompts on the same line

`println!` adds a newline. To keep the cursor right after the question (`Name: _`), use `print!` — but output is **buffered** (collected and sent in chunks), so you must **flush** it to make it appear before waiting for input:

```rust
use std::io::{self, Write};

fn main() {
    print!("Name: ");
    io::stdout().flush().expect("flush failed");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");
    println!("Hi {}", name.trim());
}
```

@@quiz
Why do we call `.trim()` on input from `read_line`?
- To make it uppercase
* To remove the newline (and any spaces) that come with the typed line
- Because `read_line` returns a number
? The Enter key adds a newline at the end of the input.

@@exercise Greeting with input
Write a program that reads a **name** and a **birth year** (one per line) and prints a greeting with their age **in 2026**. The input box already contains:

```text
Ada
1990
```

Expected output:

```text
Name?
Birth year?
Hi Ada, you turn 36 in 2026.
```
@@starter
use std::io;

fn main() {
    println!("Name?");
}
@@stdin
Ada
1990
@@expected
Name?
Birth year?
Hi Ada, you turn 36 in 2026.
@@requires
read_line
@@hint
Read each line into its own `String` with `io::stdin().read_line(&mut ...)`. Parse the year with `.trim().parse::<i32>().expect("not a year")`.
@@solution
use std::io;

fn main() {
    println!("Name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");

    println!("Birth year?");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("failed to read input");
    let year: i32 = year.trim().parse().expect("not a year");

    println!("Hi {}, you turn {} in 2026.", name.trim(), 2026 - year);
}

@@exercise Keep asking
Write a program that asks for a number **between 1 and 10**, and keeps asking until it gets one. Invalid text and numbers out of range get different messages. The input is:

```text
seven
42
7
```

Expected output:

```text
Enter a number from 1 to 10:
Not a number: seven
Enter a number from 1 to 10:
Out of range: 42
Enter a number from 1 to 10:
You chose 7
```
@@starter
use std::io;

fn main() {

}
@@stdin
seven
42
7
@@expected
Enter a number from 1 to 10:
Not a number: seven
Enter a number from 1 to 10:
Out of range: 42
Enter a number from 1 to 10:
You chose 7
@@requires
loop
@@hint
Inside a `loop`: print the question, read a line, trim it, then `match text.parse::<i32>()`: on `Err` print "Not a number" and `continue`; on `Ok(n)` check the range with `(1..=10).contains(&n)`; if it's fine, print and `break`.
@@solution
use std::io;

fn main() {
    loop {
        println!("Enter a number from 1 to 10:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let text = input.trim();

        let n = match text.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Not a number: {text}");
                continue;
            }
        };
        if !(1..=10).contains(&n) {
            println!("Out of range: {n}");
            continue;
        }
        println!("You chose {n}");
        break;
    }
}

@@exercise Number statistics
Read numbers, one per line, until the input ends. Skip blank lines and lines that aren't numbers (count them as "skipped"). Then print the statistics. The input is:

```text
4
8

banana
15
16
```

Expected output:

```text
count: 4
sum: 43
min: 4
max: 16
average: 10.75
skipped: 1
```
@@starter
use std::io;

fn main() {

}
@@stdin
4
8

banana
15
16
@@expected
count: 4
sum: 43
min: 4
max: 16
average: 10.75
skipped: 1
@@requires
stdin().lines()
@@hint
Collect valid numbers into a `Vec<i64>` while looping over `io::stdin().lines()`. Use `continue` for blank lines. Afterwards use `.iter().sum()`, `.iter().min()` and `.iter().max()`.
@@solution
use std::io;

fn main() {
    let mut numbers: Vec<i64> = Vec::new();
    let mut skipped = 0;

    for line in io::stdin().lines() {
        let line = line.expect("failed to read line");
        let text = line.trim();
        if text.is_empty() {
            continue;
        }
        match text.parse::<i64>() {
            Ok(n) => numbers.push(n),
            Err(_) => skipped += 1,
        }
    }

    let sum: i64 = numbers.iter().sum();
    println!("count: {}", numbers.len());
    println!("sum: {sum}");
    println!("min: {}", numbers.iter().min().unwrap());
    println!("max: {}", numbers.iter().max().unwrap());
    println!("average: {}", sum as f64 / numbers.len() as f64);
    println!("skipped: {skipped}");
}
