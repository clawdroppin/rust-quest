@@lesson g-ac3 Local AI Chat 3: Conversations, Personas and Saving
@@text
Your chat app works, but it has one conversation, and closing the app loses it. In this last step you'll add:

- a **sidebar** with many conversations, titled automatically from their first question,
- **saving** everything (conversations and settings) to a JSON file,
- **personas**: ready-made system prompts, plus your own,
- deleting conversations, safely, even while an answer is streaming.

@@project ai_chat Local AI Chat, step 3: conversations and saving
Adds `src/storage.rs` (saving and loading, ready to use). `src/main.rs` gets `mod storage;`. The starting `src/app.rs` is the finished window from step 2.
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
mod storage;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 680.0]).with_min_inner_size([560.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("AI Chat", options, Box::new(|_cc| Ok(Box::new(app::ChatApp::new()))))
}

@@file src/app.rs
//! The window: the chat in the middle, settings and the input at the bottom.

use crate::chat::{self, Conversation, Message, Role};
use crate::client::{self, Event};
use eframe::egui::{self, Color32, Key, Modifiers, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::sync::mpsc::Receiver;

/// How many earlier messages are sent with each question. Small local models
/// only have room for so much text (their "context window").
const MAX_HISTORY: usize = 20;
const SYSTEM_PROMPT: &str = "You are a friendly, helpful assistant. Answer clearly and concisely.";

pub struct ChatApp {
    server: String,
    model: String,
    conversation: Conversation,
    input: String,
    /// The answer being streamed.
    reply: Option<Receiver<Event>>,
    models_rx: Option<Receiver<Event>>,
    models: Vec<String>,
    status: String,
    md_cache: CommonMarkCache,
}

impl ChatApp {
    pub fn new() -> Self {
        ChatApp {
            server: "http://localhost:1234/v1".to_string(),
            model: String::new(),
            conversation: Conversation::new(),
            input: String::new(),
            reply: None,
            models_rx: None,
            models: Vec::new(),
            status: String::new(),
            md_cache: CommonMarkCache::default(),
        }
    }

    fn refresh_models(&mut self, ctx: &egui::Context) {
        self.status = "Asking the server for its models…".to_string();
        self.models_rx = Some(client::fetch_models(self.server.clone(), ctx.clone()));
    }

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

    /// Takes whatever the worker threads have sent, without waiting.
    fn poll(&mut self) {
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
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
            ui.label("Server");
            ui.text_edit_singleline(&mut self.server);
            ui.end_row();

            ui.label("Model");
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("model").selected_text(&self.model).width(220.0).show_ui(ui, |ui| {
                    for m in &self.models {
                        ui.selectable_value(&mut self.model, m.clone(), m);
                    }
                });
                if ui.button("Refresh").clicked() {
                    self.refresh_models(ui.ctx());
                }
            });
            ui.end_row();
        });
    }

    // ---------- the chat ----------

    fn messages(&mut self, ui: &mut egui::Ui) {
        let streaming = self.reply.is_some();
        if self.conversation.messages.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Ask anything. Your chats stay on this computer.").weak().size(16.0));
            });
            return;
        }
        egui::ScrollArea::vertical().auto_shrink(false).stick_to_bottom(true).show(ui, |ui| {
            ui.set_max_width(ui.available_width().min(760.0));
            for (i, message) in self.conversation.messages.iter().enumerate() {
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
                ui.add_space(10.0);
            }
        });
    }

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
        if !self.status.is_empty() {
            ui.colored_label(Color32::YELLOW, &self.status);
        }
    }
}

