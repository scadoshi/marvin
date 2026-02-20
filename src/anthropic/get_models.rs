use crate::{
    anthropic::{ModelInfo, ModelsResponse},
    chat::config::Config,
};
use reqwest::Client;

pub trait GetAnthropicModels {
    fn get_models(&self) -> impl Future<Output = anyhow::Result<Vec<ModelInfo>>>;
}

impl GetAnthropicModels for Config {
    async fn get_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        let response = Client::new()
            .get("https://api.anthropic.com/v1/models")
            .header("anthropic-version", "2023-06-01")
            .header("X-Api-Key", self.anthropic_api_key())
            .send()
            .await?;
        let body = response.json::<serde_json::Value>().await?;
        let models_response: ModelsResponse = serde_json::from_value(body)?;
        Ok(models_response.data)
    }
}
