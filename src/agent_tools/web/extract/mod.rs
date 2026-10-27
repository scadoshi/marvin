//! The `extract_url` tool, which calls Tavily's `/extract` endpoint.

pub(crate) mod request;

use std::sync::Arc;

use request::ExtractArgs;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;

use super::tavily::TavilyClient;

/// Tavily endpoint for the `extract_url` tool.
const EXTRACT_PATH: &str = "/extract";

/// Fetches the content of one or more URLs through Tavily.
#[derive(Debug)]
pub(crate) struct Extract {
    client: Arc<TavilyClient>,
}

impl Extract {
    /// A tool that sends its requests through `client`.
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Extract {
    const NAME: &'static str = "extract_url";
    type Args = ExtractArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Extract clean content from one or more URLs. Returns the full text content, optionally with images.".to_string(),
            parameters: schema_for!(ExtractArgs).into(),
        }
    }

    /// Sends the arguments to Tavily's `/extract` endpoint and returns the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, the response body is not JSON, or Tavily answers with a status other than 200 OK.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        self.client.request(EXTRACT_PATH, args, "Extract").await
    }
}
