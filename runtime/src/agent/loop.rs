use serde_json::{json, Value};
use tauri::AppHandle;

use crate::{
    mcp::manager,
    providers::gemini,
};
pub async fn run(
    mut contents: Vec<Value>,
    api_key: &str,
    model: &str,
    app_handle: &AppHandle,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    const MAX_ITERATIONS: usize = 8;

    // 1. Discover available MCP tools.
    let mcp_manager = manager::McpManager::global(app_handle)?;
    let mcp_tools = mcp_manager.discover_tools().await?;

    println!("Available MCP tools: {mcp_tools:#?}");

    // 2. Convert MCP tools into Gemini declarations.
    let gemini_tools = gemini::convert_tools(&mcp_tools);

    println!("Available Gemini tools: {gemini_tools:#?}");

    // 3. Create HTTP client.
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    // 4. Agent execution loop.
    for iteration in 0..MAX_ITERATIONS {
        println!("Agent iteration: {}", iteration + 1);

        let response = gemini::generate(
            &client,
            api_key,
            model,
            &contents,
            &gemini_tools,
            app_handle,
        )
        .await?;

        // 5. Extract function calls using snake_case.
        let function_calls: Vec<Value> = response
            .parts
            .iter()
            .filter_map(|part| {
                part.get("function_call").cloned()
            })
            .collect();

        // 6. No function calls: return Gemini's final response.
        if function_calls.is_empty() {
            if response.text.trim().is_empty() {
                return Err(
                    "Gemini returned an empty response".into()
                );
            }

            return Ok(response.text);
        }

        // 7. Preserve the complete model response.
        contents.push(json!({
            "role": "model",
            "parts": response.parts
        }));

        let mut function_responses = Vec::new();

        // 8. Execute requested MCP tools.
        for call in function_calls {
            let name = call
                .get("name")
                .and_then(Value::as_str)
                .ok_or("Function call has no name")?;

            let args = call
                .get("args")
                .cloned()
                .unwrap_or_else(|| json!({}));

            println!("Executing MCP tool: {}", name);
            println!("Arguments: {}", args);

            // Find the MCP server responsible for this tool.
            let result = match mcp_tools
                .iter()
                .find(|tool| tool.name == name)
            {
                Some(tool) => {
                    mcp_manager
                        .execute_tool(
                            &tool.server,
                            name,
                            args,
                        )
                        .await
                }

                None => {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Unknown MCP tool: {}", name),
                    ).into())
                }
            };

            // 9. Normalize MCP results into JSON objects.
            let response_body = match result {
                Ok(value) => {
                    match value {
                        Value::Object(_) => value,
                        other => json!({
                            "content": other
                        }),
                    }
                }

                Err(error) => {
                    eprintln!(
                        "MCP tool execution failed: {}",
                        error
                    );

                    json!({
                        "error": error.to_string()
                    })
                }
            };

            // 10. Build Gemini function response.
            let mut function_response = json!({
                "function_response": {
                    "name": name,
                    "response": response_body
                }
            });

            // Preserve ID if provided by the API.
            if let Some(id) = call.get("id") {
                function_response["function_response"]["id"] =
                    id.clone();
            }

            function_responses.push(function_response);
        }

        // 11. Send tool results back to Gemini.
        contents.push(json!({
            "role": "user",
            "parts": function_responses
        }));
    }

    Err(
        "Agent exceeded maximum tool-call iterations".into()
    )
}