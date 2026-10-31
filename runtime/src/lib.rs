
use std::{
    fs,
    time::Duration,
};

mod storage;
use futures_util::StreamExt;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

pub use storage::models::{Conversation, Message};
// --------------------------------------------------
// DATABASE INITIALIZATION
// --------------------------------------------------

fn open_conversation_db(
    app_handle: &AppHandle,
) -> Result<rusqlite::Connection, String> {
    let mut storage_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve application data directory: {e}"))?;

    storage_path.push("storage");

    fs::create_dir_all(&storage_path)
        .map_err(|e| format!("Could not create storage directory: {e}"))?;

    let store = app_handle
        .store("settings.json")
        .map_err(|e| format!("Could not access application settings: {e}"))?;

    let db_key = store
        .get("HARNESS_HISTORY_DB_KEY")
        .and_then(|v| {
            v.get("value")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
        })
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| "Conversation database key is missing.".to_string())?;

    let db_path = storage_path.join("conversation.db");

    storage::open_db(&db_path, &db_key)
        .map_err(|e| format!("Could not open conversation database: {e}"))
}

// --------------------------------------------------
// CREATE NEW CONVERSATION
// --------------------------------------------------

pub fn create_new_conversation(
    app_handle: &AppHandle,
) -> Result<Conversation, String> {
    let mut db = open_conversation_db(app_handle)?;

    storage::create_conversation(&mut db)
        .map_err(|e| format!("Could not create conversation: {e}"))
}
// Retrieve messages for a selected conversation.
pub fn get_chat_messages(
    app_handle: &AppHandle,
    conversation_id: &str,
) -> Result<Vec<Message>, String> {
    let db = open_conversation_db(app_handle)?;

    storage::load_history(&db, conversation_id)
        .map_err(|e| format!("Could not retrieve chat messages: {e}"))
}

// Delete a selected conversation.
pub fn delete_chat(
    app_handle: &AppHandle,
    conversation_id: &str,
) -> Result<(), String> {
    let db = open_conversation_db(app_handle)?;

    storage::delete_conversation(&db, conversation_id)
        .map_err(|e| format!("Could not delete conversation: {e}"))
}

// --------------------------------------------------
// GET ALL CONVERSATIONS
// --------------------------------------------------

pub fn get_conversations(
    app_handle: &AppHandle,
) -> Result<Vec<Conversation>, String> {
    let db = open_conversation_db(app_handle)?;

    storage::list_conversations(&db)
        .map_err(|e| format!("Could not retrieve conversations: {e}"))
}

// --------------------------------------------------
// HANDLE AGENT REQUEST
// --------------------------------------------------

