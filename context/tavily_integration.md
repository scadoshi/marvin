# Tavily Integration Guide

## Overview

Tavily is an AI-optimized search API designed specifically for AI agents and LLM applications. It provides real-time web search, content extraction, website crawling, and comprehensive research capabilities.

## What Tavily Offers

### Core APIs

1. **Tavily Search** - Execute web searches optimized for AI consumption
2. **Tavily Extract** - Extract clean content from one or more URLs
3. **Tavily Crawl** - Graph-based website traversal for comprehensive content discovery
4. **Tavily Map** - Generate complete site maps by intelligently traversing websites
5. **Tavily Research** - Automated comprehensive research with multi-search synthesis

### Key Features

- **AI-optimized results** - Search results formatted for LLM processing
- **Real-time data** - Access to current web information
- **Content extraction** - Clean, structured content from web pages
- **Parallel crawling** - Explore hundreds of paths simultaneously
- **Research synthesis** - Multi-source analysis with detailed reports

## API Credentials & Pricing

### Getting Started
1. Sign up at https://app.tavily.com
2. Get your API key from the dashboard
3. Store in `.env`: `TAVILY_API_KEY=tvly-...`

### Rate Limits
- **Development**: Rate limits apply (check docs for current limits)
- **Production**: Higher rate limits based on plan
- See: https://docs.tavily.com/documentation/rate-limits.md

### Pricing
- **Free tier** available for development
- **Pay-as-you-go** credit system
- Credits consumed per API call (varies by endpoint)
- See: https://docs.tavily.com/documentation/api-credits.md

## Integration Options

### Direct API Integration
- REST API endpoints
- JSON request/response
- Requires manual HTTP client setup

### SDK Options
- **Python SDK** - `pip install tavily-python`
- **JavaScript/TypeScript SDK** - `npm install tavily`

### Rust Integration (for Marvin)
Since there's no official Rust SDK (as of 2026-02), use:
- `reqwest` for HTTP calls
- `serde`/`serde_json` for JSON ser/de
- Build wrapper types around Tavily API endpoints

## API Endpoints

### 1. Tavily Search
**Purpose**: Execute AI-optimized web searches

**Endpoint**: `POST https://api.tavily.com/search`

**Key Parameters**:
- `query` (required) - Search query string
- `search_depth` - "basic" or "advanced"
- `max_results` - Number of results to return (default: 5)
- `include_answer` - Generate direct answer from results
- `include_domains` - Filter to specific domains
- `exclude_domains` - Exclude specific domains

**Response**: Array of results with:
- `title` - Page title
- `url` - Page URL
- `content` - Extracted relevant content
- `score` - Relevance score
- `raw_content` - Full page text (optional)

**Best Practices**:
- Use clear, specific queries
- Set appropriate `search_depth` (basic for speed, advanced for thoroughness)
- Leverage domain filtering for focused results
- See: https://docs.tavily.com/documentation/best-practices/best-practices-search.md

### 2. Tavily Extract
**Purpose**: Extract clean content from specific URLs

**Endpoint**: `POST https://api.tavily.com/extract`

**Key Parameters**:
- `urls` (required) - Array of URLs to extract from
- `include_images` - Include image URLs in response
- `include_raw_content` - Include full HTML

**Response**: Array of extracted content per URL

**Use Cases**:
- Extract article content for summarization
- Get structured data from known URLs
- Content preprocessing for RAG systems

**Best Practices**:
- Batch multiple URLs in single request
- Choose appropriate content format
- Handle extraction failures gracefully
- See: https://docs.tavily.com/documentation/best-practices/best-practices-extract.md

### 3. Tavily Crawl
**Purpose**: Graph-based website exploration and content extraction

**Endpoint**: `POST https://api.tavily.com/crawl`

**Key Parameters**:
- `url` (required) - Starting URL
- `max_depth` - How deep to crawl (default: 3)
- `max_pages` - Maximum pages to crawl
- `include_patterns` - URL patterns to include
- `exclude_patterns` - URL patterns to exclude

**Response**: Comprehensive crawl results with discovered pages and content

**Use Cases**:
- Build knowledge base from documentation sites
- Comprehensive content extraction
- Website content indexing for RAG

**Best Practices**:
- Focus crawls with include/exclude patterns
- Set reasonable depth and page limits
- Use for structured content (docs, blogs)
- See: https://docs.tavily.com/documentation/best-practices/best-practices-crawl.md

### 4. Tavily Research
**Purpose**: Automated comprehensive research with synthesis

**Endpoint**:
- Create: `POST https://api.tavily.com/research`
- Status: `GET https://api.tavily.com/research/{request_id}`

**Key Parameters**:
- `query` (required) - Research topic/question
- `model` - LLM model for synthesis (e.g., "gpt-4")
- `max_results` - Sources to analyze
- `output_format` - "markdown", "json", etc.

**Response**: Detailed research report with:
- Synthesized findings
- Source citations
- Comprehensive analysis

**Use Cases**:
- Market research
- Competitive analysis
- Topic deep-dives
- Background research for decisions

