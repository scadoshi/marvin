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

## Session 2 — 2026-02-05

### What was done
- Extracted configuration logic into `Config` module — removed model management from Config, simplified to just preamble and API key storage.
- Added `/summarize` command — asks the agent to summarize the conversation history.
- Created `ChatState` module to encapsulate chat history and user input management.
- Major architectural refactor: implemented command pattern with individual command modules (`commands/exit.rs`, `commands/model.rs`, etc.).
- Created `Input` enum for clean command routing and pattern matching.
- Extracted `Runner` module to handle main loop logic, separated from `main.rs`.
- Created `ui` module for shared UI utilities (`horizontal_line()`).
- Fixed integer underflow bug in model selection (input "0" would panic).
- Fixed input handling to preserve input across command interruptions (allows breaking out of `/model` selection to handle other commands).

### What was learned
- Command pattern in Rust: each command implements `Command` trait with async `execute()` method.
- Module organization: separating concerns into `chat/`, `commands/`, `runner`, `config`, and `ui`.
- Using enums for input routing makes the main loop clean and exhaustive.
- `checked_sub()` for safe integer arithmetic to avoid underflow panics.
- How to handle nested input loops (model selection) while preserving input state for parent loop.

### Architecture evolution
- **Before**: Monolithic `main.rs` with all logic inline (~220 lines).
- **After**:
  - `main.rs`: 13 lines, just bootstraps `Runner::run()`.
  - `chat/mod.rs`: `State` struct managing history, input, agent, model.
  - `chat/input.rs`: `Input` enum for command parsing.
  - `commands/`: Individual modules per command implementing `Command` trait.
  - `runner.rs`: Main loop logic with pattern matching on `Input`.
  - `config.rs`: Environment config (preamble, API key).
  - `ui.rs`: UI utilities.

### Polish & documentation
- Updated `README.md` with all available commands.
- Reorganized context files: `project.md` → `purpose.md`, moved commit guidelines to `context/commit_guidelines.md`.
- Refined `purpose.md` to focus on AI assistant rules and project philosophy (no code writing, educational first).

### Refinements
- Simplified `/exit` command — removed AI-generated farewell, now just prints "Farewell!" and breaks the loop.
- Split `Command` trait into `Command` (sync) and `AsyncCommand` (async) to reduce compile times for commands that don't need async.
- Updated UI styling: width from 100 to 50, horizontal line from `=` to `-`.
- Alphabetized command matching in runner for better organization.

### Ideas for next time
- Streaming responses — explore Rig's streaming API for real-time output.
- Tool use / function calling — if Rig supports it, add tools the agent can call.
- Export/import chat — save and load conversation history to/from JSON.

## Session 3 — 2026-02-05 (continued)

### What was done
- Discovered deprecated/retired Anthropic model constants in Rig causing 404 errors.
- Researched current Anthropic model API status and deprecation timeline.
- Created GitHub issue #1370 documenting the bug.
- Fixed Rig's Anthropic model constants to use current versions.
- Updated 17 files in Rig repository (model constants, examples, tests, documentation).
- Successfully submitted PR to Rig repository following all contributing guidelines.

### What was learned
- Anthropic regularly deprecates older model versions (Claude 3.5 Sonnet retired Oct 2025, Claude 3.7 Sonnet deprecated).
- Current active Anthropic models (Feb 2026): Claude Opus 4.6, Claude Sonnet 4.5, Claude Haiku 4.5.
- How to contribute to open source: fork workflow, branch naming conventions, conventional commits.
- Rig's contributing guidelines: conventional commit format, clippy/fmt checks, test requirements.
- How to search and verify API model availability through official documentation.

### Bug fixed in Rig
- **Issue**: `CLAUDE_3_5_SONNET` and `CLAUDE_3_7_SONNET` constants pointed to retired/deprecated models.
- **Impact**: Users got 404 errors when trying to use these models.
- **Solution**: Updated constants to current model IDs and removed deprecated ones.
- **PR**: https://github.com/0xPlaygrounds/rig/pull/[number] (Fixes #1370)

