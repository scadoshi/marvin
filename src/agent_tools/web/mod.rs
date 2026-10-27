//! Web tools backed by the Tavily API.

use std::sync::Arc;

use crawl::Crawl;
use extract::Extract;
use map::Map;
use rig::tool::ToolDyn;
use search::Search;
use tavily::TavilyClient;

pub(crate) mod crawl;
pub(crate) mod extract;
pub(crate) mod map;
pub(crate) mod search;
pub(crate) mod tavily;

/// The four Tavily tools, sharing `client`.
pub(crate) fn tools(client: &Arc<TavilyClient>) -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Search::new(Arc::clone(client))),
        Box::new(Extract::new(Arc::clone(client))),
        Box::new(Crawl::new(Arc::clone(client))),
        Box::new(Map::new(Arc::clone(client))),
    ]
}

#[cfg(test)]
mod tests {
    use rig::tool::Tool;
    use serde_json::json;

    use super::*;
    use crate::test_server;

    const BAD_GATEWAY: &str = "<html>502 Bad Gateway</html>";

    async fn client(status: u16, body: &'static str) -> Arc<TavilyClient> {
        let (base_url, _) = test_server::serve_once(status, body).await;
        Arc::new(TavilyClient::with_base_url("test", base_url))
    }

    #[tokio::test]
    async fn a_successful_call_returns_the_json_body() {
        let search = Search::new(client(200, r#"{"results":[]}"#).await);
        let args = serde_json::from_value(json!({ "query": "rust" })).unwrap();
        assert_eq!(
            Tool::call(&search, args).await.unwrap(),
            json!({ "results": [] })
        );
    }

    #[tokio::test]
    async fn a_failed_call_names_its_tool_and_status_even_when_the_body_is_not_json() {
        let errors = [
            Tool::call(
                &Search::new(client(502, BAD_GATEWAY).await),
                serde_json::from_value(json!({ "query": "rust" })).unwrap(),
            )
            .await
            .map(|_| ()),
            Tool::call(
                &Extract::new(client(502, BAD_GATEWAY).await),
                serde_json::from_value(json!({ "urls": ["https://example.com"] })).unwrap(),
            )
            .await
            .map(|_| ()),
            Tool::call(
                &Crawl::new(client(502, BAD_GATEWAY).await),
                serde_json::from_value(json!({ "url": "https://example.com" })).unwrap(),
            )
            .await
            .map(|_| ()),
            Tool::call(
                &Map::new(client(502, BAD_GATEWAY).await),
                serde_json::from_value(json!({ "url": "https://example.com" })).unwrap(),
            )
            .await
            .map(|_| ()),
        ];
        for (name, result) in ["Search", "Extract", "Crawl", "Map"]
            .into_iter()
            .zip(errors)
        {
            let message = result.unwrap_err().to_string();
            assert!(
                message.contains(&format!("{name} failed with 502")),
                "{message}"
            );
            assert!(message.contains("Bad Gateway"), "{message}");
        }
    }
}
