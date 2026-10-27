//! The HTTP client the web tools share to call Tavily.

use std::fmt;

use reqwest::{Client, StatusCode};
use rig::tool::ToolError;
use serde::Serialize;
use serde_json::Value;
use url::Url;

use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};
use crate::chat::config::Config;

/// An HTTP client that sends the Tavily API key with each request.
pub(crate) struct TavilyClient {
    client: Client,
    api_key: String,
    base_url: String,
}

/// Base URL of the Tavily API.
const BASE_URL: &str = "https://api.tavily.com";

/// Prints no key.
impl fmt::Debug for TavilyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TavilyClient").finish_non_exhaustive()
    }
}

impl TavilyClient {
    /// A client that authenticates with `api_key`.
    pub(crate) fn new(api_key: impl Into<String>) -> Self {
        Self::with_base_url(api_key, BASE_URL)
    }
    /// A client that authenticates with `api_key` and sends its requests to `base_url`.
    pub(crate) fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }
    /// Posts `args` as JSON to `path` with the API key as a bearer token and returns the JSON reply. `name` labels a failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent, Tavily answers with a status other than 200 OK (the error carries the status and body), or the reply is not JSON.
    pub(crate) async fn request(
        &self,
        path: &str,
        args: impl Serialize,
        name: &str,
    ) -> Result<Value, ToolError> {
        let url = Url::parse(&self.base_url)
            .into_tool_result()?
            .join(path)
            .into_tool_result()?;
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&args)
            .send()
            .await
            .into_tool_result()?;
        let status = response.status();
        if status != StatusCode::OK {
            let body = response.text().await.into_tool_result()?;
            return Err(
                ToolFailure(format!("{name} failed with {status}: {body}")).into_tool_error()
            );
        }
        response.json::<Value>().await.into_tool_result()
    }
}

impl From<&Config> for TavilyClient {
    fn from(value: &Config) -> Self {
        TavilyClient::new(value.tavily_api_key())
    }
}
