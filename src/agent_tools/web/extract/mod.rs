pub mod request;
#[allow(dead_code)]
pub mod response;

use crate::agent_tools::{SomeError, ToToolError, ToToolResult};

use super::tavily::{TavilyClient, BASE_URL};
use request::ExtractArgs;
use reqwest::StatusCode;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::schema_for;
use serde_json::Value;
use std::sync::Arc;
use url::Url;

const EXTRACT_PATH: &str = "/extract";

pub struct Extract {
    client: Arc<TavilyClient>,
}

impl Extract {
    pub fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

impl Tool for Extract {
    const NAME: &'static str = "extract_url";
    type Args = ExtractArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Extract clean content from one or more URLs. Returns the full text content, optionally with images.".to_string(),
            parameters: serde_json::to_value(schema_for!(ExtractArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
            .to_tool_result()?
            .join(EXTRACT_PATH)
            .to_tool_result()?;
        let json = serde_json::to_value(args).to_tool_result()?;
        let response = self.client.post(url, json).await.to_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.to_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => {
                Err(SomeError(format!("Extract failed with {}: {:?}", status, body)).to_tool_err())
            }
        }
    }
}
