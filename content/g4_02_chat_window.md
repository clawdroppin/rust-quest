@@lesson g-ac2 Local AI Chat 2: The Window and the Worker
@@text
Now the window. The big challenge: a request to a model can take seconds, even minutes, and the window must stay responsive the whole time, showing the answer as it streams in, with a working Stop button. You know the solution from "Timers and Background Work": a **worker thread** and a **channel**.

If you're continuing from step 1, add the new files below with **Add new file(s)** and copy the new `src/main.rs` from the starting files. Or reset to this step's files, which also give you a finished `chat.rs`.

@@project ai_chat Local AI Chat, step 2: the window and the worker
Adds `src/client.rs` (the network code, with the streaming worker for you to write) and `src/app.rs` (a bare window to build on). `src/main.rs` now opens the window.
@@file Cargo.toml
[package]
name = "ai_chat"
version = "0.1.0"
edition = "2024"

[dependencies]
eframe = { version = "0.37", default-features = false, features = ["default_fonts", "glow"] }
egui_commonmark = { version = "0.26", default-features = false, features = ["pulldown_cmark"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
ureq = { version = "3", default-features = false, features = ["json"] }

@@file src/main.rs
mod app;
mod chat;
mod client;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 680.0]).with_min_inner_size([560.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("AI Chat", options, Box::new(|_cc| Ok(Box::new(app::ChatApp::new()))))
}

@@file src/app.rs
//! The window. In this step you build it up piece by piece, following the lesson.

use crate::chat::{Conversation, Role};
use eframe::egui;

pub struct ChatApp {
    server: String,
    model: String,
    conversation: Conversation,
    input: String,
}

impl ChatApp {
    pub fn new() -> Self {
        ChatApp {
            server: "http://localhost:1234/v1".to_string(),
            model: String::new(),
            conversation: Conversation::new(),
            input: String::new(),
        }
    }
}

impl eframe::App for ChatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("input").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Server: {} | Model: {}", self.server, self.model));
            });
            ui.text_edit_multiline(&mut self.input);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            for message in &self.conversation.messages {
                let who = if message.role == Role::User { "You" } else { "AI" };
                ui.label(format!("{who}: {}", message.content));
            }
        });
    }
}

@@file src/chat.rs
//! Conversations, and the JSON the AI server speaks. No GUI and no network code here.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub title: String,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation { title: "New chat".to_string(), messages: Vec::new() }
    }
}

/// Ready-made system prompts: they tell the model who to be.
pub const PERSONAS: [(&str, &str); 4] = [
    ("Helpful assistant", "You are a friendly, helpful assistant. Answer clearly and concisely."),
    (
        "Rust tutor",
        "You are a patient Rust tutor for beginners. Explain simply, with short code examples. \
         Prefer hints over full solutions, unless asked.",
    ),
    (
        "Writing editor",
        "You are a careful editor. Improve the user's text for clarity and grammar, keep their voice, \
         and briefly list what you changed.",
    ),
    (
        "Translator",
        "You translate the user's text. If it's in English, translate it into German; otherwise into \
         English. Reply with the translation only.",
    ),
];

/// A short title for a conversation, from its first question: the first few words, at most
/// `max_chars` characters, without line breaks.
pub fn make_title(question: &str, max_chars: usize) -> String {
    let words: Vec<&str> = question.split_whitespace().collect();
    if words.is_empty() {
        return "New chat".to_string();
    }
    let mut title = String::new();
    for word in words {
        let extra = if title.is_empty() { 0 } else { 1 };
        if title.chars().count() + extra + word.chars().count() > max_chars {
            if title.is_empty() {
                title = word.chars().take(max_chars).collect();
            }
            title.push('…');
            return title;
        }
        if extra == 1 {
            title.push(' ');
        }
        title.push_str(word);
    }
    title
}

/// The JSON body of a chat request: the system prompt, then the last `max_history` messages.
pub fn request_body(model: &str, system_prompt: &str, messages: &[Message], max_history: usize) -> Value {
    let mut list = Vec::new();
    if !system_prompt.trim().is_empty() {
        list.push(json!({ "role": "system", "content": system_prompt }));
    }
    let skip = messages.len().saturating_sub(max_history);
    for m in &messages[skip..] {
        list.push(json!({ "role": m.role, "content": m.content }));
    }
    json!({ "model": model, "messages": list, "stream": true })
}

