@@lesson m2-iterating Working Through Lists
@@text
Most code that deals with lists does one of a handful of things: visit every element, add them up, find something, count something, or build a new list. This lesson collects the tools for all of these.

## Three ways to loop over a vector

```rust
fn main() {
    let mut scores = vec![10, 20, 30];

    // 1. Borrow each element (read only). The vector is still usable afterwards.
    for s in &scores {
        println!("score {s}");
    }

    // 2. Borrow each element mutably, to change them in place.
    for s in &mut scores {
        *s += 5;   // * means "the value this refers to" (explained in Module 3)
    }
    println!("{:?}", scores);

    // 3. Take the elements (the vector is used up and can't be used again).
    for s in scores {
        println!("final {s}");
    }
    // println!("{:?}", scores); // ❌ error: scores was moved
}
```

Use `for x in &list` as your default. The difference between these three forms is exactly what Module 3 explains.

## Getting the index too: `enumerate`

```rust
fn main() {
    let podium = vec!["Ferris", "Corro", "Bonnie"];
    for (i, name) in podium.iter().enumerate() {
        println!("{}. {name}", i + 1);
    }
}
```

`.iter()` gives you the elements one at a time; `.enumerate()` pairs each with its index as a tuple `(index, element)`, which we destructure in the `for`.

## Built-in helpers

You can do a lot without writing the loop yourself:

```rust
fn main() {
    let v = vec![4, 8, 15, 16, 23, 42];

    let total: i32 = v.iter().sum();
    let largest = v.iter().max().unwrap();   // Option, because the list might be empty
    let smallest = v.iter().min().unwrap();
    let count_even = v.iter().filter(|n| *n % 2 == 0).count();

    println!("sum {total}, max {largest}, min {smallest}, evens {count_even}");
    println!("first: {:?}, last: {:?}", v.first(), v.last());
    println!("index of 15: {:?}", v.iter().position(|n| *n == 15));
}
```

The `|n| *n % 2 == 0` part is a **closure** — a tiny unnamed function you give to `filter`. Read it as "for each `n`, keep it if `n` is even". Closures get a full lesson in Module 7; until then you'll only need simple ones like this.

> **Should I write the loop myself or use a helper?** Both are fine. Writing loops yourself builds understanding, so the exercises often ask you to. In real code, helpers like `sum()` and `max()` are shorter and say exactly what you mean.

## Slicing: part of a list

`&v[start..end]` gives you a **slice**: a view of part of a list, using a range:

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let middle = &v[1..4];     // indexes 1, 2, 3
    let first_two = &v[..2];   // from the start
    let rest = &v[2..];        // to the end
    println!("{:?} {:?} {:?}", middle, first_two, rest);
}
```

Slices are `&[T]` — the same type as the function parameters in the previous lesson. So you can pass *part* of a list to a function: `total(&v[1..4])`.

## Sorting

```rust
fn main() {
    let mut words = vec!["pear", "apple", "fig"];
    words.sort();
    println!("{:?}", words);

    let mut numbers = vec![5, 1, 4];
    numbers.sort();
    numbers.reverse();          // biggest first
    println!("{:?}", numbers);
}
```

Floats can't be sorted with `.sort()` — because of a strange special value called NaN ("not a number"), floats can't always be compared. Use `v.sort_by(|a, b| a.total_cmp(b));` for them. (Another closure; just copy it for now.)

## Joining text

```rust
fn main() {
    let parts = vec!["red", "green", "blue"];
    println!("{}", parts.join(", "));
}
```

@@quiz
What does this print?

```text
let v = vec![3, 1, 2];
for (i, x) in v.iter().enumerate() {
    print!("{i}:{x} ");
}
```
- `1:3 2:1 3:2`
* `0:3 1:1 2:2`
- `3:0 1:1 2:2`
? `enumerate` starts counting at 0 and gives `(index, element)` pairs.

@@quiz
`let v = vec![1, 2, 3, 4, 5];` — what is `&v[1..3]`?
- `[1, 2, 3]`
* `[2, 3]`
- `[2, 3, 4]`
? Indexes 1 and 2 (the end, 3, is excluded): values 2 and 3.

@@exercise Running total
Write `running_total`, which returns a new vector where each element is the sum of all elements **up to and including** that position.

`[1, 2, 3, 4]` → `[1, 3, 6, 10]`
@@starter
fn running_total(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

fn main() {
    println!("{:?}", running_total(&[1, 2, 3, 4]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(running_total(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_total(&[5]), vec![5]);
    assert_eq!(running_total(&[]), Vec::<i32>::new());
    assert_eq!(running_total(&[3, -3, 3]), vec![3, 0, 3]);
}
@@hint
Keep a `sum` variable. For each number, add it to `sum` and push `sum` into the result vector.
@@solution
fn running_total(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut sum = 0;
    for n in numbers {
        sum += n;
        result.push(sum);
    }
    result
}

fn main() {
    println!("{:?}", running_total(&[1, 2, 3, 4]));
}

@@exercise Longest word
Write `longest_word`, which returns the longest word in the list (as a new `String`). If several words share the longest length, return the **first** one. For an empty list, return an empty `String`.
@@starter
fn longest_word(words: &[&str]) -> String {
    todo!()
}

fn main() {
    println!("{}", longest_word(&["cat", "giraffe", "dog", "penguin"]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(longest_word(&["cat", "giraffe", "dog", "penguin"]), "giraffe", "giraffe comes before penguin (both 7 letters)");
    assert_eq!(longest_word(&["a", "bb", "ccc"]), "ccc");
    assert_eq!(longest_word(&[]), "");
}
@@hint
Start with `let mut longest = "";` then loop: `if word.len() > longest.len() { longest = word; }`. Using `>` (not `>=`) keeps the first one on a tie. Return `longest.to_string()`.
@@solution
fn longest_word(words: &[&str]) -> String {
    let mut longest = "";
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest.to_string()
}

fn main() {
    println!("{}", longest_word(&["cat", "giraffe", "dog", "penguin"]));
}

@@exercise Top three
Write `top_three`, which returns the **three highest** scores, highest first. If there are fewer than three scores, return all of them (highest first). The input must not be changed — make a copy first with `.to_vec()`.
@@starter
fn top_three(scores: &[u32]) -> Vec<u32> {
    todo!()
}

fn main() {
    println!("{:?}", top_three(&[50, 90, 70, 100, 60]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(top_three(&[50, 90, 70, 100, 60]), vec![100, 90, 70]);
    assert_eq!(top_three(&[5, 1]), vec![5, 1]);
    assert_eq!(top_three(&[]), Vec::<u32>::new());
    assert_eq!(top_three(&[3, 3, 3, 3]), vec![3, 3, 3]);
}
@@hint
`let mut sorted = scores.to_vec();` then `sorted.sort();` and `sorted.reverse();`. Finally `sorted.truncate(3);` and return it.
@@solution
fn top_three(scores: &[u32]) -> Vec<u32> {
    let mut sorted = scores.to_vec();
    sorted.sort();
    sorted.reverse();
    sorted.truncate(3);
    sorted
}

fn main() {
    println!("{:?}", top_three(&[50, 90, 70, 100, 60]));
}