### Files changed in Rig
- `src/providers/anthropic/completion.rs` — updated model constant definitions
- 14 example files — updated to use new constants
- 2 source files — updated tests and documentation

### Contributing workflow learned
1. Fork the repository on GitHub
2. Create descriptive branch (`fix/update-deprecated-anthropic-models`)
3. Make focused changes following project conventions
4. Run required checks (`cargo fmt`, `cargo clippy`, `cargo test`)
5. Write conventional commit messages with issue reference
6. Push to fork and create PR with clear description

### Documentation
- Created organized structure for tracking Rig contributions:
  - `context/rig_contributions/issues/` - Bug reports and fixes
  - `context/rig_contributions/enhancements/` - Feature additions
  - Each with subdirectories: `reported/`, `pr_submitted/`, `pr_approved/`
- Documented this discovery in `rig_contributions/issues/pr_submitted/anthropic_deprecated_models.md`

### Ideas for next time
- Continue exploring Rig's streaming API.
- Monitor PR for feedback from maintainers.
- Consider other areas in Rig that might need similar updates.

## Session 4 — 2026-02-16

### What was done
- Implemented streaming responses using Rig's `StreamingChat` trait.
- Added `futures` crate for `StreamExt` to iterate over async streams.
- Created unified `stream()` method replacing separate `send_user_message()` and `send_assistant_message()`.
- Added token tracking: `total_input_tokens_used` and `total_output_tokens_used` fields in `State`.
- Added `/tokens` command to display cumulative token usage.
- Updated `/summarize` to use streaming.
- Refactored main chat loop to use streaming for real-time output.

### What was learned
- `Stream` is Rust's async equivalent of `Iterator` — lives in `futures-core`.
- The base `Stream` trait only has `poll_next()` — need `StreamExt` for ergonomic `.next()` method.
- `StreamExt::next()` returns a `Future<Output = Option<Item>>` — each `.await` yields the next chunk.
- Streaming vs blocking: `agent.chat().await` waits for entire response; `stream.next().await` yields chunks as they arrive.
- Rig's streaming response types: `MultiTurnStreamItem::StreamAssistantContent::Text` for chunks, `Final` for usage stats, `FinalResponse` for history.

### futures crate overview
- `StreamExt`: `.next()`, `.map()`, `.filter()`, `.collect()` for streams
- `join!`: Run multiple futures concurrently, wait for all
- `select!`: Run multiple futures, return when first completes
- `oneshot`: Single-value channel (notify when done)
- `mpsc`: Async multi-producer single-consumer channel
- `.fuse()`: Safe to poll after completion
- `.then()`: Chain futures (like `.map()` but returns Future)
- `Sink`: Write-side counterpart to Stream

### Rig capabilities researched
- **Has**: `Model` struct with `context_length` field, `ModelListingClient` trait, token usage in responses
- **Missing**: Anthropic doesn't implement `ModelListingClient`, no pricing data, no input context windows
- Anthropic's `calculate_max_tokens()` is for OUTPUT limits only, hardcoded per model family

### Ideas for next time
- `/compact` command — erase history and replace with summarized conversation state
- Export/import history — serialize `Vec<Message>` to JSON
- Tool calling — let agent invoke existing commands (`/clear`, `/history`, `/summarize`)
- Contribute `ModelListingClient` for Anthropic to Rig

## Session 4 (continued) — 2026-02-16

### What was done
- Added `/compact` command — clears history and replaces with AI-generated context summary
- Implemented dynamic model discovery from Anthropic's `/v1/models` API
- Created `anthropic` module with `GetAnthropicModels` trait and `ModelInfo` struct
- Redesigned startup flow — model selection required before chat begins
- Added `welcome_message()` with loading animation in `ui` module
- Removed hardcoded model constants in favor of API-fetched list
- Refactored `State` — removed `model` field, derives current model from `agent.model.model`
- Simplified `Config` — removed `build_agent()` method
- Fixed stdout buffering issue — `print!()` without newline requires `flush()` for immediate display

