@@lesson m7-traits Traits: Shared Behaviour
@@text
Different types often share a **capability**. A circle and a square can both compute their area. A dog and a robot can both "speak". A `Movie` and a `Book` can both be summarized. A **trait** describes such a capability: a set of methods that a type promises to provide.

## Defining and implementing a trait

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
    fn name(&self) -> String {
        String::from("circle")
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> String {
        String::from("rectangle")
    }
}

fn main() {
    let c = Circle { radius: 1.0 };
    let r = Rectangle { width: 3.0, height: 4.0 };
    println!("The {} has area {:.2}", c.name(), c.area());
    println!("The {} has area {:.2}", r.name(), r.area());
}
```

- `trait Shape { ... }` lists the method **signatures** (ending in `;`, no body).
- `impl Shape for Circle { ... }` provides the bodies for one type. The compiler checks that every method is there, with exactly the right signature.

You already know this pattern from Module 5: `impl fmt::Display for CalcError` implemented the standard library's `Display` trait, which is what makes `{}` work for a type.

## Default methods

A trait can provide a **default** body that types get for free (and can override):

```rust
trait Greet {
    fn name(&self) -> String;

    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())   // uses the required method
    }
}

struct Person {
    first: String,
}

struct Robot;

impl Greet for Person {
    fn name(&self) -> String {
        self.first.clone()
    }
}

impl Greet for Robot {
    fn name(&self) -> String {
        String::from("RX-78")
    }
    fn greet(&self) -> String {                // override the default
        format!("BEEP. UNIT {} ONLINE.", self.name())
    }
}

fn main() {
    println!("{}", Person { first: String::from("Ada") }.greet());
    println!("{}", Robot.greet());
}
```

(`struct Robot;` is a **unit struct**: a type with no fields. Useful when only the behaviour matters.)

## Functions that accept "anything with this trait"

The real power: a function can accept **any type that implements a trait**, using `impl Trait`:

```rust,norun
fn describe(shape: &impl Shape) {
    println!("A {} with area {:.1}", shape.name(), shape.area());
}

describe(&Circle { radius: 2.0 });
describe(&Rectangle { width: 2.0, height: 5.0 });
```

`describe` doesn't know or care which shape it gets. It only uses what the trait promises. If you add a `Triangle` next week and implement `Shape` for it, `describe` works with triangles without changing a single line. This is **abstraction**: writing code against a *capability* instead of a concrete type.

## Different types in one list: `dyn Trait`

A `Vec` holds values of one type. To store *different* shapes together, store them as **trait objects**: `Box<dyn Shape>` means "a pointer to some value that implements Shape".

```rust,norun
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Rectangle { width: 2.0, height: 3.0 }),
];

