# Groqqle-rust

Rust reimplementation of key Groqqle behavior inspired by J. Gravelle's original Python project:
<https://github.com/jgravelle/Groqqle>

## What this includes

- CLI search mode (`web` or `news`)
- HTTP API mode with `POST /search`
- URL content fetching + summarization flow
- Optional Groq-backed summarization (`GROQ_API_KEY`)
- Provider/tool/agent separation similar to original architecture

## Build

```bash
cd /Users/bwilliams/.openclaw/workspace/Groqqle-rust
cargo build
```

## Run (CLI)

```bash
# Web search
cargo run -- search "latest rust async patterns" --search-type web --num-results 5

# News search
cargo run -- search "AI chips" --search-type news --num-results 5

# Summarize a URL (web type auto-detects URL input)
cargo run -- search "https://www.rust-lang.org/"
```

## Run (API)

```bash
cargo run -- api --port 5000 --num-results 10 --summary-length 300
```

Example request:

```bash
curl -s http://127.0.0.1:5000/search \
  -H 'content-type: application/json' \
  -d '{"query":"rust web frameworks","search_type":"web","num_results":5}' | jq
```

## Environment variables

- `GROQ_API_KEY` (optional): enables Groq summarization
- `GROQ_MODEL` (optional): defaults to `llama3-8b-8192`
- `BRAVE_API_KEY` (optional): enables Brave Search API in `web_search`

## Feature parity checklist (vs original Python Groqqle)

- [x] Agent abstraction (`Web`/`News`)
- [x] API mode with `/search` endpoint
- [x] Query routing by `search_type`
- [x] URL detection and content summarization path
- [x] LLM provider integration (Groq)
- [x] Search result schema with title/url/description (+ optional source/timestamp)
- [ ] Streamlit web UI parity
- [ ] Selenium-based browser search fallback
- [ ] Full Bing news scraping parity
- [ ] Image URL analysis parity
- [ ] Comprehension grade + humanize prompt controls

## Project files

- `ARCHITECTURE.md` — architecture and original-project analysis
- `src/main.rs` — CLI entrypoint and mode dispatch
- `src/api.rs` — Axum API server
- `src/agents/` — Web/News agent implementations
- `src/tools/` — search/content/news tools
- `src/llm.rs` — Groq + fallback summarization
- `src/models.rs` — request/response types
