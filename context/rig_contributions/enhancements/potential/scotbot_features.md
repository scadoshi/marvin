# Scotbot Feature Ideas

Features to build that exercise Rig APIs.

---

## /compact

Erase chat history and replace with a compacted summary of the conversation.

- Different from `/summarize` which only prints a summary
- `/compact` actually modifies history — clears it and inserts a system-level context message
- Useful for long conversations approaching context limits
- Agent summarizes conversation, then history is replaced with that summary as starting context

---

## /export and /import

Save and load conversation history to/from JSON files.

- `/export <filename>` — serialize current `Vec<Message>` to JSON
- `/import <filename>` — load messages from JSON and restore history
- Teaches Rig's message type serialization patterns
- Enables resuming conversations across sessions

---

## Tool calling with commands

Let the agent invoke existing commands as tools.

- Register `/clear`, `/history`, `/summarize`, `/compact` as callable tools
- Agent can decide when to invoke them based on conversation
- Teaches Rig's `Tool` trait, `ToolCall`/`ToolResult` content types
- Gentle introduction to tool use since implementations already exist

---

## Token-aware context management

Automatic conversation management based on token usage.

- Track cumulative tokens (already implemented via `/tokens`)
- Warn when approaching context window limit
- Optionally auto-compact when threshold reached
- Requires knowing model context limits (see `model_metadata.md`)
