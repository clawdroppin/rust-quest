@@lesson m2-arrays Arrays: Fixed-Size Lists
@@text
A tuple is good for two or three values of different types. But what about the temperatures of every day of the week? Or the scores of 30 students? For **many values of the same type**, we use lists. Rust's simplest list is the **array**.

## Creating arrays

```rust
fn main() {
    let temperatures = [12, 15, 14, 18, 21, 19, 16];
    let names = ["Ana", "Ben", "Cleo"];
    let zeros = [0; 5];              // five zeros: [0, 0, 0, 0, 0]
    println!("{:?}", temperatures);
    println!("{:?}", names);
    println!("{:?}", zeros);
}
```

- Square brackets `[ ]`, values separated by commas.
- **All values must be the same type.**
- `[value; count]` creates an array with `value` repeated `count` times.

An array's type includes its length: `temperatures` is `[i32; 7]` — "an array of 7 `i32`s". An array's length **can never change**. That's its key limitation (and the next lesson's `Vec` fixes it).

## Indexing: getting one element

Each value in an array is an **element**. You get an element by its **index** (position) in square brackets. **Indexes start at 0:**

```text
 index:     0    1    2    3    4    5    6
 value:   [12,  15,  14,  18,  21,  19,  16]
```

```rust
fn main() {
    let temperatures = [12, 15, 14, 18, 21, 19, 16];
    println!("First day: {}", temperatures[0]);
    println!("Third day: {}", temperatures[2]);
    println!("Last day: {}", temperatures[6]);
    println!("Number of days: {}", temperatures.len());
}
```

> **Why start at 0?** The index is really the *distance from the start*. The first element is 0 steps from the start. Almost every programming language does it this way. It means **the last index is always `len() - 1`** — a constant source of off-by-one bugs, so stay alert!

## Out of bounds

What if you use an index that doesn't exist?

```rust
fn main() {
    let numbers = [1, 2, 3];
    for i in 0..=numbers.len() {
        println!("{}", numbers[i]);
    }
}
```

This loop goes one step too far (`..=` includes `len()`, which is 3, but the last index is 2). The program prints 1, 2, 3 and then **panics**: `index out of bounds: the len is 3 but the index is 3`. (If you write an obviously wrong index like `numbers[10]` directly, the compiler often catches it before the program even runs.) In languages like C, this would silently read random memory — a source of countless crashes and security holes. Rust always checks.

## Changing elements

With `mut`, you can change elements (but not add or remove them):

```rust
fn main() {
    let mut scores = [0; 4];
    scores[0] = 10;
    scores[2] = 7;
    scores[0] += 5;
    println!("{:?}", scores);
}
```

## Looping over arrays

The most common thing to do with a list is go through **every element**. There are two ways:

```rust
fn main() {
    let prices = [4, 8, 15, 16];

    // 1. Directly over the elements — the usual way
    for price in prices {
        println!("Price: {price}");
    }

    // 2. Over the indexes — when you need the position
    for i in 0..prices.len() {
        println!("Item #{i} costs {}", prices[i]);
    }
}
```

Prefer the first way when you only need the values: it's simpler and you can't get an index wrong.

## Common patterns with lists

Everything you learned about loops comes together here. **Sum and average:**

```rust
fn main() {
    let temps = [12, 15, 14, 18, 21, 19, 16];
    let mut total = 0;
    for t in temps {
        total += t;
    }
    let average = total as f64 / temps.len() as f64;
    println!("Average: {average:.1}");
}
```

**Finding the largest**: assume the first one is the biggest, then look for something bigger:

```rust
fn main() {
    let temps = [12, 15, 14, 18, 21, 19, 16];
    let mut hottest = temps[0];
    for t in temps {
        if t > hottest {
            hottest = t;
        }
    }
    println!("Hottest: {hottest}");
}
```

**Searching**: is a value in the list?

```rust
fn main() {
    let lucky = [3, 7, 21];
    let guess = 7;
    let mut found = false;
    for n in lucky {
        if n == guess {
            found = true;
            break; // no need to keep looking
        }
    }
    println!("Found: {found}");
    println!("Or simply: {}", lucky.contains(&guess));
}
```

(Arrays have many built-in helpers like `contains`. The `&` is explained in Module 3 — for now, just write it.)

