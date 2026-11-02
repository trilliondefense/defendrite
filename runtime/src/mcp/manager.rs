use super::client;
use super::types::McpTool;
use serde_json::Value;

pub async fn discover_tools(
) -> Result<Vec<McpTool>, Box<dyn std::error::Error + Send + Sync>> {

    let client = client::connect("../../../target/debug/filesystem").await?;

    let result = client.list_all_tools().await?;

    let tools = result
        .iter()
        .map(|tool| McpTool {
            server: "filesystem".to_string(),
            name: tool.name.to_string(),
            description: tool.description.as_ref().map(|d| d.to_string()).unwrap_or_default(),
             input_schema: serde_json::to_value(tool.input_schema.as_ref())
                .unwrap_or(Value::Null),        
        })
        .collect();

    Ok(tools)
}