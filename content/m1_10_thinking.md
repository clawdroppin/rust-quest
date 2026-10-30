@@lesson m1-thinking Thinking Like a Programmer
@@text
You now know the core building blocks that **every** programming language has: values and types, variables, operators, decisions, loops and functions. Honestly, that's most of what programming is made of. The rest of this course adds more powerful tools — but the hardest part of programming isn't syntax. It's **figuring out how to solve a problem with these building blocks**.

This lesson is about that skill.

## The problem-solving process

When you face a programming task, don't start typing code right away. Professional programmers follow (roughly) these steps:

1. **Understand the problem.** What exactly goes in? What exactly should come out? Restate it in your own words.
2. **Work through examples by hand.** Pick a few inputs and work out the answer on paper. Include the edge cases: zero, one, negative numbers, the biggest value. If you can't do it by hand, you can't tell a computer how to do it.
3. **Notice how *you* did it.** What steps did your brain take? That's your algorithm.
4. **Break it into small pieces.** Each piece should be simple enough to be one function, or a few lines.
5. **Write it in pseudocode** (plain-language steps) before real code.
6. **Write code in small steps, compiling and running often.** Don't write 50 lines and then compile for the first time.
7. **Test it** with your examples from step 2.
8. **Clean it up**: names, constants, small functions.

## Example: is a number prime?

A **prime number** is a whole number greater than 1 that can only be divided evenly by 1 and itself. 7 is prime. 9 isn't (9 = 3 × 3).

**Step 2 — by hand.** Is 9 prime? Try dividing by 2: remainder 1. By 3: remainder 0 — so no. Is 7 prime? Try 2, 3, 4, 5, 6: none divide evenly, so yes. What about 1? By definition, no. 2? Yes (the only even prime).

**Step 3 — how did I do it?** I tried every number from 2 up to n-1. If any of them divided evenly, it's not prime.

**Step 5 — pseudocode:**

```text
function is_prime(n):
    if n is less than 2: answer is no
    for each d from 2 up to n - 1:
        if n divided by d has remainder 0: answer is no
    if we got through the whole loop: answer is yes
```

**Step 6 — real code.** Pseudocode translates almost line by line:

```rust
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for d in 2..n {
        if n % d == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for n in 1..=20 {
        if is_prime(n) {
            println!("{n} is prime");
        }
    }
}
```

**Step 8 — improve it.** It works, but for big numbers it does a lot of unnecessary work. (Clever trick: you only need to check divisors up to the square root of n — if n = a × b, one of a or b must be ≤ √n.) Get it **working** first, then make it better.

@@quiz
What should you do **before** writing code for a problem?
- Look up the answer online
* Work through some examples by hand to understand the steps
- Write the whole program at once, then compile
? If you can solve a few examples by hand, you can usually turn your own steps into code.

@@text
## Tracing code

**Tracing** means running code in your head (or on paper), keeping track of every variable as it changes. It's the best way to understand code and find bugs. Use a table:

```rust,norun
let mut a = 1;
let mut b = 1;
for _ in 0..4 {
    let next = a + b;
    a = b;
    b = next;
}
```

| step | a | b | next |
|---|---|---|---|
| start | 1 | 1 | – |
| iteration 1 | 1 | 2 | 2 |
| iteration 2 | 2 | 3 | 3 |
| iteration 3 | 3 | 5 | 5 |
| iteration 4 | 5 | 8 | 8 |

(These are the Fibonacci numbers: each is the sum of the two before it.)

## Debugging

A **bug** is when your program compiles and runs, but does the wrong thing. The compiler can't help you with those — you have to investigate. Strategies:

**1. Print things.** Add `println!` lines to see what's actually happening:

```rust
fn main() {
    let mut total = 0;
    for i in 1..5 {
        total += i;
        println!("DEBUG: i = {i}, total = {total}");
    }
    println!("Total: {total}");
}
```

**2. `dbg!`** — a debugging macro that prints the expression, its value, *and* the line number:

```rust
fn main() {
    let width = 7;
    let area = dbg!(width * 3);
    dbg!(area > 20);
}
```

`dbg!` prints to the error output, so in this app it appears in red — that's normal.

**3. Compare expected vs actual.** Exactly *where* does what happens start being different from what you expected? Narrow it down.

**4. Rubber duck debugging.** Explain your code line by line, out loud, to a rubber duck (or anything). Seriously — explaining forces you to notice the step where your assumption is wrong. Programmers really do this.

**5. Take a break.** Many bugs are found in the shower or on a walk.

## Classic bug types to watch for

- **Off-by-one errors** — `1..n` when you meant `1..=n`, or starting at 1 instead of 0. The most common bug in all of programming.
- **Wrong comparison** — `<` instead of `<=`, `>` instead of `>=`.
- **Wrong order of checks** — like checking `% 3` before `% 15` in FizzBuzz.
- **Forgetting to update a variable** in a `while` loop → infinite loop.
- **Integer division** when you needed a float.
- **Accumulator starting at the wrong value** — 0 for sums, 1 for products.

@@quiz
`for i in 1..10` was meant to include 10, but it stops at 9. What kind of bug is that?
- A compile error
* An off-by-one error
- An infinite loop
? Ranges with `..` exclude the end. Use `1..=10` to include it.

@@exercise Prime checker
Write `is_prime` yourself (try **not** to look at the example above — reconstruct it from the pseudocode, or from your own understanding).

