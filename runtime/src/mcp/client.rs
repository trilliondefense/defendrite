use rmcp::{
    ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
};

use tokio::process::Command;

pub async fn connect(
    executable: &str,
) -> Result<
    rmcp::service::RunningService<rmcp::RoleClient, ()>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    let transport = TokioChildProcess::new(
        Command::new(executable).configure(|cmd| {
            cmd.kill_on_drop(true);
        }),
    )?;

    let client = ().serve(transport).await?;

    Ok(client)
}