impl eframe::App for ChatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll();
        egui::Panel::bottom("input").show(ui, |ui| {
            ui.add_space(6.0);
            egui::CollapsingHeader::new("Settings").show(ui, |ui| self.settings(ui));
            self.input_row(ui);
            ui.add_space(4.0);
        });
        egui::CentralPanel::default().show(ui, |ui| self.messages(ui));
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
    let (tx, rx) = channel();
    thread::spawn(move || {
        let send = |event: Event| {
            let ok = tx.send(event).is_ok();
            ctx.request_repaint();
            ok
        };
        let response = match agent().post(&url(&base_url, "chat/completions")).send_json(&body) {
            Ok(r) => r,
            Err(e) => {
                send(Event::Error(explain(e, &base_url)));
                return;
            }
        };
        let status = response.status().as_u16();
        let mut body = response.into_body();
        if status != 200 {
            let text = body.read_to_string().unwrap_or_default();
            send(Event::Error(format!("The server answered {status}: {text}")));
            return;
        }
        for line in BufReader::new(body.into_reader()).lines() {
            let Ok(line) = line else {
                send(Event::Error("The connection was lost.".to_string()));
                return;
            };
            let keep_going = match chat::parse_stream_line(&line) {
                StreamLine::Text(text) => send(Event::Text(text)),
                StreamLine::Done => break,
                StreamLine::Error(e) => {
                    send(Event::Error(e));
                    return;
                }
                StreamLine::Nothing => true,
            };
            if !keep_going {
                return; // the app dropped the receiver: stop reading
            }
        }
        send(Event::Done);
    });
    rx
}

@@file src/storage.rs
//! Saving conversations and settings between runs.

use crate::chat::{Conversation, PERSONAS};
use serde::{Deserialize, Serialize};

const FILE: &str = "ai_chat_data.json";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub server: String,
    pub model: String,
    pub system_prompt: String,
    pub conversations: Vec<Conversation>,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            server: "http://localhost:1234/v1".to_string(),
            model: String::new(),
            system_prompt: PERSONAS[0].1.to_string(),
            conversations: Vec::new(),
        }
    }
}

pub fn load() -> SaveData {
    std::fs::read_to_string(FILE).ok().and_then(|text| serde_json::from_str(&text).ok()).unwrap_or_default()
}

pub fn save(data: &SaveData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(FILE, json).map_err(|e| format!("Couldn't save: {e}"))
}
@@text
## Everything that's saved, in one struct

`storage.rs` defines what's saved:

```rust,norun
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub server: String,
    pub model: String,
    pub system_prompt: String,
    pub conversations: Vec<Conversation>,
}
```

Replace the `server` and `model` fields of `ChatApp` with `data: SaveData`, and the single `conversation` with an index into `data.conversations`:

```rust,norun
data: SaveData,
/// Index of the open conversation. There's always at least one.
current: usize,
```

and in `new()`:

```rust,norun
let mut data = storage::load();
if data.conversations.is_empty() {
    data.conversations.push(Conversation::new());
}
```

"There's always at least one" is an **invariant**: a rule that's always true, so the rest of the code can rely on it. `self.data.conversations[self.current]` never needs a check. Every piece of code that removes conversations must keep the rule true, which is easy as long as there's only one such place.

Now let the compiler guide you: every `self.server` becomes `self.data.server`, every `self.conversation` becomes `self.data.conversations[self.current]`, and the `SYSTEM_PROMPT` constant becomes `self.data.system_prompt`. Add a `save` method like the Focus Timer's (show a failure in `self.status`), and call it after an answer is finished in `poll`, and in an `on_exit` method.

In `send`, give a new conversation its title from the first question:

```rust,norun
if conversation.messages.is_empty() {
    conversation.title = chat::make_title(&question, 32);
}
```

## Which conversation does the answer belong to?

Here's a subtle bug to avoid. You ask a question in conversation A, and while the answer is streaming, you click on conversation B. If `poll` adds the text to `self.data.conversations[self.current]`, the rest of the answer lands in B!

The fix: remember the conversation **together with the receiver**:

```rust,norun
/// The answer being streamed, and which conversation it belongs to.
reply: Option<(Receiver<Event>, usize)>,
```

`send` stores `self.current` with it, and `poll` uses that index instead of `self.current`. In the message view, show the "Thinking…" spinner only if the answer streams into the conversation being shown:

```rust,norun
let streaming_here = self.reply.as_ref().is_some_and(|(_, index)| *index == self.current);
```

