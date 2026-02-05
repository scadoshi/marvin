use dotenvy::dotenv;
use rig::{
    agent::Agent,
    client::{CompletionClient, ProviderClient},
    providers::anthropic::{
        self,
        completion::{
            CompletionModel, CLAUDE_3_5_HAIKU, CLAUDE_3_5_SONNET, CLAUDE_3_7_SONNET, CLAUDE_4_OPUS,
            CLAUDE_4_SONNET,
        },
    },
};
#[derive(Debug, Clone)]
pub struct Config {
    preamble: String,
    anthropic_api_key: String,
    model_options: Vec<String>,
    model: String,
}
impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let preamble = std::env::var("PREAMBLE").expect("PREAMBLE must exist in env");
        let anthropic_api_key =
            std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must exist in env");
        let model_options = vec![
            CLAUDE_3_5_HAIKU.to_string(),
            CLAUDE_3_5_SONNET.to_string(),
            CLAUDE_3_7_SONNET.to_string(),
            CLAUDE_4_SONNET.to_string(),
            CLAUDE_4_OPUS.to_string(),
        ];
        let model = CLAUDE_3_5_HAIKU.to_string();
        Self {
            preamble,
            anthropic_api_key,
            model_options,
            model,
        }
    }
    pub fn build_agent(&self) -> Agent<CompletionModel> {
        anthropic::Client::from_val(self.anthropic_api_key.clone())
            .agent(self.model.clone())
            .preamble(self.preamble.as_str())
            .build()
    }
    pub fn preamble(&self) -> &str {
        self.preamble.as_str()
    }
    pub fn model(&self) -> &str {
        self.model.as_str()
    }
    pub fn set_model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = model.into();
        self
    }
    pub fn model_options(&self) -> &[String] {
        self.model_options.as_slice()
    }
}
