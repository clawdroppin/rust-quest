@@lesson m6-testing Writing Tests
@@text
Every exercise in this course has been checked by **tests**: small pieces of code that call your functions and verify the results. Now you'll write your own. Testing is how professional programmers make sure code works — and *keeps* working as it changes.

## Why test?

- You **know** your code works, instead of hoping.
- When you change something later, the tests immediately tell you if you broke something that used to work (a **regression**).
- Writing tests forces you to think about **edge cases**: empty lists, zero, negative numbers, very large values.
- Tests are examples that show other people (and future you) how the code is meant to be used.

## A test is a function with `#[test]`

```rust,norun
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }
}
```

- `#[test]` marks a function as a test. Test functions take no parameters.
- A test **passes** if it finishes, and **fails** if it panics.
- Tests live in a module called `tests` (by convention), marked `#[cfg(test)]`, which means "only compile this when testing". It isn't included in your normal program.
- `use super::*;` brings everything from the parent module (your code) into the test module.

Run all tests with **`cargo test`**:

```text
running 2 tests
test tests::adds_positive_numbers ... ok
test tests::adds_negative_numbers ... ok

test result: ok. 2 passed; 0 failed
```

In this app, **✔ Check** also runs any tests you write in your code, alongside the exercise's own checks.

## The assertion macros

| Macro | Passes when |
|---|---|
| `assert!(condition)` | the condition is `true` |
| `assert_eq!(left, right)` | the two values are equal |
| `assert_ne!(left, right)` | the two values are **not** equal |

All of them accept an optional message, which is shown when they fail:

```rust,norun
assert_eq!(grade(89), 'B', "89 should be a B, got {}", grade(89));
```

When `assert_eq!` fails, it shows both values (`left` and `right`), which usually makes the bug obvious.

## Testing for panics and errors

```rust,norun
#[test]
#[should_panic(expected = "divide by zero")]
fn panics_on_zero() {
    divide(1, 0);
}

#[test]
fn rejects_bad_input() {
    assert!(parse_age("abc").is_err());
    assert_eq!(parse_age("42"), Ok(42));
}
```

## What makes a good test?

- **One idea per test**, with a descriptive name: `empty_list_has_no_average`, not `test3`.
- **Test edge cases**: empty, zero, one element, negative, maximum, duplicate values, invalid input.
- **Arrange, Act, Assert**: set up the data, call the function, check the result.
- Test the **public behaviour**, not internal details. That way you can rewrite the inside freely.

## Test-driven development

A popular workflow is to write the **tests first**, watch them fail, then write code until they pass. It forces you to decide exactly what the code should do before you write it. The second exercise below works this way, and so does this module's project.

## In your Cargo project

Open the **calculator** project from the files lesson, and add a test module at the bottom of `calc.rs`:

```rust,norun
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(evaluate("2 + 3", 0.0), Ok(5.0));
    }

    #[test]
    fn uses_memory() {
        assert_eq!(evaluate("mem * 2", 21.0), Ok(42.0));
    }

    #[test]
    fn rejects_division_by_zero() {
        assert_eq!(evaluate("1 / 0", 0.0), Err(CalcError::DivideByZero));
    }
}
```

Run `cargo test` in the terminal. Then break `evaluate` on purpose (change `x + y` to `x - y`) and run the tests again to see a failure. In VS Code, rust-analyzer also shows a little **▶ Run Test** button above each test.

@@quiz
When does a test fail?
- When it returns `false`
* When it panics (for example, a failing `assert_eq!`)
- When it prints something
? Tests pass by finishing normally and fail by panicking.

@@quiz
What does `#[cfg(test)]` on the tests module do?
* It only compiles that module when running tests, so it's not part of the normal program
- It runs the tests automatically when the program starts
- It makes the tests run faster
? `cfg` means "configuration": the module only exists in test builds.

@@exercise Write your own tests
The function `clamp` is given and works correctly. Your job is to write a test module for it with **at least three tests**:

1. a value inside the range stays the same,
2. a value below the range becomes `min`,
3. a value above the range becomes `max`.

Bonus: test what happens exactly at `min` and `max`.

When you press ✔ Check, your tests run too: all of them must pass.
@@starter
/// Limits `value` to the range `min..=max`.
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn main() {
    println!("{}", clamp(15, 0, 10));
}

// write your tests below
@@tests
#[test]
fn hidden_check() {
    assert_eq!(clamp(5, 0, 10), 5);
}
@@requires
#[cfg(test)]
#[test]
assert_eq!
use super::*;
@@hint
```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_range_unchanged() {
        assert_eq!(clamp(5, 0, 10), 5);
    }
    // two more like this...
}
```
@@solution
/// Limits `value` to the range `min..=max`.
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn main() {
    println!("{}", clamp(15, 0, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inside_range_unchanged() {
        assert_eq!(clamp(5, 0, 10), 5);
    }

    #[test]
    fn below_range_becomes_min() {
        assert_eq!(clamp(-3, 0, 10), 0);
    }

    #[test]
    fn above_range_becomes_max() {
        assert_eq!(clamp(99, 0, 10), 10);
    }

    #[test]
    fn edges_are_kept() {
        assert_eq!(clamp(0, 0, 10), 0);
        assert_eq!(clamp(10, 0, 10), 10);
    }
}

@@exercise Make the tests pass
This time the tests are written, and the function isn't. Read the tests carefully: they are the **specification**. Then write `slugify`, which turns a title into a URL-friendly "slug".

Don't change the tests!
@@starter
fn slugify(title: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", slugify("Hello, World!"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_and_joins_words_with_dashes() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn removes_punctuation() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn keeps_digits() {
        assert_eq!(slugify("Top 10 Tips"), "top-10-tips");
    }

    #[test]
    fn collapses_extra_spaces() {
        assert_eq!(slugify("  Rust   is  fun  "), "rust-is-fun");
    }

    #[test]
    fn empty_title_gives_empty_slug() {
        assert_eq!(slugify(""), "");
    }
}
@@tests
#[test]
fn hidden_check() {
    assert_eq!(slugify("Learn RUST, today!"), "learn-rust-today");
}
@@requires
fn removes_punctuation
fn collapses_extra_spaces
@@hint
Split the title with `split_whitespace()`. For each word, keep only the alphanumeric characters, lowercased (build a new `String`). Skip words that end up empty. Collect the words in a `Vec<String>` and `join("-")`.
@@solution
fn slugify(title: &str) -> String {
    let mut words = Vec::new();
    for word in title.split_whitespace() {
        let mut clean = String::new();
        for c in word.chars() {
            if c.is_alphanumeric() {
                clean.push(c.to_ascii_lowercase());
            }
        }
        if !clean.is_empty() {
            words.push(clean);
        }
    }
    words.join("-")
}

fn main() {
    println!("{}", slugify("Hello, World!"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_and_joins_words_with_dashes() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn removes_punctuation() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn keeps_digits() {
        assert_eq!(slugify("Top 10 Tips"), "top-10-tips");
    }

    #[test]
    fn collapses_extra_spaces() {
        assert_eq!(slugify("  Rust   is  fun  "), "rust-is-fun");
    }

    #[test]
    fn empty_title_gives_empty_slug() {
        assert_eq!(slugify(""), "");
    }
}
