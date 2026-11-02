use std::{
    error::Error,
    path::Path,
    process::Stdio,
};

use rmcp::{
    ServiceExt,
    transport::{ConfigureCommandExt, TokioChildProcess},
};

use tokio::process::Command;

pub async fn connect(
    executable: &Path,
) -> Result<
    rmcp::service::RunningService<rmcp::RoleClient, ()>,
    Box<dyn Error + Send + Sync>,
> {
    let transport = TokioChildProcess::new(
        Command::new(executable).configure(|cmd| {
            cmd.kill_on_drop(true);
            cmd.stderr(Stdio::inherit());
        }),
    )?;

    let client = ().serve(transport).await?;

    Ok(client)
}