let mut total = 0.0;
for shape in &shapes {
    println!("{}: {:.2}", shape.name(), shape.area());   // calls the right method for each
    total += shape.area();
}
```

Each element can be a different type; Rust figures out which `area` to call while the program runs. (`Box` puts the value on the heap; more about it later in this module.)

## Traits in the standard library

You've been using traits all along:

| Trait | What it gives you |
|---|---|
| `Display` | `{}` formatting, and `.to_string()` |
| `Debug` | `{:?}` formatting |
| `Clone` | `.clone()` |
| `PartialEq` | `==` and `!=` |
| `PartialOrd` | `<`, `>`, `<=`, `>=` |
| `Add`, `Sub`, … | `+`, `-`, … operators |
| `Iterator` | `for` loops and all the iterator methods |
| `From` / `Into` | conversions like `String::from("hi")` |

Operators are just trait methods! `a + b` is really `a.add(b)` from the `Add` trait.

@@quiz
What does `impl Shape for Circle` do?
- Creates a new type called Shape
* Provides the bodies of `Shape`'s methods for `Circle`
- Makes `Circle` inherit fields from `Shape`
? A trait lists methods; `impl Trait for Type` implements them for one type. Traits have no fields.

@@quiz
A function takes `item: &impl Summary`. What can you pass to it?
- Only a struct named Summary
* A reference to any type that implements the `Summary` trait
- Any value at all
? `impl Trait` means "some type that has this capability".

@@quiz
Why use `Vec<Box<dyn Shape>>` instead of `Vec<Circle>`?
- It's faster
* To store different types that all implement `Shape` in one list
- Because `Vec<Circle>` doesn't compile
? Trait objects let different concrete types live together, as long as they share the trait.

@@exercise Animals that speak
Define a trait `Animal` with two required methods, `name(&self) -> String` and `sound(&self) -> String`, and a **default** method `speak(&self) -> String` that returns `"<name> says <sound>!"`.

Then make two structs implement it:

- `Dog` (no fields; name `"Dog"`, sound `"woof"`)
- `Cat` with a field `name: String` (sound `"meow"`), whose `name()` returns that field.
@@starter
// define the Animal trait

// define Dog and Cat, and implement Animal for both

fn main() {
    let dog = Dog;
    let cat = Cat { name: String::from("Tom") };
    println!("{}", dog.speak());
    println!("{}", cat.speak());
}
@@expected
Dog says woof!
Tom says meow!
@@tests
#[test]
fn checks() {
    assert_eq!(Dog.speak(), "Dog says woof!");
    let c = Cat { name: String::from("Luna") };
    assert_eq!(c.name(), "Luna");
    assert_eq!(c.speak(), "Luna says meow!");
}
@@requires
trait Animal
impl Animal for Dog
impl Animal for Cat
@@hint
```
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;
    fn speak(&self) -> String {
        format!("{} says {}!", self.name(), self.sound())
    }
}
struct Dog;
```
@@solution
trait Animal {
    fn name(&self) -> String;
    fn sound(&self) -> String;

    fn speak(&self) -> String {
        format!("{} says {}!", self.name(), self.sound())
    }
}

struct Dog;

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("Dog")
    }
    fn sound(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn sound(&self) -> String {
        String::from("meow")
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat { name: String::from("Tom") };
    println!("{}", dog.speak());
    println!("{}", cat.speak());
}

@@exercise A drawing of shapes
The `Shape` trait and two shapes are given. Add a third shape, `Square` with a field `side: f64`, and implement `Shape` for it.

Then write `total_area(shapes: &Vec<Box<dyn Shape>>) -> f64`, which adds up the area of every shape in the list.
@@starter
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.radius * self.radius // a rough π, to keep the numbers simple
    }
    fn name(&self) -> String {
        String::from("circle")
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> String {
        String::from("rectangle")
    }
}

// add Square here

fn total_area(shapes: &Vec<Box<dyn Shape>>) -> f64 {
    todo!()
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    for s in &shapes {
        println!("{}: {}", s.name(), s.area());
    }
    println!("total: {}", total_area(&shapes));
}
@@expected
circle: 3
rectangle: 6
square: 4
total: 13
@@tests
#[test]
fn checks() {
    let sq = Square { side: 3.0 };
    assert_eq!(sq.area(), 9.0);
    assert_eq!(sq.name(), "square");
    let empty: Vec<Box<dyn Shape>> = Vec::new();
    assert_eq!(total_area(&empty), 0.0);
    let two: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 1.0 }), Box::new(Square { side: 2.0 })];
    assert_eq!(total_area(&two), 5.0);
}
@@requires
impl Shape for Square
@@hint
`struct Square { side: f64 }` and `impl Shape for Square { ... }` with `self.side * self.side`. For the total: start with `let mut total = 0.0;`, loop over `shapes`, add `shape.area()`.
@@solution
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.0 * self.radius * self.radius // a rough π, to keep the numbers simple
    }
    fn name(&self) -> String {
        String::from("circle")
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> String {
        String::from("rectangle")
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> String {
        String::from("square")
    }
}

fn total_area(shapes: &Vec<Box<dyn Shape>>) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    for s in &shapes {
        println!("{}: {}", s.name(), s.area());
    }
    println!("total: {}", total_area(&shapes));
}
