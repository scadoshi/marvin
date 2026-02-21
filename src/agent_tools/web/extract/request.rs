use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Depth of extraction. 'advanced' provides more thorough extraction.")]
pub enum ExtractDepth {
    #[default]
    Basic,
    Advanced,
}

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Format of the extracted content.")]
pub enum ExtractFormat {
    #[default]
    Markdown,
    Text,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ExtractArgs {
    #[schemars(description = "List of URLs to extract content from")]
    pub urls: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Optional query to rerank extracted content chunks by relevance")]
    pub query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Number of content chunks per source (max 500 chars each). Default: 3")]
    pub chunks_per_source: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Depth of extraction process")]
    pub extract_depth: Option<ExtractDepth>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include list of images extracted from the URLs")]
    pub include_images: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include favicon URL for each result")]
    pub include_favicon: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Format of extracted content")]
    pub format: Option<ExtractFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Maximum seconds to wait for extraction (1.0-60.0)")]
    pub timeout: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include credit usage information in response")]
    pub include_usage: Option<bool>,
}
