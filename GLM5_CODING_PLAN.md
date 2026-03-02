# GLM-5 Coding Plan for groqqle-rust

## Goal
Use GLM-5 as the default LLM for URL summarization and keep Groq as fallback.

## Implemented
- Added `GLM_API_KEY` support.
- Added `GLM_MODEL` (default `glm-5`).
- Added `GLM_BASE_URL` override.
- Provider selection order: **GLM first**, then Groq.
- Unified client (`LlmClient`) for both providers.

## Validation checklist
1. Build passes (`cargo build`).
2. CLI search works without any keys (fallback summary path).
3. With `GLM_API_KEY` set, URL summarization uses GLM endpoint.
4. If GLM unavailable and Groq key exists, fallback to Groq.

## Example env
```bash
export GLM_API_KEY=your_key
export GLM_MODEL=glm-5
# optional:
# export GLM_BASE_URL=https://open.bigmodel.cn/api/paas/v4/chat/completions
```
