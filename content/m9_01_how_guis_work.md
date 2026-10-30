@@lesson m9-guis How GUIs Work
@@text
Welcome to the final module! You'll build real desktop applications with windows, buttons and text boxes, ending with a polished, genuinely useful app you can use every day: **Pocket Ledger**, a personal budget tracker.

First, let's understand how graphical programs work, because they're structured differently from the command-line programs you've written so far.

## From "run top to bottom" to "react to events"

A command-line program is in control: it prints, waits for a line of input, continues. A GUI program can't work like that. The user might click any button, type in any box, resize the window, at any time, in any order.

So every GUI program is built around an **event loop**:

```text
loop {
    1. collect what happened: mouse moves, clicks, key presses, window resized...
    2. update the program's state based on those events
    3. draw the window to show the current state
}
```

This runs many times per second. Each pass through the loop is called a **frame**. You've seen this shape before: it's the "show → input → update → repeat" loop from the to-do app, just much faster.

## Two styles: retained mode and immediate mode

Most GUI toolkits (the ones behind web pages, Windows apps, phone apps) are **retained mode**: you create widget objects once ("a button at this position, with this label"), they stay in memory, and you attach **callbacks** — functions to call when something happens to them. You then have to keep the widgets and your data in sync yourself, which is a common source of bugs.

In this module, you'll use **egui**, an **immediate mode** GUI library. It works differently:

> Every frame, your code describes the **whole** user interface from scratch, based on your current data. Widgets tell you immediately what happened to them.

```rust,norun
// This runs every frame:
ui.label(format!("Clicks: {}", self.clicks));
if ui.button("Click me").clicked() {   // draws the button AND tells you if it was clicked
    self.clicks += 1;
}
```

There are no button objects to keep and no callbacks to register. The UI is just a function of your data: change `self.clicks`, and the next frame shows the new number. Because your data is the only "truth", the UI can never get out of sync with it.

It may sound wasteful to redraw everything 60 times a second, but computers are very fast at this (it's how video games work). egui also only redraws when something actually changes, like the mouse moving or a key being pressed.

## Your program's shape with egui

```rust,norun
struct MyApp {
    // ALL the state of your program lives here:
    // the data (tasks, transactions...) and the UI state (text being typed, selected tab...)
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // describe the whole window, reading and changing self
    }
}
```

- **eframe** is the framework that opens the window and runs the event loop. It calls your `ui` method every frame.
- **egui** is the library of widgets (buttons, labels, text boxes...) you use inside `ui`.
- Your `MyApp` struct implements the `eframe::App` **trait**, so eframe knows it can call `ui` on it. Traits in action!

## Where you are heading

Here's the plan for this module:

1. **Hello, egui**: your first window.
2. **Widgets and layout**: a tour of everything you can put in a window.
3. **App state**: how to structure a GUI app's data, with a shopping-list app.
4. **Pocket Ledger, part 1**: the data model, test-driven.
5. **Pocket Ledger, part 2**: the window, form and transaction list.
6. **Pocket Ledger, part 3**: charts, budgets and saving to disk.
7. **Shipping it**: building a real `.exe` you can use and share.

The exercise below simulates an event loop in plain Rust, so you can see the idea before any window appears.

@@quiz
In an immediate mode GUI like egui, when is the UI described?
- Once, when the program starts
* Every frame, from the current state of your data
- Only when a button is clicked
? Each frame your code describes the whole UI again. That's why the UI always matches your data.

@@quiz
How do you find out that a button was clicked in egui?
- Register a callback function for it
* Check the value returned when you draw it: `if ui.button("OK").clicked() { ... }`
- Read a global "last clicked" variable
? Widgets report what happened to them right when you draw them.

@@quiz
Where does all of an egui app's state live?
* In your app struct (the one that implements `eframe::App`)
- Inside the widgets
- In global variables
? Data and UI state are fields of your struct. The widgets are just drawn from them every frame.

@@exercise A tiny event loop
Let's simulate a GUI without a window. The "app" has a click counter and a text box. Events arrive one at a time; for each one, `update` changes the state, and then `view` "draws" it as a line of text.

Write:

- `update(state: &mut State, event: &Event)`:
  - `Click` → add 1 to `clicks`
  - `Type(c)` → add the character to `text`
  - `Backspace` → remove the last character of `text` (if any)
  - `Reset` → clicks back to 0 and text empty
- `view(state: &State) -> String`, returning exactly `[clicks: 2] text: "hi"` (with the current values).

`main` is the event loop.
@@starter
enum Event {
    Click,
    Type(char),
    Backspace,
    Reset,
}

struct State {
    clicks: u32,
    text: String,
}

fn update(state: &mut State, event: &Event) {
    todo!()
}

fn view(state: &State) -> String {
    todo!()
}

fn main() {
    let events = [
        Event::Click,
        Event::Type('h'),
        Event::Type('i'),
        Event::Click,
        Event::Backspace,
        Event::Type('o'),
        Event::Reset,
        Event::Backspace,
    ];
    let mut state = State { clicks: 0, text: String::new() };
    println!("{}", view(&state));
    for event in &events {
        update(&mut state, event);
        println!("{}", view(&state));
    }
}
@@expected
[clicks: 0] text: ""
[clicks: 1] text: ""
[clicks: 1] text: "h"
[clicks: 1] text: "hi"
[clicks: 2] text: "hi"
[clicks: 2] text: "h"
[clicks: 2] text: "ho"
[clicks: 0] text: ""
[clicks: 0] text: ""
@@tests
#[test]
fn checks() {
    let mut s = State { clicks: 5, text: String::from("ab") };
    update(&mut s, &Event::Backspace);
    update(&mut s, &Event::Backspace);
    update(&mut s, &Event::Backspace);
    assert_eq!(s.text, "");
    update(&mut s, &Event::Type('é'));
    assert_eq!(view(&s), "[clicks: 5] text: \"é\"");
}
@@hint
`match event { Event::Click => state.clicks += 1, Event::Type(c) => state.text.push(*c), Event::Backspace => { state.text.pop(); } ... }`

For `view`, `{:?}` on a `String` adds the quotes for you: `format!("[clicks: {}] text: {:?}", ...)`.
@@solution
enum Event {
    Click,
    Type(char),
    Backspace,
    Reset,
}

struct State {
    clicks: u32,
    text: String,
}

fn update(state: &mut State, event: &Event) {
    match event {
        Event::Click => state.clicks += 1,
        Event::Type(c) => state.text.push(*c),
        Event::Backspace => {
            state.text.pop();
        }
        Event::Reset => {
            state.clicks = 0;
            state.text.clear();
        }
    }
}

fn view(state: &State) -> String {
    format!("[clicks: {}] text: {:?}", state.clicks, state.text)
}

fn main() {
    let events = [
        Event::Click,
        Event::Type('h'),
        Event::Type('i'),
        Event::Click,
        Event::Backspace,
        Event::Type('o'),
        Event::Reset,
        Event::Backspace,
    ];
    let mut state = State { clicks: 0, text: String::new() };
    println!("{}", view(&state));
    for event in &events {
        update(&mut state, event);
        println!("{}", view(&state));
    }
}
