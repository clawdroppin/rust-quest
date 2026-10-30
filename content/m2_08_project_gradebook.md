@@lesson m2-gradebook Project: Grade Book
@@text
Time to put the whole module together in one program. You'll build a **grade book** that turns raw test scores into a full class report.

This is bigger than anything you've built so far, so we'll practise the problem-solving process from Module 1.

## 1. Understand the problem

**Input:** a list of students. Each student has a name and three test scores (0–100).

**Output:** a report showing each student's average and letter grade, the class average, the best student, how many of each grade there are, and a ranking from best to worst.

## 2. Choose the data structures

This is a new and important question: **how should the data be stored?** Good data structures make the code easy; bad ones make it painful.

- Each student is a pair of name + scores → a **tuple** `(&str, Vec<u32>)`.
- All students → a **vector** of those tuples.
- Counting grades → a **HashMap** `char → count`.
- Ranking → a **vector** of `(average, name)` tuples that we can sort.

## 3. Break it into pieces

- `average(scores) -> f64` — one student's average.
- `letter(avg) -> char` — the letter grade for an average.
- `main` — loops over the students, prints rows, collects the totals, and prints the summary.

Small functions are easy to get right on their own. Write and test them first.

## 4. One new tool: sorting by a value

To rank students, we need to sort `(average, name)` tuples by the average, **highest first**. Since averages are `f64`s, we use `sort_by` with a closure that says how to compare two items:

```rust
fn main() {
    let mut ranking = vec![(74.0_f64, "Ben"), (91.7, "Ana"), (85.3, "Cleo")];
    ranking.sort_by(|a, b| b.0.total_cmp(&a.0));   // b before a = descending
    println!("{:?}", ranking);
}
```

`a` and `b` are two tuples being compared; `.0` is the average. Comparing `b` to `a` (instead of `a` to `b`) reverses the order. Closures are fully explained in Module 7 — for now, this line is a recipe you can reuse.

## Tips

- Format each row as `{:<8}{:>5.1}  {}` — name left-aligned in 8, average right-aligned in 5 with 1 decimal, two spaces, letter.
- To print the grade counts in A–F order even though a HashMap has no order, loop over `['A', 'B', 'C', 'D', 'F']` and look each one up — use `0` if it's missing: `grade_counts.get(&g).copied().unwrap_or(0)`.
- To print names joined by `", "`, collect them into a `Vec<&str>` and use `join`.

@@exercise 🏆 Module project: Grade book
Write the grade book. The students are already in the starter code. Grade thresholds: **A** ≥ 90, **B** ≥ 80, **C** ≥ 70, **D** ≥ 60, otherwise **F**.

The class average is the average of the students' averages. Your output must be exactly:

```text
=== Class Report ===
Name      Avg  Grade
Ana      91.7  A
Ben      74.0  C
Cleo     85.3  B
Dan      58.3  F
Eve      80.3  B
--------------------
Class average: 77.9
Best student: Ana (91.7)
Grades: A=1 B=2 C=1 D=0 F=1
Ranking: Ana, Cleo, Eve, Ben, Dan
```

Requirements: write the functions `average` and `letter`, and use a `HashMap` for the grade counts.
@@starter
use std::collections::HashMap;

fn main() {
    let students = vec![
        ("Ana", vec![95, 88, 92]),
        ("Ben", vec![70, 75, 77]),
        ("Cleo", vec![80, 90, 86]),
        ("Dan", vec![50, 60, 65]),
        ("Eve", vec![80, 82, 79]),
    ];

    println!("=== Class Report ===");
}
@@expected
=== Class Report ===
Name      Avg  Grade
Ana      91.7  A
Ben      74.0  C
Cleo     85.3  B
Dan      58.3  F
Eve      80.3  B
--------------------
Class average: 77.9
Best student: Ana (91.7)
Grades: A=1 B=2 C=1 D=0 F=1
Ranking: Ana, Cleo, Eve, Ben, Dan
@@requires
fn average
fn letter
HashMap::new()
@@hint
In the main loop, `for (name, scores) in &students { ... }`: compute `avg` and `grade`, print the row, add `avg` to a running sum, count the grade with `*grade_counts.entry(grade).or_insert(0) += 1;`, and `ranking.push((avg, *name));`. After the loop, sort `ranking` and print the summary. The best student is `ranking[0]` after sorting.
@@solution
use std::collections::HashMap;

fn average(scores: &[u32]) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    let mut total = 0;
    for s in scores {
        total += s;
    }
    total as f64 / scores.len() as f64
}

fn letter(avg: f64) -> char {
    if avg >= 90.0 {
        'A'
    } else if avg >= 80.0 {
        'B'
    } else if avg >= 70.0 {
        'C'
    } else if avg >= 60.0 {
        'D'
    } else {
        'F'
    }
}

fn main() {
    let students = vec![
        ("Ana", vec![95, 88, 92]),
        ("Ben", vec![70, 75, 77]),
        ("Cleo", vec![80, 90, 86]),
        ("Dan", vec![50, 60, 65]),
        ("Eve", vec![80, 82, 79]),
    ];

    println!("=== Class Report ===");
    println!("{:<8}{:>5}  {}", "Name", "Avg", "Grade");

    let mut ranking = Vec::new();
    let mut grade_counts = HashMap::new();
    let mut sum_of_averages = 0.0;

    for (name, scores) in &students {
        let avg = average(scores);
        let grade = letter(avg);
        println!("{:<8}{:>5.1}  {}", name, avg, grade);

        sum_of_averages += avg;
        *grade_counts.entry(grade).or_insert(0) += 1;
        ranking.push((avg, *name));
    }

    ranking.sort_by(|a, b| b.0.total_cmp(&a.0));

    println!("--------------------");
    println!("Class average: {:.1}", sum_of_averages / students.len() as f64);
    let (best_avg, best_name) = ranking[0];
    println!("Best student: {best_name} ({best_avg:.1})");

    let mut parts = Vec::new();
    for g in ['A', 'B', 'C', 'D', 'F'] {
        let count = grade_counts.get(&g).copied().unwrap_or(0);
        parts.push(format!("{g}={count}"));
    }
    println!("Grades: {}", parts.join(" "));

    let mut names = Vec::new();
    for (_, name) in &ranking {
        names.push(*name);
    }
    println!("Ranking: {}", names.join(", "));
}

@@text
## 🎉 Module 2 complete!

You can now handle **lots of data**: tuples, arrays, vectors, strings, hash maps and grids. Almost every real program is built from these.

You've also met a few mysterious symbols along the way: `&`, `&mut`, `*`, and errors like "value moved". The next module finally explains them. It's the most important module for understanding Rust specifically.
