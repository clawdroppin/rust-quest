@@lesson m8-todo Project: A Command-Line To-Do App
@@text
Let's build a complete, genuinely useful program: a to-do list for your terminal that **remembers your tasks** between runs. It brings together the whole course so far: structs and enums, `Result`-based error handling, modules in files, tests, crates, file storage with JSON, and a clean layered design.

```text
=== TO-DO ===
  1. [ ] Buy milk
  2. [x] Learn Rust ownership
  3. [ ] Call grandma
2 left. Type 'help' for commands.
> done 1
✔ Completed: Buy milk
```

It works in two ways:

- **interactive**: `cargo run` and type commands in a loop;
- **one-shot**: `cargo run -- add Buy milk` runs a single command and exits. Handy for scripts. After `cargo build --release`, you can even call the `.exe` directly.

## The design

```text
src/
├── main.rs      ← interface: the loop, printing, running commands
├── command.rs   ← parse_command (from the "Structuring a program" lesson)
├── tasks.rs     ← model + logic: Task, TaskList, with tests
└── storage.rs   ← load/save todo.json with serde
```

`tasks.rs` is where your work is: some methods of `TaskList` are `todo!()`, and tests describe how they must behave. Everything else is written, so read through it: every line uses something you've learned.

## Task ids

Each task gets a permanent **id** number when it's created. Ids are never reused, even after a task is removed: `TaskList` remembers the `next_id` to hand out. That way, "task 3" always means the same task. The list shows ids, so the user types `done 3` for the task shown as `3.`.

## Your steps

1. Create the project, open it in VS Code.
2. Run `cargo test`. Several tests fail with "not yet implemented".
3. Implement the `todo!()` methods in `tasks.rs` until all tests pass.
4. `cargo run` and use your to-do app! Quit, run it again: your tasks are still there. Open `todo.json` to see them.

@@project todo_cli To-Do (command line)
A to-do list that saves to todo.json. Implement the missing methods in `src/tasks.rs`, guided by the tests.
@@file Cargo.toml
[package]
name = "todo_cli"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
@@file src/main.rs
mod command;
mod storage;
mod tasks;

use command::{Command, parse_command};
use std::io::{self, Write};
use tasks::TaskList;

const SAVE_FILE: &str = "todo.json";

const HELP: &str = "Commands:
  add <text>     add a task
  done <id>      mark a task as done
  remove <id>    delete a task (also: rm)
  clear          delete all finished tasks
  list           show the list (also: ls)
  help           show this help
  quit           exit (also: exit)";

fn print_list(list: &TaskList) {
    println!();
    println!("=== TO-DO ===");
    if list.tasks.is_empty() {
        println!("  (nothing to do. Add a task with: add <text>)");
    }
    for task in &list.tasks {
        let mark = if task.done { "x" } else { " " };
        println!("  {:>2}. [{}] {}", task.id, mark, task.text);
    }
    println!("{} left. Type 'help' for commands.", list.pending_count());
}

/// Runs one command. Returns true if the list changed (and should be saved).
fn execute(list: &mut TaskList, command: Command) -> bool {
    match command {
        Command::Add(text) => {
            let id = list.add(&text);
            println!("Added task {id}.");
            true
        }
        Command::Done(id) => match list.complete(id) {
            Ok(text) => {
                println!("✔ Completed: {text}");
                true
            }
            Err(e) => {
                println!("{e}");
                false
            }
        },
        Command::Remove(id) => match list.remove(id) {
            Some(task) => {
                println!("Removed: {}", task.text);
                true
            }
            None => {
                println!("There is no task {id}.");
                false
            }
        },
        Command::Clear => {
            let n = list.clear_done();
            println!("Removed {n} finished task(s).");
            n > 0
        }
        Command::List => {
            print_list(list);
            false
        }
        Command::Help => {
            println!("{HELP}");
            false
        }
        Command::Quit => false,
    }
}

fn save(list: &TaskList) {
    if let Err(e) = storage::save(SAVE_FILE, list) {
        println!("⚠ {e}");
    }
}