**Best Practices**:
- Write clear, focused research questions
- Choose appropriate model for depth needed
- Consider output format for consumption
- See: https://docs.tavily.com/documentation/best-practices/best-practices-research.md

## Tool Implementation Patterns for Marvin

### Tool 1: WebSearch
**Purpose**: Let agent search web for current information

**Rig Tool Implementation (with schemars)**:
```rust
use schemars::JsonSchema;
use schemars::schema_for;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct WebSearchArgs {
    #[schemars(description = "The search query")]
    pub query: String,

    #[schemars(description = "Maximum number of results (default: 5)")]
    pub max_results: Option<u32>,
}

pub struct WebSearch;

impl Tool for WebSearch {
    const NAME: &'static str = "search_web";
    type Args = WebSearchArgs;
    type Output = String; // JSON array of results
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schema_for!(WebSearchArgs);
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information on a topic".to_string(),
            parameters: serde_json::to_value(schema).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Call Tavily Search API
        // Return formatted results as JSON string
    }
}
```

**Benefits of schemars approach**:
- Type-safe: schema derived from actual Rust type
- No manual JSON schema maintenance
- Descriptions live next to fields
- Automatically handles `Option<T>`, enums, nested structs
```

### Tool 2: ExtractURL
**Purpose**: Extract content from specific URLs

**Implementation**: Similar pattern, takes URL array, returns extracted content

### Tool 3: ResearchTopic
**Purpose**: Comprehensive research on a topic

**Implementation**: Async tool that initiates research, polls for completion, returns report

## Architecture Considerations

### API Client Module
Create `src/api/tavily.rs`:
- `TavilyClient` struct with API key
- Methods for each endpoint (search, extract, crawl, research)
- Error handling and response parsing
- Rate limit handling

### Tool Organization
Create `src/agent_tools/web/`:
- `search.rs` - WebSearch tool
- `extract.rs` - ExtractURL tool
- `research.rs` - ResearchTopic tool
- `mod.rs` - Collection function `web_tools()`

### Configuration
Add to `src/chat/config.rs`:
- Load `TAVILY_API_KEY` from environment
- Validate key presence at startup
- Provide accessor method

### Dependencies
Add to `Cargo.toml`:
```toml
# Already have reqwest, serde, serde_json
schemars = "0.8"  # For automatic JSON Schema generation from Rust structs
# May need:
tokio = { version = "1.49.0", features = ["macros", "rt-multi-thread", "time"] }
# For polling research status
```

**Why schemars?**
- Automatically generates JSON Schema from Rust types using `#[derive(JsonSchema)]`
- Cleaner than manually writing JSON schema in `ToolDefinition`
- Type-safe: schema stays in sync with your Rust types
- Required for complex tool arguments (structs with optional fields, enums, nested types)

## Integration Steps

1. **Add API key to `.env`**
   ```
   TAVILY_API_KEY=tvly-your-key-here
   ```

2. **Update Config to load key**
   - Add `tavily_api_key: String` field
   - Load from `std::env::var("TAVILY_API_KEY")?`

3. **Create Tavily client module**
   - HTTP client wrapper
   - Request/response types
   - Error handling

4. **Implement tools**
   - Start with WebSearch (simplest)
   - Add ExtractURL
   - Add ResearchTopic (most complex, async)

5. **Create web_tools() function**
   - Returns `Vec<Box<dyn ToolDyn>>`
   - Similar to `math_tools()`

6. **Update agent builder**
   - Add `.tools(web_tools())`
   - Test with simple searches

## Testing Strategy

### Manual Testing
1. Test search with common queries
2. Verify extracted content quality
3. Check research report formatting

### Error Cases
- Invalid API key
- Rate limit exceeded
- Network failures
- Malformed URLs (for extract)

### Integration Testing
- Agent successfully calls tools
- Results are properly formatted
- Multi-turn conversations with web context

## Resources

- **API Documentation**: https://docs.tavily.com/documentation/api-reference/introduction.md
- **Playground**: https://app.tavily.com/playground
- **Best Practices**: https://docs.tavily.com/documentation/best-practices/
- **Use Cases**: https://docs.tavily.com/examples/use-cases/
- **Community**: https://discord.gg/TPu2gkaWp2

## Security Considerations

### API Key Management
- Never commit `.env` to git (already in `.gitignore`)
- Rotate keys if leaked
- See: https://docs.tavily.com/documentation/best-practices/api-key-management.md

### Rate Limiting
- Implement backoff for rate limit errors
- Consider caching frequent queries
- Monitor credit usage

### Content Safety
- Web content may be untrusted
- Sanitize/validate extracted content
- Be cautious with user-provided URLs

## Next Steps

1. Add `TAVILY_API_KEY` to Config
2. Implement `WebSearch` tool
3. Test with agent in conversation
4. Add `ExtractURL` for specific content
5. Explore `ResearchTopic` for deep dives
6. Consider crawl/map for documentation indexing

## Learning Opportunities

- **HTTP client patterns** - reqwest advanced usage
- **Async polling** - for research status checks
- **Error handling** - API failures, network issues
- **JSON processing** - complex response parsing
- **Tool composition** - agent chains multiple web tools
- **Caching strategies** - optimize repeated queries
