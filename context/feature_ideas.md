# Feature Ideas

## High Impact

### RAG with Local Files
Rig has embedding support. Index markdown/code files, query against them.
- "What does my auth module do?" → searches codebase
- Learn: Rig's embedding API, vector similarity, chunking strategies

### Persistent Memory
Store facts across sessions.
- "Remember that I prefer async/await over callbacks" → saves to JSON/SQLite
- Loads on startup, agent has context from previous sessions
- Learn: State persistence patterns, context injection

### Code Execution Tool
Let the AI write and run code snippets.
- Sandbox with temp directory
- Useful for "calculate this" or "test this regex"
- Learn: Process spawning, sandboxing, capturing stdout/stderr

### File Tools
`read_file`, `write_file`, `list_dir` tools.
- Agent can actually edit code, not just talk about it
- Learn: File system operations, path handling, permissions

## Ambitious

### Multi-Agent Handoff
Multiple agents with different specialties.
- "Researcher" agent crawls and gathers info
- "Coder" agent writes code
- "Reviewer" agent checks work
- Orchestrate handoffs between them
- Learn: Agent composition, workflow patterns

### MCP Server
Make marvin an MCP (Model Context Protocol) server.
- Other tools (Claude Desktop, etc.) can use your Tavily tools
- Learn: MCP spec, server implementation, tool exposure

## Quick Wins

### `/cost` Command
Calculate spend based on token usage + model pricing.
- Track cumulative cost per session
- Learn: Pricing APIs, formatted output

### Clipboard Tool
Read/write system clipboard.
- Agent can grab what you copied, paste results back
- Learn: Platform-specific APIs (pbcopy/xclip)

### `/export` Command
Export conversation to markdown file.
- Nice formatted output with timestamps
- Learn: Markdown generation, file writing

### Image Description Tool
Send images to Claude's vision API.
- "What's in this screenshot?" → describes it
- Learn: Multimodal APIs, base64 encoding
