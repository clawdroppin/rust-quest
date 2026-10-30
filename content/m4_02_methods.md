@@lesson m4-methods Methods: Giving Types Behaviour
@@text
In the last lesson you wrote functions like `area(r: &Rect)`. That works, but the function and the type it belongs to are separate. **Methods** are functions that belong to a type, defined inside an `impl` block. You've been calling methods all along: `name.len()`, `list.push(x)`.

## Defining methods with `impl`

```rust
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r = Rect { width: 3, height: 4 };
    println!("area {}, square? {}", r.area(), r.is_square());
}
```

- `impl Rect { ... }` ("implementation") holds everything that belongs to `Rect`.
- The first parameter `&self` is the instance the method was called on. `r.area()` is really `Rect::area(&r)`.
- Inside, use `self.field`.

## Three kinds of `self`

The first parameter says **how the method uses the instance** — the same borrowing choices you already know:

| First parameter | Meaning | Use when |
|---|---|---|
| `&self` | borrow to read | most methods: calculations, queries |
| `&mut self` | borrow to change | methods that modify the instance |
| `self` | take ownership | methods that consume or transform it |

```rust
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.count
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn into_total(self) -> u32 {
        self.count   // the Counter is used up
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    c.increment();
    c.increment();
    println!("{}", c.get());
    let total = c.into_total();
    println!("{total}");
    // c.increment(); // ❌ c was moved into into_total
}
```

Calling a `&mut self` method requires the variable to be `mut`.

## Associated functions and constructors

Functions in an `impl` block that **don't** take `self` are called **associated functions**. You call them with `::` on the type itself. You've used them: `String::from(...)`, `Vec::new()`.

The most common one is a **constructor** called `new`:

```rust
#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    level: u32,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            health: 100,
            level: 1,
        }
    }

    fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);   // never below 0
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

fn main() {
    let mut p = Player::new("Ferris");
    p.take_damage(30);
    p.take_damage(90);
    println!("{:?}, alive: {}", p, p.is_alive());
}
```

- `Self` (capital S) means "the type this `impl` is for" — here, `Player`.
- `new` isn't special to Rust; it's just the conventional name.
- `saturating_sub` subtracts but stops at 0 instead of panicking on underflow. Handy for health bars!

## Why methods?

- **Organisation**: everything you can do with a `Player` is in one place.
- **Discoverability**: type `player.` in VS Code and you'll see all its methods.
- **Encapsulation**: methods can keep the data valid. If health must never go below 0, `take_damage` guarantees it, and the rest of the program never has to think about it.

This idea — **data plus the operations that keep it valid** — is how all larger programs are structured. Your final GUI app will be built exactly this way.

@@quiz
Which first parameter should a method `fn rename(..., new_name: &str)` have, to change a struct's name field?
- `self`
- `&self`
* `&mut self`
? It changes the instance but shouldn't consume it: `&mut self`.

@@quiz
How do you call an associated function `new` on type `Shop`?
- `Shop.new()`
* `Shop::new()`
- `new Shop()`
? Associated functions are called on the type with `::`.

@@exercise Temperature type
Implement methods for `Temperature`, which stores degrees Celsius:

- `Temperature::new(celsius: f64) -> Self`
- `fahrenheit(&self) -> f64` — `C × 9 / 5 + 32`
- `is_freezing(&self) -> bool` — at or below 0 °C
- `warm_up(&mut self, degrees: f64)` — increases the temperature
@@starter
struct Temperature {
    celsius: f64,
}

impl Temperature {
    // your methods here
}

fn main() {
    let mut t = Temperature::new(-5.0);
    println!("{} freezing? {}", t.fahrenheit(), t.is_freezing());
    t.warm_up(10.0);
    println!("{} freezing? {}", t.fahrenheit(), t.is_freezing());
}
@@tests
#[test]
fn checks() {
    let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
    let mut t = Temperature::new(0.0);
    assert!(close(t.fahrenheit(), 32.0));
    assert!(t.is_freezing(), "0°C counts as freezing");
    t.warm_up(100.0);
    assert!(close(t.celsius, 100.0));
    assert!(close(t.fahrenheit(), 212.0));
    assert!(!t.is_freezing());
}
@@hint
`fn new(celsius: f64) -> Self { Self { celsius } }`, `fn fahrenheit(&self) -> f64 { self.celsius * 9.0 / 5.0 + 32.0 }`, and so on.
@@solution
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Self {
        Self { celsius }
    }

    fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    fn is_freezing(&self) -> bool {
        self.celsius <= 0.0
    }

    fn warm_up(&mut self, degrees: f64) {
        self.celsius += degrees;
    }
}

