use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub results: Vec<ExtractResult>,
    pub failed_results: Vec<FailedResult>,
    pub response_time: f64,
    pub usage: Option<Usage>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractResult {
    pub url: String,
    pub raw_content: String,
    pub images: Option<Vec<String>>,
    pub favicon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedResult {
    pub url: String,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub credits: u32,
}
