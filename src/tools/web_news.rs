use anyhow::Result;
use chrono::Utc;

use crate::models::SearchResult;

pub async fn search_news(query: &str, num_results: usize) -> Result<Vec<SearchResult>> {
    // Lightweight fallback implementation.
    let mut out = Vec::new();
    for i in 0..num_results.max(1).min(10) {
        out.push(SearchResult {
            title: format!("News placeholder {} for {}", i + 1, query),
            url: format!("https://www.bing.com/news/search?q={}", urlencoding::encode(query)),
            description: "Implement provider-backed news scraping/API for production use.".to_string(),
            source: Some("bing-news-fallback".to_string()),
            timestamp: Some(Utc::now()),
        });
    }
    Ok(out)
}
