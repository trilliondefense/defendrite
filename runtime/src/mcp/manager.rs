
use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
use tokio::sync::Mutex;

use super::{
    client,
    config::{McpConfig, McpServerConfig},
    types::McpTool,
};

type McpResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

type McpClient =
    rmcp::service::RunningService<rmcp::RoleClient, ()>;

pub struct McpManager {
    config: RwLock<McpConfig>,
    clients: Mutex<HashMap<String, McpClient>>,
}

static MCP_MANAGER: OnceLock<McpManager> = OnceLock::new();

impl McpManager {
    pub fn new(config: McpConfig) -> Self {
        Self {
            config: RwLock::new(config),
            clients: Mutex::new(HashMap::new()),
        }
    }

    // Get or initialize the application-wide manager.
    pub fn global(app: &AppHandle) -> McpResult<&'static Self> {
    if let Some(manager) = MCP_MANAGER.get() {
        return Ok(manager);
    }

    // Determine platform-specific resource path.
    #[cfg(target_os = "windows")]
    let (platform, binary_name) = ("windows", "filesystem.exe");

    #[cfg(target_os = "macos")]
    let (platform, binary_name) = ("macos", "filesystem");

    #[cfg(target_os = "linux")]
    let (platform, binary_name) = ("linux", "filesystem");

    // Production: resolve bundled Tauri resource.
    let resource_name = format!(
        "resources/filesystem/{platform}/{binary_name}"
    );

    let bundled = app
        .path()
        .resolve(&resource_name, BaseDirectory::Resource)
        .ok();

    // Development: workspace target/debug.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("Cannot resolve workspace directory")?
        .to_path_buf();

    let development = workspace
        .join("target")
        .join("debug")
        .join(binary_name);

    // Environment override > bundled > development.
    let executable = if let Some(path) =
        std::env::var_os("FILESYSTEM_MCP_BIN")
    {
        PathBuf::from(path)
    } else if let Some(path) = bundled.filter(|p| p.is_file()) {
        path
    } else {
        development
    };

    // Validate executable.
    if !executable.is_file() {
        return Err(format!(
            "Filesystem MCP executable not found: {}",
            executable.display()
        )
        .into());
    }

    // Initial configuration.
    let config = McpConfig {
        servers: vec![McpServerConfig {
            name: "filesystem".to_string(),
            enabled: true,
            transport: "stdio".to_string(),
            command: executable.to_string_lossy().to_string(),
            args: Vec::new(),
            env: HashMap::new(),
        }],
    };

    let manager = Self::new(config);

    // Initialize only once.
    let _ = MCP_MANAGER.set(manager);

    MCP_MANAGER
        .get()
        .ok_or_else(|| "Failed to initialize MCP manager".into())
}

    // Register or update an MCP server configuration.
    pub fn register_server(
        &self,
        server: McpServerConfig,
    ) -> McpResult<()> {
        let mut config = self.config
            .write()
            .map_err(|e| format!("Config lock failed: {e}"))?;

        if let Some(existing) = config.servers
            .iter_mut()
            .find(|item| item.name == server.name)
        {
            *existing = server;
        } else {
            config.servers.push(server);
        }

        Ok(())
    }

    // Connect only if this server isn't already connected.
    async fn ensure_connected(&self, name: &str) -> McpResult<()> {
        let mut clients = self.clients.lock().await;

        if clients.contains_key(name) {
            return Ok(());
        }

        let server = {
            let config = self.config
                .read()
                .map_err(|e| format!("Config lock failed: {e}"))?;

            config.servers
                .iter()
                .find(|server| server.name == name && server.enabled)
                .cloned()
                .ok_or_else(|| {
                    format!("MCP server not found or disabled: {name}")
                })?
        };

        let connection = client::connect(&server).await?;

        clients.insert(name.to_string(), connection);

        Ok(())
    }

    // Discover tools from every enabled server.
    pub async fn discover_tools(&self) -> McpResult<Vec<McpTool>> {
        let server_names: Vec<String> = {
            let config = self.config
                .read()
                .map_err(|e| format!("Config lock failed: {e}"))?;

            config.servers
                .iter()
                .filter(|server| server.enabled)
                .map(|server| server.name.clone())
                .collect()
        };

        let mut all_tools = Vec::new();

        for server in server_names {
            self.ensure_connected(&server).await?;

            let clients = self.clients.lock().await;

            let client = clients
                .get(&server)
                .ok_or("MCP connection missing")?;

            let result = client.list_all_tools().await?;

            for tool in result {
                all_tools.push(McpTool {
                    server: server.clone(),
                    name: tool.name.to_string(),
                    description: tool.description
                        .as_deref()
                        .unwrap_or("")
                        .to_string(),
                    input_schema: serde_json::to_value(
                        &tool.input_schema
                    )?,
                });
            }
        }

        Ok(all_tools)
    }

    // Execute a tool using its persistent server connection.
    pub async fn execute_tool(
        &self,
        server: &str,
        name: &str,
        args: Value,
    ) -> McpResult<Value> {
        self.ensure_connected(server).await?;

        let clients = self.clients.lock().await;

        let client = clients
            .get(server)
            .ok_or("MCP connection missing")?;

        let arguments = args
            .as_object()
            .cloned()
            .ok_or("Tool arguments must be a JSON object")?;

        let result = client.call_tool(
            rmcp::model::CallToolRequestParams::new(name.to_string())
                .with_arguments(arguments),
        ).await?;

        Ok(json!({
            "result": format!("{result:#?}")
        }))
    }
}
