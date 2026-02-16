# Model Metadata API

**Status**: Enhancement idea

## Problem

Rig's `Model` struct supports metadata fields (`context_length`, etc.) but providers don't populate them. There's no way to query model-specific information like pricing, context window limits, or when to consolidate conversations.

## What Rig currently has

```rust
pub struct Model {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub created_at: Option<u64>,
    pub owned_by: Option<String>,
    pub context_length: Option<u32>,  // exists but unpopulated
}
```

## What's missing

### Pricing data
- Input token cost (per 1M tokens)
- Output token cost (per 1M tokens)
- Cache read/write costs where applicable

### Context management
- Input context window size (max input tokens)
- Output limit (max output tokens)
- Recommended consolidation threshold (when to summarize history)

### Model capabilities
- Supports streaming
- Supports tool use
- Supports vision/multimodal
- Supports extended thinking

## Potential API additions

```rust
pub struct ModelMetadata {
    pub input_cost_per_million: Option<f64>,
    pub output_cost_per_million: Option<f64>,
    pub context_window: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub capabilities: ModelCapabilities,
}

pub struct ModelCapabilities {
    pub streaming: bool,
    pub tool_use: bool,
    pub vision: bool,
    pub extended_thinking: bool,
}
```

## Challenges

- Pricing changes frequently and varies by region
- Not all providers expose this via API
- Would require manual maintenance or external data source

## Related

- `dynamic_model_discovery.md` — already reported, covers listing available models
- Anthropic's `/v1/models` API returns basic model info but not pricing