## The sidebar

A "+ New chat" button, then the conversations, each with a small delete button. New conversations go at the **top**:

```rust,norun
fn sidebar(&mut self, ui: &mut egui::Ui) {
    ui.add_space(6.0);
    if ui.button(RichText::new("+ New chat").strong()).clicked() {
        self.data.conversations.insert(0, Conversation::new());
        self.current = 0;
        // The answer being streamed (if any) moved down one place too.
        if let Some((_, index)) = &mut self.reply {
            *index += 1;
        }
    }
    ui.separator();

    let mut open = None;
    let mut delete = None;
    egui::ScrollArea::vertical().max_height(ui.available_height() - 40.0).show(ui, |ui| {
        for (i, conversation) in self.data.conversations.iter().enumerate() {
            ui.horizontal(|ui| {
                if ui.small_button("x").on_hover_text("Delete this chat").clicked() {
                    delete = Some(i);
                }
                if ui.selectable_label(i == self.current, &conversation.title).clicked() {
                    open = Some(i);
                }
            });
        }
    });
    if let Some(i) = open {
        self.current = i;
    }
    if let Some(i) = delete {
        self.delete(i);
    }
}
```

Look at the "+ New chat" code again. Inserting at index 0 moves every conversation down one place, **including the one an answer is streaming into**. That's why its index is increased. (You saw the same kind of bug in Note Vault: indices into a list that changes are fragile.)

Show the sidebar in a left panel: `egui::Panel::left("chats").resizable(true).default_size(230.0)`, added before the central panel.

## Your task: `delete`

Write `fn delete(&mut self, i: usize)`. It has to keep everything consistent:

1. If an answer is streaming **into** conversation `i`, stop it (set `self.reply = None`).
2. Remove the conversation.
3. Keep the invariant: if the list is now empty, push a new empty conversation.
4. Fix `self.current`: if it was **after** the deleted one, it moved up one place, so subtract 1. If it's now past the end of the list, also subtract 1. (`saturating_sub(1)` can't go below 0.)
5. Fix the streaming answer's index the same way: if it's after `i`, subtract 1.
6. Save.

Test it carefully: delete the open chat, a chat above it, a chat below it, the last remaining chat, and a chat while another one is receiving an answer.

## Personas

`chat.rs` has `PERSONAS`, a list of `(name, system prompt)` pairs. Add a row to the settings grid with a `ComboBox` that copies a persona's prompt into `self.data.system_prompt`, and under the grid a multiline text box to edit the prompt freely:

```rust,norun
ui.label("Persona");
egui::ComboBox::from_id_salt("persona").selected_text("Choose…").show_ui(ui, |ui| {
    for (name, prompt) in PERSONAS {
        if ui.selectable_label(self.data.system_prompt == prompt, name).clicked() {
            self.data.system_prompt = prompt.to_string();
            changed = true;
        }
    }
});
ui.end_row();
```

(Collect `changed` from the other settings widgets too, using `.lost_focus()` for the text boxes so it saves when you finish typing, not on every key. Then `if changed { self.save(); }`.)

Try the Translator persona, or write your own: "You are a pirate. Answer everything like a pirate." The system prompt is a surprisingly powerful tool.

## The finished app

