@@lesson m1-operators Operators and Expressions
@@text
## Arithmetic operators

| Operator | Meaning | Example | Result |
|---|---|---|---|
| `+` | add | `7 + 2` | `9` |
| `-` | subtract | `7 - 2` | `5` |
| `*` | multiply | `7 * 2` | `14` |
| `/` | divide | `7 / 2` | `3` (integers!) |
| `%` | remainder | `7 % 2` | `1` |
| `-x` | negate | `-7` | `-7` |

## Order of operations

Just like in maths, `*`, `/` and `%` happen **before** `+` and `-`:

```rust
fn main() {
    println!("{}", 2 + 3 * 4);     // 14, not 20
    println!("{}", (2 + 3) * 4);   // 20
    println!("{}", 10 - 4 - 3);    // 3: same level goes left to right
}
```

Use **parentheses** `( )` to control the order — and also simply to make your intention clear to readers, even when they're not strictly needed.

## Expressions

An **expression** is any piece of code that produces a value. `5` is an expression. `x + 1` is an expression. `age >= 18` is an expression. Expressions can be built out of smaller expressions, like LEGO:

```text
(price * quantity) + shipping
 └─ expression ─┘
└──────── bigger expression ────────┘
```

When the program runs, the computer **evaluates** the expression — works it out step by step until there's just one value left. Being able to evaluate expressions in your head is one of the core skills of programming.

## Compound assignment

Changing a variable based on its own value is so common that there's a shortcut:

| Shortcut | Means |
|---|---|
| `x += 5` | `x = x + 5` |
| `x -= 5` | `x = x - 5` |
| `x *= 5` | `x = x * 5` |
| `x /= 5` | `x = x / 5` |
| `x %= 5` | `x = x % 5` |

```rust
fn main() {
    let mut gold = 100;
    gold += 25;  // found treasure
    gold -= 40;  // bought a sword
    println!("Gold: {gold}");
}
```

> Rust has **no** `x++` or `x--` (which some other languages have). Write `x += 1` instead.

@@quiz
What does `2 + 10 / 4 * 2` evaluate to (integers)?
- `7`
* `6`
- `14`
? `/` and `*` go first, left to right: `10 / 4` = 2 (integer division cuts off the .5!), `2 * 2` = 4, then `2 + 4` = 6. If you said 7, you forgot about integer division. If you said 14, you did the `+` first.

@@text
## Math with methods

Numbers have built-in **methods** for common math operations:

```rust
fn main() {
    let x: f64 = 2.0;
    println!("square root: {}", x.sqrt());
    println!("x to the 3rd power: {}", x.powi(3));
    println!("rounded: {}", 2.7_f64.round());
    println!("rounded down: {}", 2.7_f64.floor());
    println!("rounded up: {}", 2.2_f64.ceil());
    println!("absolute value: {}", (-5.5_f64).abs());

    let n: i32 = -7;
    println!("{}", n.abs());          // 7
    println!("{}", 2_i32.pow(10));    // 1024
    println!("{}", 3.max(9));         // the bigger one: 9
    println!("{}", 3.min(9));         // the smaller one: 3
}
```

`2.7_f64` is just the number `2.7` with its type written on the end. Sometimes Rust needs to know the exact type before it lets you call a method.

Rust also has mathematical constants:

```rust
fn main() {
    let radius = 2.0;
    let area = std::f64::consts::PI * radius * radius;
    println!("Circle area: {area}");
}
```

## Blocks are expressions too

In Rust, a block of code in `{ }` is also an expression: its value is its **last line, if that line has no semicolon**:

```rust
fn main() {
    let total = {
        let base = 10;
        let bonus = 5;
        base + bonus
    };
    println!("{total}");
}
```

The variables `base` and `bonus` only exist inside the block (this is called their **scope**). The block's value, `base + bonus` = 15, goes into `total`. This "last expression without a semicolon" idea is very important in Rust — you'll see it again with functions and `if`.

@@quiz
What's the value of `y`?

```text
let y = {
    let a = 3;
    a * a
};
```
- `3`
* `9`
- Nothing — it's a compile error
? The last line `a * a` has no semicolon, so it is the block's value: 9.

@@exercise Rectangle calculator
Calculate the **area** (width × height) and **perimeter** (the distance all the way around: 2 × width + 2 × height) of a rectangle and print:

```text
Width: 8, Height: 5
Area: 40
Perimeter: 26
```

Calculate the numbers from `width` and `height`.
@@starter
fn main() {
    let width = 8;
    let height = 5;
    println!("Width: {width}, Height: {height}");
}
@@expected
Width: 8, Height: 5
Area: 40
Perimeter: 26
@@requires
*
@@hint
`let area = width * height;` and `let perimeter = 2 * width + 2 * height;`
@@solution
fn main() {
    let width = 8;
    let height = 5;
    println!("Width: {width}, Height: {height}");
    let area = width * height;
    let perimeter = 2 * width + 2 * height;
    println!("Area: {area}");
    println!("Perimeter: {perimeter}");
}

@@exercise Pythagoras
In a right-angled triangle, the long side `c` is the square root of `a² + b²`. Calculate it for `a = 5.0` and `b = 12.0` using `.sqrt()` and print:

```text
The long side is 13
```
@@starter
fn main() {
    let a: f64 = 5.0;
    let b: f64 = 12.0;
}
@@expected
The long side is 13
@@requires
sqrt
@@hint
First compute `a * a + b * b`, then call `.sqrt()` on it. Put the sum in parentheses: `(a * a + b * b).sqrt()`
@@solution
fn main() {
    let a: f64 = 5.0;
    let b: f64 = 12.0;
    let c = (a * a + b * b).sqrt();
    println!("The long side is {c}");
}

@@exercise Battle damage
A hero starts with 100 health. Using **compound assignment** operators (`-=`, `+=`, `*=`), apply these in order, then print the final value:

1. hit by a goblin: lose 30
2. drink a potion: gain 15
3. cast a spell that doubles health

```text
Health: 170
```
@@starter
fn main() {
    let mut health = 100;

    println!("Health: {health}");
}
@@expected
Health: 170
@@requires
-=
+=
*=
@@hint
`health -= 30;` then `health += 15;` then `health *= 2;`
@@solution
fn main() {
    let mut health = 100;
    health -= 30;
    health += 15;
    health *= 2;
    println!("Health: {health}");
}
