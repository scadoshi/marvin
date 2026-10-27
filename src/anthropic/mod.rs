//! Lists the models the Anthropic API offers.

use reqwest::Client;
use serde::Deserialize;

/// Body of `GET /v1/models`, keeping only the list.
#[derive(Debug, Deserialize, Clone)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
    //first_id: String,
    //has_more: bool,
    //last_id: String,
}

/// One model from the list.
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct ModelInfo {
    /// ID sent in API requests.
    pub id: String,
    /// Name shown to the user.
    pub display_name: String,
    /// Most output tokens the model can produce, used as the agent's `max_tokens`.
    pub max_tokens: u64,
}

/// Where the Anthropic API lives.
pub(crate) const API_BASE: &str = "https://api.anthropic.com";

/// Lists the models the API key can use.
///
/// # Errors
///
/// Returns an error if the request fails, the API answers with a non-success status, or the body is not a model list.
pub(crate) async fn fetch_models(base_url: &str, api_key: &str) -> anyhow::Result<Vec<ModelInfo>> {
    let response = Client::new()
        .get(format!("{base_url}/v1/models"))
        .header("anthropic-version", "2023-06-01")
        .header("X-Api-Key", api_key)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await?;
        anyhow::bail!("listing models failed with {status}: {body}");
    }
    let body = response.json::<serde_json::Value>().await?;
    let models_response: ModelsResponse = serde_json::from_value(body)?;
    Ok(models_response.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_server;

    #[tokio::test]
    async fn models_are_read_from_the_list() {
        let (base_url, _) = test_server::serve_once(
            200,
            r#"{"data":[{"id":"claude-test","created_at":"2026-01-01T00:00:00Z","display_name":"Claude Test","type":"model","max_tokens":1024}]}"#,
        )
        .await;
        let models = fetch_models(&base_url, "key").await.unwrap();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "claude-test");
    }

    #[tokio::test]
    async fn a_rejected_key_says_so() {
        let (base_url, _) = test_server::serve_once(
            401,
            r#"{"type":"error","error":{"type":"authentication_error","message":"invalid x-api-key"}}"#,
        )
        .await;
        let err = fetch_models(&base_url, "bad").await.unwrap_err();
        let message = format!("{err:#}");
        assert!(message.contains("401"), "{message}");
        assert!(message.contains("invalid x-api-key"), "{message}");
    }
}
