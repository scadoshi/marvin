use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlResponse {
    pub base_url: String,
    pub results: Vec<CrawlResult>,
    pub response_time: f64,
    pub usage: Option<Usage>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlResult {
    pub url: String,
    pub raw_content: String,
    pub favicon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub credits: u32,
}
