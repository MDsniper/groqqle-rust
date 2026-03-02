# Groqqle-rust Architecture

This project is a Rust reimplementation of core Groqqle patterns from <https://github.com/jgravelle/Groqqle>.

## Original Groqqle (Python) architecture summary

- **Entry point:** `Groqqle.py`
  - Streamlit UI mode
  - Flask API mode (`/search`)
- **Agent layer:**
  - `Web_Agent` for web search and URL summarization
  - `News_Agent` for Bing news scraping
- **Tool layer:**
  - `WebSearch_Tool` (Selenium + HTTP fallback)
  - `WebGetContents_Tool` (requests + BeautifulSoup extraction)
- **Provider abstraction:**
  - `ProviderFactory` → `GroqProvider` / `AnthropicProvider` (stub)
- **Behavioral flow:**
  1. Parse query
  2. If URL, fetch and summarize content (or image analysis)
  3. If non-URL, run search (web/news)
  4. Return structured results (`title`, `url`, `description`, optional source/timestamp)

## Rust implementation architecture

- **Entry point:** `src/main.rs`
  - `api` subcommand starts Axum HTTP server
  - `search` subcommand performs one-off searches from CLI
- **API server:** `src/api.rs`
  - POST `/search` accepts JSON request and routes to Web/News agent
- **Agent layer:** `src/agents/*`
  - `WebAgent`: URL summarization and web search
  - `NewsAgent`: news search facade
- **Tool layer:** `src/tools/*`
  - `web_search`: Brave Search API integration (fallback if key missing)
  - `web_contents`: HTML fetch + body text extraction
  - `web_news`: placeholder/fallback news provider
- **LLM integration:** `src/llm.rs`
  - Optional Groq chat completions via `GROQ_API_KEY`
  - Deterministic fallback summarizer when key is missing/fails
- **Data contracts:** `src/models.rs`
  - `SearchRequest`, `SearchResult`

## Notes

- This version favors a clean, testable service/CLI architecture over UI parity with Streamlit.
- Core API shape and flow are preserved, with room to add stronger search/news backends and richer prompt controls.
