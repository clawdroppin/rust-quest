@@lesson m4-enums Enums: One of Several Possibilities
@@text
A struct says "a value has **all** of these fields". An **enum** (short for *enumeration*) says "a value is **one of** these options". Together, structs and enums can model almost anything.

## A simple enum

```rust
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let heading = Direction::East;
    println!("{:?}", heading);
}
```

- `Direction` is a new type with exactly four possible values, called **variants**.
- You write a variant as `Type::Variant`.
- A `Direction` can **only** be one of those four. Not a misspelled string like `"Nrth"`, not `7`.

Compare with storing the direction as a `String` or a number: nothing would stop a bug from creating `"up"` or `42`. With an enum, invalid values **can't exist**. This idea — *make invalid states unrepresentable* — is one of the most powerful in programming.

## Deciding with `match`

To act on an enum value, use `match`. It compares the value against each pattern and runs the arm that fits:

```rust
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    let d = turn_right(Direction::West);
    println!("{:?}", d);
}
```

`match` must handle **every** variant. If you add a fifth direction later and forget to update a `match`, the program won't compile, and the compiler points you to every place that needs updating. This is a huge help in big programs.

## Variants with data

Here's where Rust's enums get much more powerful than in most languages: **each variant can carry its own data**.

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),   // three side lengths
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let p = (a + b + c) / 2.0;
            (p * (p - a) * (p - b) * (p - c)).sqrt()
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle { width: 2.0, height: 3.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for s in &shapes {
        println!("{:?} has area {:.2}", s, area(s));
    }
}
```

- `Circle` holds a radius, `Rectangle` holds a width and height, `Triangle` holds three numbers.
- In `match`, the pattern **pulls the data out** into variables (`radius`, `width`, …).
- All three are the same type, `Shape`, so they fit in one `Vec`.

A real-world example: messages in a chat app.

```rust
enum Message {
    Text(String),
    Image { url: String, width: u32, height: u32 },
    Joined(String),
    Left(String),
}

fn describe(m: &Message) -> String {
    match m {
        Message::Text(t) => format!("says: {t}"),
        Message::Image { url, width, height } => format!("sent a {width}x{height} image: {url}"),
        Message::Joined(name) => format!("{name} joined"),
        Message::Left(name) => format!("{name} left"),
    }
}

fn main() {
    let log = vec![
        Message::Joined(String::from("Ana")),
        Message::Text(String::from("hi all")),
        Message::Image { url: String::from("cat.png"), width: 640, height: 480 },
        Message::Left(String::from("Ana")),
    ];
    for m in &log {
        println!("{}", describe(m));
    }
}
```

## Methods on enums

Enums can have `impl` blocks too:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    fn seconds(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
}

fn main() {
    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!("{:?} for {}s", light, light.seconds());
        light = light.next();
    }
    println!("Is it red? {}", light == TrafficLight::Red);
}
```

`#[derive(Clone, Copy, PartialEq)]` makes the enum copyable (it holds no data, so copying is free) and comparable with `==`. You'll often derive these for simple enums.

## Using `Self::` inside impl

Inside an `impl` block, you can write `Self::Red` instead of `TrafficLight::Red`. Or add `use TrafficLight::*;` to use the bare variant names. Both are common.

@@quiz
What's the main difference between a struct and an enum?
* A struct has all of its fields at once; an enum value is exactly one of its variants
- Enums can't hold data
- Structs can't have methods
? Struct = "this AND this AND this". Enum = "this OR this OR this".

@@quiz
Why is it useful that `match` must cover every variant?
- It makes the program faster
* If you add a new variant, the compiler shows you every `match` that needs updating
- It isn't useful; it's just a rule
? Exhaustiveness checking turns "forgot a case" bugs into compile errors.

@@exercise Coin values
Define an enum `Coin` with variants `Penny`, `Nickel`, `Dime` and `Quarter`, and implement a method `value(&self) -> u32` that returns its value in cents (1, 5, 10, 25).

Then write `total(coins: &[Coin]) -> u32`.
@@starter
// define Coin and its value() method here

fn total(coins: &[Coin]) -> u32 {
    todo!()
}

fn main() {
    let purse = vec![Coin::Quarter, Coin::Dime, Coin::Penny, Coin::Penny];
    println!("{} cents", total(&purse));
}
@@tests
#[test]
fn checks() {
    assert_eq!(Coin::Penny.value(), 1);
    assert_eq!(Coin::Nickel.value(), 5);
    assert_eq!(Coin::Dime.value(), 10);
    assert_eq!(Coin::Quarter.value(), 25);
    assert_eq!(total(&[Coin::Quarter, Coin::Dime, Coin::Penny, Coin::Penny]), 37);
    assert_eq!(total(&[]), 0);
}
@@hint
`impl Coin { fn value(&self) -> u32 { match self { Coin::Penny => 1, ... } } }`
@@solution
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn total(coins: &[Coin]) -> u32 {
    let mut sum = 0;
    for coin in coins {
        sum += coin.value();
    }
    sum
}

fn main() {
    let purse = vec![Coin::Quarter, Coin::Dime, Coin::Penny, Coin::Penny];
    println!("{} cents", total(&purse));
}

@@exercise Robot commands
A robot on a grid starts at `(0, 0)`. Commands are an enum:

- `Forward(i32)` — move that many steps in the current direction
- `TurnLeft`, `TurnRight` — rotate 90°

The robot starts facing **North** (North = y increases, East = x increases). Write `run`, which executes all commands and returns the final position `(x, y)`.

A `Facing` enum is given; you'll need to write how it turns.
@@starter
#[derive(Debug, Clone, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

enum Command {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

fn run(commands: &[Command]) -> (i32, i32) {
    todo!()
}

fn main() {
    let program = vec![Command::Forward(3), Command::TurnRight, Command::Forward(2)];
    println!("{:?}", run(&program));
}
@@tests
#[test]
fn checks() {
    use Command::*;
    assert_eq!(run(&[Forward(3), TurnRight, Forward(2)]), (2, 3));
    assert_eq!(run(&[TurnLeft, Forward(5)]), (-5, 0));
    assert_eq!(run(&[TurnRight, TurnRight, Forward(4)]), (0, -4));
    assert_eq!(run(&[Forward(1), TurnLeft, TurnLeft, TurnLeft, TurnLeft, Forward(1)]), (0, 2), "four left turns = facing north again");
    assert_eq!(run(&[]), (0, 0));
}
@@hint
Keep `let mut facing = Facing::North;` and `let (mut x, mut y) = (0, 0);`. For `TurnRight`, use a `match` that maps North→East, East→South, etc. For `Forward(n)`, `match facing` to decide whether to change x or y.
@@solution
#[derive(Debug, Clone, Copy)]
enum Facing {
    North,
    East,
    South,
    West,
}

enum Command {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

impl Facing {
    fn right(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::West => Facing::South,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
        }
    }
}

fn run(commands: &[Command]) -> (i32, i32) {
    let mut facing = Facing::North;
    let (mut x, mut y) = (0, 0);
    for command in commands {
        match command {
            Command::TurnLeft => facing = facing.left(),
            Command::TurnRight => facing = facing.right(),
            Command::Forward(steps) => match facing {
                Facing::North => y += steps,
                Facing::South => y -= steps,
                Facing::East => x += steps,
                Facing::West => x -= steps,
            },
        }
    }
    (x, y)
}

fn main() {
    let program = vec![Command::Forward(3), Command::TurnRight, Command::Forward(2)];
    println!("{:?}", run(&program));
}
