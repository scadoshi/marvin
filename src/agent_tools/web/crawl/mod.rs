//! The `crawl_website` tool, which calls Tavily's `/crawl` endpoint.

pub(crate) mod request;

use std::sync::Arc;

use request::CrawlArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::TavilyClient;
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

/// Tavily endpoint for the `crawl_website` tool.
const CRAWL_PATH: &str = "/crawl";

/// Crawls a site from a root URL through Tavily and returns the content of the pages it reaches.
#[derive(Debug)]
pub(crate) struct Crawl {
    client: Arc<TavilyClient>,
}

impl Crawl {
    /// A tool that sends its requests through `client`.
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Crawl {
    const NAME: &'static str = "crawl_website";
    type Args = CrawlArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Crawl a website starting from a root URL, exploring linked pages and extracting content. Use for indexing documentation sites or knowledge bases.".to_string(),
            parameters: schema_for!(CrawlArgs).into(),
        }
    }

    /// Sends the arguments to Tavily's `/crawl` endpoint and returns the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, the response body is not JSON, or Tavily answers with a status other than 200 OK.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(self.client.base_url())
            .into_tool_result()?
            .join(CRAWL_PATH)
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