fn main() {
    let mut list = match storage::load(SAVE_FILE) {
        Ok(list) => list,
        Err(e) => {
            println!("⚠ {e}");
            println!("Fix or delete the file, then try again.");
            return;
        }
    };

    // One-shot mode: `cargo run -- add Buy milk`
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        match parse_command(&args.join(" ")) {
            Ok(command) => {
                if execute(&mut list, command) {
                    save(&list);
                }
            }
            Err(e) => println!("{e}"),
        }
        return;
    }

    // Interactive mode
    print_list(&list);
    loop {
        print!("> ");
        io::stdout().flush().expect("could not flush stdout");
        let mut line = String::new();
        if io::stdin().read_line(&mut line).expect("could not read input") == 0 {
            break; // end of input
        }
        match parse_command(&line) {
            Ok(Command::Quit) => break,
            Ok(command) => {
                if execute(&mut list, command) {
                    save(&list);
                    print_list(&list);
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    println!("Bye!");
}
@@file src/command.rs
//! Turning typed text into commands.

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(String),
    Done(u32),
    Remove(u32),
    List,
    Clear,
    Help,
    Quit,
}

fn parse_id(text: &str) -> Result<u32, String> {
    text.parse().map_err(|_| format!("'{text}' is not a task number"))
}

/// Parses one line of input, like `add Buy milk` or `done 3`.
pub fn parse_command(line: &str) -> Result<Command, String> {
    let line = line.trim();
    let (word, rest) = match line.split_once(' ') {
        Some((w, r)) => (w, r.trim()),
        None => (line, ""),
    };
    match word.to_lowercase().as_str() {
        "add" => {
            if rest.is_empty() {
                Err(String::from("what should I add? Try: add Buy milk"))
            } else {
                Ok(Command::Add(rest.to_string()))
            }
        }
        "done" => Ok(Command::Done(parse_id(rest)?)),
        "remove" | "rm" => Ok(Command::Remove(parse_id(rest)?)),
        "list" | "ls" => Ok(Command::List),
        "clear" => Ok(Command::Clear),
        "help" => Ok(Command::Help),
        "quit" | "exit" => Ok(Command::Quit),
        "" => Err(String::from("type a command, or 'help'")),
        other => Err(format!("unknown command '{other}'. Type 'help' for a list.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(parse_command("add Buy milk"), Ok(Command::Add(String::from("Buy milk"))));
        assert_eq!(parse_command(" DONE 3 "), Ok(Command::Done(3)));
        assert_eq!(parse_command("rm 2"), Ok(Command::Remove(2)));
        assert!(parse_command("done x").is_err());
        assert!(parse_command("add").is_err());
        assert!(parse_command("jump").is_err());
    }
}
@@file src/storage.rs
//! Saving and loading the task list as JSON.

use crate::tasks::TaskList;
use std::fs;
use std::io::ErrorKind;

/// Loads the list from `path`. A missing file is not an error: it gives an empty list.
pub fn load(path: &str) -> Result<TaskList, String> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).map_err(|e| format!("{path} is damaged: {e}")),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(TaskList::new()),
        Err(e) => Err(format!("could not read {path}: {e}")),
    }
}

/// Saves the list to `path`, replacing the old file.
pub fn save(path: &str, list: &TaskList) -> Result<(), String> {
    let json = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("could not save {path}: {e}"))
}
@@file src/tasks.rs
//! The to-do list itself: data and rules. Nothing in here prints anything.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    /// An empty list. The first task will get id 1.
    pub fn new() -> Self {
        TaskList { tasks: Vec::new(), next_id: 1 }
    }

    /// Adds a new, unfinished task and returns its id.
    pub fn add(&mut self, text: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task { id, text: text.to_string(), done: false });
        id
    }

    /// How many tasks are not done yet.
    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.done).count()
    }

    /// Marks the task with this id as done and returns its text.
    /// Errors: `"There is no task <id>."` if it doesn't exist,
    /// `"Task <id> is already done."` if it was done before.
    pub fn complete(&mut self, id: u32) -> Result<String, String> {
        todo!()
    }

    /// Removes the task with this id and returns it, or None if there is no such task.
    pub fn remove(&mut self, id: u32) -> Option<Task> {
        todo!()
    }

    /// Removes every finished task. Returns how many were removed.
    pub fn clear_done(&mut self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> TaskList {
        let mut list = TaskList::new();
        list.add("one");
        list.add("two");
        list.add("three");
        list
    }

    #[test]
    fn ids_start_at_one_and_count_up() {
        let list = sample();
        let ids: Vec<u32> = list.tasks.iter().map(|t| t.id).collect();
        assert_eq!(ids, [1, 2, 3]);
        assert_eq!(list.pending_count(), 3);
    }

    #[test]
    fn complete_marks_the_task_done() {
        let mut list = sample();
        assert_eq!(list.complete(2), Ok(String::from("two")));
        assert!(list.tasks[1].done);
        assert!(!list.tasks[0].done);
        assert_eq!(list.pending_count(), 2);
    }

    #[test]
    fn complete_reports_errors() {
        let mut list = sample();
        assert_eq!(list.complete(9), Err(String::from("There is no task 9.")));
        list.complete(1).unwrap();
        assert_eq!(list.complete(1), Err(String::from("Task 1 is already done.")));
    }

    #[test]
    fn remove_returns_the_removed_task() {
        let mut list = sample();
        let removed = list.remove(2).expect("task 2 exists");
        assert_eq!(removed.text, "two");
        assert_eq!(list.tasks.len(), 2);
        assert_eq!(list.remove(2), None, "it's gone now");
        assert_eq!(list.remove(42), None);
    }

    #[test]
    fn ids_are_never_reused() {
        let mut list = sample();
        list.remove(3);
        assert_eq!(list.add("four"), 4);
    }

    #[test]
    fn clear_done_removes_only_finished_tasks() {
        let mut list = sample();
        list.complete(1).unwrap();
        list.complete(3).unwrap();
        assert_eq!(list.clear_done(), 2);
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].text, "two");
        assert_eq!(list.clear_done(), 0);
    }
}