@@project ai_chat_finished Local AI Chat (finished)
The complete app, for reference. Created in its own folder, `ai_chat_finished`.
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
mod storage;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 680.0]).with_min_inner_size([560.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native("AI Chat", options, Box::new(|_cc| Ok(Box::new(app::ChatApp::new()))))
}

@@file src/app.rs
//! The window: conversations on the left, the chat in the middle, the input at the bottom.

use crate::chat::{self, Conversation, Message, PERSONAS, Role};
use crate::client::{self, Event};
use crate::storage::{self, SaveData};
use eframe::egui::{self, Color32, Key, Modifiers, RichText};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::sync::mpsc::Receiver;

/// How many earlier messages are sent with each question. Small local models
/// only have room for so much text (their "context window").
const MAX_HISTORY: usize = 20;

pub struct ChatApp {
    data: SaveData,
    /// Index of the open conversation. There's always at least one.
    current: usize,
    input: String,
    /// The answer being streamed, and which conversation it belongs to.
    reply: Option<(Receiver<Event>, usize)>,
    models_rx: Option<Receiver<Event>>,
    models: Vec<String>,
    status: String,
    md_cache: CommonMarkCache,
}

impl ChatApp {
    pub fn new() -> Self {
        let mut data = storage::load();
        if data.conversations.is_empty() {
            data.conversations.push(Conversation::new());
        }
        ChatApp {
            data,
            current: 0,
            input: String::new(),
            reply: None,
            models_rx: None,
            models: Vec::new(),
            status: String::new(),
            md_cache: CommonMarkCache::default(),
        }
    }

    fn save(&mut self) {
        if let Err(e) = storage::save(&self.data) {
            self.status = e;
        }
    }

    fn refresh_models(&mut self, ctx: &egui::Context) {
        self.status = "Asking the server for its models…".to_string();
        self.models_rx = Some(client::fetch_models(self.data.server.clone(), ctx.clone()));
    }

    fn send(&mut self, ctx: &egui::Context) {
        let question = self.input.trim().to_string();
        if question.is_empty() || self.reply.is_some() {
            return;
        }
        if self.data.model.is_empty() {
            self.status = "Choose a model first (in Settings, press Refresh).".to_string();
            self.refresh_models(ctx);
            return;
        }
        let conversation = &mut self.data.conversations[self.current];
        if conversation.messages.is_empty() {
            conversation.title = chat::make_title(&question, 32);
        }
        conversation.messages.push(Message { role: Role::User, content: question });
        let body = chat::request_body(&self.data.model, &self.data.system_prompt, &conversation.messages, MAX_HISTORY);
        // An empty answer that the streamed text will be added to.
        conversation.messages.push(Message { role: Role::Assistant, content: String::new() });
        self.reply = Some((client::ask(self.data.server.clone(), body, ctx.clone()), self.current));
        self.input.clear();
        self.status.clear();
    }

    /// Takes whatever the worker threads have sent, without waiting.
    fn poll(&mut self) {
        if let Some(rx) = &self.models_rx
            && let Ok(Event::Models(result)) = rx.try_recv()
        {
            self.models_rx = None;
            match result {
                Ok(models) if models.is_empty() => self.status = "The server has no models. Load one in LM Studio.".to_string(),
                Ok(models) => {
                    if !models.contains(&self.data.model) {
                        self.data.model = models[0].clone();
                    }
                    self.status = format!("{} model(s) available.", models.len());
                    self.models = models;
                    self.save();
                }
                Err(e) => self.status = e,
            }
        }

        let Some((rx, index)) = &self.reply else { return };
        let index = *index;
        let mut finished = false;
        while let Ok(event) = rx.try_recv() {
            let messages = &mut self.data.conversations[index].messages;
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
            self.save();
        }
    }

    fn stop(&mut self) {
        // Dropping the receiver tells the worker to stop.
        self.reply = None;
        self.save();
    }

    // ---------- left side ----------

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(6.0);
        if ui.button(RichText::new("+ New chat").strong()).clicked() {
            self.data.conversations.insert(0, Conversation::new());
            self.current = 0;
            // The answer being streamed (if any) moved down one place too.
            if let Some((_, index)) = &mut self.reply {
                *index += 1;
            }
        }
        ui.separator();

        let mut open = None;
        let mut delete = None;
        egui::ScrollArea::vertical().max_height(ui.available_height() - 40.0).show(ui, |ui| {
            for (i, conversation) in self.data.conversations.iter().enumerate() {
                ui.horizontal(|ui| {
                    if ui.small_button("x").on_hover_text("Delete this chat").clicked() {
                        delete = Some(i);
                    }
                    if ui.selectable_label(i == self.current, &conversation.title).clicked() {
                        open = Some(i);
                    }
                });
            }
        });
        if let Some(i) = open {
            self.current = i;
        }
        if let Some(i) = delete {
            self.delete(i);
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Model:");
            ui.label(RichText::new(if self.data.model.is_empty() { "(none)" } else { &self.data.model }).weak());
        });
    }

    fn delete(&mut self, i: usize) {
        if self.reply.as_ref().is_some_and(|(_, index)| *index == i) {
            self.reply = None; // stop the answer that was going into it
        }
        self.data.conversations.remove(i);
        if self.data.conversations.is_empty() {
            self.data.conversations.push(Conversation::new());
        }
        // Indices after the deleted one moved up one place.
        if self.current > i || self.current >= self.data.conversations.len() {
            self.current = self.current.saturating_sub(1);
        }
        if let Some((_, index)) = &mut self.reply
            && *index > i
        {
            *index -= 1;
        }
        self.save();
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;
        egui::Grid::new("settings").num_columns(2).show(ui, |ui| {
            ui.label("Server");
            changed |= ui.text_edit_singleline(&mut self.data.server).lost_focus();
            ui.end_row();

            ui.label("Model");
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_salt("model").selected_text(&self.data.model).width(220.0).show_ui(ui, |ui| {
                    for m in &self.models {
                        changed |= ui.selectable_value(&mut self.data.model, m.clone(), m).changed();
                    }
                });
                if ui.button("Refresh").clicked() {
                    self.refresh_models(ui.ctx());
                }
            });
            ui.end_row();

            ui.label("Persona");
            egui::ComboBox::from_id_salt("persona").selected_text("Choose…").show_ui(ui, |ui| {
                for (name, prompt) in PERSONAS {
                    if ui.selectable_label(self.data.system_prompt == prompt, name).clicked() {
                        self.data.system_prompt = prompt.to_string();
                        changed = true;
                    }
                }
            });
            ui.end_row();
        });
        ui.label("System prompt (instructions the model gets before every chat):");
        changed |= ui.add(egui::TextEdit::multiline(&mut self.data.system_prompt).desired_rows(3).desired_width(f32::INFINITY)).lost_focus();
        if changed {
            self.save();
        }
    }

    // ---------- the chat ----------

    fn messages(&mut self, ui: &mut egui::Ui) {
        let streaming_here = self.reply.as_ref().is_some_and(|(_, index)| *index == self.current);
        let conversation = &self.data.conversations[self.current];
        if conversation.messages.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("Ask anything. Your chats stay on this computer.").weak().size(16.0));
            });
            return;
        }
        egui::ScrollArea::vertical().auto_shrink(false).stick_to_bottom(true).show(ui, |ui| {
            ui.set_max_width(ui.available_width().min(760.0));
            for (i, message) in conversation.messages.iter().enumerate() {
                ui.push_id(i, |ui| match message.role {
                    Role::User => {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            egui::Frame::new().fill(ui.visuals().faint_bg_color).corner_radius(8.0).inner_margin(8.0).show(ui, |ui| {
                                ui.set_max_width(520.0);
                                ui.label(&message.content);
                            });
                        });
                    }
                    Role::Assistant if message.content.is_empty() && streaming_here => {
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
                ui.add_space(10.0);
            }
        });
    }

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
                    self.stop();
                }
            } else if ui.button(RichText::new("Send").strong()).clicked() || enter {
                self.send(ui.ctx());
                ui.memory_mut(|m| m.request_focus(input_id));
            }
        });
        if !self.status.is_empty() {
            ui.colored_label(Color32::YELLOW, &self.status);
        }
    }
}

