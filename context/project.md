# Project Context — scotbot

## Purpose
scotbot is a small, personal learning project. The goal is to understand the Rig framework by building a real application against it, reading its internals, and incrementally adding features.

## Tech stack
- **Language:** Rust (2024 edition)
- **AI framework:** rig-core 0.29.0
- **Provider:** Anthropic (Claude) via `rig::providers::anthropic`
- **Async runtime:** Tokio (macros + rt-multi-thread)
- **Env vars:** dotenvy
- **Error handling:** anyhow

## Key files
| Path | Role |
|------|------|
| `src/main.rs` | Entry point. Owns the REPL loop, agent construction, and chat history. |
| `Cargo.toml` | Dependencies and package metadata. |
| `.env` | Runtime config (API key, preamble). Gitignored. |
| `context/project.md` | This file — AI-facing project context. |

## Current architecture
- The agent is constructed once at startup using the Rig builder pattern: `client.agent(model).preamble(...).build()`.
- Chat history is a `Vec<Message>` passed to `agent.chat()` each turn.
- The only special command is `/exit`, which optionally triggers a farewell response before breaking the loop.
- Model is currently hardcoded to `CLAUDE_3_5_HAIKU`.

## Known issues / next steps
- `user_input.clear()` is only called at the bottom of the loop; the `continue` path on empty input skips it, causing input to accumulate via `read_line` appends.
- Good candidates for next features: `/history` command, `/model` switching, streaming responses.

## Conventions
- Keep changes small and focused — this is a learning project, not a product.
- Each new feature should exercise a different part of the Rig API where possible.
