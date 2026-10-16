# Purpose: marvin

## What this project is
marvin is a personal learning project for understanding the Rig framework. The goal is to learn by building a real CLI chatbot application, exploring Rig's APIs incrementally through implementing features.

## Why this project exists
- Learn Rig framework: understand how Rig works by building with it, not just reading docs
- Explore AI patterns: experiment with conversation management, tool use, streaming, etc.
- Practice Rust architecture: iteratively improve code structure as complexity grows
- Stay curious: each feature should teach something new about Rig or Rust

## Tech stack
- Language: Rust (2024 edition)
- AI framework: rig-core 0.31.0
- Provider: Anthropic (Claude)
- Async runtime: Tokio

## Architecture overview
- CLI REPL: user types messages and commands, agent responds
- Command pattern: each command (`/model`, `/history`, etc.) is a separate module
- Chat state: manages conversation history, current input, and agent instance
- Config: loads environment variables (API key)
- Preamble: system prompt loaded from `src/chat/preamble.txt` at compile time

## AI assistant rules

### Do NOT write code
- **Never** write, edit, or implement code for the user
- Suggest ideas, outline approaches, point out bugs
- Let the user write the code, since they're learning

### Educational first
- Each suggestion should teach something about Rig or Rust
- Prioritize learning over shipping
- Encourage exploration of different Rig APIs

### Suggest feature ideas
When the user asks "what's next?", suggest features that:
- Exercise different parts of the Rig API (streaming, tools, embeddings, etc.)
- Solve real problems (save/load chats, token counting, cost tracking)
- Incrementally increase complexity
- Are achievable in one session

### Keep it focused
- Small, focused changes over large refactors
- One feature teaches one thing
- This is a learning project, not a product
