use std::net::SocketAddr;

use anyhow::Result;
use axum::{extract::State, routing::post, Json, Router};

use crate::{
    agents::{news::NewsAgent, web::WebAgent},
    models::SearchRequest,
};

#[derive(Clone)]
struct AppState {
    default_num_results: usize,
    default_summary_length: usize,
}

pub async fn run_api(port: u16, default_num_results: usize, default_summary_length: usize) -> Result<()> {
    let state = AppState {
        default_num_results,
        default_summary_length,
    };

    let app = Router::new()
        .route("/search", post(search_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Groqqle-rust API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn search_handler(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<Vec<crate::models::SearchResult>>, axum::http::StatusCode> {
    let num_results = if req.num_results == 0 {
        state.default_num_results
    } else {
        req.num_results
    };

    let summary_length = if req.summary_length == 0 {
        state.default_summary_length
    } else {
        req.summary_length
    };

    let result = if req.search_type.eq_ignore_ascii_case("news") {
        NewsAgent::new(num_results)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
            .process_request(&req.query)
            .await
    } else {
        WebAgent::new(num_results, summary_length)
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
            .process_request(&req.query)
            .await
    }
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}
