@@lesson m2-hashmap HashMap: Looking Things Up by Name
@@text
Lists find things by **position**: element 0, element 1… But often you want to find things by a **name** or **key**: a phone number by person, a price by product, a score by player. For that we use a **map** (also called a dictionary). Rust's main one is the `HashMap`.

A map stores **key → value** pairs. Each key appears **at most once**.

```text
  "Ana"   →  "555-1234"
  "Ben"   →  "555-9876"
  "Cleo"  →  "555-0000"
```

## Creating and filling a HashMap

`HashMap` isn't loaded automatically like `Vec`, so you **import** it with `use` at the top of the file:

```rust
use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();
    prices.insert("apple", 0.5);
    prices.insert("bread", 2.2);
    prices.insert("milk", 1.1);
    println!("{:?}", prices);
    println!("{} products", prices.len());
}
```

`std::collections::HashMap` is the full "address" of `HashMap` inside Rust's standard library (`std`). `use` lets you write just `HashMap` afterwards.

## Looking up values

`get` returns an `Option`, because the key might not exist:

```rust
use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();
    prices.insert("apple", 0.5);
    prices.insert("bread", 2.2);

    match prices.get("apple") {
        Some(p) => println!("Apples cost {p}"),
        None => println!("No apples"),
    }

    if let Some(p) = prices.get("caviar") {
        println!("Caviar costs {p}");
    } else {
        println!("We don't sell caviar");
    }

    println!("Do we sell bread? {}", prices.contains_key("bread"));
}
```

(`match` is like `if let`, but handles all cases at once — you saw a peek earlier and it gets its own lesson in Module 4.)

## Updating and removing

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("swords", 3);
    stock.insert("swords", 5);    // same key: replaces the old value
    stock.insert("shields", 2);
    stock.remove("shields");
    println!("{:?}", stock);
}
```

## The counting pattern: `entry`

A super common task: **count how often things appear**. The `entry` API makes it a one-liner:

```rust
use std::collections::HashMap;

fn main() {
    let text = "the cat and the dog and the bird";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", counts);
}
```

Read `*counts.entry(word).or_insert(0) += 1` as: *"find the entry for this word; if it's not there yet, insert 0; then add 1 to it."* The `*` again means "the value this refers to" — Module 3 explains it properly.

## Looping over a map — and its order

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Cleo", 28);
    ages.insert("Ana", 31);
    ages.insert("Ben", 25);

    for (name, age) in &ages {
        println!("{name} is {age}");
    }
}
```

Run it a few times: the order can **change between runs**! A `HashMap` stores keys in a scrambled order that makes lookups very fast. If you need a specific order, collect the keys into a vector and sort it:

```rust
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Cleo", 28);
    ages.insert("Ana", 31);
    ages.insert("Ben", 25);

    let mut names: Vec<_> = ages.keys().collect();
    names.sort();
    for name in names {
        println!("{name} is {}", ages[name]);
    }
}
```

(`Vec<_>` means "a vector — you figure out the element type". `ages[name]` looks up a key directly, and panics if it's missing.)

> If you always want sorted keys, there's also `BTreeMap` (`use std::collections::BTreeMap;`). It works almost exactly like `HashMap`, but keeps keys sorted.

## Owned keys: `String` vs `&str`

In the examples above, keys were string literals (`&str`). When keys come from data that changes (user input, files), store them as `String`s: `HashMap<String, i32>`. To insert, convert with `.to_string()`; to look up, you can still pass a `&str`.

## When to use what?

| You need… | Use |
|---|---|
| a fixed number of values that never changes | array |
| an ordered list that grows and shrinks | `Vec` |
| to find values by a key | `HashMap` (or `BTreeMap` for sorted keys) |
| a couple of values returned together | tuple |

@@quiz
What happens if you `insert` a key that's already in the map?
- The program panics
- Both values are kept
* The new value replaces the old one
? Each key appears at most once. `insert` overwrites the old value.

@@quiz
Why might a `HashMap` print its entries in a different order each time?
* It stores keys in a scrambled order to make lookups fast, so the order isn't defined
- It's a bug in Rust
- It sorts by value
? Sort the keys yourself, or use a `BTreeMap`, when order matters.

@@exercise Word frequencies
Write `word_counts`, which counts how often each word appears in the text. Words should be compared in **lowercase**, so "The" and "the" count as the same word.
@@starter
use std::collections::HashMap;

fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!()
}

