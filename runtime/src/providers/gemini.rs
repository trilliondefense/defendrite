use futures_util::StreamExt;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

use crate::mcp::types::McpTool;

pub struct GeminiTurn {
    pub parts: Vec<Value>,
    pub text: String,
}

pub fn convert_tools(tools: &[McpTool]) -> Value {
    let declarations: Vec<Value> = tools
        .iter()
        .map(|tool| {
            let mut schema = tool.input_schema.clone();

            // Clean the top-level schema and any nested object parameters
            clean_gemini_schema(&mut schema);

            json!({
                "name": tool.name,
                "description": tool.description,
                "parameters": schema
            })
        })
        .collect();

    // Correct nesting structure: An array containing a single Tool Object
    // that wraps the function declarations list.
    json!([
        {
            "function_declarations": declarations
        }
    ])
}

/// Recursively removes JSON schema keywords that Gemini's parser rejects
fn clean_gemini_schema(value: &mut Value) {
    if let Some(obj) = value.as_object_mut() {
        // Strip top-level metadata and draft keys
        obj.remove("$schema");
        obj.remove("$defs");
        obj.remove("definitions");
        obj.remove("$comment");

        // Recursively clean inner properties (e.g. if an object parameter has nested structures)
        if let Some(properties) = obj.get_mut("properties") {
            if let Some(prop_obj) = properties.as_object_mut() {
                for (_, prop_value) in prop_obj.iter_mut() {
                    clean_gemini_schema(prop_value);
                }
            }
        }

        // Also clean items array schemas if the parameter type is an "array"
        if let Some(items) = obj.get_mut("items") {
            clean_gemini_schema(items);
        }
    }
}

pub async fn generate(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    contents: &[Value],
    tools: &Value,
    system_prompt: &str,
    app_handle: &AppHandle,
) -> Result<GeminiTurn, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent?alt=sse"
    );

    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .header("Accept", "text/event-stream")
        .json(&json!({
            "contents": contents,
            "tools": tools,
            "system_instruction": {
                "parts": [
                    {
                        "text": system_prompt
                    }
                ]
            }
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        return Err(format!("Gemini HTTP {status}: {body}").into());
    }

    let mut stream = response.bytes_stream();
    let mut buffer = Vec::<u8>::new();

    let mut parts = Vec::new();
    let mut text = String::new();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        buffer.extend_from_slice(&chunk);

        while let Some(position) = buffer.iter().position(|b| *b == b'\n') {
            let mut line: Vec<u8> = buffer.drain(..=position).collect();

            line.pop();

            if line.last() == Some(&b'\r') {
                line.pop();
            }

            process_sse_line(&line, &mut parts, &mut text, app_handle)?;
        }
    }

    // Process a final line without a newline.
    if !buffer.is_empty() {
        process_sse_line(&buffer, &mut parts, &mut text, app_handle)?;
    }

    Ok(GeminiTurn { parts, text })
}

fn process_sse_line(
    line: &[u8],
    parts: &mut Vec<Value>,
    text: &mut String,
    app_handle: &AppHandle,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let line = String::from_utf8_lossy(line);

    let Some(data) = line.strip_prefix("data:") else {
        return Ok(());
    };

    let data = data.trim();

    if data.is_empty() || data == "[DONE]" {
        return Ok(());
    }

    let parsed: Value = serde_json::from_str(data)?;

    let Some(event_parts) = parsed["candidates"][0]["content"]["parts"].as_array() else {
        return Ok(());
    };

    for part in event_parts {
        parts.push(part.clone());

        if let Some(generated_text) = part["text"].as_str() {
            text.push_str(generated_text);

            let _ = app_handle.emit("agent-token", generated_text.to_string());
        }
    }

    Ok(())
}
