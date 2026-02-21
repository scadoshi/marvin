use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(
    description = "Search category. 'news' retrieves real-time updates about politics, sports, and current events. 'general' is for broader searches across a wide range of sources."
)]
pub enum Topic {
    #[default]
    General,
    News,
}

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(
    description = "Depth of search. 'advanced' (2 credits) returns more relevant snippets. 'basic', 'fast', 'ultrafast' (1 credit) trade relevance for speed."
)]
pub enum SearchDepth {
    Advanced,
    #[default]
    Basic,
    Fast,
    Ultrafast,
}

#[derive(Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(description = "Filter results by publish date.")]
pub enum TimeRange {
    #[default]
    None,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SearchArgs {
    #[schemars(description = "The search query to execute")]
    pub query: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Auto-configure search parameters based on query intent (costs 2 credits)"
    )]
    pub auto_parameters: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Search category")]
    pub topic: Option<Topic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Depth of search controlling latency vs relevance tradeoff")]
    pub search_depth: Option<SearchDepth>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Max relevant snippets per source (1-3). Only for 'advanced' depth")]
    pub chunks_per_source: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Maximum search results to return (0-20). Default: 5")]
    pub max_results: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Filter results by publish date")]
    pub time_range: Option<TimeRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Return results published after this date (YYYY-MM-DD format)")]
    pub start_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Return results published before this date (YYYY-MM-DD format)")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include an LLM-generated answer summarizing the results")]
    pub include_answer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include parsed HTML content from each result")]
    pub include_raw_content: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Perform image search and include image results")]
    pub include_images: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Add descriptive text for images. Requires include_images to be true"
    )]
    pub include_image_descriptions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include favicon URL for each result")]
    pub include_favicon: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "List of domains to restrict search to (max 300)")]
    pub include_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "List of domains to exclude from search (max 150)")]
    pub exclude_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Boost results from a specific country (ISO country code)")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include credit usage information in response")]
    pub include_usage: Option<bool>,
}
