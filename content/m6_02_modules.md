@@lesson m6-modules Modules and Visibility
@@text
As programs grow, one long list of functions becomes hard to navigate. **Modules** group related code together, the way folders group files. They also control **visibility**: which parts can be used from outside the module.

## Defining a module

```rust
mod geometry {
    pub fn circle_area(r: f64) -> f64 {
        std::f64::consts::PI * r * r
    }

    pub fn square_area(side: f64) -> f64 {
        side * side
    }
}

fn main() {
    println!("{:.2}", geometry::circle_area(1.0));
    println!("{}", geometry::square_area(3.0));
}
```

- `mod geometry { ... }` creates a module named `geometry`.
- Outside the module, refer to its items with a **path**: `geometry::circle_area`.
- `pub` makes an item **public**: usable from outside the module.

## Private by default

Everything in a module is **private** unless marked `pub`. Private items can only be used inside the module (and its sub-modules):

```rust
mod bank {
    pub fn interest(balance: f64) -> f64 {
        balance * rate()       // ✔ inside the module: fine
    }

    fn rate() -> f64 {         // private helper
        0.03
    }
}

fn main() {
    println!("{}", bank::interest(1000.0));
    // println!("{}", bank::rate());   // ❌ error: function `rate` is private
}
```

This is **encapsulation**: a module shows a small, clean public interface and hides its internal details. You can change private helpers freely without breaking any code that uses the module.

## Structs and visibility

For structs, the struct **and each field** have their own visibility:

```rust
mod game {
    pub struct Player {
        pub name: String,   // anyone can read/change the name
        health: u32,        // private: only the game module can change it
    }

    impl Player {
        pub fn new(name: &str) -> Self {
            Player { name: name.to_string(), health: 100 }
        }

        pub fn health(&self) -> u32 {
            self.health
        }

        pub fn hit(&mut self, damage: u32) {
            self.health = self.health.saturating_sub(damage);
        }
    }
}

fn main() {
    let mut p = game::Player::new("Ferris");
    p.hit(30);
    println!("{} has {} HP", p.name, p.health());
    // p.health = 9999;  // ❌ error: field `health` is private
}
```

Because `health` is private, the only way to change it is through `hit` — which guarantees it never goes below zero. Outside code **can't break the rules**. This is how well-designed libraries protect their data.

(Enums are different: if an enum is `pub`, all of its variants are public automatically.)

## `use`: shortcuts for paths

Writing full paths gets tedious. `use` brings a name into scope:

```rust
mod shapes {
    pub mod round {
        pub fn circle(r: f64) -> f64 { 3.14159 * r * r }
    }
}

use shapes::round::circle;

fn main() {
    println!("{}", circle(2.0));
}
```

You've been doing this all along: `use std::collections::HashMap;` means "from the standard library (`std`), in the `collections` module, bring in `HashMap`".

## Paths: `crate`, `self`, `super`

- `crate::...` — start from the root of your project (an **absolute** path).
- `self::...` — start from the current module.
- `super::...` — start from the **parent** module (one level up).

```rust
mod app {
    pub fn version() -> &'static str {
        "1.0"
    }

    pub mod ui {
        pub fn title() -> String {
            format!("My App v{}", super::version())   // the parent module's function
        }
    }
}

fn main() {
    println!("{}", crate::app::ui::title());
}
```

`super::*` is what the exercise checker's test module uses (`use super::*;`) to see your code!

@@quiz
By default, items inside a module are…
- public
* private
- copied into `main`
? Add `pub` to make something usable from outside the module.

@@quiz
A struct is `pub` but one of its fields is not. What can code outside the module do?
- Read and change that field
* Use the struct, but not read or change that field directly
- Nothing at all with the struct
? Field visibility is separate. Private fields can only be accessed through the module's public methods.

@@exercise Temperature module
Create a module `convert` with two **public** functions, `c_to_f(c: f64) -> f64` and `f_to_c(f: f64) -> f64`. Put the shared constants in **private** items inside the module: `const RATIO: f64 = 1.8;` and `const OFFSET: f64 = 32.0;`.

`main` is already written and uses the module.
@@starter
// write the convert module here

fn main() {
    println!("{}", convert::c_to_f(100.0));
    println!("{}", convert::f_to_c(32.0));
}
@@tests
#[test]
fn checks() {
    assert_eq!(convert::c_to_f(100.0), 212.0);
    assert_eq!(convert::f_to_c(212.0), 100.0);
    assert_eq!(convert::f_to_c(-40.0), -40.0);
}
@@requires
mod convert
pub fn c_to_f
const RATIO
@@hint
`mod convert { const RATIO: f64 = 1.8; const OFFSET: f64 = 32.0; pub fn c_to_f(c: f64) -> f64 { c * RATIO + OFFSET } ... }`
@@solution
mod convert {
    const RATIO: f64 = 1.8;
    const OFFSET: f64 = 32.0;

    pub fn c_to_f(c: f64) -> f64 {
        c * RATIO + OFFSET
    }

    pub fn f_to_c(f: f64) -> f64 {
        (f - OFFSET) / RATIO
    }
}

fn main() {
    println!("{}", convert::c_to_f(100.0));
    println!("{}", convert::f_to_c(32.0));
}

@@exercise A protected counter
Write a module `stats` containing a public struct `Counter` with a **private** field `count: u32` and a private field `max: u32`. Give it public methods:

- `Counter::new(max: u32) -> Counter`
- `increment(&mut self) -> bool` — adds 1 unless the count has reached `max`; returns whether it changed
- `value(&self) -> u32`

Outside code must not be able to break the rule that the count never goes above `max`.
@@starter
// write the stats module here

fn main() {
    let mut c = stats::Counter::new(2);
    println!("{} {} {}", c.increment(), c.increment(), c.increment());
    println!("value: {}", c.value());
}
@@tests
#[test]
fn checks() {
    let mut c = stats::Counter::new(2);
    assert!(c.increment());
    assert!(c.increment());
    assert!(!c.increment(), "can't go past max");
    assert_eq!(c.value(), 2);
    let mut z = stats::Counter::new(0);
    assert!(!z.increment());
    assert_eq!(z.value(), 0);
}
@@requires
mod stats
pub struct Counter
@@hint
Inside `mod stats`: `pub struct Counter { count: u32, max: u32 }` (fields without `pub`), and `impl Counter { pub fn new(...) ... }`. Every method needs `pub` too.
@@solution
mod stats {
    pub struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        pub fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }

        pub fn increment(&mut self) -> bool {
            if self.count >= self.max {
                return false;
            }
            self.count += 1;
            true
        }

        pub fn value(&self) -> u32 {
            self.count
        }
    }
}

fn main() {
    let mut c = stats::Counter::new(2);
    println!("{} {} {}", c.increment(), c.increment(), c.increment());
    println!("value: {}", c.value());
}
