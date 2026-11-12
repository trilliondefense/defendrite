use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};
use std::future::Future;
use std::pin::Pin;
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::PathBuf,
    sync::{OnceLock, RwLock},
};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use super::{
    client,
    config::{McpConfig, McpServerConfig},
    types::McpTool,
};

type McpResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

type McpClient = rmcp::service::RunningService<rmcp::RoleClient, ()>;

pub struct McpManager {
    config: RwLock<McpConfig>,
    config_path: PathBuf,
    clients: Mutex<HashMap<String, McpClient>>,
    app: AppHandle,
}

static MCP_MANAGER: OnceLock<McpManager> = OnceLock::new();

impl McpManager {
    pub fn new(app: AppHandle, config: McpConfig, config_path: PathBuf) -> Self {
        Self {
            config: RwLock::new(config),
            config_path,
            clients: Mutex::new(HashMap::new()),
            app,
        }
    }

    /// Get or initialize the application-wide MCP manager.
    ///
    /// Loads all MCP server configurations from mcp.json.
    pub fn global(app: &AppHandle) -> McpResult<&'static Self> {
        if let Some(manager) = MCP_MANAGER.get() {
            return Ok(manager);
        }

        // Resolve application configuration directory.
        let config_dir = app.path().app_config_dir()?;

        // Ensure the directory exists.
        fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("mcp.json");

        // Load existing configuration or create an empty one.
        let config = if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;

            serde_json::from_str::<McpConfig>(&contents).map_err(|e| {
                format!(
                    "Failed to parse MCP configuration {}: {}",
                    config_path.display(),
                    e
                )
            })?
        } else {
            let config = McpConfig {
                servers: Vec::new(),
            };

            let contents = serde_json::to_string_pretty(&config)?;

            fs::write(&config_path, contents)?;

            config
        };

        let manager = Self::new(app.clone(), config, config_path);

        // Initialize only once.
        let _ = MCP_MANAGER.set(manager);

        MCP_MANAGER
            .get()
            .ok_or_else(|| "Failed to initialize MCP manager".into())
    }

    /// Save the current configuration to mcp.json.
    fn save_config(&self, config: &McpConfig) -> McpResult<()> {
        let contents = serde_json::to_string_pretty(config)?;

        fs::write(&self.config_path, contents)?;

        Ok(())
    }

    /// Register or update an MCP server configuration.
    ///
    /// Changes are persisted to mcp.json.
    pub fn register_server(&self, server: McpServerConfig) -> McpResult<()> {
        let mut config = self
            .config
            .write()
            .map_err(|e| format!("Config lock failed: {e}"))?;

        let mut updated_config = config.clone();

        if let Some(existing) = updated_config
            .servers
            .iter_mut()
            .find(|item| item.name == server.name)
        {
            *existing = server;
        } else {
            updated_config.servers.push(server);
        }

        // Persist before updating in-memory configuration.
        self.save_config(&updated_config)?;

        *config = updated_config;

        Ok(())
    }

    /// Remove an MCP server configuration.
    pub async fn remove_server(&self, name: &str) -> McpResult<()> {
        // Disconnect the server if currently connected.
        {
            let mut clients = self.clients.lock().await;
            clients.remove(name);
        }

        let mut config = self
            .config
            .write()
            .map_err(|e| format!("Config lock failed: {e}"))?;

        let mut updated_config = config.clone();

        updated_config.servers.retain(|server| server.name != name);

        self.save_config(&updated_config)?;

        *config = updated_config;

        Ok(())
    }

    /// Get or initialize a persistent MCP connection.
    async fn ensure_connected(&self, name: &str) -> McpResult<()> {
        let mut clients = self.clients.lock().await;

        if clients.contains_key(name) {
            return Ok(());
        }

        let server = {
            let config = self
                .config
                .read()
                .map_err(|e| format!("Config lock failed: {e}"))?;

            config
                .servers
                .iter()
                .find(|server| server.name == name && server.enabled)
                .cloned()
                .ok_or_else(|| format!("MCP server not found or disabled: {name}"))?
        };

        // Transport selection happens inside client::connect().
        // It must support both stdio and streamable_http.
        let connection = client::connect(&self.app, &server).await?;

        clients.insert(name.to_string(), connection);

        Ok(())
    }

    /// Discover tools from every enabled MCP server.
    pub async fn discover_tools(&self) -> McpResult<Vec<McpTool>> {
        let server_names: Vec<String> = {
            let config = self
                .config
                .read()
                .map_err(|e| format!("Config lock failed: {e}"))?;

            config
                .servers
                .iter()
                .filter(|server| server.enabled)
                .map(|server| server.name.clone())
                .collect()
        };

        let mut all_tools = Vec::new();

        for server in server_names {
            self.ensure_connected(&server).await?;

            let clients = self.clients.lock().await;

            let client = clients.get(&server).ok_or("MCP connection missing")?;

            let result = client.list_all_tools().await?;

            for tool in result {
                all_tools.push(McpTool {
                    server: server.clone(),
                    name: tool.name.to_string(),
                    description: tool.description.as_deref().unwrap_or("").to_string(),
                    input_schema: serde_json::to_value(&tool.input_schema)?,
                });
            }
        }

        Ok(all_tools)
    }

    /// Execute a tool using its persistent server connection.
    pub async fn execute_tool(&self, server: &str, name: &str, args: Value) -> McpResult<Value> {
        let hydrated_args = hydrate_local_files(args).await?;

        let arguments = hydrated_args
            .as_object()
            .cloned()
            .ok_or("Tool arguments must be a JSON object")?;

        self.ensure_connected(server).await?;

        let clients = self.clients.lock().await;

        let client = clients.get(server).ok_or("MCP connection missing")?;

        let result = client
            .call_tool(
                rmcp::model::CallToolRequestParams::new(name.to_string()).with_arguments(arguments),
            )
            .await?;

        Ok(json!({
            "result": format!("{result:#?}")
        }))
    }

    /// Get a copy of the current MCP configuration.
    pub fn get_config(&self) -> McpResult<McpConfig> {
        let config = self
            .config
            .read()
            .map_err(|e| format!("Config lock failed: {e}"))?;

        Ok(config.clone())
    }
}

fn hydrate_local_files(val: Value) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>> {
    Box::pin(async move {
        match val {
            Value::String(s) if s.starts_with("local-file://") => {
                let path = s.trim_start_matches("local-file://");

                // OPTIONAL: Add path validation here (e.g., check allowed_paths)

                let bytes = tokio::fs::read(path)
                    .await
                    .map_err(|e| format!("Failed to read local file '{path}': {e}"))?;

                let encoded = STANDARD.encode(&bytes);
                Ok(Value::String(encoded))
            }
            Value::Array(arr) => {
                let mut out = Vec::with_capacity(arr.len());
                for item in arr {
                    out.push(hydrate_local_files(item).await?);
                }
                Ok(Value::Array(out))
            }
            Value::Object(map) => {
                let mut out = serde_json::Map::with_capacity(map.len());
                for (k, v) in map {
                    out.insert(k, hydrate_local_files(v).await?);
                }
                Ok(Value::Object(out))
            }
            other => Ok(other),
        }
    })
}
