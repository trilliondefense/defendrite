
use std::error::Error;

use rmcp::{
    transport::{
        StreamableHttpClientTransport,
        TokioChildProcess,
    },
    ServiceExt,
};

use tokio::process::Command;

use super::config::McpServerConfig;

type McpResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

use tauri::{AppHandle, Manager};
use std::path::PathBuf;

fn resolve_mcp_command(
    app: &tauri::AppHandle,
    command: &str,
) -> Result<PathBuf, String> {
    if command == "filesystem" {
        let platform = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "linux"
        };

        let binary = if cfg!(target_os = "windows") {
            "filesystem.exe"
        } else {
            "filesystem"
        };

        return app
            .path()
            .resolve(
                format!("resources/filesystem/{platform}/{binary}"),
                tauri::path::BaseDirectory::Resource,
            )
            .map_err(|e| e.to_string());
    }

    Ok(PathBuf::from(command))
}

pub async fn connect(
    app:&AppHandle,
    config: &McpServerConfig,
) -> McpResult<rmcp::service::RunningService<rmcp::RoleClient, ()>> {
    match config.transport.as_str() {
        "stdio" => {
            if config.command.is_empty() {
                return Err("stdio MCP server requires a command".into());
            }

            let resolved_command =
                resolve_mcp_command(app, &config.command)?;

            let mut command = Command::new(resolved_command);

            command
                .args(&config.args)
                .envs(&config.env)
                .kill_on_drop(true);

            let transport = TokioChildProcess::new(command)?;

            let client = ().serve(transport).await?;

            Ok(client)
        }

        "streamable_http" => {
            let url = config
                .url
                .as_deref()
                .filter(|url| !url.trim().is_empty())
                .ok_or("Streamable HTTP MCP server requires a URL")?;

            let transport = StreamableHttpClientTransport::from_uri(url);

            let client = ().serve(transport).await?;

            Ok(client)
        }

        other => {
            Err(format!("Unsupported MCP transport: {other}").into())
        }
    }
}
