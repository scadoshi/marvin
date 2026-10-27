use std::fmt;

use dotenvy::dotenv;

pub(crate) struct Config {
    anthropic_api_key: String,
    tavily_api_key: String,
}

/// Prints no keys.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config").finish_non_exhaustive()
    }
}

impl Config {
    pub(crate) fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let anthropic_api_key = std::env::var("ANTHROPIC_API_KEY")?;
        let tavily_api_key = std::env::var("TAVILY_API_KEY")?;
        Ok(Self {
            anthropic_api_key,
            tavily_api_key,
        })
    }
    pub(crate) fn anthropic_api_key(&self) -> &str {
        &self.anthropic_api_key
    }
    pub(crate) fn tavily_api_key(&self) -> &str {
        &self.tavily_api_key
    }
}
