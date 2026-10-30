@@lesson m3-slices Slices Revisited: &str and &[T]
@@text
A **slice** is a reference to a **continuous part** of a collection. It doesn't own anything; it's a "window" onto data that someone else owns.

## Slices of lists: `&[T]`

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let window = &data[2..5];
    println!("{:?}", window);   // [3, 4, 5]
}
```

```text
  data   ──►  [ 1 | 2 | 3 | 4 | 5 | 6 ]   (heap)
                      ▲─────────▲
  window: pointer to element 2, length 3
```

A slice is just a pointer and a length: no copying. Since it's a borrow, all the borrowing rules apply: while `window` is in use, you can't change `data`.

## String slices: `&str`

The same idea for text. `&str` is a slice of a string:

```rust
fn main() {
    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world: &str = &s[6..];
    println!("{hello} | {world}");
}
```

(Remember: string slice ranges are in **bytes**, and must not cut a character in half.)

**String literals are slices too!** `"hello"` is a `&str` pointing at text stored inside the program's executable file itself. That's why literals are `&str` and not `String`.

## `String` vs `&str` — the full picture

| | `String` | `&str` |
|---|---|---|
| Owns its text? | yes | no — borrows it |
| Can grow/change? | yes (if `mut`) | no |
| Where's the text? | heap | anywhere: a `String`'s heap memory, or inside the program |
| Make one from the other | `s.to_string()`, `String::from(s)` | `&my_string`, `my_string.as_str()` |

Rules of thumb:

- **Parameters** that only read text: `&str`.
- **Struct fields and return values** that need to keep the text: `String`.
- Same for lists: read with `&[T]`, own with `Vec<T>`.

## Returning a slice

A function can return a slice *into its input*. This finds the first word without copying anything:

```rust
fn first_word(text: &str) -> &str {
    for (i, c) in text.char_indices() {
        if c == ' ' {
            return &text[..i];
        }
    }
    text  // no space: the whole text is one word
}

fn main() {
    let sentence = String::from("hello brave new world");
    let word = first_word(&sentence);
    println!("{word}");
}
```

`char_indices()` is like `chars().enumerate()`, but gives the **byte** position of each character — exactly what string slicing needs.

The returned `&str` borrows from `sentence`. So while `word` is in use, `sentence` can't be changed or dropped. The compiler guarantees `word` can never point at text that no longer exists:

```rust
fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let mut sentence = String::from("hello world");
    let word = first_word(&sentence);
    sentence.clear();          // ❌ error: sentence is still borrowed by word
    println!("{word}");
}
```

## Useful slice methods

Slices have most of the read-only methods you used on vectors and strings:

```rust
fn main() {
    let v = [5, 3, 8, 1];
    let s: &[i32] = &v;
    println!("{} {:?} {:?} {}", s.len(), s.first(), s.iter().max(), s.contains(&8));
    let (left, right) = s.split_at(2);
    println!("{:?} {:?}", left, right);

    let t = "  padded  ";
    println!("[{}]", t.trim());          // trim returns a &str slice of t
}
```

@@quiz
What does a slice contain?
- A copy of the selected elements
* A pointer to the first selected element and a length
- The whole original collection
? A slice is a lightweight view: pointer + length. Nothing is copied.

@@quiz
What is the best type for a parameter that only reads text?
- `String`
* `&str`
- `&String`
? `&str` accepts literals, borrowed `String`s and slices of either.

@@exercise Middle part
Write `middle`, which returns a slice of the list **without its first and last elements**. If the list has 2 or fewer elements, return an empty slice (`&[]`).
@@starter
fn middle(list: &[i32]) -> &[i32] {
    todo!()
}

fn main() {
    println!("{:?}", middle(&[1, 2, 3, 4, 5]));
}
@@tests
#[test]
fn checks() {
    assert_eq!(middle(&[1, 2, 3, 4, 5]), &[2, 3, 4]);
    assert_eq!(middle(&[1, 2, 3]), &[2]);
    assert_eq!(middle(&[1, 2]), &[] as &[i32]);
    assert_eq!(middle(&[]), &[] as &[i32]);
}
@@hint
`if list.len() <= 2 { return &[]; }` and then `&list[1..list.len() - 1]`.
@@solution
fn middle(list: &[i32]) -> &[i32] {
    if list.len() <= 2 {
        return &[];
    }
    &list[1..list.len() - 1]
}

fn main() {
    println!("{:?}", middle(&[1, 2, 3, 4, 5]));
}

@@exercise File extension
Write `extension`, which returns the part of a file name **after the last dot**, as a slice of the input. If there's no dot, return `""`.

`"photo.jpeg"` → `"jpeg"`, `"archive.tar.gz"` → `"gz"`, `"README"` → `""`
@@starter
fn extension(file_name: &str) -> &str {
    todo!()
}

fn main() {
    println!("{}", extension("archive.tar.gz"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(extension("photo.jpeg"), "jpeg");
    assert_eq!(extension("archive.tar.gz"), "gz", "use the LAST dot");
    assert_eq!(extension("README"), "");
    assert_eq!(extension("main.rs"), "rs");
}
@@hint
`file_name.rfind('.')` returns `Some(byte_index)` of the **last** dot (the r is for "reverse"), or `None`. Use `match`/`if let`, and slice from `index + 1` to the end.
@@solution
fn extension(file_name: &str) -> &str {
    match file_name.rfind('.') {
        Some(i) => &file_name[i + 1..],
        None => "",
    }
}

fn main() {
    println!("{}", extension("archive.tar.gz"));
}

@@exercise Initials
Write `initials`, which takes a full name and returns the initials as a new `String`, with a dot after each: `"Ada King Lovelace"` → `"A.K.L."`. Words may be separated by several spaces.

Think about the types: the input is borrowed (`&str`), but the result is **new** text, so it must be an owned `String`.
@@starter
fn initials(name: &str) -> String {
    todo!()
}

fn main() {
    println!("{}", initials("Ada King Lovelace"));
}
@@tests
#[test]
fn checks() {
    assert_eq!(initials("Ada King Lovelace"), "A.K.L.");
    assert_eq!(initials("grace   hopper"), "g.h.", "keep the original case, ignore extra spaces");
    assert_eq!(initials(""), "");
}
@@hint
Loop over `name.split_whitespace()`; for each word, push its first character (`word.chars().next()`) and a `'.'`.
@@solution
fn initials(name: &str) -> String {
    let mut result = String::new();
    for word in name.split_whitespace() {
        if let Some(c) = word.chars().next() {
            result.push(c);
            result.push('.');
        }
    }
    result
}

fn main() {
    println!("{}", initials("Ada King Lovelace"));
}
