@@lesson m7-movies Project: Movie Night
@@text
Let's use everything from this module to build the core of a useful little program: a **movie library** that tracks what you own, what you've watched, and suggests what to watch tonight.

You'll use:

- **derive** to make types comparable, sortable and printable,
- **closures** to search with any rule,
- **iterator chains** for searches, statistics and rankings,
- **`Display`** implementations for user-friendly output,
- a `BTreeMap` (like `HashMap`, but it keeps keys **sorted**) to count movies per genre.

## The types

```rust,norun
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Animation,
}

#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    year: u16,
    genre: Genre,
    rating: u8,      // 1 to 10
    watched: bool,
}

struct Library {
    movies: Vec<Movie>,
}
```

`Genre` derives `Ord` so it can be a `BTreeMap` key; the map will list genres in the order they're declared.

## Part 1: the library's methods

| Method | What it does |
|---|---|
| `Library::new()` | an empty library |
| `add(&mut self, title, year, genre, rating) -> bool` | adds an **unwatched** movie. If a movie with the same title already exists (ignoring case), adds nothing and returns `false`. |
| `mark_watched(&mut self, title) -> bool` | marks the movie with that title (ignoring case) as watched. `false` if not found. |
| `search(&self, query) -> Vec<&Movie>` | movies whose title **contains** the query, ignoring case |
| `matching(&self, test: impl Fn(&Movie) -> bool) -> Vec<&Movie>` | all movies for which the closure returns true |
| `top_rated(&self, n) -> Vec<&Movie>` | the `n` best movies: highest rating first, ties by title A→Z |
| `average_rating(&self) -> Option<f64>` | the average rating, or `None` if the library is empty |
| `count_by_genre(&self) -> BTreeMap<Genre, usize>` | how many movies per genre (genres with no movies are left out) |

`matching` is the most powerful: with it, `search`-like features become one-liners at the call site:

```rust,norun
let old_gems = library.matching(|m| m.year < 2000 && m.rating >= 8);
let unwatched_comedies = library.matching(|m| !m.watched && m.genre == Genre::Comedy);
```

@@exercise 🏆 Part 1: The library
Implement every method in the table. The types are given. Try to use iterator methods for the searches and statistics.
@@starter
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Animation,
}

#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    year: u16,
    genre: Genre,
    rating: u8,
    watched: bool,
}

struct Library {
    movies: Vec<Movie>,
}

impl Library {
    fn new() -> Self {
        todo!()
    }

    fn add(&mut self, title: &str, year: u16, genre: Genre, rating: u8) -> bool {
        todo!()
    }

    fn mark_watched(&mut self, title: &str) -> bool {
        todo!()
    }

    fn search(&self, query: &str) -> Vec<&Movie> {
        todo!()
    }

    fn matching(&self, test: impl Fn(&Movie) -> bool) -> Vec<&Movie> {
        todo!()
    }

    fn top_rated(&self, n: usize) -> Vec<&Movie> {
        todo!()
    }

    fn average_rating(&self) -> Option<f64> {
        todo!()
    }

    fn count_by_genre(&self) -> BTreeMap<Genre, usize> {
        todo!()
    }
}

fn main() {
    let mut lib = Library::new();
    lib.add("The Matrix", 1999, Genre::SciFi, 9);
    lib.add("Paddington 2", 2017, Genre::Comedy, 8);
    lib.add("Arrival", 2016, Genre::SciFi, 8);
    println!("{}", lib.add("the matrix", 1999, Genre::SciFi, 9));
    lib.mark_watched("ARRIVAL");
    for m in lib.top_rated(2) {
        println!("{} {}", m.title, m.rating);
    }
    println!("{:?}", lib.average_rating());
    println!("{:?}", lib.count_by_genre());
    println!("{}", lib.matching(|m| m.watched).len());
}
@@tests
fn sample() -> Library {
    let mut lib = Library::new();
    assert!(lib.add("The Matrix", 1999, Genre::SciFi, 9));
    assert!(lib.add("Spirited Away", 2001, Genre::Animation, 9));
    assert!(lib.add("Paddington 2", 2017, Genre::Comedy, 8));
    assert!(lib.add("Arrival", 2016, Genre::SciFi, 8));
    assert!(lib.add("The Room", 2003, Genre::Drama, 3));
    lib
}

fn titles(movies: Vec<&Movie>) -> Vec<String> {
    movies.iter().map(|m| m.title.clone()).collect()
}

#[test]
fn new_library_is_empty() {
    let lib = Library::new();
    assert!(lib.movies.is_empty());
    assert_eq!(lib.average_rating(), None);
    assert!(lib.count_by_genre().is_empty());
    assert!(lib.top_rated(3).is_empty());
}

