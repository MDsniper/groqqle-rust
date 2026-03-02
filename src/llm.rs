use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::json;

pub struct GroqClient {
    api_key: String,
    http: Client,
    model: String,
}

impl GroqClient {
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("GROQ_API_KEY").ok()?;
        let model = std::env::var("GROQ_MODEL").unwrap_or_else(|_| "llama3-8b-8192".to_string());
        Some(Self {
            api_key,
            http: Client::new(),
            model,
        })
    }

    pub async fn summarize(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "messages": [{"role":"user","content":prompt}],
            "max_tokens": max_tokens,
            "temperature": 0.0
        });

        let resp = self
            .http
            .post("https://api.groq.com/openai/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let v: serde_json::Value = resp.json().await?;
        let content = v["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("unexpected Groq response shape"))?;
        Ok(content.to_string())
    }
}

pub fn fallback_summary(text: &str, max_chars: usize) -> String {
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    cleaned.chars().take(max_chars).collect()
}
