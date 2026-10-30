@@lesson g-ac1 Local AI Chat 1: Talking to a Model
@@text
In this last build you'll make your own **desktop chat app for AI models**, like the ChatGPT app, but for models running **on your own computer** with LM Studio or Ollama: free, private, and offline.

- answers appear **word by word** as the model writes them (streaming), and you can **stop** them,
- answers are shown as formatted **Markdown**, with code blocks,
- **several conversations**, saved between runs, with automatic titles,
- **personas**: tell the model who to be (a tutor, an editor, a translator…).

Along the way you'll learn how programs talk to each other over **HTTP** with **JSON**, which is how almost every web service works, and you'll use worker threads for real.

**You need a model running.** If you haven't already, follow "Getting help from an AI model" in **How Guided Builds Work** to set up LM Studio (or Ollama) and start its server. A small model is fine for this build; you're testing your app, not the model.

## How programs talk: HTTP

When your browser opens a web page, it sends an **HTTP request** to a server and gets a **response**. Programs do exactly the same to talk to each other. A request has:

- a **method**: `GET` to fetch something, `POST` to send something,
- a **URL**: which server, and which thing on it: `http://localhost:1234/v1/models`. `localhost` means "this computer", and `1234` is the **port**, the "door" LM Studio listens at,
- for `POST`, a **body**: the data you send, usually **JSON**.

The response has a **status code** (`200` means OK, `404` not found, `500` the server had a problem) and a body.

LM Studio, Ollama and many others speak the same "OpenAI-compatible" API. Two requests are all we need:

**`GET /v1/models`** lists the models. The answer looks like this:

```text
{ "data": [ { "id": "qwen2.5-coder-7b-instruct" }, { "id": "llama-3.2-3b" } ] }
```

**`POST /v1/chat/completions`** sends a conversation and gets the model's answer. The body:

```text
{
  "model": "qwen2.5-coder-7b-instruct",
  "messages": [
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user", "content": "What is a closure?" }
  ],
  "stream": true
}
```

- The **system** message tells the model how to behave. It's sent first, every time.
- Then the conversation so far: `user` messages (you) and `assistant` messages (the model). The model doesn't remember anything between requests! To have a conversation, **we send the whole history every time**. That's why the history is limited: models can only read so much text at once (their *context window*).

You can try it without writing any code. In a terminal (PowerShell), with LM Studio's server running:

```text
curl.exe http://localhost:1234/v1/models
```

## Streaming

With `"stream": true`, the server doesn't wait until the whole answer is ready. It sends it in small pieces, as **server-sent events**: lines starting with `data: `, each with a little JSON object:

```text
data: {"choices":[{"delta":{"role":"assistant"}}]}

data: {"choices":[{"delta":{"content":"A clos"}}]}

data: {"choices":[{"delta":{"content":"ure is"}}]}

data: [DONE]
```

Each piece's text is at `choices[0].delta.content`. Some chunks have no text (like the first one above), there are empty lines in between, and `data: [DONE]` marks the end. Reading this line by line, we can show the answer as it's written.

@@project ai_chat Local AI Chat, step 1: talking to a model
`src/chat.rs`: messages, conversations and the JSON, with some functions for you to write. `src/main.rs` is a complete chat in the terminal that uses them.
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
mod chat;

use chat::{Message, Role, StreamLine};
use std::io::{BufRead, BufReader, Write};

const SERVER: &str = "http://localhost:1234/v1"; // LM Studio. Ollama: http://localhost:11434/v1

/// A chat in the terminal: type a question, see the answer appear word by word.
/// No window and no threads yet: while it waits for the answer, the program just waits.
fn main() {
    let model = match first_model() {
        Ok(model) => model,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    println!("Chatting with {model}. Type a question, or an empty line to quit.");

    let mut history: Vec<Message> = Vec::new();
    loop {
        print!("\nYou: ");
        std::io::stdout().flush().unwrap();
        let mut question = String::new();
        if std::io::stdin().read_line(&mut question).is_err() || question.trim().is_empty() {
            break;
        }
        history.push(Message { role: Role::User, content: question.trim().to_string() });

        let body = chat::request_body(&model, "You are a helpful assistant. Keep answers short.", &history, 10);
        print!("AI: ");
        match stream_answer(&body) {
            Ok(answer) => history.push(Message { role: Role::Assistant, content: answer }),
            Err(e) => println!("\n(error: {e})"),
        }
        println!();
    }
}

/// Asks the server which models it has, and picks the first.
fn first_model() -> Result<String, String> {
    let mut response = ureq::get(&format!("{SERVER}/models"))
        .call()
        .map_err(|e| format!("Can't reach {SERVER}: {e}. Is LM Studio's server running?"))?;
    let json = response.body_mut().read_to_string().map_err(|e| e.to_string())?;
    let models = chat::parse_models(&json)?;
    models.into_iter().next().ok_or("The server has no model loaded.".to_string())
}

/// Sends the request and prints the answer as it arrives. Returns the whole answer.
fn stream_answer(body: &serde_json::Value) -> Result<String, String> {
    let response = ureq::post(&format!("{SERVER}/chat/completions")).send_json(body).map_err(|e| e.to_string())?;
    let reader = BufReader::new(response.into_body().into_reader());
    let mut answer = String::new();
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        match chat::parse_stream_line(&line) {
            StreamLine::Text(text) => {
                print!("{text}");
                std::io::stdout().flush().unwrap(); // show it now, not at the end of the line
                answer.push_str(&text);
            }
            StreamLine::Done => break,
            StreamLine::Error(e) => return Err(e),
            StreamLine::Nothing => {}
        }
    }
    Ok(answer)
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
    todo!()
}

