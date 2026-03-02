use anyhow::Result;
use url::Url;

use crate::{
    llm::{fallback_summary, LlmClient},
    models::SearchResult,
    tools::{web_contents, web_search},
};

pub struct WebAgent {
    num_results: usize,
    summary_length: usize,
    llm: Option<LlmClient>,
}

impl WebAgent {
    pub fn new(num_results: usize, summary_length: usize) -> Result<Self> {
        Ok(Self {
            num_results,
            summary_length,
            llm: LlmClient::from_env(),
        })
    }

    pub async fn process_request(&self, req: &str) -> Result<Vec<SearchResult>> {
        if Url::parse(req).is_ok() {
            return self.process_url(req).await;
        }
        web_search::search(req, self.num_results).await
    }

    async fn process_url(&self, url: &str) -> Result<Vec<SearchResult>> {
        let content = web_contents::get_contents(url).await.unwrap_or_default();
        let prompt = format!(
            "Summarize this page into an SEO headline + concise body:\n{}",
            content
        );

        let description = if let Some(llm) = &self.llm {
            llm.summarize(&prompt, 1024)
                .await
                .unwrap_or_else(|_| fallback_summary(&content, self.summary_length * 5))
        } else {
            fallback_summary(&content, self.summary_length * 5)
        };

        Ok(vec![SearchResult {
            title: format!("Summary of {}", url),
            url: url.to_string(),
            description,
            source: None,
            timestamp: None,
        }])
    }
}
