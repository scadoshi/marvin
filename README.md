# marvin

A small CLI chat application built to learn the [Rig](https://github.com/21-labs/rig) framework in Rust.

## What it does

Runs an interactive terminal chat loop backed by Anthropic's Claude via Rig. Supports a configurable preamble (system prompt) loaded from a `.env` file and maintains conversation history across turns.

### Commands

| Command | Description |
|---------|-------------|
| `/model` | Switch between available Claude models |
| `/tokens` | Show token usage |
| `/history` | Show last 10 messages from chat history |
| `/summarize` | Ask the agent to summarize the conversation |
| `/compact` | Condense conversation history |
| `/clear` | Clear chat history |
| `/help` | List all available commands |
| `/exit` | Print farewell message and end the session |

## Setup

1. Clone the repo and make sure you have Rust installed (`rustup` / `cargo`).
2. Copy `.env.example` to `.env` and fill in your values:
   - `ANTHROPIC_API_KEY` — your Anthropic API key
3. Build and run:

```sh
cargo run
```

## Goal

This project exists to learn Rig by reading its source, experimenting with its APIs, and building up features incrementally. Contributions and experiments are welcome.

## License

MIT — see [LICENSE](LICENSE).
