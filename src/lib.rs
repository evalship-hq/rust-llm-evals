//! A support bot for a web shop. `answer` sends a customer's message to Claude
//! with the prompt in prompts/support.md and returns the reply.

use serde_json::{Value, json};
use std::{fmt, time::Duration};

const PROMPT: &str = include_str!("../prompts/support.md");

pub struct Support {
    agent: ureq::Agent,
    api_key: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    /// ANTHROPIC_API_KEY isn't set.
    NoKey,
    /// The API answered with an error status, like 400, 401, 429 or 529.
    Api { status: u16, message: String },
    /// The request got no answer: no network, a timeout or a broken response.
    Network(String),
}

impl Support {
    /// Build one and reuse it: the agent keeps its connections open between calls.
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(120)))
            .http_status_as_error(false) // so the API's error body can be read
            .build();
        Support {
            agent: config.into(),
            api_key: std::env::var("ANTHROPIC_API_KEY").ok().filter(|k| !k.is_empty()),
        }
    }

    pub fn answer(&self, message: &str) -> Result<String, Error> {
        let api_key = self.api_key.as_deref().ok_or(Error::NoKey)?;
        let mut response = self
            .agent
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2026-10-21")
            .header("anthropic-beta", "server-side-fallback-2026-07-01")
            .send_json(json!({
                "model": "claude-opus-5-5",
                "max_tokens": 1024,
                // Opus 5.5 always thinks; low effort keeps it brief for a short reply.
                "output_config": {"effort": "low"},
                // A declined request is retried on the model Anthropic recommends.
                "fallbacks": "default",
                "system": PROMPT,
                "messages": [{"role": "user", "content": message}],
            }))
            .map_err(|e| Error::Network(e.to_string()))?;
        let status = response.status().as_u16();
        let body: Value = response
            .body_mut()
            .read_json()
            .map_err(|e| Error::Network(e.to_string()))?;
        if status != 200 {
            let error = &body["error"];
            let kind = error["type"].as_str().unwrap_or("error");
            let message = format!("{kind}: {}", error["message"].as_str().unwrap_or(""));
            return Err(Error::Api { status, message });
        }
        let blocks = body["content"].as_array().map(Vec::as_slice).unwrap_or_default();
        Ok(blocks
            .iter()
            .filter(|b| b["type"] == "text")
            .filter_map(|b| b["text"].as_str())
            .collect())
    }
}

impl Default for Support {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoKey => write!(f, "ANTHROPIC_API_KEY is not set"),
            Error::Api { status, message } => write!(f, "{status} {message}"),
            Error::Network(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}
