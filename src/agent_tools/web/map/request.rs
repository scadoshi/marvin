use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MapArgs {
    #[schemars(description = "The root URL to begin the mapping")]
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Natural language instructions for the crawler (costs 2 credits per 10 pages)"
    )]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Max depth of mapping - how far from base URL crawler can explore. Default: 1"
    )]
    pub max_depth: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Max links to follow per page level. Default: 20")]
    pub max_breadth: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Total links the crawler will process before stopping. Default: 50")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns to select only URLs with specific path patterns")]
    pub select_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns to select specific domains or subdomains")]
    pub select_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns to exclude URLs with specific path patterns")]
    pub exclude_paths: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Regex patterns to exclude specific domains or subdomains")]
    pub exclude_domains: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether to include external domain links in results. Default: true")]
    pub allow_external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Timeout in seconds for the map operation (10-150). Default: 150")]
    pub timeout: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Include credit usage information in response")]
    pub include_usage: Option<bool>,
}
