# Progress

## Session 1 — 2026-02-04

### What was done
- Set up the initial project: working CLI chat loop against Anthropic via Rig using `CLAUDE_3_5_HAIKU`.
- Added `/model` command — presents a numbered menu of available Anthropic models and rebuilds the agent on selection. Taught the builder pattern (`client.agent().preamble().build()`) and that the agent is immutable once built.
- Added `/history` command — prints the last 10 messages with truncation at 300 chars. Required digging into `Message`, `UserContent`, and `AssistantContent` enums to pattern-match and extract text. Learned that there are a lot of abstractions/variants between you and the raw message text.
- Added `.env.example`, `README.md`, `LICENSE` (MIT), and `context/project.md`.

### What was learned
- Rig's `Message` type is an enum with nested content enums (`UserContent`, `AssistantContent`) — getting to the actual text string requires matching through multiple layers.
- The agent is built via a builder and is not mutable after construction; swapping models means rebuilding it.
- The available Anthropic model constants live in `rig::providers::anthropic::completion`.

### Fixed
- Added `user_input.clear()` on the empty-input `continue` path — no more input accumulation.
- Moved `user_input.clear()` earlier (before the `agent.chat()` call) so it's always cleared regardless of path.
- Added `continue` after `/model` and `/history` blocks so they no longer fall through to the regular message send.

### Added later in session
- `/clear` command to reset chat history.
- `/help` command — lists all available commands with descriptions.
- `/history` improvements: shows empty-history message when there's nothing to show, re-reversed the `.rev().take().rev()` so messages display in chronological order, added `---` separators between messages.
- Fixed the `document` typo in the `UserContent::Document` branch.

### Ideas for next time
- Streaming responses — next Rig API surface worth exploring.