#[test]
fn add_creates_unwatched_movies_and_rejects_duplicates() {
    let mut lib = sample();
    assert_eq!(lib.movies.len(), 5);
    assert!(lib.movies.iter().all(|m| !m.watched));
    assert!(!lib.add("THE ROOM", 2003, Genre::Drama, 3), "duplicate titles are rejected, ignoring case");
    assert_eq!(lib.movies.len(), 5);
    let m = &lib.movies[0];
    assert_eq!((m.title.as_str(), m.year, m.genre, m.rating), ("The Matrix", 1999, Genre::SciFi, 9));
}

#[test]
fn mark_watched_ignores_case() {
    let mut lib = sample();
    assert!(lib.mark_watched("the matrix"));
    assert!(lib.movies[0].watched);
    assert!(!lib.movies[1].watched);
    assert!(!lib.mark_watched("Titanic"));
}

#[test]
fn search_is_case_insensitive_substring() {
    let lib = sample();
    assert_eq!(titles(lib.search("the")), ["The Matrix", "The Room"]);
    assert_eq!(titles(lib.search("AWAY")), ["Spirited Away"]);
    assert!(lib.search("zzz").is_empty());
}

#[test]
fn matching_uses_the_closure() {
    let lib = sample();
    assert_eq!(titles(lib.matching(|m| m.year > 2010)), ["Paddington 2", "Arrival"]);
    assert_eq!(lib.matching(|_| true).len(), 5);
    assert!(lib.matching(|_| false).is_empty());
}

#[test]
fn top_rated_sorts_by_rating_then_title() {
    let lib = sample();
    assert_eq!(titles(lib.top_rated(3)), ["Spirited Away", "The Matrix", "Arrival"]);
    assert_eq!(lib.top_rated(10).len(), 5);
    assert!(lib.top_rated(0).is_empty());
}

#[test]
fn average_and_genre_counts() {
    let lib = sample();
    assert_eq!(lib.average_rating(), Some(7.4));
    let counts = lib.count_by_genre();
    assert_eq!(counts.get(&Genre::SciFi), Some(&2));
    assert_eq!(counts.get(&Genre::Drama), Some(&1));
    assert_eq!(counts.get(&Genre::Action), None);
    assert_eq!(counts.len(), 4);
}
@@hint
- `add`: check `self.movies.iter().any(|m| m.title.to_lowercase() == title.to_lowercase())` first.
- `mark_watched`: `self.movies.iter_mut().find(...)` gives `Option<&mut Movie>`; use `match` or `if let Some(m) = ...`.
- `search`: lowercase the query once, then `self.matching(|m| m.title.to_lowercase().contains(&q))`!
- `matching`: `self.movies.iter().filter(|m| test(m)).collect()`
- `top_rated`: collect references into a `Vec<&Movie>`, `sort_by(|a, b| b.rating.cmp(&a.rating).then_with(|| a.title.cmp(&b.title)))`, then `.into_iter().take(n).collect()`.
- `average_rating`: return `None` if empty; otherwise sum the ratings as `f64` (`m.rating as f64`) and divide by the count.
- `count_by_genre`: `*counts.entry(m.genre).or_insert(0) += 1;` works on a `BTreeMap` just like a `HashMap`.
@@solution
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Animation,
}

#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    year: u16,
    genre: Genre,
    rating: u8,
    watched: bool,
}

struct Library {
    movies: Vec<Movie>,
}

impl Library {
    fn new() -> Self {
        Library { movies: Vec::new() }
    }

    fn add(&mut self, title: &str, year: u16, genre: Genre, rating: u8) -> bool {
        let lower = title.to_lowercase();
        if self.movies.iter().any(|m| m.title.to_lowercase() == lower) {
            return false;
        }
        self.movies.push(Movie { title: title.to_string(), year, genre, rating, watched: false });
        true
    }

    fn mark_watched(&mut self, title: &str) -> bool {
        let lower = title.to_lowercase();
        match self.movies.iter_mut().find(|m| m.title.to_lowercase() == lower) {
            Some(movie) => {
                movie.watched = true;
                true
            }
            None => false,
        }
    }

    fn search(&self, query: &str) -> Vec<&Movie> {
        let q = query.to_lowercase();
        self.matching(|m| m.title.to_lowercase().contains(&q))
    }

    fn matching(&self, test: impl Fn(&Movie) -> bool) -> Vec<&Movie> {
        self.movies.iter().filter(|m| test(m)).collect()
    }

