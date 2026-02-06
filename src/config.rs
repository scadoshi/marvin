use dotenvy::dotenv;
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    providers::anthropic::{self, completion::CompletionModel},
};
#[derive(Debug, Clone)]
pub struct Config {
    preamble: String,
    anthropic_api_key: String,
}
impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let preamble = std::env::var("PREAMBLE").expect("PREAMBLE must exist in env");
        let anthropic_api_key =
            std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must exist in env");
        Self {
            preamble,
            anthropic_api_key,
        }
    }
    pub fn preamble(&self) -> &str {
        &self.preamble
    }
    pub fn anthropic_api_key(&self) -> &str {
        &self.anthropic_api_key
    }
    pub fn build_agent(&self, model: impl Into<String>) -> Agent<CompletionModel> {
        anthropic::Client::from_val(self.anthropic_api_key().into())
            .agent(model.into())
            .preamble(self.preamble())
            .build()
    }
}
