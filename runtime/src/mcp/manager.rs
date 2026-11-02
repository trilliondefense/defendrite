use super::client;
use super::types::McpTool;
use serde_json::Value;
use rmcp::model::CallToolRequestParams;

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

pub async fn execute_tool(
    server: &str,
    name: &str,
    args: Value,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {

    let executable = match server {
        "filesystem" => "./target/debug/filesystem",
        _ => return Err(format!("Unknown MCP server: {server}").into()),
    };

    let client = client::connect(executable).await?;

    let arguments = args
        .as_object()
        .cloned()
        .unwrap_or_default();

    let result = client
        .call_tool(
            CallToolRequestParams::new(name.to_string())
                .with_arguments(arguments),
        )
        .await?;

    Ok(serde_json::json!({
        "result": format!("{result:#?}")
    }))
}