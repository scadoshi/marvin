pub(crate) mod request;

use std::sync::Arc;

use request::MapArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::{BASE_URL, TavilyClient};
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

const MAP_PATH: &str = "/map";

pub(crate) struct Map {
    client: Arc<TavilyClient>,
}

impl Map {
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

// rig's Tool trait declares these methods async.
#[allow(clippy::unused_async_trait_impl)]
impl Tool for Map {
    const NAME: &'static str = "map_website";
    type Args = MapArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Generate a site map by discovering all URLs on a website. Returns a list of discovered URLs without extracting content. Use to understand site structure before crawling.".to_string(),
            parameters: serde_json::to_value(schema_for!(MapArgs)).expect("schema is serializable"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
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
