pub(crate) mod request;

use std::sync::Arc;

use request::SearchArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::{BASE_URL, TavilyClient};
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

const SEARCH_PATH: &str = "/search";

pub(crate) struct Search {
    client: Arc<TavilyClient>,
}

impl Search {
    pub(crate) fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

// rig's Tool trait declares these methods async.
#[allow(clippy::unused_async_trait_impl)]
impl Tool for Search {
    const NAME: &'static str = "search_web";
    type Args = SearchArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information using Tavily API".to_string(),
            parameters: serde_json::to_value(schema_for!(SearchArgs))
                .expect("schema is serializable"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
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
