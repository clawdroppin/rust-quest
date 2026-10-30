@@lesson m1-variables Values and Variables
@@text
Programs work with **data**: numbers, text, true/false values and more. A single piece of data is called a **value**. `42` is a value. `"hello"` is a value.

To keep a value around and use it later, you put it in a **variable**: a *name* that refers to a value stored in the computer's memory. Think of a variable as a labelled box with something inside.

## Creating a variable with `let`

```rust
fn main() {
    let age = 30;
    println!("I am {} years old.", age);
}
```

- `let` creates a new variable.
- `age` is the variable's name.
- `=` puts the value on the right into the variable on the left. In programming, `=` means **"store into"**, *not* "is equal to" like in maths.
- `30` is the value.

Notice the `{}` in the text: it's a **placeholder**. `println!` replaces it with the value of `age`. With several placeholders, the values fill them in order:

```rust
fn main() {
    let name = "Ferris";
    let age = 7;
    println!("{} is {} years old.", name, age);
}
```

You can also write the variable name *inside* the braces, which is often easier to read:

```rust
fn main() {
    let name = "Ferris";
    let age = 7;
    println!("{name} is {age} years old.");
}
```

## Using variables

Once a variable exists, you can use its name anywhere you'd use the value itself — including to make new values:

```rust
fn main() {
    let apples = 5;
    let oranges = 3;
    let fruit = apples + oranges;
    println!("I have {fruit} pieces of fruit.");
}
```

The computer works this out step by step: first `apples` becomes 5, then `oranges` becomes 3, then it calculates `apples + oranges` → `5 + 3` → `8`, and stores 8 in `fruit`.

## Variables are immutable by default

Here's something special about Rust. Try this:

```rust
fn main() {
    let score = 10;
    score = 20;
    println!("{score}");
}
```

Error! `cannot assign twice to immutable variable`. In Rust, variables **can't be changed** after they're created, unless you say so. This is called being **immutable** (unchangeable). It sounds limiting, but it prevents a huge number of bugs: if a value isn't supposed to change, the compiler guarantees that it won't.

## `mut` — variables that can change

When you *do* want a variable to change, add `mut` (short for **mutable**):

```rust
fn main() {
    let mut score = 10;
    println!("Score: {score}");
    score = 20;
    println!("Score: {score}");
    score = score + 5;
    println!("Score: {score}");
}
```

Look at `score = score + 5;`. In maths that's nonsense, but in programming it means: *take the current value of `score` (20), add 5, and store the result (25) back into `score`*. The right side is always worked out first.

@@quiz
What does this print?

```text
let mut x = 4;
x = x * 2;
x = x + 1;
println!("{x}");
```
- `4`
- `8`
* `9`
- It doesn't compile
? `x` starts at 4, becomes `4 * 2` = 8, then `8 + 1` = 9. (`*` means multiply.)

@@text
## Shadowing — reusing a name

You can create a **new** variable with the same name as an old one, using `let` again. The new one *shadows* (hides) the old one:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is {x}");
}
```

This is different from `mut`: each `let` makes a brand-new variable. It's handy when you're transforming a value step by step and don't need the old version any more.

## Naming variables

- Variable names use **snake_case**: all lowercase, words separated by underscores: `player_score`, `max_speed`, `first_name`.
- Names can contain letters, digits and `_`, but can't start with a digit.
- Choose **descriptive** names. `seconds_left` is much clearer than `s` or `x2`. Code is read far more often than it's written!

## Declaring now, assigning later

You can create a variable without a value and give it one later — but the compiler makes sure you never *use* it before it has a value:

```rust
fn main() {
    let message;
    message = "Assigned later!";
    println!("{message}");
}
```

@@quiz
Which variable name follows Rust's style?
- `PlayerName`
* `player_name`
- `playername!`
- `2nd_player`
? Rust variables use `snake_case`. `2nd_player` starts with a digit, which isn't allowed.

@@exercise Your info card
Create three variables: `name` with the value `"Alex"`, `city` with the value `"Prague"`, and `year` with the value `2026`. Then print them using placeholders so the output is exactly:

```text
Name: Alex
City: Prague
Year: 2026
```
@@starter
fn main() {
    // Create your three variables here

    // Then print them
}
@@expected
Name: Alex
City: Prague
Year: 2026
@@requires
let name
let city
let year
@@hint
Create a variable like this: `let name = "Alex";` — then print it with `println!("Name: {name}");`
@@solution
fn main() {
    let name = "Alex";
    let city = "Prague";
    let year = 2026;

    println!("Name: {name}");
    println!("City: {city}");
    println!("Year: {year}");
}

@@exercise Fix the counter
This program tries to count up, but it doesn't compile. Fix it with the smallest possible change so that it prints:

```text
Count: 1
Count: 2
Count: 3
```
@@starter
fn main() {
    let count = 1;
    println!("Count: {count}");
    count = count + 1;
    println!("Count: {count}");
    count = count + 1;
    println!("Count: {count}");
}
@@expected
Count: 1
Count: 2
Count: 3
@@requires
let mut count
@@hint
Read the error. Which keyword allows a variable to change?
@@solution
fn main() {
    let mut count = 1;
    println!("Count: {count}");
    count = count + 1;
    println!("Count: {count}");
    count = count + 1;
    println!("Count: {count}");
}

@@exercise Piggy bank
You start with `savings` of 100. Make `savings` mutable, then:

1. add 50 (birthday money) and print it,
2. subtract 30 (a new game) and print it,
3. double it (interest!) and print it.

Use `savings = savings + ...` style lines — don't just print the numbers directly. Output:

```text
Savings: 100
After birthday: 150
After buying a game: 120
After doubling: 240
```
@@starter
fn main() {
    let savings = 100;
    println!("Savings: {savings}");
}
@@expected
Savings: 100
After birthday: 150
After buying a game: 120
After doubling: 240
@@requires
savings = savings
@@hint
`-` subtracts and `*` multiplies. For example: `savings = savings - 30;`
@@solution
fn main() {
    let mut savings = 100;
    println!("Savings: {savings}");
    savings = savings + 50;
    println!("After birthday: {savings}");
    savings = savings - 30;
    println!("After buying a game: {savings}");
    savings = savings * 2;
    println!("After doubling: {savings}");
}