fn main() {
    let mut t = Temperature::new(-5.0);
    println!("{} freezing? {}", t.fahrenheit(), t.is_freezing());
    t.warm_up(10.0);
    println!("{} freezing? {}", t.fahrenheit(), t.is_freezing());
}

@@exercise Bank account
Build a `BankAccount` type with methods:

- `new(owner: &str) -> Self` — starts with a balance of 0
- `deposit(&mut self, amount: u64)`
- `withdraw(&mut self, amount: u64) -> bool` — only if there's enough money; returns whether it worked
- `balance(&self) -> u64`
- `history(&self) -> &[String]` — a log of every successful action, like `"deposit 50"` and `"withdraw 20"`

Store the log in a `Vec<String>` field.
@@starter
struct BankAccount {
    owner: String,
    balance: u64,
    log: Vec<String>,
}

impl BankAccount {
    // your methods here
}

fn main() {
    let mut acc = BankAccount::new("Ana");
    acc.deposit(100);
    acc.withdraw(30);
    acc.withdraw(500);
    println!("{} has {}", acc.owner, acc.balance());
    println!("{:?}", acc.history());
}
@@tests
#[test]
fn checks() {
    let mut acc = BankAccount::new("Ana");
    assert_eq!(acc.balance(), 0);
    acc.deposit(100);
    assert!(acc.withdraw(30));
    assert!(!acc.withdraw(500), "can't withdraw more than the balance");
    assert_eq!(acc.balance(), 70);
    assert_eq!(acc.history(), &["deposit 100", "withdraw 30"], "failed withdrawals aren't logged");
    assert_eq!(acc.owner, "Ana");
}
@@hint
In `withdraw`: `if amount > self.balance { return false; }`, then subtract, push `format!("withdraw {amount}")` to `self.log`, and return `true`. `history` returns `&self.log`.
@@solution
struct BankAccount {
    owner: String,
    balance: u64,
    log: Vec<String>,
}

impl BankAccount {
    fn new(owner: &str) -> Self {
        Self { owner: owner.to_string(), balance: 0, log: Vec::new() }
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
        self.log.push(format!("deposit {amount}"));
    }

    fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        self.log.push(format!("withdraw {amount}"));
        true
    }

    fn balance(&self) -> u64 {
        self.balance
    }

    fn history(&self) -> &[String] {
        &self.log
    }
}

fn main() {
    let mut acc = BankAccount::new("Ana");
    acc.deposit(100);
    acc.withdraw(30);
    acc.withdraw(500);
    println!("{} has {}", acc.owner, acc.balance());
    println!("{:?}", acc.history());
}

@@exercise Todo list type
Build a `TodoList` that holds `Task`s:

- `TodoList::new()`
- `add(&mut self, title: &str)` — adds an unfinished task
- `complete(&mut self, index: usize) -> bool` — marks task `index` done; returns `false` if the index doesn't exist
- `remaining(&self) -> usize` — how many tasks aren't done
- `titles_done(&self) -> Vec<String>` — titles of finished tasks
@@starter
struct Task {
    title: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    // your methods here
}

fn main() {
    let mut list = TodoList::new();
    list.add("learn structs");
    list.add("learn methods");
    list.complete(0);
    println!("{} remaining, done: {:?}", list.remaining(), list.titles_done());
}
@@tests
#[test]
fn checks() {
    let mut list = TodoList::new();
    list.add("a");
    list.add("b");
    list.add("c");
    assert_eq!(list.remaining(), 3);
    assert!(list.complete(1));
    assert!(!list.complete(10), "index 10 doesn't exist");
    assert_eq!(list.remaining(), 2);
    assert_eq!(list.titles_done(), vec!["b"]);
}
@@hint
For `complete`, use `self.tasks.get_mut(index)`, which gives `Some(&mut Task)` or `None`: `if let Some(task) = self.tasks.get_mut(index) { task.done = true; true } else { false }`.
@@solution
struct Task {
    title: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add(&mut self, title: &str) {
        self.tasks.push(Task { title: title.to_string(), done: false });
    }

    fn complete(&mut self, index: usize) -> bool {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
            true
        } else {
            false
        }
    }

    fn remaining(&self) -> usize {
        let mut count = 0;
        for task in &self.tasks {
            if !task.done {
                count += 1;
            }
        }
        count
    }

    fn titles_done(&self) -> Vec<String> {
        let mut titles = Vec::new();
        for task in &self.tasks {
            if task.done {
                titles.push(task.title.clone());
            }
        }
        titles
    }
}

fn main() {
    let mut list = TodoList::new();
    list.add("learn structs");
    list.add("learn methods");
    list.complete(0);
    println!("{} remaining, done: {:?}", list.remaining(), list.titles_done());
}
