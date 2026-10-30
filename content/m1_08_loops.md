@@lesson m1-loops Loops: Doing Things Again and Again
@@text
Computers are brilliant at repetition: they never get bored. A **loop** runs a block of code multiple times. Rust has three kinds: `loop`, `while` and `for`.

## `for` — repeat for each item in a range

The most common loop. It runs once for each number in a **range**:

```rust
fn main() {
    for i in 1..6 {
        println!("Round {i}");
    }
    println!("Done!");
}
```

- `1..6` is a **range**: the numbers 1, 2, 3, 4, 5. The end is **not** included!
- `1..=5` (with `=`) **does** include the end: 1, 2, 3, 4, 5.
- Each time around the loop (each **iteration**), `i` holds the next number.

```rust
fn main() {
    for i in 0..3 {
        println!("exclusive: {i}");
    }
    for i in 1..=3 {
        println!("inclusive: {i}");
    }
    for i in (1..=3).rev() {
        println!("backwards: {i}");
    }
    for i in (0..=10).step_by(5) {
        println!("in steps of five: {i}");
    }
}
```

If you don't need the number, name it `_`:

```rust
fn main() {
    for _ in 0..3 {
        println!("Hip hip hooray!");
    }
}
```

## The accumulator pattern

A hugely common pattern: start with a variable, then update it on each iteration.

```rust
fn main() {
    let mut total = 0;
    for n in 1..=100 {
        total += n;
    }
    println!("1 + 2 + ... + 100 = {total}");
}
```

Trace it in your head: `total` starts at 0. Iteration 1: `total` = 0 + 1 = 1. Iteration 2: 1 + 2 = 3. Iteration 3: 3 + 3 = 6… and after 100 iterations it's 5050.

The **counter pattern** is similar — count how many times something is true:

```rust
fn main() {
    let mut multiples_of_7 = 0;
    for n in 1..=100 {
        if n % 7 == 0 {
            multiples_of_7 += 1;
        }
    }
    println!("There are {multiples_of_7} multiples of 7 up to 100");
}
```

Notice the `if` inside the `for`. Loops and decisions combine to solve real problems.

@@quiz
How many times does `for i in 2..5 { ... }` run?
- 5
- 4
* 3
? `2..5` contains 2, 3 and 4 — the end (5) is excluded. That's 3 iterations.

@@text
## `while` — repeat as long as a condition is true

`while` checks a condition before every iteration and stops when it becomes `false`. Use it when you don't know in advance how many times you'll loop:

```rust
fn main() {
    let mut money = 1;
    let mut days = 0;
    while money < 1000 {
        money *= 2;
        days += 1;
    }
    println!("Doubling every day, it takes {days} days to reach {money}");
}
```

> ⚠ **Infinite loops**: if the condition never becomes `false`, the loop runs forever. Forgetting to update the variable (e.g. leaving out `money *= 2`) is a classic bug. This app stops programs after 5 seconds, so it's safe to make that mistake here.

## `loop` — repeat forever (until `break`)

`loop` repeats forever. You leave it with `break`:

```rust
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 4 {
            break;
        }
        println!("count is {count}");
    }
    println!("Left the loop");
}
```

`loop` can even give back a value with `break value`:

```rust
fn main() {
    let mut n = 1;
    let first_big_square = loop {
        if n * n > 200 {
            break n * n;
        }
        n += 1;
    };
    println!("First square above 200: {first_big_square}");
}
```

## `break` and `continue`

These work in all three kinds of loops:

- `break` — leave the loop immediately.
- `continue` — skip the rest of this iteration and go on to the next one.

```rust
fn main() {
    for n in 1..=10 {
        if n % 2 == 0 {
            continue; // skip even numbers
        }
        if n > 7 {
            break; // stop completely
        }
        println!("{n}");
    }
}
```

@@quiz
What does `continue` do?
- Ends the whole loop
* Skips the rest of the current iteration and starts the next one
- Pauses the program
? `break` ends the loop; `continue` just jumps to the next iteration.

@@text
## Nested loops

A loop inside a loop: for **each** iteration of the outer loop, the inner loop runs **completely**.

```rust
fn main() {
    for row in 1..=3 {
        for col in 1..=4 {
            print!("{:4}", row * col);
        }
        println!(); // end the row
    }
}
```

This prints a multiplication table. Nested loops are how you work with grids: game boards, images, tables. The inner `print!` stays on the same line; the `println!()` after the inner loop ends each row.

Patterns of characters are a classic exercise for understanding nested loops:

```rust
fn main() {
    for row in 1..=4 {
        for _ in 0..row {
            print!("#");
        }
        println!();
    }
}
```

## Loop labels

When loops are nested, `break` only leaves the innermost one. To break an outer loop, give it a **label** (starting with `'`):

```rust
fn main() {
    'search: for a in 1..10 {
        for b in 1..10 {
            if a * b == 42 {
                println!("Found it: {a} x {b} = 42");
                break 'search;
            }
        }
    }
}
```