/// The JSON body of a chat request: the system prompt, then the last `max_history` messages.
pub fn request_body(model: &str, system_prompt: &str, messages: &[Message], max_history: usize) -> Value {
    todo!()
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
    todo!()
}

/// The model names from the answer to `GET /models`.
pub fn parse_models(json: &str) -> Result<Vec<String>, String> {
    todo!()
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
@@text
## The new crate: `ureq`

`Cargo.toml` adds **`ureq`**, a simple HTTP client. (With `default-features = false` and the `json` feature, it only does plain `http://`, which is all we need for `localhost`, and builds quickly.) Look at how `main.rs` uses it:

```rust,norun
let mut response = ureq::get(&format!("{SERVER}/models")).call().map_err(...)?;
let json = response.body_mut().read_to_string().map_err(|e| e.to_string())?;
```

and for the chat:

```rust,norun
let response = ureq::post(&format!("{SERVER}/chat/completions")).send_json(body).map_err(...)?;
let reader = BufReader::new(response.into_body().into_reader());
for line in reader.lines() { ... }
```

`send_json` turns a `serde_json::Value` into the request body. `into_reader()` gives the response body as something you can read from, like a file, and `BufReader::lines()` reads it line by line **as the lines arrive**: that's what makes streaming work.

## `serde_json::Value` and `json!`

So far you've used serde with **structs**. For JSON whose shape you only need a bit of, `serde_json::Value` is handy: it can hold any JSON. You build one with the `json!` macro, which looks just like JSON:

```rust,norun
let message = json!({ "role": "user", "content": question });
```

and you read one by **indexing**: `value["choices"][0]["delta"]["content"]`. Indexing never panics: if anything along the way is missing, you get `Value::Null`. Then `.as_str()` gives an `Option<&str>`: `Some` if it's a string. So "dig in, then check" is safe and short.

## Roles as an enum

```rust,norun
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}
```

`#[serde(rename_all = "lowercase")]` makes serde write `Role::Assistant` as `"assistant"`, exactly what the API expects. So inside `json!`, `"role": m.role` just works.

## Your tasks

Run `cargo test`: four tests fail.

**1. `parse_models(json)`** turns the answer to `GET /models` into a list of names.
- Parse with `serde_json::from_str::<Value>(json)`; turn its error into a friendly `String` with `map_err`.
- `value["data"].as_array()` gives an `Option<&Vec<Value>>`. `.ok_or("...")?` turns `None` into an error and returns it.
- Collect every element's `["id"].as_str()`, skipping any that aren't strings: `filter_map` is perfect for that.

**2. `request_body(model, system_prompt, messages, max_history)`** builds the JSON body shown above:
- a list (a `Vec<Value>`) that starts with the system message (unless the prompt is empty or only spaces),
- then only the **last** `max_history` messages. If there are 30 messages and the limit is 20, skip the first 10: `messages.len().saturating_sub(max_history)` is the number to skip, and `&messages[skip..]` is the rest,
- each one as `json!({ "role": m.role, "content": m.content })`,
- and finally `json!({ "model": model, "messages": list, "stream": true })`.

**3. `parse_stream_line(line)`** says what one streamed line means, as a `StreamLine`:
- if the trimmed line doesn't start with `data:`, it's `Nothing` (`strip_prefix` again!),
- if the data is `[DONE]`, it's `Done`,
- otherwise parse the JSON (if it fails, `Nothing`). If it has an `"error"` field, return `Error` with `error["message"]`. Otherwise, if `choices[0].delta.content` is a non-empty string, that's `Text`; anything else is `Nothing`.

**4. `make_title(question, max_chars)`** makes a short title for a conversation from its first question: the first words, joined with single spaces, as many as fit in `max_chars`. If not all words fit, end with `…`. If even the first word is too long, cut it to `max_chars` characters and add `…`. An empty question gives `"New chat"`. (The tests show the exact cases.) We'll use it in step 3.

Then make sure LM Studio's server is running, and `cargo run`. Ask something, and watch the answer stream in. Ask a follow-up question like "say that more simply": it works because `history` is sent every time.

Notice what happens while the model is writing: the program can't do anything else. In a terminal that's fine. In a window, it would freeze. That's the next step.

@@quiz
The model answered your first question. You ask a second one. What does the app send?
- Only the new question; the model remembers the conversation
* The system prompt and the whole conversation so far (up to the limit), including the new question
- Only the new question and the model's last answer
? Models don't remember anything between requests. Memory is an illusion created by resending the history.

@@quiz
What does `value["choices"][0]["delta"]["content"]` give if the chunk has no `content`?
- It panics
* `Value::Null`, so `.as_str()` returns `None`
- An empty string
? Indexing a `serde_json::Value` never panics: missing parts become `Null`.

@@quiz
Which status code means "OK"?
* 200
- 404
- 500
? 2xx is success, 4xx means the request was wrong, 5xx means the server had a problem.
