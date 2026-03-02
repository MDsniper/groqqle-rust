use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::json;

pub enum Provider {
    Groq,
    Glm,
}

pub struct LlmClient {
    api_key: String,
    http: Client,
    model: String,
    base_url: String,
    provider: Provider,
}

impl LlmClient {
    pub fn from_env() -> Option<Self> {
        // Prefer GLM when configured
        if let Ok(api_key) = std::env::var("GLM_API_KEY") {
            let model = std::env::var("GLM_MODEL").unwrap_or_else(|_| "glm-5".to_string());
            let base_url = std::env::var("GLM_BASE_URL")
                .unwrap_or_else(|_| "https://open.bigmodel.cn/api/paas/v4/chat/completions".to_string());
            return Some(Self {
                api_key,
                http: Client::new(),
                model,
                base_url,
                provider: Provider::Glm,
            });
        }

        if let Ok(api_key) = std::env::var("GROQ_API_KEY") {
            let model = std::env::var("GROQ_MODEL").unwrap_or_else(|_| "llama3-8b-8192".to_string());
            return Some(Self {
                api_key,
                http: Client::new(),
                model,
                base_url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
                provider: Provider::Groq,
            });
        }

        None
    }

    pub async fn summarize(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "messages": [{"role":"user","content":prompt}],
            "max_tokens": max_tokens,
            "temperature": 0.0
        });

        let mut req = self
            .http
            .post(&self.base_url)
            .json(&payload);

        req = match self.provider {
            Provider::Groq => req.bearer_auth(&self.api_key),
            Provider::Glm => req.header("Authorization", format!("Bearer {}", self.api_key)),
        };

        let resp = req.send().await?.error_for_status()?;

        let v: serde_json::Value = resp.json().await?;
        let content = v["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("unexpected LLM response shape"))?;
        Ok(content.to_string())
    }
}

pub fn fallback_summary(text: &str, max_chars: usize) -> String {
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    cleaned.chars().take(max_chars).collect()
}
