pub mod request;
#[allow(dead_code)]
pub mod response;

use std::sync::Arc;

use request::CrawlArgs;
use reqwest::StatusCode;
use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::schema_for;
use serde_json::Value;
use url::Url;

use super::tavily::{BASE_URL, TavilyClient};
use crate::agent_tools::{ToolErrorExt, ToolFailure, ToolResultExt};

const CRAWL_PATH: &str = "/crawl";

pub struct Crawl {
    client: Arc<TavilyClient>,
}

impl Crawl {
    pub fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

// rig's Tool trait declares these methods async.
#[allow(clippy::unused_async_trait_impl)]
impl Tool for Crawl {
    const NAME: &'static str = "crawl_website";
    type Args = CrawlArgs;
    type Output = Value;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Crawl a website starting from a root URL, exploring linked pages and extracting content. Use for indexing documentation sites or knowledge bases.".to_string(),
            parameters: serde_json::to_value(schema_for!(CrawlArgs)).expect("schema is serializable"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let url = Url::parse(BASE_URL)
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
