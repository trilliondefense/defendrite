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

    // Discover available MCP tools.
    let mcp_tools = manager::discover_tools().await?;

    println!("Available MCP tools: {mcp_tools:#?}");

    // Convert MCP tools into Gemini declarations.
    let gemini_tools = gemini::convert_tools(&mcp_tools);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    // Prevent unlimited tool-call loops.
    const MAX_ITERATIONS: usize = 8;

    for iteration in 0..MAX_ITERATIONS {

        println!("Agent iteration: {}", iteration + 1);

        // Ask Gemini for the next response.
        let response = gemini::generate(
            &client,
            api_key,
            model,
            &contents,
            &gemini_tools,
            app_handle,
        ).await?;

        // Find function calls in Gemini's response.
        let function_calls: Vec<&Value> = response.parts
            .iter()
            .filter_map(|part| part.get("functionCall"))
            .collect();

        // No function calls means Gemini has finished.
        if function_calls.is_empty() {
            if response.text.trim().is_empty() {
                return Err("Gemini returned an empty response".into());
            }

            return Ok(response.text);
        }

        // Preserve Gemini's original function-call response.
        contents.push(json!({
            "role": "model",
            "parts": response.parts
        }));

        let mut function_responses = Vec::new();

        // Execute each requested MCP tool.
        for call in function_calls {

            let name = call["name"]
                .as_str()
                .ok_or("Function call has no name")?;

            let args = call.get("args")
                .cloned()
                .unwrap_or_else(|| json!({}));

            println!("Executing MCP tool: {name}");
            println!("Arguments: {args}");

            // Find which MCP server owns this tool.
            let tool = mcp_tools
                .iter()
                .find(|tool| tool.name == name);

            let result = match tool {
                Some(tool) => {
                    manager::execute_tool(
                        &tool.server,
                        name,
                        args,
                    ).await
                }

                None => {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Unknown tool: {name}"),
                    ).into())
                }
            };

            // Send either the successful result or an error to Gemini.
            let response_body = match result {
                Ok(value) => value,
                Err(error) => {
                    json!({
                        "error": error.to_string()
                    })
                }
            };

            let mut function_response = json!({
                "functionResponse": {
                    "name": name,
                    "response": response_body
                }
            });

            // Preserve function-call IDs when supplied by Gemini.
            if let Some(id) = call.get("id") {
                function_response["functionResponse"]["id"] = id.clone();
            }

            function_responses.push(function_response);
        }

        // Return all tool results to Gemini.
        contents.push(json!({
            "role": "user",
            "parts": function_responses
        }));
    }

    Err("Agent exceeded maximum tool-call iterations".into())
}