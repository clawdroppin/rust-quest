@@lesson m8-collections More Collections: Sets, Sorted Maps and Queues
@@text
So far you've used two collections for almost everything: `Vec` (a list) and `HashMap` (key → value). Real apps often need a few more shapes of data. The standard library has them all in `std::collections`, and they work just like the ones you know: `new()`, `insert`, `len()`, `contains`, iterate with `for`.

| You need… | Use | Example |
|---|---|---|
| a list in order | `Vec<T>` | the lines of a file |
| to look things up by a key | `HashMap<K, V>` | word → count |
| *unique* values, "have I seen this?" | `HashSet<T>` | tags on a note, visited cells |
| a map, sorted by key | `BTreeMap<K, V>` | date → total spent, for a report |
| a sorted set | `BTreeSet<T>` | an alphabetical list of tags |
| a queue: add at the back, take from the front | `VecDeque<T>` | "last 10 messages", a to-do queue |

## HashSet: each value once

A **set** is like a bag that can hold each value only once. Inserting a value that's already there does nothing (and `insert` returns `false` to tell you).

```rust
use std::collections::HashSet;

fn main() {
    let mut tags = HashSet::new();
    tags.insert("rust");
    tags.insert("gui");
    let was_new = tags.insert("rust"); // already there
    println!("was new: {was_new}, count: {}", tags.len());

    if tags.contains("gui") {
        println!("this note is about GUIs");
    }
}
```

Sets are perfect for questions like "which letters did the player already guess?" or "have I visited this folder?". `contains` is very fast, even for millions of values, much faster than searching a `Vec`.

Two sets can be combined like in maths class:

```rust
use std::collections::HashSet;

fn main() {
    let alice: HashSet<&str> = ["rust", "chess", "piano"].into_iter().collect();
    let bob: HashSet<&str> = ["chess", "football", "rust"].into_iter().collect();

    let mut both: Vec<&str> = alice.intersection(&bob).copied().collect();
    both.sort();
    println!("both like: {both:?}");

    let mut only_alice: Vec<&str> = alice.difference(&bob).copied().collect();
    only_alice.sort();
    println!("only Alice: {only_alice:?}");
}
```

Notice the `sort()`. A `HashSet` (like a `HashMap`) keeps its items in **no particular order**, and the order can change between runs. Whenever you show items to a user, either sort them, or use the B-tree versions below.

## BTreeMap: a map that stays sorted

`BTreeMap` has the same methods as `HashMap` (`insert`, `get`, `entry`, …), but when you iterate over it, the keys come out **sorted**. That's exactly what you want for reports: totals per day, scores per name, counts per category.

```rust
use std::collections::BTreeMap;

fn main() {
    let spending = [("2026-03-02", 12.5), ("2026-03-01", 4.0), ("2026-03-02", 3.0), ("2026-03-01", 6.0)];

    let mut per_day: BTreeMap<&str, f64> = BTreeMap::new();
    for (day, amount) in spending {
        *per_day.entry(day).or_insert(0.0) += amount;
    }
    for (day, total) in &per_day {
        println!("{day}: {total:.2}");
    }
    println!("first day: {:?}", per_day.keys().next());
}
```

Dates written as `YYYY-MM-DD` sort correctly as plain text, which is one reason that format is so popular. `BTreeSet` is the sorted version of `HashSet`.

**Which one should I pick?** Use `HashMap`/`HashSet` by default; switch to `BTreeMap`/`BTreeSet` when you need the order. (The B-tree ones are a little slower for huge amounts of data, which rarely matters in an app.)

## VecDeque: a queue

A `VecDeque` ("vec-deck", a *double-ended queue*) is a list you can efficiently add to and remove from at **both ends**. `Vec::remove(0)` has to shift every other item one place, which gets slow; `VecDeque::pop_front()` doesn't.

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back("print report");
    queue.push_back("send email");
    queue.push_front("URGENT: save file");

    while let Some(job) = queue.pop_front() {
        println!("doing: {job}");
    }
}
```

A classic use is keeping only the **last N** items, like a chat that shows the 3 newest messages:

```rust
use std::collections::VecDeque;

fn main() {
    let mut recent = VecDeque::new();
    for msg in ["hi", "how are you?", "fine", "great", "bye"] {
        recent.push_back(msg);
        if recent.len() > 3 {
            recent.pop_front(); // forget the oldest
        }
    }
    println!("{recent:?}");
}
```

## Collecting into any of them

`collect()` can build any of these collections; you choose which with the type:

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let words = "the cat and the dog and the bird";
    let unique: HashSet<&str> = words.split_whitespace().collect();
    let sorted: BTreeSet<&str> = words.split_whitespace().collect();
    println!("{} different words", unique.len());
    println!("{sorted:?}");
}
```

@@quiz
You need to remember which files your program has already processed, so it never processes one twice. Which collection fits best?
- `Vec<String>`
* `HashSet<String>`
- `VecDeque<String>`
? A set answers "have I seen this?" quickly, and can't contain duplicates.

@@quiz
You print the contents of a `HashMap` twice in different runs, and the order is different. Why?
- It's a bug in your program
- The map was changed by another thread
* `HashMap` doesn't keep any order; use `BTreeMap` (or sort) if order matters
? Hash-based collections are fast because they don't care about order.

