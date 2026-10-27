//! The `map_website` tool, which calls Tavily's `/map` endpoint.

pub(crate) mod request;

use std::sync::Arc;

use request::MapArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::TavilyClient;
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

/// Tavily endpoint for the `map_website` tool.
const MAP_PATH: &str = "/map";

/// Lists the URLs of a site through Tavily, without their content.
#[derive(Debug)]
pub(crate) struct Map {
    client: Arc<TavilyClient>,
}

impl Map {
    /// A tool that sends its requests through `client`.
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Map {
    const NAME: &'static str = "map_website";
    type Args = MapArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate a site map by discovering all URLs on a website. Returns a list of discovered URLs without extracting content. Use to understand site structure before crawling.".to_string(),
            parameters: schema_for!(MapArgs).into(),
        }
    }

    /// Sends the arguments to Tavily's `/map` endpoint and returns the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, the response body is not JSON, or Tavily answers with a status other than 200 OK.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(self.client.base_url())
            .into_tool_result()?
            .join(MAP_PATH)
            .into_tool_result()?;
        let json = serde_json::to_value(args).into_tool_result()?;
        let response = self.client.post(url, json).await.into_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.into_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => {
                Err(ToolFailure(format!("Crawl failed with {status}: {body:?}")).into_tool_error())
            }
        }
    }
}