/// What one line of a streamed answer means.
#[derive(Debug, PartialEq)]
pub enum StreamLine {
    /// A piece of the answer.
    Text(String),
    /// The answer is complete.
    Done,
    /// The server reported an error.
    Error(String),
    /// Empty lines, comments, chunks without text: nothing to do.
    Nothing,
}

/// Understands one line of a "server-sent events" stream, like
/// `data: {"choices":[{"delta":{"content":"Hel"}}]}`.
pub fn parse_stream_line(line: &str) -> StreamLine {
    let Some(data) = line.trim().strip_prefix("data:") else { return StreamLine::Nothing };
    let data = data.trim();
    if data == "[DONE]" {
        return StreamLine::Done;
    }
    let Ok(value) = serde_json::from_str::<Value>(data) else { return StreamLine::Nothing };
    if let Some(error) = value.get("error") {
        let message = error["message"].as_str().map(String::from).unwrap_or_else(|| error.to_string());
        return StreamLine::Error(message);
    }
    match value["choices"][0]["delta"]["content"].as_str() {
        Some(text) if !text.is_empty() => StreamLine::Text(text.to_string()),
        _ => StreamLine::Nothing,
    }
}

/// The model names from the answer to `GET /models`.
pub fn parse_models(json: &str) -> Result<Vec<String>, String> {
    let value: Value = serde_json::from_str(json).map_err(|_| "The server's answer wasn't valid JSON.".to_string())?;
    let list = value["data"].as_array().ok_or("The server's answer has no list of models.")?;
    Ok(list.iter().filter_map(|m| m["id"].as_str().map(String::from)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(role: Role, content: &str) -> Message {
        Message { role, content: content.to_string() }
    }

    #[test]
    fn titles() {
        assert_eq!(make_title("How do I\nsort a Vec?", 40), "How do I sort a Vec?");
        assert_eq!(make_title("one two three four", 9), "one two…");
        assert_eq!(make_title("   ", 10), "New chat");
        assert_eq!(make_title("Supercalifragilistic", 5), "Super…");
    }

    #[test]
    fn builds_request() {
        let messages = [msg(Role::User, "a"), msg(Role::Assistant, "b"), msg(Role::User, "c")];
        let body = request_body("qwen", "Be nice.", &messages, 2);
        assert_eq!(body["model"], "qwen");
        assert_eq!(body["stream"], true);
        let list = body["messages"].as_array().unwrap();
        assert_eq!(list.len(), 3); // the system prompt + the last 2
        assert_eq!(list[0]["role"], "system");
        assert_eq!(list[1]["role"], "assistant");
        assert_eq!(list[2]["content"], "c");
        // No system prompt: no system message.
        assert_eq!(request_body("m", " ", &messages, 10)["messages"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn parses_stream() {
        assert_eq!(parse_stream_line(r#"data: {"choices":[{"delta":{"content":"Hel"}}]}"#), StreamLine::Text("Hel".into()));
        assert_eq!(parse_stream_line("data: [DONE]"), StreamLine::Done);
        assert_eq!(parse_stream_line(""), StreamLine::Nothing);
        assert_eq!(parse_stream_line(": keep-alive"), StreamLine::Nothing);
        assert_eq!(parse_stream_line(r#"data: {"choices":[{"delta":{"role":"assistant"}}]}"#), StreamLine::Nothing);
        assert_eq!(parse_stream_line(r#"data: {"error":{"message":"no model"}}"#), StreamLine::Error("no model".into()));
    }

    #[test]
    fn parses_models() {
        let json = r#"{"data":[{"id":"qwen2.5-coder-7b"},{"id":"llama-3.2-3b"}]}"#;
        assert_eq!(parse_models(json).unwrap(), ["qwen2.5-coder-7b", "llama-3.2-3b"]);
        assert!(parse_models("oops").is_err());
    }

    #[test]
    fn roles_are_saved_in_lowercase() {
        assert_eq!(serde_json::to_string(&Role::Assistant).unwrap(), "\"assistant\"");
    }
}

@@file src/client.rs
//! Talking to the AI server over HTTP, on worker threads so the window never freezes.

use crate::chat::{self, StreamLine};
use eframe::egui;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;

/// What a worker thread tells the app.
pub enum Event {
    Models(Result<Vec<String>, String>),
    Text(String),
    Done,
    Error(String),
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(4)))
        // Big models can think for a while before the first word.
        .timeout_per_read(Some(Duration::from_secs(300)))
        // We want to read the error message of a 4xx/5xx answer ourselves.
        .http_status_as_error(false)
        .build()
        .into()
}

fn url(base_url: &str, path: &str) -> String {
    format!("{}/{path}", base_url.trim().trim_end_matches('/'))
}

fn explain(error: ureq::Error, base_url: &str) -> String {
    match error {
        ureq::Error::Io(_) | ureq::Error::ConnectionFailed | ureq::Error::HostNotFound => {
            format!("Can't reach {base_url}. Is LM Studio's server (or Ollama) running?")
        }
        ureq::Error::Timeout(_) => "The server took too long to answer.".to_string(),
        other => other.to_string(),
    }
}

/// Asks the server for its models, on a worker thread.
pub fn fetch_models(base_url: String, ctx: egui::Context) -> Receiver<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let result = agent()
            .get(&url(&base_url, "models"))
            .call()
            .map_err(|e| explain(e, &base_url))
            .and_then(|mut response| response.body_mut().read_to_string().map_err(|e| e.to_string()))
            .and_then(|body| chat::parse_models(&body));
        let _ = tx.send(Event::Models(result));
        ctx.request_repaint();
    });
    rx
}

/// Sends a chat request on a worker thread and streams the answer back, piece by piece.
/// To stop it, drop the receiver: the worker notices when its next `send` fails.
pub fn ask(base_url: String, body: Value, ctx: egui::Context) -> Receiver<Event> {
    todo!()
}
@@text
## The plan

```text
   GUI thread (app.rs)                       worker thread (client.rs)
   ───────────────────                       ─────────────────────────
   send(): build the request body ──────►    POST /chat/completions
           keep the Receiver                 for each streamed line:
                                               parse it
   poll() every frame:          ◄──────────    send Event::Text("A clos")
     try_recv() all events                     ctx.request_repaint()
     add the text to the answer ◄──────────    send Event::Text("ure is")
                                ◄──────────    send Event::Done
```

The messages between the threads are an enum, in `client.rs`:

```rust,norun
pub enum Event {
    Models(Result<Vec<String>, String>),
    Text(String),
    Done,
    Error(String),
}
```

## Reading `client.rs`

`agent()` creates a configured `ureq::Agent`: a connection timeout (if nothing answers at all in 4 seconds, give up), a long read timeout (big models can think for a while), and `http_status_as_error(false)`, so that an error answer like `404` doesn't become a Rust error, but a response whose status and body **we** can read and show.

`explain` turns `ureq`'s errors into messages a user can act on. "Connection refused" means nothing to most people; "Is LM Studio's server running?" does.

`fetch_models` is a complete example of the worker pattern, and is worth reading closely:

```rust,norun
pub fn fetch_models(base_url: String, ctx: egui::Context) -> Receiver<Event> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let result = agent()
            .get(&url(&base_url, "models"))
            .call()
            .map_err(|e| explain(e, &base_url))
            .and_then(|mut response| response.body_mut().read_to_string().map_err(|e| e.to_string()))
            .and_then(|body| chat::parse_models(&body));
        let _ = tx.send(Event::Models(result));
        ctx.request_repaint();
    });
    rx
}
```

It returns the `Receiver` right away; the thread does the slow part. The `and_then` chain runs each step only if the previous one succeeded, with every error turned into a `String`, so `result` is a single `Result<Vec<String>, String>`.

## Your task: `ask`

`ask(base_url, body, ctx)` works like `fetch_models`, but streams: it sends **many** events. Write it, using step 1's `stream_answer` from `main.rs` as your guide:

1. Create a channel, spawn a thread (with `move`), return the receiver.
2. In the thread, define a small helper closure that sends an event, wakes the GUI, and tells you whether the send worked:

```rust,norun
let send = |event: Event| {
    let ok = tx.send(event).is_ok();
    ctx.request_repaint();
    ok
};
```

3. POST the body with `agent().post(&url(&base_url, "chat/completions")).send_json(&body)`. On error, `send` an `Event::Error` (use `explain`) and `return`.
4. Check `response.status().as_u16()`. If it isn't 200, read the body as text and send it as an error, so the user sees what the server complained about.
5. Read the lines like in step 1. For each one, match `chat::parse_stream_line`: `Text` → send it; `Done` → `break`; `Error` → send it and return; `Nothing` → nothing.
6. After the loop, send `Event::Done`.

**Stopping.** How does the GUI stop a running answer? It simply **drops the receiver**. Then the worker's next `tx.send(...)` fails, and the worker should stop reading. So when `send` returns `false`, `return`. No extra flags or locks needed: the channel itself carries the message "nobody's listening any more".

## The GUI, part 1: models and settings

Add fields to `ChatApp` for what the workers send and for messages to the user:

```rust,norun
/// The answer being streamed.
reply: Option<Receiver<Event>>,
models_rx: Option<Receiver<Event>>,
models: Vec<String>,
status: String,
```

Asking for the models:

```rust,norun
fn refresh_models(&mut self, ctx: &egui::Context) {
    self.status = "Asking the server for its models…".to_string();
    self.models_rx = Some(client::fetch_models(self.server.clone(), ctx.clone()));
}
```

and a `settings` method with an `egui::Grid`: a row with "Server" and a `text_edit_singleline(&mut self.server)`, and a row with "Model", a `ComboBox` listing `self.models` (with `selectable_value(&mut self.model, m.clone(), m)`) and a **Refresh** button that calls `refresh_models`.

Then `poll`, called at the top of `ui()` every frame. First the models:

```rust,norun
if let Some(rx) = &self.models_rx
    && let Ok(Event::Models(result)) = rx.try_recv()
{
    self.models_rx = None;
    match result {
        Ok(models) if models.is_empty() => self.status = "The server has no models. Load one in LM Studio.".to_string(),
        Ok(models) => {
            if !models.contains(&self.model) {
                self.model = models[0].clone();
            }
            self.status = format!("{} model(s) available.", models.len());
            self.models = models;
        }
        Err(e) => self.status = e,
    }
}
```

Put the settings in a `CollapsingHeader` in the bottom panel, and show `self.status` in yellow under it if it isn't empty. Run it, open Settings, press Refresh: your models appear. Stop LM Studio's server and press Refresh again: a helpful error, and the window never freezes.

## Part 2: sending and receiving

`send` adds the question to the conversation, builds the body, and starts the worker. One trick: right after the question, it adds an **empty** assistant message. The streamed text will be added to that one, so the answer grows in place:

```rust,norun
fn send(&mut self, ctx: &egui::Context) {
    let question = self.input.trim().to_string();
    if question.is_empty() || self.reply.is_some() {
        return;
    }
    if self.model.is_empty() {
        self.status = "Choose a model first (in Settings, press Refresh).".to_string();
        self.refresh_models(ctx);
        return;
    }
    let conversation = &mut self.conversation;
    conversation.messages.push(Message { role: Role::User, content: question });
    let body = chat::request_body(&self.model, SYSTEM_PROMPT, &conversation.messages, MAX_HISTORY);
    // An empty answer that the streamed text will be added to.
    conversation.messages.push(Message { role: Role::Assistant, content: String::new() });
    self.reply = Some(client::ask(self.server.clone(), body, ctx.clone()));
    self.input.clear();
    self.status.clear();
}
```

Note the order: the body is built **before** the empty answer is added, so the model doesn't get an empty "assistant" message. Define the constants at the top: `MAX_HISTORY: usize = 20` and a `SYSTEM_PROMPT` of your choice.

The second half of `poll` handles the reply:

```rust,norun
let Some(rx) = &self.reply else { return };
let mut finished = false;
while let Ok(event) = rx.try_recv() {
    let messages = &mut self.conversation.messages;
    match event {
        Event::Text(text) => {
            if let Some(answer) = messages.last_mut() {
                answer.content.push_str(&text);
            }
        }
        Event::Done => finished = true,
        Event::Error(e) => {
            // Remove the empty answer, and show what went wrong.
            if messages.last().is_some_and(|m| m.content.is_empty()) {
                messages.pop();
            }
            self.status = e;
            finished = true;
        }
        Event::Models(_) => {}
    }
}
if finished {
    self.reply = None;
}
```

`while let` takes **all** waiting events each frame: a fast model may send dozens between two frames.

## Part 3: showing the chat

Add `md_cache: CommonMarkCache` and show the messages in the central panel, in a `ScrollArea` with `.stick_to_bottom(true)`, which keeps it scrolled to the newest text while the answer grows (unless you scroll up to read). Your messages go on the right in a bubble, the model's as Markdown:

```rust,norun
ui.push_id(i, |ui| match message.role {
    Role::User => {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            egui::Frame::new().fill(ui.visuals().faint_bg_color).corner_radius(8.0).inner_margin(8.0).show(ui, |ui| {
                ui.set_max_width(520.0);
                ui.label(&message.content);
            });
        });
    }
    Role::Assistant if message.content.is_empty() && streaming => {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(RichText::new("Thinking…").weak());
        });
    }
    Role::Assistant => {
        CommonMarkViewer::new().show(ui, &mut self.md_cache, &message.content);
        if ui.small_button("Copy").clicked() {
            ui.ctx().copy_text(message.content.clone());
        }
    }
    Role::System => {}
});
```

(`streaming` is `self.reply.is_some()`, worked out before the loop. `push_id(i, ...)` gives each message's widgets their own ids, since many messages contain the same kind of widgets.) Inside the scroll area, limit the width so long lines stay readable: `ui.set_max_width(ui.available_width().min(760.0));`.

## Part 4: the input box

In a chat, **Enter** should send, and **Shift+Enter** should make a new line. But a multiline `TextEdit` handles Enter itself (adding a line break) as soon as it's drawn. The trick: give the text box a fixed id, and **before** drawing it, if it has focus, *consume* the Enter key, so the text box never sees it:

```rust,norun
fn input_row(&mut self, ui: &mut egui::Ui) {
    let input_id = egui::Id::unique("chat_input");
    // Enter sends, Shift+Enter makes a new line. We take the Enter key *before* the text
    // box sees it; otherwise it would add a line break first.
    let enter = ui.memory(|m| m.has_focus(input_id)) && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Enter));
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::multiline(&mut self.input)
                .id(input_id)
                .desired_rows(2)
                .desired_width(ui.available_width() - 70.0)
                .hint_text("Message (Enter to send, Shift+Enter for a new line)"),
        );
        if self.reply.is_some() {
            if ui.button("Stop").clicked() {
                // Dropping the receiver tells the worker to stop.
                self.reply = None;
            }
        } else if ui.button(RichText::new("Send").strong()).clicked() || enter {
            self.send(ui.ctx());
            ui.memory_mut(|m| m.request_focus(input_id));
        }
    });
}
```

`consume_key(Modifiers::NONE, Key::Enter)` only matches Enter **without** Shift, so Shift+Enter still reaches the text box. After sending, `request_focus` puts the cursor back in the box, so you can type the next question right away.

Run it, and have a real conversation with your model, in your own app. Try Stop in the middle of a long answer ("write a 500-word story" works well).

@@quiz
How does the Stop button stop the worker thread?
- It calls `thread.kill()`
- It sets a global variable that the worker checks
* It drops the `Receiver`; the worker's next `send` fails, and it returns
? A channel whose receiver is gone tells the sender that nobody's listening. That's all the signal the worker needs.

@@quiz
Why does `input_row` consume the Enter key *before* adding the `TextEdit`?
* Otherwise the text box would handle Enter first and add a line break to the message
- Because `consume_key` only works before any widgets are drawn
- To make Shift+Enter send the message
? Widgets process input when they're added. Taking the key first means the text box never sees it.

@@quiz
`send` adds an empty assistant message after building the request body. Why that order?
- The order doesn't matter
* The streamed text is added to that empty message, but the model shouldn't receive an empty answer in its history
- Because `request_body` deletes empty messages
? Build what you send first, then prepare the place the answer will go.
