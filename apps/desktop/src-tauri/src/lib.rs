
use tauri::Manager;
use tauri_plugin_store::StoreExt;

// --------------------------------------------------
// ASK AGENT
// --------------------------------------------------

#[tauri::command]
async fn ask_agent(
    prompt: String,
    conversation_id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {

    // Verify that the API key exists.

    let store = app_handle
        .store("settings.json")
        .map_err(|e| {
            format!("Configuration store failed: {e}")
        })?;

    let key_is_missing = match store.get("HARNESS_MODEL_API_KEY") {
        Some(value) => match value.get("value").and_then(|v| v.as_str()) {
            Some(s) => s.trim().is_empty(),
            None => true,
        },
        None => true,
    };

    if key_is_missing {
        return Err(
            "SETUP_REQUIRED: Google AI Studio API key is missing. Please add your key in Workspace settings.".into()
        );
    }

    // Send the prompt to the selected conversation.

    let result = runtime::handle_agent_request(
        &prompt,
        &conversation_id,
        &app_handle,
    )
    .await;

    if result.starts_with("Error:") {
        return Err(result);
    }

    Ok(())
}

// --------------------------------------------------
// CREATE NEW CHAT
// --------------------------------------------------

#[tauri::command]
fn create_chat(
    app_handle: tauri::AppHandle,
) -> Result<runtime::Conversation, String> {

    runtime::create_new_conversation(&app_handle)
}

// --------------------------------------------------
// LIST ALL CHATS
// --------------------------------------------------

#[tauri::command]
fn list_chats(
    app_handle: tauri::AppHandle,
) -> Result<Vec<runtime::Conversation>, String> {

    runtime::get_conversations(&app_handle)
}

// --------------------------------------------------
// GET MESSAGES FOR A CHAT
// --------------------------------------------------

#[tauri::command]
fn get_chat_messages(
    conversation_id: String,
    app_handle: tauri::AppHandle,
) -> Result<Vec<runtime::Message>, String> {

    runtime::get_chat_messages(
        &app_handle,
        &conversation_id,
    )
}

// --------------------------------------------------
// DELETE CHAT
// --------------------------------------------------

#[tauri::command]
fn delete_chat(
    conversation_id: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {

    runtime::delete_chat(
        &app_handle,
        &conversation_id,
    )
}

// --------------------------------------------------
// APPLICATION ENTRY POINT
// --------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        // Single instance plugin.
        .plugin(
            tauri_plugin_single_instance::init(
                |app, _args, _cwd| {
                    if let Some(window) =
                        app.get_webview_window("main")
                    {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                },
            ),
        )

        // Settings storage.
        .plugin(
            tauri_plugin_store::Builder::new().build()
        )

        // External URL opener.
        .plugin(
            tauri_plugin_opener::init()
        )

        // Register frontend commands.
        .invoke_handler(
            tauri::generate_handler![
                ask_agent,
                create_chat,
                list_chats,
                get_chat_messages,
                delete_chat
            ]
        )

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