Bonus challenge: make it fast by only checking divisors `d` while `d * d <= n`.
@@starter
fn is_prime(n: u32) -> bool {
    todo!()
}

fn main() {
    for n in 1..=30 {
        if is_prime(n) {
            print!("{n} ");
        }
    }
    println!();
}
@@tests
#[test]
fn checks() {
    assert!(!is_prime(0), "0 is not prime");
    assert!(!is_prime(1), "1 is not prime");
    assert!(is_prime(2), "2 is prime");
    assert!(is_prime(3), "3 is prime");
    assert!(!is_prime(4), "4 is not prime");
    assert!(!is_prime(9), "9 is not prime");
    assert!(is_prime(13), "13 is prime");
    assert!(!is_prime(25), "25 is not prime");
    assert!(is_prime(97), "97 is prime");
    assert!(!is_prime(91), "91 = 7 * 13 is not prime");
    assert!(is_prime(7919), "7919 is prime");
}
@@hint
Handle `n < 2` first with an early `return false;`. Then loop over possible divisors from 2 and return `false` as soon as one divides evenly (`n % d == 0`). If the loop finishes, return `true`.
@@solution
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn main() {
    for n in 1..=30 {
        if is_prime(n) {
            print!("{n} ");
        }
    }
    println!();
}

@@exercise Squash the bugs
This function should return the **sum of all even numbers from 1 to n, including n**. For example, `sum_of_evens(6)` = 2 + 4 + 6 = 12.

It compiles and runs, but it has **two bugs**. Use the checker's messages, tracing, and `println!` debugging to find and fix them.
@@starter
fn sum_of_evens(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..n {
        if i % 2 == 1 {
            total += i;
        }
    }
    total
}

fn main() {
    println!("{}", sum_of_evens(6));
}
@@tests
#[test]
fn checks() {
    assert_eq!(sum_of_evens(1), 0, "no even numbers up to 1");
    assert_eq!(sum_of_evens(2), 2, "sum_of_evens(2) should be 2");
    assert_eq!(sum_of_evens(6), 12, "2 + 4 + 6 = 12");
    assert_eq!(sum_of_evens(7), 12, "2 + 4 + 6 = 12");
    assert_eq!(sum_of_evens(10), 30);
}
@@hint
Bug 1: does the range include `n`? Bug 2: when is a number even?
@@solution
fn sum_of_evens(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            total += i;
        }
    }
    total
}

fn main() {
    println!("{}", sum_of_evens(6));
}

@@exercise 🏆 Module project: Number report
Time to combine **everything** from this module! Write a program that prints a report about the numbers 1 to 10:

```text
Number report
=============
  n  square  parity  prime
  1       1  odd     no
  2       4  even    yes
  3       9  odd     yes
  4      16  even    no
  5      25  odd     yes
  6      36  even    no
  7      49  odd     yes
  8      64  even    no
  9      81  odd     no
 10     100  even    no
=============
Primes found: 4
Sum of squares: 385
```

Requirements:

- Use a **constant** `LIMIT` for the 10.
- Write a function `is_prime(n: u32) -> bool`.
- Write a function `parity(n: u32) -> &'static str` returning `"even"` or `"odd"`.
- Use a **`for` loop**, and **accumulators** for the prime count and the sum of squares.
- Use **formatting** for the columns: `n` is right-aligned in 3 characters, `square` right-aligned in 8, then two spaces, `parity` left-aligned in 6, then two spaces, then `yes`/`no`. (The header row uses exactly the same widths.)

Plan it before you code it! What are the pieces? Which part goes in which function?
@@starter
// Number report — your module 1 project.
// Plan first: what pieces do you need?

fn main() {
    println!("Number report");
}
@@expected
Number report
=============
  n  square  parity  prime
  1       1  odd     no
  2       4  even    yes
  3       9  odd     yes
  4      16  even    no
  5      25  odd     yes
  6      36  even    no
  7      49  odd     yes
  8      64  even    no
  9      81  odd     no
 10     100  even    no
=============
Primes found: 4
Sum of squares: 385
@@requires
const LIMIT
fn is_prime
fn parity
for
@@hint
One row looks like: `println!("{:>3}{:>8}  {:<6}  {}", n, n * n, parity(n), prime_text);` where `prime_text` is `if is_prime(n) { "yes" } else { "no" }`. The header uses the same format string with text instead of numbers.
@@solution
const LIMIT: u32 = 10;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn parity(n: u32) -> &'static str {
    if n % 2 == 0 { "even" } else { "odd" }
}

fn main() {
    println!("Number report");
    println!("=============");
    println!("{:>3}{:>8}  {:<6}  {}", "n", "square", "parity", "prime");

    let mut prime_count = 0;
    let mut sum_of_squares = 0;

    for n in 1..=LIMIT {
        let square = n * n;
        let prime = is_prime(n);
        if prime {
            prime_count += 1;
        }
        sum_of_squares += square;

        let prime_text = if prime { "yes" } else { "no" };
        println!("{:>3}{:>8}  {:<6}  {}", n, square, parity(n), prime_text);
    }

    println!("=============");
    println!("Primes found: {prime_count}");
    println!("Sum of squares: {sum_of_squares}");
}

@@text
## 🎉 Module complete!

If you've worked through everything up to here: congratulations. You can now write real programs that calculate, decide and repeat — and you've practised the problem-solving process that matters far more than any syntax.

Before moving on, it's worth going back to any exercise that felt shaky and solving it again from scratch, without hints.

**Next up:** collections of data — handling not just one value at a time, but lists and tables of them.
