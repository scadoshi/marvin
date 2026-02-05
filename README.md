# scotbot

A small CLI chat application built to learn the [Rig](https://github.com/21-labs/rig) framework in Rust.

## What it does

Runs an interactive terminal chat loop backed by Anthropic's Claude via Rig. Supports a configurable preamble (system prompt) loaded from a `.env` file and maintains conversation history across turns.

### Commands

| Command | Description |
|---------|-------------|
| `/exit` | Ends the session. If there's chat history, the model generates a farewell first. |

## Setup

1. Clone the repo and make sure you have Rust installed (`rustup` / `cargo`).
2. Copy `.env.example` to `.env` and fill in your values:
   - `ANTHROPIC_API_KEY` — your Anthropic API key
   - `PREAMBLE` — the system prompt for the agent
3. Build and run:

```sh
cargo run
```

## Goal

This project exists to learn Rig by reading its source, experimenting with its APIs, and building up features incrementally. Contributions and experiments are welcome.

## License

MIT — see [LICENSE](LICENSE).