    fn top_rated(&self, n: usize) -> Vec<&Movie> {
        let mut sorted: Vec<&Movie> = self.movies.iter().collect();
        sorted.sort_by(|a, b| b.rating.cmp(&a.rating).then_with(|| a.title.cmp(&b.title)));
        sorted.into_iter().take(n).collect()
    }

    fn average_rating(&self) -> Option<f64> {
        if self.movies.is_empty() {
            return None;
        }
        let total: f64 = self.movies.iter().map(|m| m.rating as f64).sum();
        Some(total / self.movies.len() as f64)
    }

    fn count_by_genre(&self) -> BTreeMap<Genre, usize> {
        let mut counts = BTreeMap::new();
        for m in &self.movies {
            *counts.entry(m.genre).or_insert(0) += 1;
        }
        counts
    }
}

fn main() {
    let mut lib = Library::new();
    lib.add("The Matrix", 1999, Genre::SciFi, 9);
    lib.add("Paddington 2", 2017, Genre::Comedy, 8);
    lib.add("Arrival", 2016, Genre::SciFi, 8);
    println!("{}", lib.add("the matrix", 1999, Genre::SciFi, 9));
    lib.mark_watched("ARRIVAL");
    for m in lib.top_rated(2) {
        println!("{} {}", m.title, m.rating);
    }
    println!("{:?}", lib.average_rating());
    println!("{:?}", lib.count_by_genre());
    println!("{}", lib.matching(|m| m.watched).len());
}

@@exercise 🏆 Part 2: Tonight's report
Now make the library pleasant to read, and smart:

1. Implement `Display` for `Genre`: `Action`, `Comedy`, `Drama`, `Sci-Fi`, `Animation`.
2. Implement `Display` for `Movie`: `Title (year) [Genre] ★rating`, followed by ` ✔` if watched. For example `The Matrix (1999) [Sci-Fi] ★9 ✔`.
3. Write the method `pick_for_tonight(&self, genre: Option<Genre>, min_rating: u8) -> Option<&Movie>`: the best **unwatched** movie with at least `min_rating`, and of the given genre if it's `Some`. "Best" means the same order as `top_rated`.
4. Write `report(lib: &Library) -> String`, producing exactly the output shown in the expected output (run it to compare). Each line ends with `\n`. The average has one decimal place. "Still to watch" lists unwatched titles **alphabetically**, separated by `, `.

Part 1's methods are filled in for you.
@@starter
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Animation,
}

#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    year: u16,
    genre: Genre,
    rating: u8,
    watched: bool,
}

struct Library {
    movies: Vec<Movie>,
}

impl Library {
    fn new() -> Self {
        Library { movies: Vec::new() }
    }

    fn add(&mut self, title: &str, year: u16, genre: Genre, rating: u8) -> bool {
        let lower = title.to_lowercase();
        if self.movies.iter().any(|m| m.title.to_lowercase() == lower) {
            return false;
        }
        self.movies.push(Movie { title: title.to_string(), year, genre, rating, watched: false });
        true
    }

    fn mark_watched(&mut self, title: &str) -> bool {
        let lower = title.to_lowercase();
        match self.movies.iter_mut().find(|m| m.title.to_lowercase() == lower) {
            Some(movie) => {
                movie.watched = true;
                true
            }
            None => false,
        }
    }

    fn matching(&self, test: impl Fn(&Movie) -> bool) -> Vec<&Movie> {
        self.movies.iter().filter(|m| test(m)).collect()
    }

    fn top_rated(&self, n: usize) -> Vec<&Movie> {
        let mut sorted: Vec<&Movie> = self.movies.iter().collect();
        sorted.sort_by(|a, b| b.rating.cmp(&a.rating).then_with(|| a.title.cmp(&b.title)));
        sorted.into_iter().take(n).collect()
    }

    fn average_rating(&self) -> Option<f64> {
        if self.movies.is_empty() {
            return None;
        }
        let total: f64 = self.movies.iter().map(|m| m.rating as f64).sum();
        Some(total / self.movies.len() as f64)
    }

    fn count_by_genre(&self) -> BTreeMap<Genre, usize> {
        let mut counts = BTreeMap::new();
        for m in &self.movies {
            *counts.entry(m.genre).or_insert(0) += 1;
        }
        counts
    }

    fn pick_for_tonight(&self, genre: Option<Genre>, min_rating: u8) -> Option<&Movie> {
        todo!()
    }
}

// implement Display for Genre and Movie

fn report(lib: &Library) -> String {
    todo!()
}