impl eframe::App for ChatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll();
        egui::Panel::left("chats").resizable(true).default_size(230.0).min_size(160.0).show(ui, |ui| {
            self.sidebar(ui);
        });
        egui::Panel::bottom("input").show(ui, |ui| {
            ui.add_space(6.0);
            egui::CollapsingHeader::new("Settings").show(ui, |ui| self.settings(ui));
            self.input_row(ui);
            ui.add_space(4.0);
        });
        egui::CentralPanel::default().show(ui, |ui| self.messages(ui));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save();
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
    let (tx, rx) = channel();
    thread::spawn(move || {
        let send = |event: Event| {
            let ok = tx.send(event).is_ok();
            ctx.request_repaint();
            ok
        };
        let response = match agent().post(&url(&base_url, "chat/completions")).send_json(&body) {
            Ok(r) => r,
            Err(e) => {
                send(Event::Error(explain(e, &base_url)));
                return;
            }
        };
        let status = response.status().as_u16();
        let mut body = response.into_body();
        if status != 200 {
            let text = body.read_to_string().unwrap_or_default();
            send(Event::Error(format!("The server answered {status}: {text}")));
            return;
        }
        for line in BufReader::new(body.into_reader()).lines() {
            let Ok(line) = line else {
                send(Event::Error("The connection was lost.".to_string()));
                return;
            };
            let keep_going = match chat::parse_stream_line(&line) {
                StreamLine::Text(text) => send(Event::Text(text)),
                StreamLine::Done => break,
                StreamLine::Error(e) => {
                    send(Event::Error(e));
                    return;
                }
                StreamLine::Nothing => true,
            };
            if !keep_going {
                return; // the app dropped the receiver: stop reading
            }
        }
        send(Event::Done);
    });
    rx
}

