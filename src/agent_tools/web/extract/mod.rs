pub(crate) mod request;

use std::sync::Arc;

use request::ExtractArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::{BASE_URL, TavilyClient};
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

const EXTRACT_PATH: &str = "/extract";

#[derive(Debug)]
pub(crate) struct Extract {
    client: Arc<TavilyClient>,
}

impl Extract {
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

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
            .into_tool_result()?
            .join(EXTRACT_PATH)
            .into_tool_result()?;
        let json = serde_json::to_value(args).into_tool_result()?;
        let response = self.client.post(url, json).await.into_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.into_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => Err(
                ToolFailure(format!("Extract failed with {status}: {body:?}")).into_tool_error(),
            ),
        }
    }
}