fn main() {
    let mut lib = Library::new();
    lib.add("The Matrix", 1999, Genre::SciFi, 9);
    lib.add("Spirited Away", 2001, Genre::Animation, 9);
    lib.add("Paddington 2", 2017, Genre::Comedy, 8);
    lib.add("Mad Max: Fury Road", 2015, Genre::Action, 8);
    lib.add("Arrival", 2016, Genre::SciFi, 8);
    lib.add("The Room", 2003, Genre::Drama, 3);
    lib.add("Toy Story", 1995, Genre::Animation, 8);
    lib.mark_watched("The Matrix");
    lib.mark_watched("Toy Story");

    print!("{}", report(&lib));

    match lib.pick_for_tonight(Some(Genre::SciFi), 7) {
        Some(movie) => println!("Tonight: {movie}"),
        None => println!("Nothing to watch tonight."),
    }
    match lib.pick_for_tonight(Some(Genre::Drama), 5) {
        Some(movie) => println!("Tonight: {movie}"),
        None => println!("No good dramas left."),
    }
}
@@expected
🎬 7 movies, 2 watched, average rating 7.6
By genre:
  Action: 1
  Comedy: 1
  Drama: 1
  Sci-Fi: 2
  Animation: 2
Top 3:
  1. Spirited Away (2001) [Animation] ★9
  2. The Matrix (1999) [Sci-Fi] ★9 ✔
  3. Arrival (2016) [Sci-Fi] ★8
Still to watch: Arrival, Mad Max: Fury Road, Paddington 2, Spirited Away, The Room
Tonight: Arrival (2016) [Sci-Fi] ★8
No good dramas left.
@@tests
#[test]
fn display_formats() {
    assert_eq!(Genre::SciFi.to_string(), "Sci-Fi");
    assert_eq!(Genre::Action.to_string(), "Action");
    let mut m = Movie { title: String::from("Up"), year: 2009, genre: Genre::Animation, rating: 8, watched: false };
    assert_eq!(m.to_string(), "Up (2009) [Animation] ★8");
    m.watched = true;
    assert_eq!(m.to_string(), "Up (2009) [Animation] ★8 ✔");
}

#[test]
fn picking() {
    let mut lib = Library::new();
    assert_eq!(lib.pick_for_tonight(None, 0), None);
    lib.add("B", 2000, Genre::Comedy, 7);
    lib.add("A", 2000, Genre::Comedy, 7);
    lib.add("C", 2000, Genre::Drama, 9);
    assert_eq!(lib.pick_for_tonight(None, 0).map(|m| m.title.as_str()), Some("C"));
    assert_eq!(lib.pick_for_tonight(Some(Genre::Comedy), 0).map(|m| m.title.as_str()), Some("A"));
    lib.mark_watched("C");
    assert_eq!(lib.pick_for_tonight(None, 8), None);
}

#[test]
fn empty_report() {
    let lib = Library::new();
    let text = report(&lib);
    assert!(text.starts_with("🎬 0 movies, 0 watched"), "an empty library should still report: {text}");
}
@@requires
impl fmt::Display for Genre
impl fmt::Display for Movie
@@hint
- `pick_for_tonight`: `self.top_rated(self.movies.len())` gives every movie in the right order. Then `.into_iter().find(|m| !m.watched && m.rating >= min_rating && (genre.is_none() || genre == Some(m.genre)))`.
- In `report`, build a `String` with `out.push_str(&format!(...))`. For an empty library the average can't be computed: use `lib.average_rating().unwrap_or(0.0)`.
- Still to watch: collect the unwatched titles as `Vec<&str>` (use `.as_str()`), `.sort()`, then `.join(", ")`.
@@solution
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Animation,
}

#[derive(Debug, Clone, PartialEq)]
struct Movie {
    title: String,
    year: u16,
    genre: Genre,
    rating: u8,
    watched: bool,
}

struct Library {
    movies: Vec<Movie>,
}

impl Library {
    fn new() -> Self {
        Library { movies: Vec::new() }
    }

    fn add(&mut self, title: &str, year: u16, genre: Genre, rating: u8) -> bool {
        let lower = title.to_lowercase();
        if self.movies.iter().any(|m| m.title.to_lowercase() == lower) {
            return false;
        }
        self.movies.push(Movie { title: title.to_string(), year, genre, rating, watched: false });
        true
    }

    fn mark_watched(&mut self, title: &str) -> bool {
        let lower = title.to_lowercase();
        match self.movies.iter_mut().find(|m| m.title.to_lowercase() == lower) {
            Some(movie) => {
                movie.watched = true;
                true
            }
            None => false,
        }
    }

