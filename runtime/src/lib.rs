use std::{fs, path::PathBuf, time::Duration};
mod storage;
use serde_json::{json, Value};
use tauri::{Emitter, Manager,AppHandle};
use tauri_plugin_store::StoreExt;

pub async fn handle_agent_request(prompt: &str, app_handle: &AppHandle) -> String {
    // 1. Initialize application local data layout folders
    let mut storage_path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("./"));

    storage_path.push("storage");

    if let Err(error) = fs::create_dir_all(&storage_path) {
        eprintln!("Failed to create storage directory: {error}");
    }

    // 2. Read Configuration from the Tauri v2 Store instance
    let store = match app_handle.store("settings.json") {
        Ok(s) => s,
        Err(e) => {
            return emit_error(
                app_handle,
                &format!("Failed to access application settings store: {e}"),
            );
        }
    };

    // 3. Extract the Google AI Studio Key string value
    let api_key = match store.get("HARNESS_MODEL_API_KEY") {
        Some(value) => match value.as_str() {
            Some(s) if !s.trim().is_empty() => s.to_string(),
            _ => {
                return emit_error(
                    app_handle,
                    "HARNESS_MODEL_API_KEY is empty in application settings.",
                );
            }
        },
        None => {
            return emit_error(
                app_handle,
                "HARNESS_MODEL_API_KEY is missing from application settings.",
            );
        }
    };

    // 4. Extract the active target Model name string profile
    let model = match store.get("HARNESS_MODEL_NAME") {
        Some(value) => match value.as_str() {
            Some(s) if !s.trim().is_empty() => s.to_string(),
            _ => "gemini-1.5-flash".to_string(), // Fallback if entry is malformed
        },
        None => "gemini-1.5-flash".to_string(), // Default baseline profile
    };

    // --- rest of your network streaming and tool execution loop goes here ---

    // Create the HTTP client.
    let client: reqwest::Client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            eprintln!("HTTP client initialization failed: {error}");
            return emit_error(app_handle, "Could not initialize the HTTP client.");
        }
    };

    let url =
        format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent");

    // Send the prompt to Gemini.
    let response: reqwest::Response = match client
        .post(&url)
        .header("x-goog-api-key", &api_key)
        .json(&json!({
            "contents": [{
                "role": "user",
                "parts": [{
                    "text": prompt
                }]
            }]
        }))
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Gemini request failed: {error}");
            return emit_error(
                app_handle,
                "Could not connect to Gemini. Check your network.",
            );
        }
    };

    // Handle unsuccessful API responses.
    let status = response.status();

    if !status.is_success() {
        let details = response
            .text()
            .await
            .unwrap_or_else(|_| "No error details available".to_string());

        eprintln!("Gemini API returned {status}: {details}");

        return emit_error(
            app_handle,
            &format!("Gemini API returned HTTP {status}. Check application logs."),
        );
    }

    // Parse the JSON response.
    let data: Value = match response.json().await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse Gemini response: {error}");
            return emit_error(app_handle, "Could not parse the Gemini response.");
        }
    };

    // Extract generated text.
    let answer = data["candidates"][0]["content"]["parts"]
        .as_array()
        .map(|parts| {
            parts
                .iter()
                .filter_map(|part| part["text"].as_str())
                .collect::<String>()
        })
        .unwrap_or_default();

    if answer.trim().is_empty() {
        eprintln!("Gemini returned no text: {data}");

        return emit_error(app_handle, "Gemini returned an empty response.");
    }

    // Emit chunks to the Svelte frontend.
    // Note: this chunks an already-completed response.
    for chunk in answer.split_inclusive(char::is_whitespace) {
        if let Err(error) = app_handle.emit("agent-token", chunk) {
            eprintln!("Failed to emit agent token: {error}");
            break;
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    answer
}

fn emit_error(app_handle: &tauri::AppHandle, message: &str) -> String {
    let display_message = format!("Error: {message}");

    if let Err(error) = app_handle.emit("agent-token", display_message.as_str()) {
        eprintln!("Failed to emit error event: {error}");
    }

    display_message
}