pub async fn handle_agent_request(
    prompt: &str,
    conversation_id: &str,
    app_handle: &AppHandle,
) -> String {

    // 1. Validate conversation ID.

    if conversation_id.trim().is_empty() {
        return emit_error(
            app_handle,
            "A valid conversation ID is required.",
        );
    }

    // 2. Open the encrypted conversation database.

    let mut db = match open_conversation_db(app_handle) {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Database initialization failed: {error}");
            return emit_error(
                app_handle,
                "Could not open conversation history.",
            );
        }
    };

    // 3. Verify that the requested conversation exists.

    let conversations = match storage::list_conversations(&db) {
        Ok(conversations) => conversations,
        Err(error) => {
            eprintln!("Could not retrieve conversations: {error}");
            return emit_error(
                app_handle,
                "Could not retrieve conversations.",
            );
        }
    };

    if !conversations
        .iter()
        .any(|conversation| conversation.id == conversation_id)
    {
        return emit_error(
            app_handle,
            "The requested conversation does not exist.",
        );
    }

    // 4. Read application settings.

    let store = match app_handle.store("settings.json") {
        Ok(store) => store,
        Err(error) => {
            return emit_error(
                app_handle,
                &format!("Failed to access application settings: {error}"),
            );
        }
    };

    // 5. Retrieve Gemini API key.

    let api_key = match store.get("HARNESS_MODEL_API_KEY") {
        Some(value) => match value.get("value").and_then(|v| v.as_str()) {
            Some(key) if !key.trim().is_empty() => key.to_string(),
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

    // 6. Retrieve configured Gemini model.

    let model = match store.get("HARNESS_MODEL_NAME") {
        Some(value) => match value.get("value").and_then(|v| v.as_str()) {
            Some(name) if !name.trim().is_empty() => name.to_string(),
            _ => "gemini-1.5-flash".to_string(),
        },
        None => "gemini-1.5-flash".to_string(),
    };

    // 7. Load history ONLY for the selected conversation.

    let history = match storage::load_history(&db, conversation_id) {
        Ok(history) => history,
        Err(error) => {
            eprintln!("Could not load conversation history: {error}");
            return emit_error(
                app_handle,
                "Could not load conversation history.",
            );
        }
    };

    // 8. Convert history into Gemini contents format.

    let mut contents: Vec<Value> = history
        .into_iter()
        .map(|message| {
            json!({
                "role": message.role,
                "parts": [
                    { "text": message.content }
                ]
            })
        })
        .collect();

    // 9. Append current user prompt.

    contents.push(json!({
        "role": "user",
        "parts": [
            { "text": prompt }
        ]
    }));

    // --------------------------------------------------
    // GEMINI STREAMING API REQUEST
    // --------------------------------------------------


    let client = match reqwest::Client::builder()
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
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent?alt=sse"
    );

    let response = match client
        .post(&url)
        .header("x-goog-api-key", &api_key)
        .header("Accept", "text/event-stream")
        .json(&json!({
            "contents": contents
        }))
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Gemini streaming request failed: {error}");

            return emit_error(
                app_handle,
                "Could not connect to Gemini. Check your network.",
            );
        }
    };

    // --------------------------------------------------
    // HANDLE HTTP ERRORS
    // --------------------------------------------------

    let status = response.status();

    if !status.is_success() {
        let details = response
            .text()
            .await
            .unwrap_or_else(|_| {
                "No error details available".to_string()
            });

        eprintln!("Gemini API returned {status}: {details}");

        return emit_error(
            app_handle,
            &format!(
                "Gemini API returned HTTP {status}. Check application logs."
            ),
        );
    }

    // --------------------------------------------------
    // READ GEMINI STREAMING RESPONSE
    // --------------------------------------------------

    let mut stream = response.bytes_stream();

    // Use bytes instead of String to safely handle
    // UTF-8 characters split across network chunks.
    let mut buffer: Vec<u8> = Vec::new();

    let mut answer = String::new();
    let mut last_event = String::new();

    while let Some(chunk_result) = stream.next().await {

        let chunk = match chunk_result {
            Ok(chunk) => chunk,
            Err(error) => {
                eprintln!("Gemini stream failed: {error}");

                return emit_error(
                    app_handle,
                    "Gemini streaming connection was interrupted.",
                );
            }
        };

        // Append incoming network bytes.
        buffer.extend_from_slice(&chunk);

        // Process complete SSE lines.
        while let Some(line_end) = buffer.iter().position(|b| *b == b'\n') {

            let mut line: Vec<u8> =
                buffer.drain(..=line_end).collect();

            // Remove newline.
            line.pop();

            // Handle CRLF (\r\n) and LF (\n).
            if line.last() == Some(&b'\r') {
                line.pop();
            }

            let line = String::from_utf8_lossy(&line);

            // Only process SSE data lines.
            let Some(data) = line.strip_prefix("data:") else {
                continue;
            };

            let data = data.trim();

            if data.is_empty() || data == "[DONE]" {
                continue;
            }

            last_event = data.to_string();

            // Parse Gemini JSON event.
            let parsed: Value = match serde_json::from_str(data) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("Gemini SSE JSON parsing failed: {error}");
                    eprintln!("Received event: {data}");
                    continue;
                }
            };

            // Extract generated text.
            let text = parsed["candidates"][0]
                ["content"]["parts"]
                .as_array()
                .map(|parts| {
                    parts
                        .iter()
                        .filter_map(|part| {
                            part["text"].as_str()
                        })
                        .collect::<String>()
                })
                .unwrap_or_default();

            if text.is_empty() {
                continue;
            }

            // Accumulate complete response.
            answer.push_str(&text);

            // Emit generated text immediately to frontend.
            if let Err(error) =
                app_handle.emit("agent-token", text.clone())
            {
                eprintln!("Failed to emit agent token: {error}");
            }
        }
    }

    // --------------------------------------------------
    // PROCESS FINAL INCOMPLETE SSE LINE
    // --------------------------------------------------

    // Some streams may end without a trailing newline.
    if !buffer.is_empty() {
        buffer.push(b'\n');
    }

    while let Some(line_end) = buffer.iter().position(|b| *b == b'\n') {

        let mut line: Vec<u8> =
            buffer.drain(..=line_end).collect();

        line.pop();

        if line.last() == Some(&b'\r') {
            line.pop();
        }

        let line = String::from_utf8_lossy(&line);

        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };

        let data = data.trim();

        if data.is_empty() || data == "[DONE]" {
            continue;
        }

        last_event = data.to_string();

        let parsed: Value = match serde_json::from_str(data) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Final Gemini SSE parsing failed: {error}");
                continue;
            }
        };

        let text = parsed["candidates"][0]
            ["content"]["parts"]
            .as_array()
            .map(|parts| {
                parts
                    .iter()
                    .filter_map(|part| part["text"].as_str())
                    .collect::<String>()
            })
            .unwrap_or_default();

        if !text.is_empty() {
            answer.push_str(&text);

            if let Err(error) =
                app_handle.emit("agent-token", text.clone())
            {
                eprintln!("Failed to emit agent token: {error}");
            }
        }
    }

    // --------------------------------------------------
    // VALIDATE COMPLETED RESPONSE
    // --------------------------------------------------

    if answer.trim().is_empty() {

        eprintln!("Gemini returned no generated text.");
        eprintln!("Last Gemini event: {last_event}");

        return emit_error(
            app_handle,
            "Gemini returned an empty response.",
        );
    }

    // --------------------------------------------------
    // SAVE CONVERSATION EXCHANGE
    // --------------------------------------------------

    if let Err(error) = storage::save_exchange(
        &mut db,
        conversation_id,
        prompt,
        &answer,
    ) {
        eprintln!(
            "Failed to save conversation exchange: {error}"
        );

        return emit_error(
            app_handle,
            "Could not save conversation history.",
        );
    }

    answer


}

// --------------------------------------------------
// ERROR EVENT
// --------------------------------------------------

fn emit_error(
    app_handle: &AppHandle,
    message: &str,
) -> String {
    let display_message = format!("Error: {message}");

    if let Err(error) = app_handle.emit(
        "agent-token",
        display_message.as_str(),
    ) {
        eprintln!("Failed to emit error event: {error}");
    }

    display_message
}