@@quiz
`let a = [5, 10, 15, 20];` — what is `a[a.len() - 1]`?
- `15`
* `20`
- It panics
? `a.len()` is 4, so this is `a[3]` — the last element, 20.

@@quiz
What does `[7; 3]` create?
- `[7, 3]`
* `[7, 7, 7]`
- `[3, 3, 3, 3, 3, 3, 3]`
? `[value; count]` repeats `value` `count` times.

@@exercise Weekly weather report
Using the `temps` array (Monday to Sunday) and the `days` array, print the report below. Find the hottest and coldest days with a loop — you'll need to remember the **index** of the hottest/coldest day, so you can look up its name in `days`.

```text
Average: 16.4
Hottest: Fri (21)
Coldest: Mon (12)
Days above 15: 4
```
@@starter
fn main() {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let temps = [12, 15, 14, 18, 21, 19, 16];

}
@@expected
Average: 16.4
Hottest: Fri (21)
Coldest: Mon (12)
Days above 15: 4
@@requires
for
@@hint
Keep `hottest_index` and `coldest_index` variables, starting at 0. Loop with `for i in 0..temps.len()`, and when `temps[i] > temps[hottest_index]`, set `hottest_index = i`. Print the average with `{:.1}`.
@@solution
fn main() {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let temps = [12, 15, 14, 18, 21, 19, 16];

    let mut total = 0;
    let mut hottest_index = 0;
    let mut coldest_index = 0;
    let mut above_15 = 0;

    for i in 0..temps.len() {
        total += temps[i];
        if temps[i] > temps[hottest_index] {
            hottest_index = i;
        }
        if temps[i] < temps[coldest_index] {
            coldest_index = i;
        }
        if temps[i] > 15 {
            above_15 += 1;
        }
    }

    let average = total as f64 / temps.len() as f64;
    println!("Average: {average:.1}");
    println!("Hottest: {} ({})", days[hottest_index], temps[hottest_index]);
    println!("Coldest: {} ({})", days[coldest_index], temps[coldest_index]);
    println!("Days above 15: {above_15}");
}

@@exercise Count above a limit
Write `count_above`, which counts how many of the five numbers are **greater than** `limit`.
@@starter
fn count_above(numbers: [i32; 5], limit: i32) -> u32 {
    todo!()
}

fn main() {
    println!("{}", count_above([3, 9, 1, 12, 7], 5));
}
@@tests
#[test]
fn checks() {
    assert_eq!(count_above([3, 9, 1, 12, 7], 5), 3);
    assert_eq!(count_above([1, 2, 3, 4, 5], 5), 0, "5 is not greater than 5");
    assert_eq!(count_above([-1, -2, 0, 1, 2], -3), 5);
}
@@hint
The counter pattern: `let mut count = 0;` then `for n in numbers { if n > limit { count += 1; } }`.
@@solution
fn count_above(numbers: [i32; 5], limit: i32) -> u32 {
    let mut count = 0;
    for n in numbers {
        if n > limit {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", count_above([3, 9, 1, 12, 7], 5));
}

@@exercise Reverse in place
Write `reverse`, which takes an array of 6 numbers and returns it **reversed**. Do it yourself with a loop, by **swapping** elements: swap the first with the last, the second with the second-to-last, and so on.

(Rust has a built-in `.reverse()`, but don't use it — this is about understanding indexes.)
@@starter
fn reverse(mut numbers: [i32; 6]) -> [i32; 6] {
    todo!()
}

fn main() {
    println!("{:?}", reverse([1, 2, 3, 4, 5, 6]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(reverse([1, 2, 3, 4, 5, 6]), [6, 5, 4, 3, 2, 1]);
    assert_eq!(reverse([7, 7, 0, 0, 1, 2]), [2, 1, 0, 0, 7, 7]);
}
@@hint
Loop `i` from 0 up to (but not including) `len / 2`. The partner of index `i` is `len - 1 - i`. To swap: `let temp = numbers[i]; numbers[i] = numbers[j]; numbers[j] = temp;` (or use `numbers.swap(i, j)`).
@@solution
fn reverse(mut numbers: [i32; 6]) -> [i32; 6] {
    let len = numbers.len();
    for i in 0..len / 2 {
        let j = len - 1 - i;
        let temp = numbers[i];
        numbers[i] = numbers[j];
        numbers[j] = temp;
    }
    numbers
}

fn main() {
    println!("{:?}", reverse([1, 2, 3, 4, 5, 6]));
}
