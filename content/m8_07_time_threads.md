@@lesson m8-time-threads Time and Doing Things at Once
@@text
Two things almost every app needs sooner or later:

1. **Time**: measure how long something took, wait a bit, run a timer.
2. **Doing two things at once**: a long download or file scan shouldn't freeze the whole app.

Rust's standard library handles both, and Rust's ownership rules make the second one much safer than in most languages.

## Measuring time: `Instant` and `Duration`

`Instant::now()` takes a reading from a stopwatch. `.elapsed()` tells you how much time has passed since then, as a `Duration`.

```rust
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let mut total: u64 = 0;
    for i in 0..5_000_000 {
        total += i % 7;
    }
    let took = start.elapsed();
    println!("total = {total}");
    println!("that took {} ms", took.as_millis());

    let break_time = Duration::from_secs(5 * 60);
    println!("a 5 minute break is {} seconds", break_time.as_secs());
}
```

A `Duration` is a length of time. Make one with `Duration::from_secs`, `from_millis` or `from_secs_f32`, read it with `as_secs()`, `as_millis()` or `as_secs_f32()`. You can add and subtract durations and compare them with `<` and `>`.

An `Instant` is only good for measuring; it isn't a date or time of day. For "what's today's date" you'd use `std::time::SystemTime` (seconds since 1970) or a crate like `chrono`.

## Waiting: `thread::sleep`

`std::thread::sleep(duration)` pauses the program:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    for n in (1..=3).rev() {
        println!("{n}…");
        thread::sleep(Duration::from_millis(300));
    }
    println!("Go!");
}
```

**Never sleep in a GUI's update function!** The GUI can only redraw between updates, so a sleeping update freezes the whole window. GUIs ask to be woken up later instead; you'll see that in the GUI lesson "Timers and Background Work".

## Threads: doing two things at once

A **thread** is a separate line of execution inside your program. Your program starts with one, the *main thread*. `thread::spawn` starts another one, running a closure:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let worker = thread::spawn(|| {
        for i in 1..=3 {
            println!("  worker: step {i}");
            thread::sleep(Duration::from_millis(100));
        }
        42 // the thread's result
    });

    for i in 1..=3 {
        println!("main: step {i}");
        thread::sleep(Duration::from_millis(100));
    }

    let answer = worker.join().unwrap(); // wait for it to finish
    println!("the worker returned {answer}");
}
```

Run it a few times: the lines from the two threads interleave, and not always in the same order. Both really run at the same time (on different CPU cores).

- `spawn` returns a **`JoinHandle`**. `handle.join()` waits until the thread has finished, and gives you its return value. (It's a `Result`, because the thread might have panicked.)
- When `main` ends, the whole program ends, even if other threads are still running. So `join` the threads whose work you need.

## `move`: giving data to a thread

A thread might outlive the function that started it, so it can't just *borrow* local variables. You **move** them into the thread's closure with the `move` keyword:

```rust
use std::thread;

fn main() {
    let names = vec!["ana".to_string(), "ben".to_string()];
    let handle = thread::spawn(move || {
        // `names` now belongs to this thread
        names.iter().map(|n| n.len()).sum::<usize>()
    });
    println!("total letters: {}", handle.join().unwrap());
    // println!("{names:?}"); // error: `names` was moved into the thread
}
```

This is ownership doing its job: two threads can never accidentally change the same data at the same time, because the compiler makes sure each value has one owner. Rust calls this **fearless concurrency**.

## Channels: threads sending messages

How does a worker thread report back while it's still running? With a **channel**: a pipe with a sending end (`Sender`) and a receiving end (`Receiver`). `mpsc` stands for *multiple producer, single consumer*: you can clone the sender for many threads, but there's one receiver.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for percent in [25, 50, 75, 100] {
            thread::sleep(Duration::from_millis(150)); // pretend to work
            tx.send(percent).unwrap();
        }
        // `tx` is dropped here, which closes the channel
    });

    // `for` over a receiver waits for each message, until the channel is closed.
    for percent in rx {
        println!("progress: {percent}%");
    }
    println!("done!");
}
```

The receiver has three ways to read:

| Method | What it does |
|---|---|
| `rx.recv()` | **waits** for the next message (`Err` once all senders are gone) |
| `for msg in rx` | the same, in a loop |
| `rx.try_recv()` | **doesn't wait**: `Ok(msg)` if one is ready, otherwise `Err` right away |

`try_recv` is the one GUIs use: every frame, check if the worker sent something, and carry on drawing either way. This exact pattern, a worker thread plus a channel, is how Rust Quest itself compiles your code without freezing, and how the AI helper streams its answers.

## Sharing data: `Arc<Mutex<T>>` (a quick look)

Sometimes several threads really need the *same* data, like one shared counter. Two tools work together:

- `Arc<T>` is like `Rc<T>` (shared ownership, from the smart pointers lesson) but safe to use across threads ("**A**tomically **R**eference **C**ounted").
- `Mutex<T>` (*mutual exclusion*) guards the data: a thread must `lock()` it first, and only one thread can hold the lock at a time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("count = {}", *counter.lock().unwrap());
}
```

Always 4000. In many languages, forgetting the lock gives a wrong count that's different on every run (a *data race*); in Rust, forgetting it is a **compile error**. Prefer channels when you can: "share memory by sending messages" is simpler to get right.

