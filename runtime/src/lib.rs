
use std::{env, fs, path::PathBuf, time::Duration};
mod storage;
use serde_json::{json, Value};
use tauri::{Emitter, Manager};

pub async fn handle_agent_request(
    prompt: &str,
    app_handle: &tauri::AppHandle,
) -> String {
    // Load .env from the monorepo root.
    let env_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../.env");

    if let Err(error) = dotenvy::from_path(&env_path) {
        eprintln!("Could not load root .env: {error}");
    }

    // Initialize application storage.
    let mut storage_path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("./"));

    storage_path.push("storage");

    if let Err(error) = fs::create_dir_all(&storage_path) {
        eprintln!("Failed to create storage directory: {error}");
    }

    // Read Gemini configuration.
    let api_key = match env::var("HARNESS_MODEL_API_KEY") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return emit_error(
                app_handle,
                "HARNESS_MODEL_API_KEY is missing from the root .env",
            );
        }
    };

    let model = match env::var("HARNESS_MODEL_NAME") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return emit_error(
                app_handle,
                "HARNESS_MODEL_NAME is missing from the root .env",
            );
        }
    };

    // Create the HTTP client.
    let client: reqwest::Client =
        match reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
        {
            Ok(client) => client,
            Err(error) => {
                eprintln!("HTTP client initialization failed: {error}");
                return emit_error(
                    app_handle,
                    "Could not initialize the HTTP client.",
                );
            }
        };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
    );

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
            return emit_error(
                app_handle,
                "Could not parse the Gemini response.",
            );
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

        return emit_error(
            app_handle,
            "Gemini returned an empty response.",
        );
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

fn emit_error(
    app_handle: &tauri::AppHandle,
    message: &str,
) -> String {
    let display_message = format!("Error: {message}");

    if let Err(error) =
        app_handle.emit("agent-token", display_message.as_str())
    {
        eprintln!("Failed to emit error event: {error}");
    }

    display_message
}