### What was learned
- Stdout is line-buffered by default — `print!()` accumulates until newline or manual `flush()`
- RPITIT (Return Position Impl Trait In Trait) — `impl Future` in trait signatures works without importing `Future` since Rust 1.75
- `std::thread::sleep` blocks tokio runtime — use `tokio::time::sleep` in async contexts (acceptable for one-time startup animation)
- Anthropic's `/v1/models` endpoint returns `id`, `display_name`, `created_at`, `type` for each model

### New dependencies
- `reqwest` — HTTP client for Anthropic API calls
- `serde` + `serde_json` — JSON deserialization for API responses

### Architecture changes
- `State::new()` is now `async` and fallible
- Agent building moved from `Config` to `State` (inline in `new()` and `set_agent()`)
- Model selection happens at startup, not defaulted

### Ideas for next time
- Tool calling — let agent invoke `/clear`, `/history`, `/summarize`, `/compact`
- Export/import history — `/export <file>` and `/import <file>` for JSON persistence
- `/cost` command — estimate cost based on token usage and model pricing

## Session 5 — 2026-02-17

### What was done
- Renamed project from "scotbot" to "marvin"
- Moved preamble from `.env` to `src/chat/preamble.txt` — loaded at compile time via `include_str!()`
- Created Marvin preamble as a Rust learning companion that asks multiple choice questions
- Added `/save` command — persists chat history to `chats/<id>.json`
- Added `/import <id>` command — loads previous chat session history into current context
- Each chat session now gets a unique `u16` ID, displayed at startup
- Renamed `Input` enum variants for consistency with command file names (e.g., `ExitCommand` → `ExitProcess`)
- Renamed command files for consistency (`show_message_history.rs` → `show_chat_history.rs`, `summarize_context.rs` → `show_context_summary.rs`)
- Updated README and help messages to document new commands
- Added `chats/` directory to `.gitignore`

### What was learned
- Using `HashSet` for efficient ID lookup when finding next available chat ID
- `std::path::Path::file_prefix()` for getting filename without extension
- `std::fs::create_dir_all()` for ensuring directory exists before writing
- Importance of consistent naming between enums and module files — reduces cognitive load
- `serde_json::to_writer_pretty()` for human-readable JSON output

### Architecture changes
- `State` now holds a `u16` ID field for the current chat session
- `next_chat_id()` scans `chats/` directory to find first unused ID
- `Config` simplified — no longer holds preamble (removed `preamble()` method)
- Preamble is now a `static` string compiled into the binary
- `welcome_message()` moved earlier in startup, now displays chat ID

### Refactoring
- `Input::Empty` → `Input::None`, `is_empty()` → `is_none()`
- `Input::Message` → `Input::SendMessage`
- All command-related enum variants now match their file names exactly

### Ideas for next time
- Tool calling — let agent invoke commands based on conversation context
- `/cost` command — estimate cost based on token usage and model pricing
- Better error message for `/import` without valid ID (currently falls through to SendMessage)

## Session 6 — 2026-02-19

### What was done
- Major refactoring: renamed `State` → `Chat` and `Input` → `ChatInput` throughout codebase (20 files)
- Renamed history methods: `history()` → `chat_history()`, `clear_history()` → `clear_chat_history()`, etc.
- Reorganized module structure:
  - Moved `src/config.rs` → `src/chat/config.rs`
  - Split `src/anthropic.rs` into `src/anthropic/mod.rs` and `src/anthropic/get_models.rs`
- Created `src/agent_tools/` module with `Adder` tool implementation
- Integrated `Adder` tool into agent builder with `.tool(Adder)` and `.default_max_turns(100)`
- Enhanced token display: added `Formatted` trait to render counts with comma separators (e.g., `1,234,567`)
- Added unit test for token formatting
- Consolidated imports: combined multi-line `use crate::` statements into single grouped imports
- Updated all user_commands trait implementations to use `Chat` and `ChatInput`

