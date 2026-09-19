pub mod request;
#[allow(dead_code)]
pub mod response;

use crate::agent_tools::{SomeError, ToToolError, ToToolResult};

use super::tavily::{BASE_URL, TavilyClient};
use request::CrawlArgs;
use reqwest::StatusCode;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::schema_for;
use serde_json::Value;
use std::sync::Arc;
use url::Url;

const CRAWL_PATH: &str = "/crawl";

pub struct Crawl {
    client: Arc<TavilyClient>,
}

impl Crawl {
    pub fn new(client: Arc<TavilyClient>) -> Self {
        Self { client }
    }
}

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
            .to_tool_result()?
            .join(CRAWL_PATH)
            .to_tool_result()?;
        let json = serde_json::to_value(args).to_tool_result()?;
        let response = self.client.post(url, json).await.to_tool_result()?;
        let status = response.status();
        let body = response.json::<Value>().await.to_tool_result()?;
        match status {
            StatusCode::OK => Ok(body),
            status => Err(SomeError(format!("Crawl failed with {status}: {body:?}")).to_tool_err()),
        }
    }
}
