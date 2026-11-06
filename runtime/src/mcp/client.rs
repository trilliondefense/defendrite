
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

pub async fn connect(
    config: &McpServerConfig,
) -> McpResult<rmcp::service::RunningService<rmcp::RoleClient, ()>> {
    match config.transport.as_str() {
        "stdio" => {
            if config.command.is_empty() {
                return Err("stdio MCP server requires a command".into());
            }

            let mut command = Command::new(&config.command);

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
