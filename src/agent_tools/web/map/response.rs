use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapResponse {
    pub base_url: String,
    pub results: Vec<String>,
    pub response_time: f64,
    pub usage: Option<Usage>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub credits: u32,
}
