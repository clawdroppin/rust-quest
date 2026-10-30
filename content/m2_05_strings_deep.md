@@lesson m2-strings Strings in Depth
@@text
Text is everywhere in programs: names, messages, commands, files, data from the internet. A string is really a **collection of characters**, so many list ideas apply — with a few twists.

## Going through characters

`.chars()` gives you the characters one by one:

```rust
fn main() {
    let word = "Rust";
    for c in word.chars() {
        println!("{c}");
    }
    println!("{} characters", word.chars().count());
}
```

## Why `text[0]` doesn't work

Try this:

```rust
fn main() {
    let s = String::from("hello");
    let first = s[0];
    println!("{first}");
}
```

Error! Rust doesn't let you index a string by position. Here's why: strings are stored as **UTF-8**, where one character can take **1 to 4 bytes** (`'a'` takes 1, `'é'` takes 2, `'🦀'` takes 4). So "the 5th character" isn't at a simple position — you'd have to count from the start. Rust refuses to pretend that's cheap or simple.

Instead:

```rust
fn main() {
    let s = "héllo";
    let first = s.chars().next().unwrap();       // first character
    let third = s.chars().nth(2).unwrap();       // character at position 2
    println!("{first} {third}");
    println!("bytes: {}, chars: {}", s.len(), s.chars().count());
}
```

You *can* take a slice of bytes with a range, `&s[0..3]`, but it panics if the range cuts a character in half. It's safe when you know the text is plain English letters.

## Building strings piece by piece

```rust
fn main() {
    let word = "stressed";
    let mut reversed = String::new();
    for c in word.chars().rev() {
        reversed.push(c);
    }
    println!("{reversed}");

    // shortcut: collect the characters into a String
    let again: String = word.chars().rev().collect();
    println!("{again}");
}
```

`.collect()` gathers items into a collection. It can build different collections, so you tell it which one you want with a type annotation (`: String` or `: Vec<...>`).

## Splitting text into pieces

This is how programs read structured text like commands or data files:

```rust
fn main() {
    let sentence = "the quick  brown fox";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    let csv = "Ana,31,Prague";
    let fields: Vec<&str> = csv.split(',').collect();
    println!("name={} age={} city={}", fields[0], fields[1], fields[2]);

    let text = "line one\nline two\nline three";
    for (i, line) in text.lines().enumerate() {
        println!("{}: {line}", i + 1);
    }
}
```

- `split_whitespace()` splits on any amount of spaces, tabs or newlines.
- `split(x)` splits on exactly `x` (a `char` or a `&str`).
- `lines()` splits into lines.

## Turning text into numbers: `parse`

Text like `"42"` is not a number — it's two characters. To convert, use `parse`:

```rust
fn main() {
    let text = "42";
    let n: i32 = text.parse().unwrap();
    println!("{}", n + 1);

    let price: f64 = "3.99".trim().parse().unwrap();
    println!("{}", price * 2.0);
}
```

Parsing can fail — what number is `"hello"`? So `parse` returns a `Result`, which is like `Option` but carries an *error* instead of `None`. For now we use `unwrap()`, which panics if parsing fails. Module 5 shows how to handle bad input gracefully.

## Character tests and conversions

```rust
fn main() {
    for c in "Hi 5!".chars() {
        println!(
            "{c:?}: letter={} digit={} space={} upper={}",
            c.is_alphabetic(),
            c.is_ascii_digit(),
            c.is_whitespace(),
            c.to_ascii_uppercase()
        );
    }
}
```

## Comparing strings

`==` compares text exactly (case matters). To ignore case, compare lowercase versions:

```rust
fn main() {
    let input = "YES";
    println!("{}", input == "yes");
    println!("{}", input.to_lowercase() == "yes");
    println!("{}", input.eq_ignore_ascii_case("yes"));
}
```

@@quiz
Why can't you write `my_string[0]` in Rust?
- Strings are always empty
* Characters can take different numbers of bytes, so a position doesn't map simply to a character
- Indexes start at 1 for strings
? Rust strings are UTF-8. Use `.chars().nth(i)` to get the i-th character.

@@quiz
What does `"a,b,,c".split(',').count()` return?
- `3`
* `4`
- `5`
? Splitting on `,` gives `"a"`, `"b"`, `""` (the empty piece between the two commas) and `"c"`.

@@exercise Count the vowels
Write `count_vowels`, which counts the vowels (a, e, i, o, u) in some text, **ignoring case**.
@@starter
fn count_vowels(text: &str) -> usize {
    todo!()
}

