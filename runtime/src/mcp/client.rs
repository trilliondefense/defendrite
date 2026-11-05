use std::error::Error;

use rmcp::{transport::TokioChildProcess, ServiceExt};

use tokio::process::Command;

use super::config::McpServerConfig;

pub async fn connect(
    config: &McpServerConfig,
) -> Result<rmcp::service::RunningService<rmcp::RoleClient, ()>, Box<dyn Error + Send + Sync>> {
    if config.transport != "stdio" {
        return Err(format!("Unsupported MCP transport: {}", config.transport).into());
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
