@@lesson m5-question The ? Operator: Passing Errors Up
@@text
Often a function does several steps that can each fail, and if any step fails, the whole function should fail with that error. Writing a `match` for every step gets long:

```rust
fn add_strings(a: &str, b: &str) -> Result<i32, String> {
    let x = match a.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };
    let y = match b.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e.to_string()),
    };
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("2", "40"));
    println!("{:?}", add_strings("2", "forty"));
}
```

## `?` does that for you

Put `?` after a `Result`: if it's `Ok(value)`, you get the `value`; if it's `Err(e)`, the function **immediately returns** `Err(e)`.

```rust
use std::num::ParseIntError;

fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.trim().parse::<i32>()?;
    let y = b.trim().parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    println!("{:?}", add_strings("2", "40"));
    println!("{:?}", add_strings("2", "forty"));
}
```

This is called **propagating** the error: passing it up to the caller to deal with. Errors bubble up through function calls until someone handles them, usually close to the user interface, where you know how to show a message.

## Rules for `?`

1. You can only use `?` inside a function that returns `Result` (or `Option` — for `Option`, `?` returns `None` early).
2. The error types must fit. `?` can automatically **convert** the error into the function's error type if a conversion exists. For example, many errors convert into `Box<dyn std::error::Error>` (see below), and you'll learn to write your own conversions.

A common trick when you just want a `String` error: convert first with `map_err`, then `?`:

```rust
fn parse_age(text: &str) -> Result<u32, String> {
    let age: u32 = text.trim().parse().map_err(|_| format!("'{text}' is not a valid age"))?;
    if age > 150 {
        return Err(format!("{age} is not a realistic age"));
    }
    Ok(age)
}

fn main() {
    println!("{:?}", parse_age("37"));
    println!("{:?}", parse_age("old"));
    println!("{:?}", parse_age("200"));
}
```

## `main` can return a Result

If `main` returns `Result`, you can use `?` in it. If an error reaches the end of `main`, the program prints it and exits with an error code:

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n: i32 = "123".parse()?;
    println!("parsed {n}");
    let m: i32 = "oops".parse()?;
    println!("never printed {m}");
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error". It's a convenient catch-all for `main` and small programs. (`Box` and `dyn` are explained in Module 7.)

## `ok_or`: from Option to Result

When a step gives you an `Option` but your function returns `Result`, convert with `ok_or`:

```rust
fn first_word_length(text: &str) -> Result<usize, String> {
    let word = text.split_whitespace().next().ok_or("the text is empty")?;
    Ok(word.len())
}

fn main() {
    println!("{:?}", first_word_length("hello world"));
    println!("{:?}", first_word_length("   "));
}
```

(`"the text is empty"` is a `&str`, and `?` converts it into a `String` for us.)

@@quiz
What does `let x = something()?;` do when `something()` returns `Err(e)`?
- `x` becomes `e`
- The program panics
* The current function returns `Err(e)` immediately
? `?` propagates the error to the caller.

@@quiz
Where can you use `?` on a `Result`?
- Anywhere
* Inside a function that returns `Result`
- Only in `main`
? The function must be able to return the error — so it must return a `Result` (or `Option`, for `Option`s).

@@exercise Read a coordinate
Write `parse_point`, which turns text like `"3,-4"` into a tuple `(3, -4)`. Use `?` for every step:

- If there's no comma → `Err("expected x,y")` (use `split_once(',')` and `ok_or`)
- If either side isn't a number → `Err("invalid number: <that part>")`

Spaces around the numbers are allowed.
@@starter
fn parse_point(text: &str) -> Result<(i32, i32), String> {
    todo!()
}

fn main() {
    println!("{:?}", parse_point("3,-4"));
    println!("{:?}", parse_point("3;4"));
    println!("{:?}", parse_point("3,four"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(parse_point("3,-4"), Ok((3, -4)));
    assert_eq!(parse_point(" 10 , 20 "), Ok((10, 20)));
    assert_eq!(parse_point("3;4"), Err(String::from("expected x,y")));
    assert_eq!(parse_point("3,four"), Err(String::from("invalid number: four")));
    assert_eq!(parse_point("x,1"), Err(String::from("invalid number: x")));
}
@@requires
?;
@@hint
`let (a, b) = text.split_once(',').ok_or("expected x,y")?;` then for each part: `let x = a.trim().parse::<i32>().map_err(|_| format!("invalid number: {}", a.trim()))?;`
@@solution
fn parse_point(text: &str) -> Result<(i32, i32), String> {
    let (a, b) = text.split_once(',').ok_or("expected x,y")?;
    let x = a.trim().parse::<i32>().map_err(|_| format!("invalid number: {}", a.trim()))?;
    let y = b.trim().parse::<i32>().map_err(|_| format!("invalid number: {}", b.trim()))?;
    Ok((x, y))
}

fn main() {
    println!("{:?}", parse_point("3,-4"));
    println!("{:?}", parse_point("3;4"));
    println!("{:?}", parse_point("3,four"));
}

@@exercise Total of a shopping list
Each line of a shopping list looks like `name,quantity,unit_price` — for example `apple,3,0.5`. Write:

- `parse_line(line) -> Result<f64, String>` returning `quantity × unit_price` for one line. The errors are `"expected 3 fields"`, `"bad quantity"` (the quantity isn't a whole number) and `"bad price"`.
- `total(list) -> Result<f64, String>` adding up all non-empty lines, and **stopping at the first error** using `?`.
@@starter
fn parse_line(line: &str) -> Result<f64, String> {
    todo!()
}

fn total(list: &str) -> Result<f64, String> {
    todo!()
}

fn main() {
    println!("{:?}", total("apple,3,0.5\nbread,1,2.25\n"));
    println!("{:?}", total("apple,3,0.5\nbread,one,2.25"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(parse_line("apple,3,0.5"), Ok(1.5));
    assert_eq!(parse_line("apple,3"), Err(String::from("expected 3 fields")));
    assert_eq!(parse_line("apple,x,0.5"), Err(String::from("bad quantity")));
    assert_eq!(parse_line("apple,3,cheap"), Err(String::from("bad price")));
    assert_eq!(total("apple,3,0.5\nbread,1,2.25\n"), Ok(3.75));
    assert_eq!(total("apple,3,0.5\n\nbread,one,2.25"), Err(String::from("bad quantity")));
    assert_eq!(total(""), Ok(0.0));
}
@@hint
In `parse_line`: `let fields: Vec<&str> = line.split(',').collect();`, check `fields.len() != 3`, then parse `fields[1]` as `u32` and `fields[2]` as `f64` with `map_err(|_| String::from("bad quantity"))?`. In `total`: loop over lines, skip empty ones, `sum += parse_line(line)?;`.
@@solution
fn parse_line(line: &str) -> Result<f64, String> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != 3 {
        return Err(String::from("expected 3 fields"));
    }
    let quantity: u32 = fields[1].trim().parse().map_err(|_| String::from("bad quantity"))?;
    let price: f64 = fields[2].trim().parse().map_err(|_| String::from("bad price"))?;
    Ok(quantity as f64 * price)
}

fn total(list: &str) -> Result<f64, String> {
    let mut sum = 0.0;
    for line in list.lines() {
        if line.trim().is_empty() {
            continue;
        }
        sum += parse_line(line)?;
    }
    Ok(sum)
}

fn main() {
    println!("{:?}", total("apple,3,0.5\nbread,1,2.25\n"));
    println!("{:?}", total("apple,3,0.5\nbread,one,2.25"));
}