fn main() {
    let counts = word_counts("The cat and the hat");
    println!("{:?}", counts);
}
@@tests
#[test]
fn checks() {
    let c = word_counts("The cat and the hat");
    assert_eq!(c.get("the"), Some(&2), "'the' should appear 2 times");
    assert_eq!(c.get("cat"), Some(&1));
    assert_eq!(c.len(), 4, "there are 4 different words");
    assert_eq!(word_counts("").len(), 0);
}
@@hint
Loop over `text.split_whitespace()`, and use `*counts.entry(word.to_lowercase()).or_insert(0) += 1;`
@@solution
use std::collections::HashMap;

fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = word_counts("The cat and the hat");
    println!("{:?}", counts);
}

@@exercise Phone book
Build a small phone book program:

1. Create a `HashMap` and add three people: Ana → 555-1234, Ben → 555-9876, Cleo → 555-0000.
2. Ben gets a new number: 555-1111. Update it.
3. Remove Cleo.
4. Look up "Ana" and "Dan" — print the number, or `not found`.
5. Print everyone **sorted by name**.

Expected output:

```text
Ana: 555-1234
Dan: not found
--- All contacts ---
Ana: 555-1234
Ben: 555-1111
```
@@starter
use std::collections::HashMap;

fn main() {
    let mut book = HashMap::new();

}
@@expected
Ana: 555-1234
Dan: not found
--- All contacts ---
Ana: 555-1234
Ben: 555-1111
@@requires
insert
remove
get(
sort
@@hint
For step 4, loop over `["Ana", "Dan"]` and use `match book.get(name)`. For step 5, collect `book.keys()` into a vector, sort it, then loop.
@@solution
use std::collections::HashMap;

fn main() {
    let mut book = HashMap::new();
    book.insert("Ana", "555-1234");
    book.insert("Ben", "555-9876");
    book.insert("Cleo", "555-0000");

    book.insert("Ben", "555-1111");
    book.remove("Cleo");

    for name in ["Ana", "Dan"] {
        match book.get(name) {
            Some(number) => println!("{name}: {number}"),
            None => println!("{name}: not found"),
        }
    }

    println!("--- All contacts ---");
    let mut names: Vec<_> = book.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {}", book[name]);
    }
}

@@exercise Most common letter
Write `most_common_letter`, which returns the letter that appears most often in the text (ignore everything that isn't a letter, and ignore case — return it lowercase). If there's a tie, return the one that comes **first in the alphabet**. If there are no letters at all, return `None`.

This returns an `Option<char>`: write `Some(letter)` or `None`.
@@starter
use std::collections::HashMap;

fn most_common_letter(text: &str) -> Option<char> {
    todo!()
}

fn main() {
    println!("{:?}", most_common_letter("Hello, World!"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(most_common_letter("Hello, World!"), Some('l'));
    assert_eq!(most_common_letter("aabb"), Some('a'), "tie: a comes before b");
    assert_eq!(most_common_letter("BbB a"), Some('b'), "case is ignored");
    assert_eq!(most_common_letter("123 !!"), None);
}
@@hint
Count letters in a `HashMap<char, u32>`. Then go through the letters **in alphabetical order** with `for c in 'a'..='z'` and keep the one with the highest count (only replace on a strictly greater count, so ties keep the earlier letter).
@@solution
use std::collections::HashMap;

fn most_common_letter(text: &str) -> Option<char> {
    let mut counts = HashMap::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    let mut best = None;
    let mut best_count = 0;
    for c in 'a'..='z' {
        let n = counts.get(&c).copied().unwrap_or(0);
        if n > best_count {
            best = Some(c);
            best_count = n;
        }
    }
    best
}

fn main() {
    println!("{:?}", most_common_letter("Hello, World!"));
}
