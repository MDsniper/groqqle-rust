use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(default = "default_num_results")]
    pub num_results: usize,
    #[serde(default = "default_summary_length")]
    pub summary_length: usize,
    #[serde(default = "default_search_type")]
    pub search_type: String,
}

fn default_num_results() -> usize {
    10
}
fn default_summary_length() -> usize {
    300
}
fn default_search_type() -> String {
    "web".to_string()
}
