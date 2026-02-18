use dotenvy::dotenv;
#[derive(Debug, Clone)]
pub struct Config {
    anthropic_api_key: String,
}
impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")?;
        Ok(Self { anthropic_api_key })
    }
    pub fn anthropic_api_key(&self) -> &str {
        &self.anthropic_api_key
    }
}