## Which loop should I use?

- **`for`** — when you're going through a known range or collection. Your default choice.
- **`while`** — when you loop *until something happens* and the stopping condition is simple.
- **`loop`** — when the stop condition is in the middle of the body, or for programs that run until the user quits (like games and GUI apps!).

@@quiz
What does this print?

```text
let mut x = 0;
for i in 1..=3 {
    x += i * 10;
}
println!("{x}");
```
- `30`
* `60`
- `40`
? 0 + 10 = 10, then 10 + 20 = 30, then 30 + 30 = 60.

@@exercise Sum up to n
Write `sum_to(n)`, which returns 1 + 2 + 3 + … + n. For `n = 0` the answer is 0. Use a loop.
@@starter
fn sum_to(n: u64) -> u64 {
    todo!()
}

fn main() {
    println!("{}", sum_to(10));
}
@@tests
#[test]
fn checks() {
    assert_eq!(sum_to(1), 1);
    assert_eq!(sum_to(3), 6, "1 + 2 + 3 = 6");
    assert_eq!(sum_to(10), 55);
    assert_eq!(sum_to(100), 5050);
    assert_eq!(sum_to(0), 0, "sum_to(0) should be 0");
}
@@requires
for
@@hint
Use the accumulator pattern: `let mut total = 0;`, then `for i in 1..=n { total += i; }`, then return `total`.
@@solution
fn sum_to(n: u64) -> u64 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

fn main() {
    println!("{}", sum_to(10));
}

@@exercise FizzBuzz
The most famous programming exercise of all — it's even used in job interviews! Print the numbers from **1 to 15**, but:

- for multiples of **3**, print `Fizz` instead of the number,
- for multiples of **5**, print `Buzz`,
- for multiples of **both** 3 and 5, print `FizzBuzz`.

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```
@@starter
fn main() {
    for n in 1..=15 {
        println!("{n}");
    }
}
@@expected
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
@@hint
The order of your checks matters! Check for "both 3 and 5" (`n % 15 == 0`) **first**, otherwise 15 would print `Fizz` and stop there.
@@solution
fn main() {
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}

@@exercise Factorial
The **factorial** of n (written n!) is 1 × 2 × 3 × … × n. For example 5! = 120. By definition, 0! = 1.

Write `factorial`.
@@starter
fn factorial(n: u64) -> u64 {
    todo!()
}

fn main() {
    println!("5! = {}", factorial(5));
}
@@tests
#[test]
fn checks() {
    assert_eq!(factorial(0), 1, "0! is 1");
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
    assert_eq!(factorial(20), 2432902008176640000);
}
@@hint
Like the sum, but start the accumulator at **1** (not 0!) and multiply: `result *= i`.
@@solution
fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    println!("5! = {}", factorial(5));
}

@@exercise Staircase
Use **nested loops** to print a right-aligned staircase of height 5. Each row has some spaces followed by some `#`:

```text
    #
   ##
  ###
 ####
#####
```

Row 1 has 4 spaces and 1 `#`, row 2 has 3 spaces and 2 `#`, and so on.
@@starter
fn main() {
    let height = 5;
    for row in 1..=height {
        // print the spaces, then the #s, then end the line
    }
}
@@expected
    #
   ##
  ###
 ####
#####
@@requires
print!(
@@hint
In row `row`, print `height - row` spaces with one inner loop, then `row` hashes with another inner loop, then `println!()`.
@@solution
fn main() {
    let height = 5;
    for row in 1..=height {
        for _ in 0..(height - row) {
            print!(" ");
        }
        for _ in 0..row {
            print!("#");
        }
        println!();
    }
}

@@exercise Collatz steps
Here's a famous puzzle. Start with any positive number n and repeat:

- if n is even, divide it by 2,
- if n is odd, multiply by 3 and add 1,

until n becomes 1. For example, 6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 takes **8 steps**. Nobody has ever proved that this always reaches 1 — but it does for every number anyone has tried!

Write `collatz_steps(n)`, which returns how many steps it takes to reach 1. (For n = 1 it's 0 steps.) Since you don't know how many steps in advance, a `while` loop fits well.
@@starter
fn collatz_steps(n: u64) -> u32 {
    todo!()
}

fn main() {
    println!("6 takes {} steps", collatz_steps(6));
}
@@tests
#[test]
fn checks() {
    assert_eq!(collatz_steps(1), 0, "1 is already 1: 0 steps");
    assert_eq!(collatz_steps(2), 1);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(7), 16);
    assert_eq!(collatz_steps(27), 111, "27 takes a famously long time: 111 steps");
}
@@requires
while
@@hint
You need to change `n`, but parameters aren't mutable. Start with `let mut n = n;` (shadowing!) and `let mut steps = 0;`, then `while n != 1 { ... }`.
@@solution
fn collatz_steps(n: u64) -> u32 {
    let mut n = n;
    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        steps += 1;
    }
    steps
}

fn main() {
    println!("6 takes {} steps", collatz_steps(6));
}
