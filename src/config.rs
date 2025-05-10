use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tenants: HashMap<String, TenantConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TenantConfig {
    pub connection_string: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        // For local development, read from a TOML file
        // Later, adapt to read from Vault or Kubernetes secrets
        let config_str = std::fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}