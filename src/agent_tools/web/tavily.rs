use crate::chat::config::Config;
use reqwest::{Client, Response};
use serde_json::Value;
use url::Url;

pub struct TavilyClient {
    client: Client,
    api_key: String,
}

pub(super) const BASE_URL: &str = "https://api.tavily.com";

impl TavilyClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }
    pub async fn post(&self, url: Url, json: Value) -> Result<Response, reqwest::Error> {
        self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&json)
            .send()
            .await
    }
}

impl From<&Config> for TavilyClient {
    fn from(value: &Config) -> Self {
        TavilyClient::new(value.tavily_api_key())
    }
}
