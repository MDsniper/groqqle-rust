use anyhow::Result;

use crate::{models::SearchResult, tools::web_news};

pub struct NewsAgent {
    num_results: usize,
}

impl NewsAgent {
    pub fn new(num_results: usize) -> Result<Self> {
        Ok(Self { num_results })
    }

    pub async fn process_request(&self, req: &str) -> Result<Vec<SearchResult>> {
        web_news::search_news(req, self.num_results).await
    }
}
