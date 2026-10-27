//! The `search_web` tool, which calls Tavily's `/search` endpoint.

pub(crate) mod request;

use std::sync::Arc;

use request::SearchArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::TavilyClient;
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

/// Tavily endpoint for the `search_web` tool.
const SEARCH_PATH: &str = "/search";

/// Searches the web through Tavily.
#[derive(Debug)]
pub(crate) struct Search {
    client: Arc<TavilyClient>,
}

impl Search {
    /// A tool that sends its requests through `client`.
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Search {
    const NAME: &'static str = "search_web";
    type Args = SearchArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information using Tavily API".to_string(),
            parameters: schema_for!(SearchArgs).into(),
        }
    }

    /// Sends the arguments to Tavily's `/search` endpoint and returns the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, the response body is not JSON, or Tavily answers with a status other than 200 OK.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(self.client.base_url())
            .into_tool_result()?
            .join(SEARCH_PATH)
            .into_tool_result()?;
        let json = serde_json::to_value(args).into_tool_result()?;
        let response = self.client.post(url, json).await.into_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.into_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => {
                Err(ToolFailure(format!("Search failed with {status}: {body:?}")).into_tool_error())
            }
        }
    }
}
