use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Depth of content extraction.")]
pub enum ExtractDepth {
    #[default]
    Basic,
    Advanced,
}

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Format of the extracted content.")]
pub enum CrawlFormat {
    #[default]
    Markdown,
    Text,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CrawlArgs {
    #[schemars(description = "The root URL to begin the crawl")]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Natural language directives for crawler behavior (costs 2 credits per 10 pages)"
    )]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Max relevant chunks per source (1-5). Default: 3")]
    pub chunks_per_source: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "How far from the base URL the crawler can explore (1-5). Default: 1")]
    pub max_depth: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Links followed per page level (1-500). Default: 20")]
    pub max_breadth: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Total links the crawler will process. Default: 50")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns for targeting specific URL paths")]
    pub select_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns for targeting specific domains")]
    pub select_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns for excluding URL paths")]
    pub exclude_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns for excluding domains")]
    pub exclude_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether to include external domain links. Default: true")]
    pub allow_external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include images in results")]
    pub include_images: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Extraction method depth")]
    pub extract_depth: Option<ExtractDepth>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Content format")]
    pub format: Option<CrawlFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include favicon URL for each result")]
    pub include_favicon: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Operation timeout in seconds (10-150). Default: 150")]
    pub timeout: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include credit usage information in response")]
    pub include_usage: Option<bool>,
}