    fn matching(&self, test: impl Fn(&Movie) -> bool) -> Vec<&Movie> {
        self.movies.iter().filter(|m| test(m)).collect()
    }

    fn top_rated(&self, n: usize) -> Vec<&Movie> {
        let mut sorted: Vec<&Movie> = self.movies.iter().collect();
        sorted.sort_by(|a, b| b.rating.cmp(&a.rating).then_with(|| a.title.cmp(&b.title)));
        sorted.into_iter().take(n).collect()
    }

    fn average_rating(&self) -> Option<f64> {
        if self.movies.is_empty() {
            return None;
        }
        let total: f64 = self.movies.iter().map(|m| m.rating as f64).sum();
        Some(total / self.movies.len() as f64)
    }

    fn count_by_genre(&self) -> BTreeMap<Genre, usize> {
        let mut counts = BTreeMap::new();
        for m in &self.movies {
            *counts.entry(m.genre).or_insert(0) += 1;
        }
        counts
    }

    fn pick_for_tonight(&self, genre: Option<Genre>, min_rating: u8) -> Option<&Movie> {
        self.top_rated(self.movies.len())
            .into_iter()
            .find(|m| !m.watched && m.rating >= min_rating && (genre.is_none() || genre == Some(m.genre)))
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Genre::Action => "Action",
            Genre::Comedy => "Comedy",
            Genre::Drama => "Drama",
            Genre::SciFi => "Sci-Fi",
            Genre::Animation => "Animation",
        };
        write!(f, "{name}")
    }
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}) [{}] ★{}", self.title, self.year, self.genre, self.rating)?;
        if self.watched {
            write!(f, " ✔")?;
        }
        Ok(())
    }
}

fn report(lib: &Library) -> String {
    let mut out = String::new();
    let watched = lib.matching(|m| m.watched).len();
    out.push_str(&format!(
        "🎬 {} movies, {} watched, average rating {:.1}\n",
        lib.movies.len(),
        watched,
        lib.average_rating().unwrap_or(0.0)
    ));

    out.push_str("By genre:\n");
    for (genre, count) in lib.count_by_genre() {
        out.push_str(&format!("  {genre}: {count}\n"));
    }

    out.push_str("Top 3:\n");
    for (i, movie) in lib.top_rated(3).iter().enumerate() {
        out.push_str(&format!("  {}. {}\n", i + 1, movie));
    }

    let mut to_watch: Vec<&str> = lib.matching(|m| !m.watched).iter().map(|m| m.title.as_str()).collect();
    to_watch.sort();
    out.push_str(&format!("Still to watch: {}\n", to_watch.join(", ")));
    out
}

fn main() {
    let mut lib = Library::new();
    lib.add("The Matrix", 1999, Genre::SciFi, 9);
    lib.add("Spirited Away", 2001, Genre::Animation, 9);
    lib.add("Paddington 2", 2017, Genre::Comedy, 8);
    lib.add("Mad Max: Fury Road", 2015, Genre::Action, 8);
    lib.add("Arrival", 2016, Genre::SciFi, 8);
    lib.add("The Room", 2003, Genre::Drama, 3);
    lib.add("Toy Story", 1995, Genre::Animation, 8);
    lib.mark_watched("The Matrix");
    lib.mark_watched("Toy Story");

    print!("{}", report(&lib));

    match lib.pick_for_tonight(Some(Genre::SciFi), 7) {
        Some(movie) => println!("Tonight: {movie}"),
        None => println!("Nothing to watch tonight."),
    }
    match lib.pick_for_tonight(Some(Genre::Drama), 5) {
        Some(movie) => println!("Tonight: {movie}"),
        None => println!("No good dramas left."),
    }
}

@@text
## What you built

This little library is a real **data model**: types that describe the world, and methods that answer questions about it. Notice how short most methods are. Closures and iterators let each method say *what* it computes rather than *how* to loop.

Here's where it's going: in Module 8 you'll learn to **save** data like this to a file as JSON, so it survives closing the program. In Module 9 you'll put a **GUI** on a model just like this one. The model itself won't need to change, because it never prints anything. Good design pays off.

**Ideas to extend it** (all doable with what you know):

- `Library::remove(&mut self, title) -> Option<Movie>` (look at `Vec::position` and `Vec::remove`)
- a `Rating` newtype that refuses values outside 1–10 (`fn new(n: u8) -> Option<Rating>`)
- `by_decade(&self) -> BTreeMap<u16, Vec<&Movie>>` grouping movies into 1990, 2000, 2010…
- a "surprise me" pick using `rand` in a Cargo project