@@text
## Hints

- **`complete`**: find the task with `self.tasks.iter_mut().find(|t| t.id == id)`. That gives an `Option<&mut Task>`. Use `let Some(task) = ... else { return Err(...) };`, then check `task.done`.
- **`remove`**: `self.tasks.iter().position(|t| t.id == id)` gives the index (an `Option<usize>`). Then `self.tasks.remove(index)` removes and returns the task. Combine them with `.map(...)`, or use `let Some(index) = ... else { return None; };`.
- **`clear_done`**: remember the length, `self.tasks.retain(|t| !t.done)`, and return the difference.

## Take it further

You now have a real tool. Make it *yours*:

- **Priorities**: add a `priority` field (an enum: Low/Normal/High, with `#[serde(default)]` so old files still load!) and an `add! <text>` command for important tasks. Show high-priority tasks first.
- **Edit**: `edit 3 New text` replaces a task's text. New `Command` variant, new method, new test.
- **Undo**: `undo 3` marks a task as not done.
- **Due dates**: store a date as a `String` like `"2026-11-05"`, and show overdue tasks in a separate section.
- **Install it**: `cargo install --path .` builds a release version and puts `todo_cli.exe` on your PATH, so you can type `todo_cli add Something` in any terminal. (The list is saved in whatever folder you run it from; making it always use one file in your home folder is a nice challenge.)

In the next module, you'll build a **graphical** app with the same layered design, so this structure will feel very familiar.

@@quiz
Why does `TaskList` keep a `next_id` instead of using `tasks.len() + 1` for new ids?
- It's faster
* Because after removing a task, `len() + 1` could produce an id that was already used
- serde requires it
? Ids must stay unique and permanent, even after removals.

@@quiz
Where would you add the code for a new `undo <id>` command?
* A new `Command` variant and parsing in `command.rs`, a method with tests in `tasks.rs`, and a `match` arm in `main.rs`'s `execute`
- Only in `main.rs`
- In `storage.rs`
? Each layer handles its own part. The compiler will point at the `match` that's missing the new variant.
