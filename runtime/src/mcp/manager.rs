use super::client;
use super::types::McpTool;

pub async fn discover_tools(
) -> Result<Vec<McpTool>, Box<dyn std::error::Error + Send + Sync>> {

    let client = client::connect("../../../target/debug/filesystem").await?;

    let result = client.list_all_tools().await?;

    let tools = result
        .iter()
        .map(|tool| McpTool {
            server: "filesystem".to_string(),
            name: tool.name.to_string(),
            description: tool.description.as_ref().map(|d| d.to_string()).unwrap_or_default(),        })
        .collect();

    Ok(tools)
}