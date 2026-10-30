@@lesson m4-inventory Project: Game Shop Inventory
@@text
Time to model a small system with your own types. You'll build the inventory for a shop in a role-playing game: it stocks weapons, armour and potions, sells them, and reports on its stock.

## Designing the types

Before coding, think about **what things exist** and **what you can do with them**:

- An item has a **category**, and there's a fixed set of them → an **enum** `Category`.
- An item has a name, a category, a price and a quantity in stock → a **struct** `Item`.
- The shop has a list of items and some gold → a **struct** `Shop`, with **methods** for everything you can do with a shop.

```rust,norun
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    price: u32,
    quantity: u32,
}

struct Shop {
    items: Vec<Item>,
    gold: u32,
}
```

`Category` derives `PartialEq` so you can compare categories with `==`, and `Clone, Copy` so it's copied around freely like a number.

## The methods

| Method | What it does |
|---|---|
| `Shop::new(gold)` | an empty shop with some gold |
| `stock(&mut self, name, category, price, qty)` | adds stock. If an item with that name exists, **increase its quantity** instead of adding a duplicate. |
| `find(&self, name) -> Option<&Item>` | look up an item by name |
| `sell(&mut self, name, qty) -> bool` | a customer buys `qty` of an item: reduce the quantity and add `price * qty` gold. Fails (returns `false`, changes nothing) if the item doesn't exist or there isn't enough stock. |
| `stock_value(&self) -> u32` | total value of everything in stock (price × quantity, summed) |
| `in_category(&self, cat) -> Vec<&Item>` | all items in a category |
| `low_stock(&self, limit) -> Vec<String>` | names of items with quantity **below** `limit` |

A useful helper you'll need: `find_mut`, a version of `find` that returns `Option<&mut Item>` so you can change the item. Inside it, loop with `for item in &mut self.items`, or use `self.items.iter_mut()`.

Notice how the methods protect the shop's data: nobody can sell stock that doesn't exist, and gold always matches what was sold. **The rules live in one place.**

@@exercise 🏆 Part 1: The shop's methods
Implement all the methods from the table. The types are given. The checker tests every method.
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    price: u32,
    quantity: u32,
}

struct Shop {
    items: Vec<Item>,
    gold: u32,
}

impl Shop {
    fn new(gold: u32) -> Self {
        todo!()
    }

    fn stock(&mut self, name: &str, category: Category, price: u32, quantity: u32) {
        todo!()
    }

    fn find(&self, name: &str) -> Option<&Item> {
        todo!()
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Item> {
        todo!()
    }

    fn sell(&mut self, name: &str, quantity: u32) -> bool {
        todo!()
    }

    fn stock_value(&self) -> u32 {
        todo!()
    }

    fn in_category(&self, category: Category) -> Vec<&Item> {
        todo!()
    }

    fn low_stock(&self, limit: u32) -> Vec<String> {
        todo!()
    }
}

fn main() {
    let mut shop = Shop::new(100);
    shop.stock("Iron Sword", Category::Weapon, 50, 2);
    shop.stock("Health Potion", Category::Potion, 10, 5);
    shop.sell("Health Potion", 3);
    println!("gold: {}, value: {}", shop.gold, shop.stock_value());
}
@@tests
fn sample() -> Shop {
    let mut s = Shop::new(100);
    s.stock("Iron Sword", Category::Weapon, 50, 2);
    s.stock("Leather Cap", Category::Armor, 15, 1);
    s.stock("Health Potion", Category::Potion, 10, 5);
    s.stock("Mana Potion", Category::Potion, 12, 1);
    s
}

#[test]
fn new_shop() {
    let s = Shop::new(42);
    assert_eq!(s.gold, 42);
    assert!(s.items.is_empty());
}

#[test]
fn stocking() {
    let mut s = sample();
    assert_eq!(s.items.len(), 4);
    s.stock("Iron Sword", Category::Weapon, 50, 3);
    assert_eq!(s.items.len(), 4, "restocking an existing item must not add a duplicate");
    assert_eq!(s.find("Iron Sword").map(|i| i.quantity), Some(5), "restocking should increase the quantity");
    assert!(s.find("Dragon").is_none());
}

#[test]
fn selling() {
    let mut s = sample();
    assert!(s.sell("Health Potion", 3));
    assert_eq!(s.find("Health Potion").unwrap().quantity, 2);
    assert_eq!(s.gold, 130, "selling 3 potions at 10 should add 30 gold");
    assert!(!s.sell("Health Potion", 3), "only 2 left: can't sell 3");
    assert_eq!(s.gold, 130, "a failed sale must not change the gold");
    assert!(!s.sell("Unicorn", 1), "unknown item");
}

#[test]
fn reports() {
    let s = sample();
    assert_eq!(s.stock_value(), 100 + 15 + 50 + 12);
    let potions: Vec<&str> = s.in_category(Category::Potion).iter().map(|i| i.name.as_str()).collect();
    assert_eq!(potions, vec!["Health Potion", "Mana Potion"]);
    assert_eq!(s.low_stock(2), vec!["Leather Cap", "Mana Potion"]);
}
@@hint
`stock`: `if let Some(item) = self.find_mut(name) { item.quantity += quantity; } else { self.items.push(Item { ... }); }`.
`sell`: `let Some(item) = self.find_mut(name) else { return false; };` then check `item.quantity < quantity`. Compute the gold before you change `self.gold` (copy `item.price` into a variable first, because `item` borrows `self`).
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    price: u32,
    quantity: u32,
}

