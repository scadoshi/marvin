use std::{env, fmt};

use anyhow::Context;

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
        dotenvy::dotenv().ok();
        Self::from_vars(|name| env::var(name))
    }

    /// Reads the keys through `var`, which looks one up by name.
    fn from_vars(var: impl Fn(&str) -> Result<String, env::VarError>) -> anyhow::Result<Self> {
        let read = |name| var(name).with_context(|| format!("{name} is not set; see .env.example"));
        let anthropic_api_key = read("ANTHROPIC_API_KEY")?;
        let tavily_api_key = read("TAVILY_API_KEY")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_keys_are_read() {
        let config = Config::from_vars(|name| Ok(format!("{name}-value"))).unwrap();
        assert_eq!(config.anthropic_api_key(), "ANTHROPIC_API_KEY-value");
        assert_eq!(config.tavily_api_key(), "TAVILY_API_KEY-value");
    }

    #[test]
    fn a_missing_key_is_named() {
        for missing in ["ANTHROPIC_API_KEY", "TAVILY_API_KEY"] {
            let err = Config::from_vars(|name| {
                if name == missing {
                    Err(env::VarError::NotPresent)
                } else {
                    Ok(String::from("set"))
                }
            })
            .unwrap_err();
            assert!(format!("{err:#}").contains(missing), "{err:#}");
        }
    }
}