@@quiz
Why do you usually need `move` in `thread::spawn(move || ...)`?
- To make the thread run faster
* The thread may outlive the current function, so it must own the data it uses instead of borrowing it
- `move` starts the thread immediately
? Without `move`, the closure would borrow locals that might be gone before the thread finishes.

@@quiz
A GUI wants to check, every frame, whether a worker thread has sent a result, without ever freezing. Which method should it use?
- `rx.recv()`
* `rx.try_recv()`
- `handle.join()`
? `recv` and `join` both wait. `try_recv` returns immediately whether or not something arrived.

@@exercise Format a duration
Timers show time as `minutes:seconds`. Write `mm_ss`, which turns a `Duration` into text like `"04:05"`: minutes and seconds, each with **two digits** (ignore any fraction of a second). Minutes can go above 59 (`"75:00"` is fine).
@@starter
use std::time::Duration;

fn mm_ss(d: Duration) -> String {
    todo!()
}

fn main() {
    println!("{}", mm_ss(Duration::from_secs(245)));
    println!("{}", mm_ss(Duration::from_secs(25 * 60)));
    println!("{}", mm_ss(Duration::from_millis(9_900)));
}
@@expected
04:05
25:00
00:09
@@tests
#[test]
fn checks() {
    assert_eq!(mm_ss(Duration::ZERO), "00:00");
    assert_eq!(mm_ss(Duration::from_secs(59)), "00:59");
    assert_eq!(mm_ss(Duration::from_secs(60)), "01:00");
    assert_eq!(mm_ss(Duration::from_secs(75 * 60)), "75:00");
}
@@hint
`let secs = d.as_secs();` then minutes are `secs / 60` and seconds `secs % 60`. `{:02}` prints a number with at least two digits, padding with a 0.
@@solution
use std::time::Duration;

fn mm_ss(d: Duration) -> String {
    let secs = d.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

fn main() {
    println!("{}", mm_ss(Duration::from_secs(245)));
    println!("{}", mm_ss(Duration::from_secs(25 * 60)));
    println!("{}", mm_ss(Duration::from_millis(9_900)));
}

@@exercise Split the work
`parallel_sum` should add up all the numbers using **two threads**: one sums the first half of the list, the other the second half, and the results are added together.

The list is moved in as a `Vec`. `split_off` already cuts it into two `Vec`s. Spawn a thread for each half (use `move`), then `join` both and return the sum.
@@starter
use std::thread;

fn parallel_sum(mut numbers: Vec<u64>) -> u64 {
    let second_half = numbers.split_off(numbers.len() / 2);
    let first_half = numbers;
    todo!()
}

fn main() {
    let numbers: Vec<u64> = (1..=1000).collect();
    println!("{}", parallel_sum(numbers));
}
@@expected
500500
@@tests
#[test]
fn checks() {
    assert_eq!(parallel_sum(vec![]), 0);
    assert_eq!(parallel_sum(vec![7]), 7);
    assert_eq!(parallel_sum(vec![1, 2, 3, 4, 5]), 15);
}
@@requires
thread::spawn
join
@@hint
`let a = thread::spawn(move || first_half.iter().sum::<u64>());` Do the same for the second half, then `a.join().unwrap() + b.join().unwrap()`.
@@solution
use std::thread;

fn parallel_sum(mut numbers: Vec<u64>) -> u64 {
    let second_half = numbers.split_off(numbers.len() / 2);
    let first_half = numbers;
    let a = thread::spawn(move || first_half.iter().sum::<u64>());
    let b = thread::spawn(move || second_half.iter().sum::<u64>());
    a.join().unwrap() + b.join().unwrap()
}

fn main() {
    let numbers: Vec<u64> = (1..=1000).collect();
    println!("{}", parallel_sum(numbers));
}

@@exercise Progress reports
A worker thread "downloads" some files and reports on a channel. Complete `main`:

1. Create a channel.
2. Spawn a thread that, for each name in `files`, sends a message `format!("downloaded {name}")`.
3. In the main thread, receive and print every message, then print `all done`.

The worker should get its own sender (move `tx` into the closure), so that the channel closes when the worker ends.
@@starter
use std::sync::mpsc;
use std::thread;

fn main() {
    let files = vec!["a.txt", "b.png", "c.pdf"];
    // your code here
}
@@expected
downloaded a.txt
downloaded b.png
downloaded c.pdf
all done
@@requires
mpsc::channel
thread::spawn
@@hint
`let (tx, rx) = mpsc::channel();` then `thread::spawn(move || for name in files { tx.send(format!("downloaded {name}")).unwrap(); });` and finally `for msg in rx { println!("{msg}"); }`.
@@solution
use std::sync::mpsc;
use std::thread;

fn main() {
    let files = vec!["a.txt", "b.png", "c.pdf"];
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for name in files {
            tx.send(format!("downloaded {name}")).unwrap();
        }
    });
    for msg in rx {
        println!("{msg}");
    }
    println!("all done");
}
@@text
That's the core of concurrent programming in Rust: `spawn` + `move` to start work, `join` to wait for a result, channels to send messages, and `Arc<Mutex<…>>` for the rare case where data must be shared. In the GUI module you'll combine these with egui to keep an app responsive while it works.
