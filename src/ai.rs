//! Optional AI helper. Talks to a model running on the learner's own computer through the
//! OpenAI-compatible HTTP API that LM Studio, Ollama, llama.cpp's server and Jan all offer.
//! Nothing is sent anywhere unless the learner sets it up, and only to the URL they choose.

use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

/// Well-known local servers and their default addresses.
pub const PRESETS: [(&str, &str); 4] = [
    ("LM Studio", "http://localhost:1234/v1"),
    ("Ollama", "http://localhost:11434/v1"),
    ("llama.cpp", "http://localhost:8080/v1"),
    ("Jan", "http://localhost:1337/v1"),
];

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AiSettings {
    pub base_url: String,
    /// Empty means "whatever the server lists first".
    pub model: String,
    /// Ask the model to guide instead of handing out full solutions.
    pub hints_only: bool,
    /// Attach the current lesson, code and errors to each question.
    pub include_context: bool,
}

impl Default for AiSettings {
    fn default() -> Self {
        Self { base_url: PRESETS[0].1.to_string(), model: String::new(), hints_only: true, include_context: true }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    /// "user" or "assistant"
    pub role: String,
    /// What the chat shows.
    pub content: String,
    /// What was actually sent for this message (the question plus app context), if different.
    #[serde(skip)]
    pub sent: Option<String>,
}

pub enum Event {
    Delta(String),
    Done,
    Error(String),
}

const SYSTEM_PROMPT: &str = "You are a patient, encouraging programming tutor built into Rust Quest, \
an app that teaches complete beginners to program in Rust (edition 2024). Explain things simply, \
in plain language, and connect them to what the learner is studying. Prefer short answers (under \
250 words) unless asked for more. Use Markdown, with ```rust code blocks for code. When explaining \
a compiler error, say in one sentence what it means, point at the line, and explain how to fix it.";

const HINTS_ONLY: &str = " The learner wants to learn by doing: do NOT write the complete solution \
to their exercise or project step. Give hints, ask guiding questions, point at the problem, and \
show small examples on different data. Only if they explicitly insist, give the solution, and \
explain it line by line.";

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(4)))
        .timeout_per_read(Some(Duration::from_secs(180)))
        .http_status_as_error(false)
        .build()
        .into()
}

fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}/{path}", base_url.trim().trim_end_matches('/'))
}

fn friendly_connect_error(e: &ureq::Error, base_url: &str) -> String {
    match e {
        ureq::Error::Io(_) | ureq::Error::ConnectionFailed | ureq::Error::HostNotFound => format!(
            "Couldn't reach an AI server at {base_url}. Is LM Studio (or Ollama) running, with its local server started?"
        ),
        ureq::Error::Timeout(_) => "The AI server took too long to answer.".to_string(),
        other => format!("Connection problem: {other}"),
    }
}

/// Asks the server which models it offers. Blocking: call it from a background thread.
pub fn list_models(base_url: &str) -> Result<Vec<String>, String> {
    let mut resp = agent().get(&endpoint(base_url, "models")).call().map_err(|e| friendly_connect_error(&e, base_url))?;
    let status = resp.status().as_u16();
    let body = resp.body_mut().read_to_string().map_err(|e| e.to_string())?;
    if status != 200 {
        return Err(format!("The server answered with status {status}: {}", shorten(&body, 200)));
    }
    let value: Value = serde_json::from_str(&body).map_err(|_| "The server's answer wasn't the expected JSON.".to_string())?;
    let models: Vec<String> = value["data"]
        .as_array()
        .map(|list| list.iter().filter_map(|m| m["id"].as_str().map(String::from)).collect())
        .unwrap_or_default();
    // Embedding models can't chat; put them last.
    let (mut chat, embed): (Vec<String>, Vec<String>) = models.into_iter().partition(|m| !m.contains("embed"));
    chat.extend(embed);
    Ok(chat)
}

/// Runs `list_models` on a thread.
pub fn list_models_async(base_url: String, ctx: egui::Context) -> Receiver<Result<Vec<String>, String>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let _ = tx.send(list_models(&base_url));
        ctx.request_repaint();
    });
    rx
}

/// A chat request running on a background thread. Drop it or call `stop` to cancel.
pub struct Stream {
    pub rx: Receiver<Event>,
    stop: Arc<AtomicBool>,
}

