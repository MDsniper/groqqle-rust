use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn get_contents(url: &str) -> Result<String> {
    let client = Client::new();
    let html = client
        .get(url)
        .header("User-Agent", "groqqle-rust/0.1")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let doc = Html::parse_document(&html);
    let body_selector = Selector::parse("body").unwrap();
    let text = doc
        .select(&body_selector)
        .flat_map(|n| n.text())
        .collect::<Vec<_>>()
        .join(" ");

    Ok(text)
}
