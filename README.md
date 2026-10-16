# marvin

A small CLI chat application built to learn the [Rig](https://github.com/0xPlaygrounds/rig) framework in Rust.

## What it does

Runs an interactive terminal chat loop backed by Anthropic's Claude via Rig. Supports a configurable preamble (system prompt) in `src/chat/preamble.txt` and maintains conversation history across turns.

### User Commands

| Command | Description |
|---------|-------------|
| `/model` | Switch between available Claude models |
| `/tokens` | Show token usage |
| `/history` | Show last 10 messages from chat history |
| `/save` | Save chat history to file |
| `/import <id>` | Import chat history from a saved file |
| `/summarize` | Ask the agent to summarize the conversation |
| `/compact` | Condense conversation history |
| `/clear` | Clear chat history |
| `/help` | List all available commands |
| `/exit` | Print farewell message and end the session |

### Agent Tools

The AI agent can invoke the following tools during conversation:

**Math Tools**
| Tool | Description |
|------|-------------|
| `add` | Add two numbers |
| `subtract` | Subtract two numbers |
| `multiply` | Multiply two numbers |
| `divide` | Divide two numbers |

**Web Tools** (via [Tavily API](https://tavily.com))
| Tool | Description |
|------|-------------|
| `search_web` | Search the web for current information |
| `extract_url` | Extract clean content from specific URLs |
| `crawl_website` | Crawl a website and extract content from linked pages |
| `map_website` | Discover all URLs on a website without extracting content |

## Setup

1. Clone the repo and make sure you have Rust installed (`rustup` / `cargo`).
2. Copy `.env.example` to `.env` and fill in your values:
   - `ANTHROPIC_API_KEY`: your Anthropic API key
   - `TAVILY_API_KEY`: your Tavily API key (for web tools)
3. Build and run:

```sh
cargo run
```

## Goal

This project exists to learn Rig by reading its source, experimenting with its APIs, and building up features incrementally. Contributions and experiments are welcome.

## License

MIT. See [LICENSE](LICENSE).