### What was learned
- Rig's `Tool` trait: implement `name`, `definition()`, and `call()` for the agent to invoke
- `ToolDefinition` construction: requires name, description, and JSON Schema for parameters
- Agent builder accepts multiple `.tool()` calls to register available tools
- `.default_max_turns(n)` limits agentic loops to prevent infinite tool recursion
- Naming consistency across files reduces cognitive load when navigating large projects
- Module reorganization patterns: grouping related functionality by domain

### Architecture improvements
- **Naming**: State → Chat is more descriptive of the struct's purpose
- **Organization**: Config now lives with Chat module (logical cohesion)
- **Tools**: Foundation laid for extensible tool system — easy to add new tools
- **Imports**: Consolidated for readability and reduced boilerplate

### Ideas for next time
- **Math tools**: `Multiply`, `Subtract`, `Divide` (exercise different Tool traits)
- **String tools**: `Reverse`, `Uppercase`, `Lowercase`, `CountWords`
- **Validator tools**: `IsValidEmail`, `IsValidJSON`
- **File tools**: `ReadFile`, `WriteFile`, `ListDirectory` (careful with sandboxing)
- **Network tools**: `FetchURL`, `CheckDomain` (explore reqwest integration)
- **Calculator with memory**: tool that maintains state across invocations
- **Tool chaining**: agent combines multiple tools to solve complex tasks

## Session 7 — 2026-02-21

### What was done
- Implemented four Tavily web tools: `search_web`, `extract_url`, `crawl_website`, `map_website`
- Added `schemars` crate for automatic JSON Schema generation from Rust types
- Refactored all math tools (`Add`, `Subtract`, `Multiply`, `Divide`) to use schemars
- Created `TavilyClient` with `Arc` sharing pattern for efficient client reuse across tools
- Implemented `WebTools` trait on `Arc<TavilyClient>` for tool construction
- Added error handling utilities: `ToToolError`, `ToToolResult` traits and `SomeError` wrapper
- Fixed Tavily API errors by adding `#[serde(skip_serializing_if = "Option::is_none")]` to optional fields
- Simplified tool output to pass-through `serde_json::Value` instead of typed response structs

### What was learned
- **schemars**: `#[derive(JsonSchema)]` + `schema_for!()` eliminates manual JSON Schema writing
- **serde skip_serializing_if**: APIs often reject `null` values for optional fields — omit them entirely
- **Arc sharing**: Store shared clients in `Arc`, clone into each tool that needs it
- **Tool output types**: Can use `serde_json::Value` for pass-through when typed parsing isn't needed
- **Extension traits**: `ToToolResult` pattern for ergonomic error conversion (`.to_tool_result()?`)
- **thiserror**: Quick `#[derive(Error)]` for simple error wrappers

### New dependencies
- `schemars = "1.2.1"` — JSON Schema generation from Rust types
- `thiserror = "2.0.18"` — Derive macro for Error trait
- `url = "2.5.8"` — URL parsing and manipulation

### Architecture
- `src/agent_tools/web/tavily.rs` — `TavilyClient` with HTTP client and API key
- `src/agent_tools/web/{search,extract,crawl,map}/` — Each tool has:
  - `mod.rs` — Tool struct and `impl Tool`
  - `request.rs` — Args struct with schemars annotations
  - `response.rs` — Response types (currently unused, kept for reference)
- `src/agent_tools/mod.rs` — Error utilities (`ToToolError`, `ToToolResult`, `SomeError`)
- `src/chat/config.rs` — Added `tavily_api_key()` accessor
- `src/chat/mod.rs` — Added `tavily_client: Arc<TavilyClient>` field

### Debugging insights
- Tavily returns 400/422 when optional fields are sent as `null`
- Parse response body before checking status to include error details in messages
- Bearer auth is correct for Tavily API (not body-based api_key)

### Ideas for next time
- Re-enable typed response parsing with validation
- Add retry/backoff for rate limits
- Implement Research API (async polling pattern)
- Consider caching for repeated queries
