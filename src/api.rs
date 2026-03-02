use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Json, Router,
};
use serde::Deserialize;

use crate::{
    agents::{news::NewsAgent, web::WebAgent},
    config::{load_config, save_config, AppConfig},
    models::SearchRequest,
};

#[derive(Clone)]
struct AppState {
    default_num_results: usize,
    default_summary_length: usize,
}

#[derive(Debug, Deserialize)]
struct SettingsForm {
    glm_api_key: Option<String>,
    glm_model: Option<String>,
    glm_base_url: Option<String>,
    brave_api_key: Option<String>,
    groq_api_key: Option<String>,
    groq_model: Option<String>,
}

pub async fn run_api(port: u16, default_num_results: usize, default_summary_length: usize) -> Result<()> {
    let state = AppState {
        default_num_results,
        default_summary_length,
    };

    let app = Router::new()
        .route("/", get(index_page))
        .route("/search", post(search_handler))
        .route("/settings", get(settings_page).post(save_settings))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Groqqle-rust API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html><head><meta charset='utf-8'><title>Groqqle Rust</title></head>
<body style='font-family:system-ui;max-width:900px;margin:2rem auto;'>
<h1>Groqqle Rust</h1>
<p>Simple UI</p>
<form id='f'>
  <input id='q' name='query' placeholder='Search query or URL' style='width:70%' />
  <select id='t' name='search_type'><option value='web'>web</option><option value='news'>news</option></select>
  <button type='submit'>Search</button>
</form>
<pre id='out' style='white-space:pre-wrap;background:#f6f6f6;padding:1rem;'></pre>
<p><a href='/settings'>Settings</a></p>
<script>
document.getElementById('f').addEventListener('submit', async (e) => {
 e.preventDefault();
 const body = {query: document.getElementById('q').value, search_type: document.getElementById('t').value, num_results: 5, summary_length: 300};
 const r = await fetch('/search', {method:'POST', headers:{'content-type':'application/json'}, body: JSON.stringify(body)});
 const j = await r.json();
 document.getElementById('out').textContent = JSON.stringify(j, null, 2);
});
</script>
</body></html>"#,
    )
}

async fn settings_page() -> Html<String> {
    let c = load_config();
    let v = |s: &Option<String>| s.clone().unwrap_or_default();
    Html(format!(
        "<!doctype html><html><head><meta charset='utf-8'><title>Settings</title></head><body style='font-family:system-ui;max-width:900px;margin:2rem auto;'><h1>API Key Settings</h1><form method='post' action='/settings'>
        <label>GLM API Key</label><br/><input type='password' name='glm_api_key' value='{glm_api_key}' style='width:100%'/><br/><br/>
        <label>GLM Model</label><br/><input name='glm_model' value='{glm_model}' style='width:100%'/><br/><br/>
        <label>GLM Base URL</label><br/><input name='glm_base_url' value='{glm_base_url}' style='width:100%'/><br/><br/>
        <label>Brave API Key</label><br/><input type='password' name='brave_api_key' value='{brave_api_key}' style='width:100%'/><br/><br/>
        <label>Groq API Key (fallback)</label><br/><input type='password' name='groq_api_key' value='{groq_api_key}' style='width:100%'/><br/><br/>
        <label>Groq Model</label><br/><input name='groq_model' value='{groq_model}' style='width:100%'/><br/><br/>
        <button type='submit'>Save Settings</button>
        </form><p>Leave a key field blank to remove it.</p><p><a href='/'>Back</a></p></body></html>",
        glm_api_key = html_escape::encode_double_quoted_attribute(&v(&c.glm_api_key)),
        glm_model = html_escape::encode_double_quoted_attribute(&v(&c.glm_model)),
        glm_base_url = html_escape::encode_double_quoted_attribute(&v(&c.glm_base_url)),
        brave_api_key = html_escape::encode_double_quoted_attribute(&v(&c.brave_api_key)),
        groq_api_key = html_escape::encode_double_quoted_attribute(&v(&c.groq_api_key)),
        groq_model = html_escape::encode_double_quoted_attribute(&v(&c.groq_model)),
    ))
}

async fn save_settings(Form(f): Form<SettingsForm>) -> impl IntoResponse {
    let clean = |v: Option<String>| {
        v.and_then(|s| {
            let t = s.trim().to_string();
            if t.is_empty() { None } else { Some(t) }
        })
    };

    let cfg = AppConfig {
        glm_api_key: clean(f.glm_api_key),
        glm_model: clean(f.glm_model),
        glm_base_url: clean(f.glm_base_url),
        brave_api_key: clean(f.brave_api_key),
        groq_api_key: clean(f.groq_api_key),
        groq_model: clean(f.groq_model),
    };

    match save_config(&cfg) {
        Ok(_) => (StatusCode::SEE_OTHER, [("Location", "/settings")]).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("save failed: {e}"))
            .into_response(),
    }
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