fn main() {
    println!("{}", count_vowels("Hello World"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(count_vowels("Hello World"), 3);
    assert_eq!(count_vowels("AEIOU aeiou"), 10, "uppercase vowels count too");
    assert_eq!(count_vowels("rhythm"), 0);
    assert_eq!(count_vowels(""), 0);
}
@@hint
Loop over `text.to_lowercase().chars()`. A vowel check: `"aeiou".contains(c)`.
@@solution
fn count_vowels(text: &str) -> usize {
    let mut count = 0;
    for c in text.to_lowercase().chars() {
        if "aeiou".contains(c) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_vowels("Hello World"));
}

@@exercise Palindromes
A **palindrome** reads the same forwards and backwards, like "racecar". Write `is_palindrome`, which **ignores case, spaces and punctuation** — so "A man, a plan, a canal: Panama" counts!

Plan: first build a cleaned-up `String` that has only the lowercase letters (and digits), then compare it with its reverse.
@@starter
fn is_palindrome(text: &str) -> bool {
    todo!()
}

fn main() {
    println!("{}", is_palindrome("A man, a plan, a canal: Panama"));
}
@@tests
#[test]
fn checks() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(!is_palindrome("Rust"), "Rust is not a palindrome");
    assert!(is_palindrome(""), "empty text counts as a palindrome");
}
@@hint
Clean: loop over `text.chars()`, and if `c.is_alphanumeric()`, push `c.to_ascii_lowercase()` into a `String`. Reverse: `cleaned.chars().rev().collect::<String>()`.
@@solution
fn is_palindrome(text: &str) -> bool {
    let mut cleaned = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            cleaned.push(c.to_ascii_lowercase());
        }
    }
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
}

fn main() {
    println!("{}", is_palindrome("A man, a plan, a canal: Panama"));
}

@@exercise Add up a receipt
A receipt is a piece of text with one item per line, written as `name: price`. Write `receipt_total`, which adds up all the prices. Ignore empty lines. Extra spaces around the prices may appear.

```text
coffee: 3.50
bagel: 2.25

juice:   4.00
```
→ `9.75`
@@starter
fn receipt_total(receipt: &str) -> f64 {
    todo!()
}

fn main() {
    let receipt = "coffee: 3.50\nbagel: 2.25\n\njuice:   4.00";
    println!("{:.2}", receipt_total(receipt));
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    assert!(close(receipt_total("coffee: 3.50\nbagel: 2.25\n\njuice:   4.00"), 9.75));
    assert!(close(receipt_total("tea: 1.5"), 1.5));
    assert!(close(receipt_total(""), 0.0), "an empty receipt costs 0");
}
@@hint
For each line in `receipt.lines()`: skip it if `line.trim().is_empty()`. Otherwise split on `':'`, collect into a `Vec<&str>`, and parse `parts[1].trim()` as an `f64`.
@@solution
fn receipt_total(receipt: &str) -> f64 {
    let mut total = 0.0;
    for line in receipt.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let price: f64 = parts[1].trim().parse().unwrap();
        total += price;
    }
    total
}

fn main() {
    let receipt = "coffee: 3.50\nbagel: 2.25\n\njuice:   4.00";
    println!("{:.2}", receipt_total(receipt));
}

@@exercise Title case
Write `title_case`, which capitalizes the first letter of every word and makes the rest lowercase. Words are separated by single spaces.

`"the lord of the RINGS"` → `"The Lord Of The Rings"`
@@starter
fn title_case(text: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", title_case("the lord of the RINGS"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(title_case("the lord of the RINGS"), "The Lord Of The Rings");
    assert_eq!(title_case("rust"), "Rust");
    assert_eq!(title_case("a b c"), "A B C");
}
@@hint
Collect the result words into a `Vec<String>` and `join(" ")` them at the end. For one word: take the first char with `word.chars().next()`, uppercase it, then add the rest of the word lowercased. The rest is `&word[1..]` (fine here because the words are plain English letters).
@@solution
fn title_case(text: &str) -> String {
    let mut words = Vec::new();
    for word in text.split(' ') {
        let mut fixed = String::new();
        if let Some(first) = word.chars().next() {
            fixed.push(first.to_ascii_uppercase());
            fixed.push_str(&word[1..].to_lowercase());
        }
        words.push(fixed);
    }
    words.join(" ")
}

fn main() {
    println!("{}", title_case("the lord of the RINGS"));
}
