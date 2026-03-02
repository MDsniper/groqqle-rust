mod agents;
mod api;
mod config;
mod llm;
mod models;
mod tools;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use dotenvy::dotenv;
use tracing_subscriber::EnvFilter;

use crate::agents::{news::NewsAgent, web::WebAgent};
use crate::api::run_api;

#[derive(Parser, Debug)]
#[command(name = "groqqle-rust")]
#[command(about = "Rust reimplementation of Groqqle", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run HTTP API server
    Api {
        #[arg(long, default_value_t = 5000)]
        port: u16,
        #[arg(long, default_value_t = 10)]
        num_results: usize,
        #[arg(long, default_value_t = 300)]
        summary_length: usize,
    },
    /// Perform one-off search from CLI
    Search {
        query: String,
        #[arg(long, value_enum, default_value_t = SearchType::Web)]
        search_type: SearchType,
        #[arg(long, default_value_t = 10)]
        num_results: usize,
        #[arg(long, default_value_t = 300)]
        summary_length: usize,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum SearchType {
    Web,
    News,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Api {
            port,
            num_results,
            summary_length,
        } => run_api(port, num_results, summary_length).await,
        Command::Search {
            query,
            search_type,
            num_results,
            summary_length,
        } => {
            let output = match search_type {
                SearchType::Web => {
                    let agent = WebAgent::new(num_results, summary_length)?;
                    agent.process_request(&query).await?
                }
                SearchType::News => {
                    let agent = NewsAgent::new(num_results)?;
                    agent.process_request(&query).await?
                }
            };
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
    }
}