@@file src/storage.rs
//! Saving conversations and settings between runs.

use crate::chat::{Conversation, PERSONAS};
use serde::{Deserialize, Serialize};

const FILE: &str = "ai_chat_data.json";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData {
    pub server: String,
    pub model: String,
    pub system_prompt: String,
    pub conversations: Vec<Conversation>,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            server: "http://localhost:1234/v1".to_string(),
            model: String::new(),
            system_prompt: PERSONAS[0].1.to_string(),
            conversations: Vec::new(),
        }
    }
}

pub fn load() -> SaveData {
    std::fs::read_to_string(FILE).ok().and_then(|text| serde_json::from_str(&text).ok()).unwrap_or_default()
}

pub fn save(data: &SaveData) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    std::fs::write(FILE, json).map_err(|e| format!("Couldn't save: {e}"))
}
@@text
## Make it yours

- **Thinking models:** some models (like DeepSeek R1 or Qwen 3) first write their reasoning between `<think>` and `</think>`. Show that part folded away in a `CollapsingHeader`, and only the answer below it. (`split_once("</think>")` is a good start.)
- **Regenerate:** a button under the last answer that removes it and asks again.
- **Edit and resend:** click your own message to edit it, which removes everything after it.
- **Export:** save a conversation as a Markdown file, with `## You` and `## AI` headings.
- **Temperature:** a slider from 0.0 (focused) to 1.5 (creative), added to the request body as `"temperature"`.
- **Search** across all conversations, like Note Vault's search.
- **Rename** conversations by double-clicking their title.

## You did it!

That's the last guided build. Look at what you've made: a productivity timer with charts, a notes app with Markdown and search, a file organizer with undo, and a streaming AI chat client. Real apps, built from the same pieces you learned one lesson at a time: structs and enums, `Vec` and `HashMap`, `Result` and `?`, traits, closures and iterators, files, JSON, threads and channels, and egui.

The best next step is **your own idea**. Something you'd use yourself, or something a friend needs. Start small, model the data first, write tests for the logic, then build the window around it, just like you did here. And when you're stuck: read the error message, `dbg!` your values, ask your AI helper, and search the Rust docs (`docs.rs`) and the community (users.rust-lang.org). Happy building! 🦀

@@quiz
`reply` stores the conversation's index together with the receiver. What bug does that prevent?
- Two answers streaming at once
* An answer continuing into a different conversation when the user switches chats while it streams
- Saving the wrong conversation
? The answer belongs to the conversation the question was asked in, not whichever one is open now.

@@quiz
What is an "invariant"?
* A rule about your data that's always true, like "there's always at least one conversation", so other code can rely on it
- A variable that can't be changed
- A type that can't be generic
? Keeping invariants in one place (like `delete`) makes the rest of the code simpler and safer.
