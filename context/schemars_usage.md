# Schemars for Rig Tool Development

## What is schemars?

`schemars` is a Rust crate that automatically generates JSON Schema from Rust types using derive macros. It's essential for building Rig tools with complex argument structures.

## Why Use Schemars?

### The Problem Without Schemars

When building Rig tools, you must provide a `ToolDefinition` with a JSON Schema describing the tool's parameters. Without schemars, you write this manually:

```rust
async fn definition(&self, _prompt: String) -> ToolDefinition {
    ToolDefinition {
        name: "search_web".to_string(),
        description: "Search the web".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Max results"
                }
            },
            "required": ["query"]
        }),
    }
}
```

**Problems**:
- Duplication: schema must match your `Args` struct manually
- Error-prone: typos, mismatched types, forgotten fields
- Hard to maintain: changes to struct require updating JSON
- No validation: Rust compiler can't check JSON correctness

### The Solution With Schemars

```rust
use schemars::{JsonSchema, schema_for};

#[derive(Deserialize, Serialize, JsonSchema)]
struct WebSearchArgs {
    #[schemars(description = "The search query")]
    query: String,

    #[schemars(description = "Max results")]
    max_results: Option<u32>,
}

async fn definition(&self, _prompt: String) -> ToolDefinition {
    let schema = schema_for!(WebSearchArgs);
    ToolDefinition {
        name: "search_web".to_string(),
        description: "Search the web".to_string(),
        parameters: serde_json::to_value(schema).unwrap(),
    }
}
```

**Benefits**:
- Single source of truth: schema derives from struct
- Type-safe: compiler enforces correctness
- Automatic: handles complex types (enums, Option, Vec, nested structs)
- Maintainable: change struct once, schema updates automatically

## Installation

Add to `Cargo.toml`:
```toml
[dependencies]
schemars = "0.8"
```

## Basic Usage

### Simple Struct

```rust
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
struct AddArgs {
    #[schemars(description = "First number")]
    lhs: i64,

    #[schemars(description = "Second number")]
    rhs: i64,
}
```

Generated schema:
```json
{
  "type": "object",
  "properties": {
    "lhs": {
      "type": "integer",
      "description": "First number"
    },
    "rhs": {
      "type": "integer",
      "description": "Second number"
    }
  },
  "required": ["lhs", "rhs"]
}
```

### Optional Fields

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct SearchArgs {
    query: String,
    max_results: Option<u32>,  // Automatically makes this optional in schema
}
```

Generated schema marks `max_results` as optional, `query` as required.

### Enums

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum SearchDepth {
    Basic,
    Advanced,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct SearchArgs {
    query: String,
    depth: SearchDepth,
}
```

Schema includes enum variants as allowed values.

### Nested Structs

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct DateRange {
    start: String,
    end: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct SearchArgs {
    query: String,
    date_range: Option<DateRange>,
}
```

Schemars automatically generates schema for nested types.

## Advanced Attributes

### Field Descriptions

```rust
#[derive(JsonSchema)]
struct Args {
    #[schemars(description = "User's email address")]
    email: String,
}
```

### Default Values

```rust
#[derive(JsonSchema)]
struct Args {
    #[schemars(default = "default_max")]
    max_results: u32,
}

fn default_max() -> u32 {
    10
}
```

### Field Validation

```rust
#[derive(JsonSchema)]
struct Args {
    #[schemars(range(min = 1, max = 100))]
    page_size: u32,

    #[schemars(regex(pattern = r"^\d{3}-\d{3}-\d{4}$"))]
    phone: String,
}
```

### Skipping Fields

```rust
#[derive(JsonSchema)]
struct Args {
    pub query: String,

    #[schemars(skip)]
    internal_field: String,  // Won't appear in schema
}
```

## Integration with Rig Tools

### Complete Example

```rust
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
struct WebSearchArgs {
    #[schemars(description = "The search query to execute")]
    query: String,

    #[schemars(description = "Maximum number of results to return (1-20)")]
    #[schemars(range(min = 1, max = 20))]
    max_results: Option<u32>,

    #[schemars(description = "Include domains to search within")]
    include_domains: Option<Vec<String>>,
}

pub struct WebSearch;

impl Tool for WebSearch {
    const NAME: &'static str = "search_web";
    type Args = WebSearchArgs;
    type Output = String;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schema_for!(WebSearchArgs);
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information".to_string(),
            parameters: serde_json::to_value(schema).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Implementation
        Ok("results".to_string())
    }
}
```

## Pattern for Marvin Tools

### Standard Template

All tools in Marvin should follow this pattern:

```rust
use schemars::{JsonSchema, schema_for};

// 1. Derive JsonSchema on Args struct
#[derive(Deserialize, Serialize, JsonSchema)]
struct ToolArgs {
    #[schemars(description = "Field description here")]
    field_name: FieldType,
}

// 2. Implement Tool
pub struct ToolName;

impl Tool for ToolName {
    const NAME: &'static str = "tool_name";
    type Args = ToolArgs;
    type Output = OutputType;
    type Error = ToolError;

    // 3. Use schema_for! in definition
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schema_for!(ToolArgs);
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Clear description of what this tool does".to_string(),
            parameters: serde_json::to_value(schema).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Implementation
    }
}
```

## Common Patterns

### Search/Query Tools

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct QueryArgs {
    #[schemars(description = "Search query or question")]
    query: String,

    #[schemars(description = "Number of results")]
    #[schemars(range(min = 1, max = 50))]
    limit: Option<u32>,
}
```

### URL-Based Tools

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct UrlArgs {
    #[schemars(description = "URL to process")]
    #[schemars(regex(pattern = r"^https?://"))]
    url: String,
}
```

### Multi-URL Tools

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
struct ExtractArgs {
    #[schemars(description = "List of URLs to extract content from")]
    urls: Vec<String>,

    #[schemars(description = "Include images in extraction")]
    include_images: Option<bool>,
}
```

### Enum Options

```rust
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    Json,
    Markdown,
    Text,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct ResearchArgs {
    query: String,
    format: OutputFormat,
}
```

## Debugging Tips

### View Generated Schema

```rust
let schema = schema_for!(YourArgs);
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

### Common Issues

1. **Missing descriptions**: Always add `#[schemars(description = "...")]`
2. **Wrong types**: Ensure Rust type matches intended JSON type
3. **Required vs optional**: Use `Option<T>` for optional fields
4. **Enum serialization**: Add `#[serde(rename_all = "...")]` for consistency

## Resources

- **Crate docs**: https://docs.rs/schemars/
- **JSON Schema spec**: https://json-schema.org/
- **Rig Tool trait**: https://docs.rs/rig-core/latest/rig/tool/trait.Tool.html

## Why This Matters for Marvin

As Marvin grows more sophisticated tools (web search, content extraction, research), argument structures become complex:

- Multiple optional parameters
- Nested configuration objects
- Enums for choices (search depth, output format)
- Array inputs (multiple URLs)

Schemars keeps this manageable by:
1. Eliminating manual JSON schema writing
2. Ensuring schema matches Rust types
3. Making tools easier to maintain and extend
4. Providing clear documentation via field descriptions

**Bottom line**: Use schemars for all Rig tools in Marvin. It's the standard approach in the ecosystem and prevents entire classes of bugs.
