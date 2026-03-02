use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use crate::models::SearchResult;

pub async fn search(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    let key = crate::config::get_setting("BRAVE_API_KEY").or_else(|| std::env::var("BRAVE_API_KEY").ok());
    if let Some(key) = key {
        let resp = Client::new()
            .get("https://api.search.brave.com/res/v1/web/search")
            .query(&[("q", query), ("count", &num_results.to_string())])
            .header("X-Subscription-Token", key)
            .send()
            .await?
            .error_for_status()?;
        let data: Value = resp.json().await?;
        let items = data["web"]["results"].as_array().cloned().unwrap_or_default();
        let results = items
            .into_iter()
            .map(|item| SearchResult {
                title: item["title"].as_str().unwrap_or("Untitled").to_string(),
                url: item["url"].as_str().unwrap_or_default().to_string(),
                description: item["description"].as_str().unwrap_or_default().to_string(),
                source: None,
                timestamp: None,
            })
            .collect();
        return Ok(results);
    }

    Ok(vec![SearchResult {
        title: format!("Search {}", query),
        url: format!("https://duckduckgo.com/?q={}", urlencoding::encode(query)),
        description: "No BRAVE_API_KEY set; returning direct search link fallback.".to_string(),
        source: Some("fallback".to_string()),
        timestamp: None,
    }])
}