@@exercise Unique visitors
`unique_visitors` gets a list of names (one per visit, people can visit many times) and returns **how many different people** visited. Use a `HashSet`.
@@starter
use std::collections::HashSet;

fn unique_visitors(visits: &[&str]) -> usize {
    todo!()
}

fn main() {
    let visits = ["ana", "ben", "ana", "cleo", "ben", "ana"];
    println!("{}", unique_visitors(&visits));
}
@@expected
3
@@tests
#[test]
fn checks() {
    assert_eq!(unique_visitors(&[]), 0);
    assert_eq!(unique_visitors(&["x", "x", "x"]), 1);
    assert_eq!(unique_visitors(&["a", "b", "c", "a"]), 3);
}
@@requires
HashSet
@@hint
Create an empty `HashSet`, `insert` every name, and return its `len()`. Or in one line: `visits.iter().collect::<HashSet<_>>().len()`.
@@solution
use std::collections::HashSet;

fn unique_visitors(visits: &[&str]) -> usize {
    let mut seen = HashSet::new();
    for name in visits {
        seen.insert(*name);
    }
    seen.len()
}

fn main() {
    let visits = ["ana", "ben", "ana", "cleo", "ben", "ana"];
    println!("{}", unique_visitors(&visits));
}

@@exercise Word lengths report
Write `by_length`, which groups words by their length: it returns a `BTreeMap` from length to the list of words with that length (in the order they appeared). `main` already prints the report, which comes out sorted by length because it's a `BTreeMap`.
@@starter
use std::collections::BTreeMap;

fn by_length(text: &str) -> BTreeMap<usize, Vec<String>> {
    todo!()
}

fn main() {
    for (len, words) in by_length("a fast app is a happy app") {
        println!("{len}: {}", words.join(", "));
    }
}
@@expected
1: a, a
2: is
3: app, app
4: fast
5: happy
@@tests
#[test]
fn checks() {
    assert!(by_length("").is_empty());
    let m = by_length("hi yo hello");
    assert_eq!(m[&2], vec!["hi".to_string(), "yo".to_string()]);
    assert_eq!(m[&5], vec!["hello".to_string()]);
    assert_eq!(m.len(), 2);
}
@@requires
BTreeMap
@@hint
Loop over `text.split_whitespace()`. For each word: `map.entry(word.len()).or_default().push(word.to_string())`.
@@solution
use std::collections::BTreeMap;

fn by_length(text: &str) -> BTreeMap<usize, Vec<String>> {
    let mut map: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for word in text.split_whitespace() {
        map.entry(word.len()).or_default().push(word.to_string());
    }
    map
}

fn main() {
    for (len, words) in by_length("a fast app is a happy app") {
        println!("{len}: {}", words.join(", "));
    }
}

@@exercise Undo history
An editor keeps an **undo history**, but only the last 3 actions, to save memory. Complete `History`:

- `record` adds an action at the back. If there are now more than `MAX` actions, forget the **oldest** one.
- `undo` removes and returns the **newest** action (or `None` if there's nothing to undo).
@@starter
use std::collections::VecDeque;

const MAX: usize = 3;

struct History {
    actions: VecDeque<String>,
}

impl History {
    fn new() -> Self {
        History { actions: VecDeque::new() }
    }

    fn record(&mut self, action: &str) {
        todo!()
    }

    fn undo(&mut self) -> Option<String> {
        todo!()
    }
}

fn main() {
    let mut h = History::new();
    for a in ["type A", "type B", "bold", "delete line"] {
        h.record(a);
    }
    while let Some(a) = h.undo() {
        println!("undo {a}");
    }
}
@@expected
undo delete line
undo bold
undo type B
@@tests
#[test]
fn checks() {
    let mut h = History::new();
    assert_eq!(h.undo(), None);
    h.record("x");
    assert_eq!(h.undo(), Some("x".to_string()));
    for a in ["1", "2", "3", "4", "5"] {
        h.record(a);
    }
    assert_eq!(h.actions.len(), 3);
    assert_eq!(h.undo(), Some("5".to_string()));
    assert_eq!(h.undo(), Some("4".to_string()));
    assert_eq!(h.undo(), Some("3".to_string()));
    assert_eq!(h.undo(), None);
}
@@hint
`record`: `push_back`, then `if self.actions.len() > MAX { self.actions.pop_front(); }`. `undo`: the newest is at the back, so `pop_back()`.
@@solution
use std::collections::VecDeque;

const MAX: usize = 3;

struct History {
    actions: VecDeque<String>,
}

impl History {
    fn new() -> Self {
        History { actions: VecDeque::new() }
    }

    fn record(&mut self, action: &str) {
        self.actions.push_back(action.to_string());
        if self.actions.len() > MAX {
            self.actions.pop_front();
        }
    }

    fn undo(&mut self) -> Option<String> {
        self.actions.pop_back()
    }
}

fn main() {
    let mut h = History::new();
    for a in ["type A", "type B", "bold", "delete line"] {
        h.record(a);
    }
    while let Some(a) = h.undo() {
        println!("undo {a}");
    }
}
@@text
You now know the whole "standard toolbox" of collections. When you design an app, picking the right one is half the work: a note app's tags are a `HashSet`, a report is a `BTreeMap`, an undo stack is a `Vec` or `VecDeque`. You'll use all of them in the Guided Builds.
