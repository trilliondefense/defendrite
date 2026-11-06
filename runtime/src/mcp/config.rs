use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub servers: Vec<McpServerConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub enabled: bool,
    /// stdio or streamable_http
    pub transport: String,
    /// Used by stdio
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Used by remote HTTP transports
    #[serde(default)]
    pub url: Option<String>,
}

impl McpConfig {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let contents = fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;

        Ok(config)
    }
}
