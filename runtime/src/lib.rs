
use std::{
    fs,
    path::PathBuf,
    time::Duration,
};

mod storage;

use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

use storage::models::Conversation;

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
    // GEMINI API REQUEST
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
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
    );

    let response = match client
        .post(&url)
        .header("x-goog-api-key", &api_key)
        .json(&json!({
            "contents": contents
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

    // --------------------------------------------------
    // HANDLE API RESPONSE
    // --------------------------------------------------

    let status = response.status();

    if !status.is_success() {
        let details = response
            .text()
            .await
            .unwrap_or_else(|_| "No error details available".to_string());

        eprintln!("Gemini API returned {status}: {details}");

        return emit_error(
            app_handle,
            &format!(
                "Gemini API returned HTTP {status}. Check application logs."
            ),
        );
    }

    // Parse response JSON.

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

    // Extract generated response text.

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

    // --------------------------------------------------
    // SAVE CONVERSATION EXCHANGE
    // --------------------------------------------------

    // Both messages are saved in one transaction.
    // Neither message is saved if the transaction fails.

    if let Err(error) = storage::save_exchange(
        &mut db,
        conversation_id,
        prompt,
        &answer,
    ) {
        eprintln!("Failed to save conversation exchange: {error}");

        return emit_error(
            app_handle,
            "Could not save conversation history.",
        );
    }

    // --------------------------------------------------
    // EMIT RESPONSE TO FRONTEND
    // --------------------------------------------------

    // Note: this is chunking an already completed response,
    // not true Gemini API streaming.

    for chunk in answer.split_inclusive(char::is_whitespace) {
        if let Err(error) = app_handle.emit("agent-token", chunk) {
            eprintln!("Failed to emit agent token: {error}");
            break;
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
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
