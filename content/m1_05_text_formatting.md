@@lesson m1-text Text: Strings and Formatting
@@text
Text in programming is called a **string** (a "string of characters"). Rust has two main string types, and the difference matters a lot later. For now, here's the beginner-friendly version:

| Type | What it is | Example |
|---|---|---|
| `&str` | a fixed piece of text you can read but not grow. String literals are `&str`. | `"hello"` |
| `String` | text you **own** and can change and grow | `String::from("hello")` |

```rust
fn main() {
    let greeting: &str = "Hello";            // a string literal
    let mut name = String::from("Fer");      // an owned String
    name.push_str("ris");                    // add text to the end
    name.push('!');                          // add a single char
    println!("{greeting}, {name}");
}
```

Ways to get a `String`:

- `String::from("text")`
- `"text".to_string()`
- `String::new()` — an empty string, ready to fill
- `format!(...)` — see below

> Why two types? It's to do with **ownership** and memory — one of Rust's big ideas, with a whole module later. For now: use `&str` for fixed text, `String` when you need to build or change text.

## Useful string methods

```rust
fn main() {
    let s = "  Hello, World!  ";
    let t = s.trim();                        // remove spaces at both ends
    println!("[{t}]");
    println!("{}", t.len());                 // length in bytes
    println!("{}", t.to_uppercase());
    println!("{}", t.to_lowercase());
    println!("{}", t.contains("World"));     // true
    println!("{}", t.starts_with("Hell"));   // true
    println!("{}", t.replace("World", "Rust"));
    println!("{}", t.is_empty());            // false
}
```

> `.len()` counts **bytes**, not characters. For plain English letters each character is one byte, so it's the same. But `"é"` is 2 bytes and `"🦀"` is 4! To count characters, use `.chars().count()`.

## `format!` — build a String with placeholders

`format!` works exactly like `println!`, but instead of printing, it gives you the result as a `String`:

```rust
fn main() {
    let name = "Ferris";
    let level = 12;
    let status = format!("{name} (level {level})");
    println!("Status: {status}");
    println!("Length: {}", status.len());
}
```

You *can* join strings with `+`, but it has some odd rules you'll understand after the ownership module. `format!` is simpler and clearer — prefer it.

@@quiz
What does `format!` do?
- It prints text in a nicer font
* It builds a `String` using placeholders, without printing it
- It converts a number to a `char`
? `format!` is like `println!` but gives you the text as a `String` value instead of printing it.

@@text
## Formatting in detail

The `{}` placeholder can contain extra instructions after a colon `:`. This lets you produce neat, aligned output.

**Debug format `{:?}`** — shows a value the way a programmer would want to see it. Strings get quotes, which makes spaces visible:

```rust
fn main() {
    let word = " hi ";
    println!("Display: {}", word);
    println!("Debug:   {:?}", word);
}
```

**Decimal places `{:.N}`:**

```rust
fn main() {
    let price = 3.14159;
    println!("{:.2}", price);    // 3.14
    println!("{:.0}", price);    // 3
    println!("{price:.3}");      // 3.142 — works with names too
}
```

**Width and alignment** — `{:10}` makes the value take up at least 10 characters. `<` aligns left, `>` right, `^` centre:

```rust
fn main() {
    println!("[{:<8}]", "left");
    println!("[{:>8}]", "right");
    println!("[{:^8}]", "mid");
    println!("[{:*^12}]", "fancy");   // fill with * instead of spaces
    println!("[{:05}]", 42);          // pad numbers with zeros
    println!("[{:>8.2}]", 2.5);       // width 8 AND 2 decimals
}
```

Alignment is how you make tables:

```rust
fn main() {
    println!("{:<10}{:>6}", "Player", "Score");
    println!("{:<10}{:>6}", "Ferris", 1200);
    println!("{:<10}{:>6}", "Corro", 950);
}
```

**Printing braces themselves:** write them twice: `{{` and `}}`.

```rust
fn main() {
    println!("Use {{}} as a placeholder");
}
```

**Errors go to `eprintln!`:** it works like `println!` but writes to the *error output*, which programs use for error messages. In this app, it shows up in red.

@@quiz
What does `println!("[{:>5}]", 42);` print?
- `[42   ]`
* `[   42]`
- `[00042]`
? `>` means right-aligned in a space 5 characters wide: three spaces, then `42`.

@@exercise Café receipt
Print this receipt **exactly**, using width, alignment and `.2` precision formatting (not by typing the spaces yourself):

```text
Item        Price
Coffee       3.50
Croissant    2.25
-----------------
Total        5.75
```

Each line is a name **left-aligned in 10 characters** followed by a value **right-aligned in 7 characters**. The dashed line is 17 dashes. Calculate the total from the two prices.
@@starter
fn main() {
    let coffee = 3.5;
    let croissant = 2.25;

    println!("Item Price");
    println!("Coffee {coffee}");
    println!("Croissant {croissant}");
}
@@expected
Item        Price
Coffee       3.50
Croissant    2.25
-----------------
Total        5.75
@@requires
:<10
:>7
.2
@@hint
For the header: `println!("{:<10}{:>7}", "Item", "Price");`
For a row: `println!("{:<10}{:>7.2}", "Coffee", coffee);`
@@solution
fn main() {
    let coffee = 3.5;
    let croissant = 2.25;
    let total = coffee + croissant;

    println!("{:<10}{:>7}", "Item", "Price");
    println!("{:<10}{:>7.2}", "Coffee", coffee);
    println!("{:<10}{:>7.2}", "Croissant", croissant);
    println!("-----------------");
    println!("{:<10}{:>7.2}", "Total", total);
}

@@exercise String tools
Starting from `let name = "ada lovelace";`, use **string methods** to print:

```text
Original: ada lovelace
Shout: ADA LOVELACE
Length: 12
Has 'love': true
Married name: ada byron
```
@@starter
fn main() {
    let name = "ada lovelace";
    println!("Original: {name}");
}
@@expected
Original: ada lovelace
Shout: ADA LOVELACE
Length: 12
Has 'love': true
Married name: ada byron
@@requires
to_uppercase
len()
contains
replace
@@hint
`name.replace("lovelace", "byron")` returns a new String with the replacement.
@@solution
fn main() {
    let name = "ada lovelace";
    println!("Original: {name}");
    println!("Shout: {}", name.to_uppercase());
    println!("Length: {}", name.len());
    println!("Has 'love': {}", name.contains("love"));
    println!("Married name: {}", name.replace("lovelace", "byron"));
}

@@exercise Build a message
Build a `String` called `message` step by step:

1. Use `format!` to create `"Hello, Ferris!"` from the `name` variable.
2. Use `push_str` to add `" Welcome to Rust Quest."` to the end.

Then print it:

```text
Hello, Ferris! Welcome to Rust Quest.
```
@@starter
fn main() {
    let name = "Ferris";
    let message = String::new();

    println!("{message}");
}
@@expected
Hello, Ferris! Welcome to Rust Quest.
@@requires
format!
push_str
@@hint
`let mut message = format!("Hello, {name}!");` — it needs `mut` because `push_str` changes it.
@@solution
fn main() {
    let name = "Ferris";
    let mut message = format!("Hello, {name}!");
    message.push_str(" Welcome to Rust Quest.");
    println!("{message}");
}