impl Stream {
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Sends the conversation and streams the answer back as `Event`s.
pub fn start_chat(settings: &AiSettings, history: &[Message], ctx: egui::Context) -> Stream {
    let mut messages = vec![json!({
        "role": "system",
        "content": if settings.hints_only { format!("{SYSTEM_PROMPT}{HINTS_ONLY}") } else { SYSTEM_PROMPT.to_string() },
    })];
    // Older turns are sent as shown (to keep within small models' context);
    // the newest question goes with its full app context.
    let last = history.len().saturating_sub(1);
    for (i, m) in history.iter().enumerate().skip(history.len().saturating_sub(12)) {
        let content = if i == last { m.sent.as_deref().unwrap_or(&m.content) } else { &m.content };
        messages.push(json!({ "role": m.role, "content": content }));
    }

    let (tx, rx) = channel();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = stop.clone();
    let base_url = settings.base_url.clone();
    let mut model = settings.model.clone();

    std::thread::spawn(move || {
        let send = |event: Event| {
            let _ = tx.send(event);
            ctx.request_repaint();
        };
        if model.trim().is_empty() {
            match list_models(&base_url) {
                Ok(list) if !list.is_empty() => model = list[0].clone(),
                Ok(_) => return send(Event::Error("The server has no model loaded. Load one in LM Studio (or `ollama pull` one) first.".into())),
                Err(e) => return send(Event::Error(e)),
            }
        }
        let body = json!({ "model": model, "messages": messages, "stream": true, "temperature": 0.4 });
        let resp = match agent().post(&endpoint(&base_url, "chat/completions")).send_json(&body) {
            Ok(r) => r,
            Err(e) => return send(Event::Error(friendly_connect_error(&e, &base_url))),
        };
        let status = resp.status().as_u16();
        let mut body = resp.into_body();
        if status != 200 {
            let text = body.read_to_string().unwrap_or_default();
            return send(Event::Error(format!("The AI server answered with status {status}: {}", shorten(&text, 300))));
        }

        let reader = BufReader::new(body.into_reader());
        for line in reader.lines() {
            if stop_flag.load(Ordering::Relaxed) {
                return;
            }
            let Ok(line) = line else {
                return send(Event::Error("The connection to the AI server was lost.".into()));
            };
            let data = line.trim().strip_prefix("data:").unwrap_or(line.trim()).trim();
            if data.is_empty() || data.starts_with(':') {
                continue;
            }
            if data == "[DONE]" {
                break;
            }
            let Ok(value) = serde_json::from_str::<Value>(data) else { continue };
            if let Some(err) = value.get("error") {
                let msg = err["message"].as_str().map(String::from).unwrap_or_else(|| err.to_string());
                return send(Event::Error(msg));
            }
            let choice = &value["choices"][0];
            // Streaming chunks carry `delta`; a server that ignores `stream` sends one `message`.
            let text = choice["delta"]["content"].as_str().or_else(|| choice["message"]["content"].as_str());
            if let Some(text) = text
                && !text.is_empty()
            {
                send(Event::Delta(text.to_string()));
            }
        }
        send(Event::Done);
    });
    Stream { rx, stop }
}

/// Splits a reasoning model's `<think>…</think>` part from its answer.
/// Returns (thinking, answer, still_thinking).
pub fn split_thinking(text: &str) -> (Option<&str>, &str, bool) {
    let trimmed = text.trim_start();
    let Some(rest) = trimmed.strip_prefix("<think>") else { return (None, text, false) };
    match rest.split_once("</think>") {
        Some((thinking, answer)) => (Some(thinking.trim()), answer.trim_start(), false),
        None => (Some(rest.trim()), "", true),
    }
}

pub fn shorten(text: &str, max_chars: usize) -> String {
    let text = text.trim();
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let cut: String = text.chars().take(max_chars).collect();
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_thinking() {
        assert_eq!(split_thinking("hello"), (None, "hello", false));
        assert_eq!(split_thinking("<think>hmm</think>\n\nAnswer"), (Some("hmm"), "Answer", false));
        assert_eq!(split_thinking("<think>still going"), (Some("still going"), "", true));
    }

    #[test]
    fn builds_endpoints() {
        assert_eq!(endpoint("http://localhost:1234/v1/", "models"), "http://localhost:1234/v1/models");
        assert_eq!(shorten("abcdef", 3), "abc…");
    }
}
