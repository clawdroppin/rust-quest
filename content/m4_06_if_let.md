@@lesson m4-if-let if let, let else and while let
@@text
`match` is powerful, but when you only care about **one** pattern, it's wordy. Rust has three shorter forms.

## `if let` — run code if one pattern matches

```rust
fn main() {
    let config_volume: Option<u32> = Some(70);

    // With match:
    match config_volume {
        Some(v) => println!("Volume set to {v}"),
        None => {}
    }

    // With if let — same thing, shorter:
    if let Some(v) = config_volume {
        println!("Volume set to {v}");
    }
}
```

Read `if let Some(v) = config_volume` as: *"if `config_volume` matches the pattern `Some(v)`, then run this block, with `v` being the value inside."* It can have an `else`:

```rust
#[derive(Debug)]
enum Event {
    KeyPress(char),
    Click { x: i32, y: i32 },
    Quit,
}

fn main() {
    let e = Event::Click { x: 10, y: 20 };
    if let Event::Click { x, y } = e {
        println!("clicked at {x},{y}");
    } else {
        println!("something else happened");
    }
}
```

## `let else` — get the value or leave

Often you need a value to continue, and if it isn't there, you want to **exit early**. `let ... else` does exactly that:

```rust
fn describe_age(text: &str) -> String {
    let Ok(age) = text.trim().parse::<u32>() else {
        return String::from("that's not a valid age");
    };
    // from here on, `age` is a plain u32
    if age >= 18 { format!("{age}: adult") } else { format!("{age}: minor") }
}

fn main() {
    println!("{}", describe_age("42"));
    println!("{}", describe_age("abc"));
}
```

- If the pattern matches, the variables (here `age`) are available **after** the statement.
- If not, the `else` block runs, and it **must** leave: `return`, `break`, `continue`, or panic.

This keeps the "happy path" un-indented and handles problems at the top. (`Ok(...)` is from `Result`, which `parse` returns — next module!)

## `while let` — loop while a pattern matches

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
    println!("stack is empty");
}
```

`pop()` returns `Some(value)` until the vector is empty, then `None`, which ends the loop.

## Which one should I use?

| Situation | Use |
|---|---|
| Several patterns/variants to handle | `match` |
| Only one pattern matters (maybe with an `else`) | `if let` |
| Need the value to continue, else bail out | `let ... else` |
| Repeat while a pattern keeps matching | `while let` |
| Just need true/false | `matches!(value, pattern)` |

@@quiz
In `let Some(x) = maybe else { ... };`, what must the `else` block do?
- Nothing special
* Leave the current function/loop (e.g. `return`, `break`, `continue`) or panic
- Assign a value to `x`
? After `let else`, the code continues only if `x` exists, so the `else` must not fall through.

@@exercise Sum the valid numbers
A list of text values comes from a messy form. Write `sum_valid`, which adds up all the entries that are valid whole numbers, and ignores the rest. Use `if let` (or `let else` with `continue`).

`["10", "abc", "5", "", "-3"]` → `12`

`text.trim().parse::<i32>()` returns `Ok(number)` or `Err(...)`.
@@starter
fn sum_valid(entries: &[&str]) -> i32 {
    todo!()
}

fn main() {
    println!("{}", sum_valid(&["10", "abc", "5", "", "-3"]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(sum_valid(&["10", "abc", "5", "", "-3"]), 12);
    assert_eq!(sum_valid(&[" 7 ", "x"]), 7, "spaces around numbers are okay");
    assert_eq!(sum_valid(&[]), 0);
}
@@hint
`for e in entries { if let Ok(n) = e.trim().parse::<i32>() { total += n; } }`
@@solution
fn sum_valid(entries: &[&str]) -> i32 {
    let mut total = 0;
    for e in entries {
        if let Ok(n) = e.trim().parse::<i32>() {
            total += n;
        }
    }
    total
}

fn main() {
    println!("{}", sum_valid(&["10", "abc", "5", "", "-3"]));
}

@@exercise Process the queue
A print queue is a `Vec<Job>` used as a stack (the last job added is printed first). Write `process`, which uses `while let` with `pop()` to print every job, **skipping** jobs with 0 pages, and returns how many pages were printed in total.

Each printed job outputs a line like `printing report.pdf (3 pages)`. For this queue the output should be:

```text
printing photo.png (1 pages)
printing report.pdf (3 pages)
total: 4
```
@@starter
struct Job {
    name: String,
    pages: u32,
}

fn process(mut queue: Vec<Job>) -> u32 {
    todo!()
}

fn main() {
    let queue = vec![
        Job { name: String::from("report.pdf"), pages: 3 },
        Job { name: String::from("empty.txt"), pages: 0 },
        Job { name: String::from("photo.png"), pages: 1 },
    ];
    let total = process(queue);
    println!("total: {total}");
}
@@expected
printing photo.png (1 pages)
printing report.pdf (3 pages)
total: 4
@@requires
while let
@@hint
`while let Some(job) = queue.pop() { if job.pages == 0 { continue; } println!(...); total += job.pages; }`
@@solution
struct Job {
    name: String,
    pages: u32,
}

fn process(mut queue: Vec<Job>) -> u32 {
    let mut total = 0;
    while let Some(job) = queue.pop() {
        if job.pages == 0 {
            continue;
        }
        println!("printing {} ({} pages)", job.name, job.pages);
        total += job.pages;
    }
    total
}

fn main() {
    let queue = vec![
        Job { name: String::from("report.pdf"), pages: 3 },
        Job { name: String::from("empty.txt"), pages: 0 },
        Job { name: String::from("photo.png"), pages: 1 },
    ];
    let total = process(queue);
    println!("total: {total}");
}

@@exercise Parse a command
Write `parse_move`, which reads commands like `"move 3 -2"` and returns the two numbers as `Some((3, -2))`. Return `None` if the text doesn't start with `move`, doesn't have exactly two numbers after it, or if they aren't valid numbers.

Use `let ... else` to bail out early at each step.
@@starter
fn parse_move(command: &str) -> Option<(i32, i32)> {
    todo!()
}

fn main() {
    println!("{:?}", parse_move("move 3 -2"));
    println!("{:?}", parse_move("jump 1 1"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(parse_move("move 3 -2"), Some((3, -2)));
    assert_eq!(parse_move("move   10 0"), Some((10, 0)), "extra spaces are fine");
    assert_eq!(parse_move("jump 1 1"), None);
    assert_eq!(parse_move("move 1"), None, "only one number");
    assert_eq!(parse_move("move 1 2 3"), None, "too many numbers");
    assert_eq!(parse_move("move a b"), None);
    assert_eq!(parse_move(""), None);
}
@@requires
else {
@@hint
Collect `command.split_whitespace()` into a `Vec<&str>`. Then: `let ["move", a, b] = parts.as_slice() else { return None; };` — yes, you can match a slice pattern with exactly three elements! Then `let Ok(x) = a.parse::<i32>() else { return None; };` and the same for `y`.
@@solution
fn parse_move(command: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let ["move", a, b] = parts.as_slice() else {
        return None;
    };
    let Ok(x) = a.parse::<i32>() else {
        return None;
    };
    let Ok(y) = b.parse::<i32>() else {
        return None;
    };
    Some((x, y))
}

fn main() {
    println!("{:?}", parse_move("move 3 -2"));
    println!("{:?}", parse_move("jump 1 1"));
}