struct Shop {
    items: Vec<Item>,
    gold: u32,
}

impl Shop {
    fn new(gold: u32) -> Self {
        Self { items: Vec::new(), gold }
    }

    fn stock(&mut self, name: &str, category: Category, price: u32, quantity: u32) {
        if let Some(item) = self.find_mut(name) {
            item.quantity += quantity;
        } else {
            self.items.push(Item { name: name.to_string(), category, price, quantity });
        }
    }

    fn find(&self, name: &str) -> Option<&Item> {
        for item in &self.items {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Item> {
        for item in &mut self.items {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }

    fn sell(&mut self, name: &str, quantity: u32) -> bool {
        let Some(item) = self.find_mut(name) else {
            return false;
        };
        if item.quantity < quantity {
            return false;
        }
        item.quantity -= quantity;
        let earned = item.price * quantity;
        self.gold += earned;
        true
    }

    fn stock_value(&self) -> u32 {
        let mut total = 0;
        for item in &self.items {
            total += item.price * item.quantity;
        }
        total
    }

    fn in_category(&self, category: Category) -> Vec<&Item> {
        let mut result = Vec::new();
        for item in &self.items {
            if item.category == category {
                result.push(item);
            }
        }
        result
    }

    fn low_stock(&self, limit: u32) -> Vec<String> {
        let mut names = Vec::new();
        for item in &self.items {
            if item.quantity < limit {
                names.push(item.name.clone());
            }
        }
        names
    }
}

fn main() {
    let mut shop = Shop::new(100);
    shop.stock("Iron Sword", Category::Weapon, 50, 2);
    shop.stock("Health Potion", Category::Potion, 10, 5);
    shop.sell("Health Potion", 3);
    println!("gold: {}, value: {}", shop.gold, shop.stock_value());
}

@@exercise 🏆 Part 2: The stock report
Now add a `report` method that returns a nicely formatted report as a `String`, grouped by category in the order Weapon, Armor, Potion. Categories with no items are left out. The starter contains a working shop (the Part 1 solution) — you only need to write `report`.

For the shop built in `main`, `println!("{}", shop.report())` must print:

```text
== Weapon ==
  Iron Sword      x2   50g
== Potion ==
  Health Potion   x2   10g
  Mana Potion     x1   12g
Gold: 130 | Stock value: 132g
```

Each item line is two spaces, the name left-aligned in 16 characters, then `x` and the quantity left-aligned in 3, then the price right-aligned in 4 followed by `g`. Tip: `format!("x{}", qty)` first, then pad that string.

To print a category's name, use `{:?}` (that's what `derive(Debug)` gives you).
@@starter
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    price: u32,
    quantity: u32,
}

struct Shop {
    items: Vec<Item>,
    gold: u32,
}

impl Shop {
    fn new(gold: u32) -> Self {
        Self { items: Vec::new(), gold }
    }

