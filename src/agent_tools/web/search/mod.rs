pub mod request;
#[allow(dead_code)]
pub mod response;

use super::tavily::{TavilyClient, BASE_URL};
use crate::agent_tools::{SomeError, ToToolError, ToToolResult};
use request::SearchArgs;
use reqwest::StatusCode;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::schema_for;
use serde_json::Value;
use std::sync::Arc;
use url::Url;

const SEARCH_PATH: &str = "/search";

pub struct Search {
    client: Arc<TavilyClient>,
}

impl Search {
    pub fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

impl Tool for Search {
    const NAME: &'static str = "search_web";
    type Args = SearchArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information using Tavily API".to_string(),
            parameters: serde_json::to_value(schema_for!(SearchArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
            .to_tool_result()?
            .join(SEARCH_PATH)
            .to_tool_result()?;
        let json = serde_json::to_value(args).to_tool_result()?;
        let response = self.client.post(url, json).await.to_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.to_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => {
                Err(SomeError(format!("Search failed with {}: {:?}", status, body)).to_tool_err())
            }
        }
    }
}
