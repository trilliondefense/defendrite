use runtime::{Attachment,McpConfig,McpServerConfig};
use tauri::Manager;
use tauri_plugin_store::StoreExt;

// Read complete MCP configuration.
#[tauri::command]
fn get_mcp_config(
    app_handle: tauri::AppHandle,
) -> Result<McpConfig, String> {
    runtime::get_mcp_config(&app_handle)
        .map_err(|e| format!("Could not read MCP config: {e}"))
}

// Save complete MCP configuration.
#[tauri::command]
fn save_mcp_config(
    app_handle: tauri::AppHandle,
    config: McpConfig,
) -> Result<(), String> {
    runtime::save_mcp_config(&app_handle, &config)
        .map_err(|e| format!("Could not save MCP config: {e}"))
}

// Add or update an MCP server.
#[tauri::command]
fn add_mcp_server(
    app_handle: tauri::AppHandle,
    server: McpServerConfig,
) -> Result<McpConfig, String> {
    runtime::add_mcp_server(&app_handle, server)
        .map_err(|e| format!("Could not add MCP server: {e}"))
}

// Remove an MCP server.
#[tauri::command]
fn remove_mcp_server(
    app_handle: tauri::AppHandle,
    name: String,
) -> Result<McpConfig, String> {
    runtime::remove_mcp_server(&app_handle, &name)
        .map_err(|e| format!("Could not remove MCP server: {e}"))
}

#[tauri::command]
async fn ask_agent(
    prompt: String,
    conversation_id: String,
    attachments: Vec<Attachment>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Verify that the API key exists.

    let store = app_handle
        .store("settings.json")
        .map_err(|e| format!("Configuration store failed: {e}"))?;

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

    let result =
        runtime::handle_agent_request(&prompt, &conversation_id, &attachments[..], &app_handle)
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
fn create_chat(app_handle: tauri::AppHandle) -> Result<runtime::Conversation, String> {
    runtime::create_new_conversation(&app_handle)
}

// --------------------------------------------------
// LIST ALL CHATS
// --------------------------------------------------

#[tauri::command]
fn list_chats(app_handle: tauri::AppHandle) -> Result<Vec<runtime::Conversation>, String> {
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
    runtime::get_chat_messages(&app_handle, &conversation_id)
}

// --------------------------------------------------
// DELETE CHAT
// --------------------------------------------------

#[tauri::command]
fn delete_chat(conversation_id: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    runtime::delete_chat(&app_handle, &conversation_id)
}

#[tauri::command]
fn create_playbook(
    app_handle: tauri::AppHandle,
    name: String,
    content: String,
) -> Result<runtime::Playbook, String> {
    runtime::create_new_playbook(&app_handle, name, content)
}

#[tauri::command]
fn get_all_playbooks(app_handle: tauri::AppHandle) -> Result<Vec<runtime::Playbook>, String> {
    runtime::get_all_playbooks(&app_handle)
}

#[tauri::command]
fn get_playbook_by_id(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<runtime::PlaybookContent, String> {
    runtime::get_playbook_by_id(&app_handle, &id)
}

#[tauri::command]
fn export_playbook(
    app_handle: tauri::AppHandle,
    id: String,
    destination: String,
) -> Result<(), String> {
    runtime::export_existing_playbook(&app_handle, &id, std::path::PathBuf::from(destination))
}

#[tauri::command]
fn update_playbook(
    app_handle: tauri::AppHandle,
    id: String,
    name: String,
    content: String,
) -> Result<runtime::Playbook, String> {
    runtime::update_existing_playbook(&app_handle, &id, name, content)
}

#[tauri::command]
fn delete_playbook(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    runtime::delete_existing_playbook(&app_handle, &id)
}

#[tauri::command]
fn log_to_terminal(message: String) {
    println!("Frontend :{}", message);
}

// --------------------------------------------------
// APPLICATION ENTRY POINT
// --------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Single instance plugin.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        // Settings storage.
        .plugin(tauri_plugin_store::Builder::new().build())
        // External URL opener.
        .plugin(tauri_plugin_opener::init())
        // Dialog opener.
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // Register frontend commands.
        .invoke_handler(tauri::generate_handler![
            ask_agent,
            create_chat,
            list_chats,
            get_chat_messages,
            delete_chat,
            log_to_terminal,
            create_playbook,
            get_all_playbooks,
            get_playbook_by_id,
            update_playbook,
            delete_playbook,
            export_playbook,
            get_mcp_config,save_mcp_config,add_mcp_server,remove_mcp_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