    fn stock(&mut self, name: &str, category: Category, price: u32, quantity: u32) {
        if let Some(item) = self.find_mut(name) {
            item.quantity += quantity;
        } else {
            self.items.push(Item { name: name.to_string(), category, price, quantity });
        }
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Item> {
        for item in &mut self.items {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }

    fn sell(&mut self, name: &str, quantity: u32) -> bool {
        let Some(item) = self.find_mut(name) else {
            return false;
        };
        if item.quantity < quantity {
            return false;
        }
        item.quantity -= quantity;
        let earned = item.price * quantity;
        self.gold += earned;
        true
    }

    fn stock_value(&self) -> u32 {
        let mut total = 0;
        for item in &self.items {
            total += item.price * item.quantity;
        }
        total
    }

    fn in_category(&self, category: Category) -> Vec<&Item> {
        let mut result = Vec::new();
        for item in &self.items {
            if item.category == category {
                result.push(item);
            }
        }
        result
    }

    fn report(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut shop = Shop::new(100);
    shop.stock("Iron Sword", Category::Weapon, 50, 2);
    shop.stock("Health Potion", Category::Potion, 10, 5);
    shop.stock("Mana Potion", Category::Potion, 12, 1);
    shop.sell("Health Potion", 3);
    println!("{}", shop.report());
}
@@expected
== Weapon ==
  Iron Sword      x2   50g
== Potion ==
  Health Potion   x2   10g
  Mana Potion     x1   12g
Gold: 130 | Stock value: 132g
@@requires
fn report
in_category
@@hint
Build a `String` called `out`. Loop over `[Category::Weapon, Category::Armor, Category::Potion]`; get `self.in_category(cat)`; skip it if empty (`continue`). Add the header with `out.push_str(&format!("== {:?} ==\n", cat));`. For each item: `let qty = format!("x{}", item.quantity);` then `format!("  {:<16}{:<3}{:>4}g\n", item.name, qty, item.price)`. Finish with the gold line (no `\n` at the end, since `println!` adds one).
@@solution
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    price: u32,
    quantity: u32,
}

struct Shop {
    items: Vec<Item>,
    gold: u32,
}

impl Shop {
    fn new(gold: u32) -> Self {
        Self { items: Vec::new(), gold }
    }

    fn stock(&mut self, name: &str, category: Category, price: u32, quantity: u32) {
        if let Some(item) = self.find_mut(name) {
            item.quantity += quantity;
        } else {
            self.items.push(Item { name: name.to_string(), category, price, quantity });
        }
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Item> {
        for item in &mut self.items {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }

    fn sell(&mut self, name: &str, quantity: u32) -> bool {
        let Some(item) = self.find_mut(name) else {
            return false;
        };
        if item.quantity < quantity {
            return false;
        }
        item.quantity -= quantity;
        let earned = item.price * quantity;
        self.gold += earned;
        true
    }

    fn stock_value(&self) -> u32 {
        let mut total = 0;
        for item in &self.items {
            total += item.price * item.quantity;
        }
        total
    }

    fn in_category(&self, category: Category) -> Vec<&Item> {
        let mut result = Vec::new();
        for item in &self.items {
            if item.category == category {
                result.push(item);
            }
        }
        result
    }

    fn report(&self) -> String {
        let mut out = String::new();
        for cat in [Category::Weapon, Category::Armor, Category::Potion] {
            let items = self.in_category(cat);
            if items.is_empty() {
                continue;
            }
            out.push_str(&format!("== {:?} ==\n", cat));
            for item in items {
                let qty = format!("x{}", item.quantity);
                out.push_str(&format!("  {:<16}{:<3}{:>4}g\n", item.name, qty, item.price));
            }
        }
        out.push_str(&format!("Gold: {} | Stock value: {}g", self.gold, self.stock_value()));
        out
    }
}

fn main() {
    let mut shop = Shop::new(100);
    shop.stock("Iron Sword", Category::Weapon, 50, 2);
    shop.stock("Health Potion", Category::Potion, 10, 5);
    shop.stock("Mana Potion", Category::Potion, 12, 1);
    shop.sell("Health Potion", 3);
    println!("{}", shop.report());
}

@@text
## 🎉 Module 4 complete!

You can now design your **own types**: structs for "things with properties", enums for "one of several options", methods for behaviour, and `match` to make decisions about them. Combined with collections and ownership, you have everything you need to model real-world problems.

**Next up:** making programs robust: handling errors, reading user input, and dealing with bad data without crashing.
