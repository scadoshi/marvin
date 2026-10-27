//! The HTTP client the web tools share to call Tavily.

use std::fmt;

use reqwest::{Client, Response};
use serde_json::Value;
use url::Url;

use crate::chat::config::Config;

/// An HTTP client that sends the Tavily API key with each request.
pub(crate) struct TavilyClient {
    client: Client,
    api_key: String,
}

/// Base URL of the Tavily API.
pub(super) const BASE_URL: &str = "https://api.tavily.com";

/// Prints no key.
impl fmt::Debug for TavilyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TavilyClient").finish_non_exhaustive()
    }
}

impl TavilyClient {
    /// A client that authenticates with `api_key`.
    pub(crate) fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }
    /// Posts `json` to `url` with the API key as a bearer token.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent or no response arrives. A non-success status is not an error here.
    pub(crate) async fn post(&self, url: Url, json: Value) -> Result<Response, reqwest::Error> {
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
