use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub answer: Option<String>,
    pub images: Vec<String>,
    pub results: Vec<SearchResult>,
    pub response_time: String,
    pub auto_parameters: Option<AutoParameters>,
    pub usage: Option<Usage>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f64,
    pub raw_content: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoParameters {
    pub topic: String,
    pub search_depth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub credits: u32,
}
