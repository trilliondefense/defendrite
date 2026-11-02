use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

use serde_json::{json, Value};
use tokio::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

use super::{client, types::McpTool};

type McpResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct McpManager {
    servers: RwLock<HashMap<String, PathBuf>>,

    clients: Mutex<
        HashMap<
            String,
            rmcp::service::RunningService<rmcp::RoleClient, ()>,
        >
    >,
}

static MCP_MANAGER: OnceLock<McpManager> = OnceLock::new();

impl McpManager {
    fn new(servers: HashMap<String, PathBuf>) -> Self {
        Self {
            servers: RwLock::new(servers),
            clients: Mutex::new(HashMap::new()),
        }
    }

    // Get or initialize the application-wide manager.
    pub fn global(app: &AppHandle) -> McpResult<&'static Self> {
        if let Some(manager) = MCP_MANAGER.get() {
            return Ok(manager);
        }

        let mut servers = HashMap::new();

        #[cfg(target_os = "windows")]
        let resource_name = "filesystem.exe";

        #[cfg(not(target_os = "windows"))]
        let resource_name = "filesystem";

        // Production: resolve bundled Tauri resource.
        let bundled = app.path()
            .resolve(resource_name, BaseDirectory::Resource)
            .ok();

        // Development: workspace target/debug.
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("Cannot resolve workspace directory")?
            .to_path_buf();

        let development = workspace
            .join("target")
            .join("debug")
            .join(resource_name);

        // Explicit environment override > bundled > development.
        let executable = if let Some(path) =
            std::env::var_os("FILESYSTEM_MCP_BIN")
        {
            PathBuf::from(path)
        } else if let Some(path) = bundled.filter(|p| p.is_file()) {
            path
        } else {
            development
        };

        if !executable.is_file() {
            return Err(format!(
                "Filesystem MCP executable not found: {}",
                executable.display()
            ).into());
        }

        servers.insert("filesystem".to_string(), executable);

        let manager = Self::new(servers);

        // Another concurrent caller may have initialized it.
        let _ = MCP_MANAGER.set(manager);

        Ok(MCP_MANAGER.get().unwrap())
    }

    // Register another MCP server.
    pub fn register_server(
        &self,
        name: impl Into<String>,
        executable: PathBuf,
    ) -> McpResult<()> {
        self.servers
            .write()
            .map_err(|e| format!("Server registry lock failed: {e}"))?
            .insert(name.into(), executable);

        Ok(())
    }

    // Connect only if this server isn't already connected.
    async fn ensure_connected(&self, server: &str) -> McpResult<()> {
        let mut clients = self.clients.lock().await;

        if clients.contains_key(server) {
            return Ok(());
        }

        let executable = self.servers
            .read()
            .map_err(|e| format!("Server registry lock failed: {e}"))?
            .get(server)
            .cloned()
            .ok_or_else(|| format!("Unknown MCP server: {server}"))?;

        println!("Starting MCP server: {server}");

        let connection = client::connect(&executable).await?;

        clients.insert(server.to_string(), connection);

        println!("MCP server connected: {server}");

        Ok(())
    }

    // Discover tools from every registered server.
    pub async fn discover_tools(&self) -> McpResult<Vec<McpTool>> {
        let server_names: Vec<String> = self.servers
            .read()
            .map_err(|e| format!("Server registry lock failed: {e}"))?
            .keys()
            .cloned()
            .collect();

        let mut all_tools = Vec::new();

        for server in server_names {
            self.ensure_connected(&server).await?;

            let clients = self.clients.lock().await;
            let client = clients.get(&server).unwrap();

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

    // Execute a tool using its registered server connection.
    pub async fn execute_tool(
        &self,
        server: &str,
        name: &str,
        args: Value,
    ) -> McpResult<Value> {
        self.ensure_connected(server).await?;

        let clients = self.clients.lock().await;
        let client = clients.get(server).unwrap();

        let arguments = args
            .as_object()
            .cloned()
            .unwrap_or_default();

        let result = client.call_tool(
            rmcp::model::CallToolRequestParams::new(name.to_string())
                .with_arguments(arguments),
        ).await?;

        Ok(json!({
            "result": format!("{result:#?}")
        }))
    }
}