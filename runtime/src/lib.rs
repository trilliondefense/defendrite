use std::fs;
mod storage;
use serde_json::{json, Value};
use std::collections::HashSet;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;
pub mod agent;
pub mod mcp;
pub mod playbooks;
pub mod providers;
pub mod reports;
pub use mcp::config::{McpConfig, McpServerConfig};
pub use mcp::types::McpTool;
pub use playbooks::models::{Playbook, PlaybookContent};
pub use reports::models::Report;
pub use storage::models::{Attachment, Conversation, Log, Message};

// Resolve the shared MCP configuration file.
fn mcp_config_path(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Could not resolve config directory: {e}"))?;

    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Could not create config directory: {e}"))?;

    Ok(config_dir.join("mcp.json"))
}

// Validate an individual MCP server.
fn validate_server(server: &McpServerConfig) -> Result<(), String> {
    if server.name.trim().is_empty() {
        return Err("MCP server name cannot be empty".into());
    }

    match server.transport.as_str() {
        "stdio" => {
            if server.command.trim().is_empty() {
                return Err(format!(
                    "Command is required for stdio server '{}'",
                    server.name
                ));
            }

            if server.url.is_some() {
                return Err("stdio server cannot have an HTTP URL".into());
            }
        }

        "streamable_http" => {
            let url = server
                .url
                .as_deref()
                .filter(|url| !url.trim().is_empty())
                .ok_or_else(|| format!("URL is required for server '{}'", server.name))?;

            let parsed =
                url::Url::parse(url).map_err(|e| format!("Invalid MCP server URL: {e}"))?;

            if !["http", "https"].contains(&parsed.scheme()) {
                return Err("Only HTTP and HTTPS URLs are supported".into());
            }

            if parsed.host_str().is_none() {
                return Err("MCP server URL must contain a hostname".into());
            }

            if !server.command.is_empty() || !server.args.is_empty() || !server.env.is_empty() {
                return Err("HTTP server cannot contain stdio command, args or env".into());
            }
        }

        other => {
            return Err(format!("Unsupported MCP transport: {other}"));
        }
    }

    Ok(())
}

// Validate the entire configuration.
fn validate_config(config: &McpConfig) -> Result<(), String> {
    let mut names = HashSet::new();

    for server in &config.servers {
        validate_server(server)?;

        if !names.insert(server.name.trim().to_lowercase()) {
            return Err(format!("Duplicate MCP server name: {}", server.name));
        }
    }

    Ok(())
}

// Read the complete MCP configuration.
pub fn get_mcp_config(app_handle: &tauri::AppHandle) -> Result<McpConfig, String> {
    let path = mcp_config_path(app_handle)?;

    if !path.exists() {
        return Ok(McpConfig {
            servers: Vec::new(),
        });
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Could not read mcp.json: {e}"))?;

    serde_json::from_str(&contents).map_err(|e| format!("Could not parse mcp.json: {e}"))
}

// Save the complete MCP configuration.
pub fn save_mcp_config(app_handle: &tauri::AppHandle, config: &McpConfig) -> Result<(), String> {
    validate_config(config)?;

    let path = mcp_config_path(app_handle)?;

    let contents = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Could not serialize MCP config: {e}"))?;

    let temp_path = path.with_extension("json.tmp");

    fs::write(&temp_path, contents).map_err(|e| format!("Could not write MCP config: {e}"))?;

    fs::rename(&temp_path, &path).map_err(|e| format!("Could not replace mcp.json: {e}"))?;

    Ok(())
}

// Add or update an MCP server.
pub fn add_mcp_server(
    app_handle: &tauri::AppHandle,
    server: McpServerConfig,
) -> Result<McpConfig, String> {
    validate_server(&server)?;

    let mut config = get_mcp_config(app_handle)?;

    if let Some(existing) = config
        .servers
        .iter_mut()
        .find(|s| s.name.eq_ignore_ascii_case(&server.name))
    {
        *existing = server;
    } else {
        config.servers.push(server);
    }

    save_mcp_config(app_handle, &config)?;

    Ok(config)
}

// Remove an MCP server.
pub fn remove_mcp_server(app_handle: &tauri::AppHandle, name: &str) -> Result<McpConfig, String> {
    if name.trim().is_empty() {
        return Err("MCP server name cannot be empty".into());
    }

    let mut config = get_mcp_config(app_handle)?;

    let original_len = config.servers.len();

    config
        .servers
        .retain(|s| !s.name.eq_ignore_ascii_case(name));

    if original_len == config.servers.len() {
        return Err(format!("MCP server not found: {name}"));
    }

    save_mcp_config(app_handle, &config)?;

    Ok(config)
}

pub fn get_all_reports(app_handle: &AppHandle) -> Result<Vec<Report>, String> {
    reports::list_reports(app_handle).map_err(|e| format!("Could not retrieve playbooks: {e}"))
}

pub async fn get_all_tools(app_handle: &AppHandle) -> Result<Vec<McpTool>, String> {
    let mcp_manager =
        mcp::manager::McpManager::global(app_handle).map_err(|err| err.to_string())?;
    let mcp_tools = mcp_manager
        .discover_tools()
        .await
        .map_err(|err| err.to_string())?;
    Ok(mcp_tools)
}

pub fn get_all_logs(app_handle: &AppHandle) -> Result<Vec<Log>, String> {
    let db = open_conversation_db(app_handle)?;
    storage::list_logs(&db).map_err(|e| format!("Could not retrieve playbooks: {e}"))
}

pub fn save_log(
    app_handle: &AppHandle,
    level: String,
    message: String,
    details: String,
) -> Result<(), String> {
    let log = Log {
        id: 0,                    // Ignored when inserting.
        timestamp: String::new(), // Generated by SQLite.
        level,
        source: "Runtime Engine".to_string(),
        message,
        details: Some(details),
    };
    let mut db = open_conversation_db(app_handle)?;
    storage::save_log(&mut db, &log).map_err(|err| err.to_string())?;
    Ok(())
}

/// Create a new Markdown playbook.
pub fn create_new_playbook(
    app_handle: &AppHandle,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    playbooks::create_playbook(app_handle, name, content)
        .map_err(|e| format!("Could not create playbook: {e}"))
}

pub fn export_existing_playbook(
    app_handle: &AppHandle,
    id: &str,
    destination: std::path::PathBuf,
) -> Result<(), String> {
    playbooks::export_playbook(app_handle, id, destination)
        .map_err(|e| format!("Could not export playbook: {e}"))
}

/// Retrieve all playbooks.
pub fn get_all_playbooks(app_handle: &AppHandle) -> Result<Vec<Playbook>, String> {
    playbooks::list_playbooks(app_handle).map_err(|e| format!("Could not retrieve playbooks: {e}"))
}

/// Retrieve a playbook by ID, including its Markdown content.
pub fn get_playbook_by_id(app_handle: &AppHandle, id: &str) -> Result<PlaybookContent, String> {
    playbooks::get_playbook(app_handle, id).map_err(|e| format!("Could not retrieve playbook: {e}"))
}

/// Update an existing playbook.
pub fn update_existing_playbook(
    app_handle: &AppHandle,
    id: &str,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    playbooks::update_playbook(app_handle, id, name, content)
        .map_err(|e| format!("Could not update playbook: {e}"))
}

/// Delete an existing playbook.
pub fn delete_existing_playbook(app_handle: &AppHandle, id: &str) -> Result<(), String> {
    playbooks::delete_playbook(app_handle, id)
        .map_err(|e| format!("Could not delete playbook: {e}"))
}

pub fn delete_chat_before(app_handle: &AppHandle, cutoff_timestamp: i64) -> Result<(), String> {
    let db = open_conversation_db(app_handle)?;

    storage::delete_conversations_before(&db, cutoff_timestamp)
        .map_err(|e| format!("Could not delete conversation: {e}"))
}

pub fn delete_logs_before(app_handle: &AppHandle, cutoff_timestamp: i64) -> Result<(), String> {
    let db = open_conversation_db(app_handle)?;
    storage::delete_logs_before(&db, cutoff_timestamp)
        .map_err(|e| format!("Could not delete conversation: {e}"))
}


fn open_conversation_db(app_handle: &AppHandle) -> Result<rusqlite::Connection, String> {
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
        .and_then(|v| v.get("value").and_then(|v| v.as_str()).map(str::to_owned))
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| "Conversation database key is missing.".to_string())?;

    let db_path = storage_path.join("conversation.db");

    storage::open_db(&db_path, &db_key)
        .map_err(|e| format!("Could not open conversation database: {e}"))
}

// --------------------------------------------------
// CREATE NEW CONVERSATION
// --------------------------------------------------

pub fn create_new_conversation(app_handle: &AppHandle) -> Result<Conversation, String> {
    let mut db = open_conversation_db(app_handle)?;

    storage::create_conversation(&mut db).map_err(|e| format!("Could not create conversation: {e}"))
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
pub fn delete_chat(app_handle: &AppHandle, conversation_id: &str) -> Result<(), String> {
    let db = open_conversation_db(app_handle)?;

    storage::delete_conversation(&db, conversation_id)
        .map_err(|e| format!("Could not delete conversation: {e}"))
}

pub fn get_conversations(app_handle: &AppHandle) -> Result<Vec<Conversation>, String> {
    let db = open_conversation_db(app_handle)?;

    storage::list_conversations(&db).map_err(|e| format!("Could not retrieve conversations: {e}"))
}

pub fn get_active_conversations(app_handle: &AppHandle) -> Result<Vec<Conversation>, String> {
    let db = open_conversation_db(app_handle)?;

    storage::list_active_conversations(&db)
        .map_err(|e| format!("Could not retrieve conversations: {e}"))
}

pub async fn handle_agent_request(
    prompt: &str,
    conversation_id: &str,
    attachments: &[Attachment],
    app_handle: &AppHandle,
) -> String {
    // 1. Validate conversation ID.

    if conversation_id.trim().is_empty() {
        return emit_error(app_handle, "A valid conversation ID is required.");
    }

    // 2. Open the encrypted conversation database.

    let mut db = match open_conversation_db(app_handle) {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Database initialization failed: {error}");
            return emit_error(app_handle, "Could not open conversation history.");
        }
    };

    // 3. Verify that the requested conversation exists.

    let conversations = match storage::list_conversations(&db) {
        Ok(conversations) => conversations,
        Err(error) => {
            eprintln!("Could not retrieve conversations: {error}");
            return emit_error(app_handle, "Could not retrieve conversations.");
        }
    };

    if !conversations
        .iter()
        .any(|conversation| conversation.id == conversation_id)
    {
        return emit_error(app_handle, "The requested conversation does not exist.");
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
            return emit_error(app_handle, "Could not load conversation history.");
        }
    };
    // Validate attachment references.

    for attachment in attachments {
        if attachment.id.trim().is_empty()
            || attachment.name.trim().is_empty()
            || attachment.path.trim().is_empty()
        {
            return emit_error(app_handle, "Invalid attachment information.");
        }

        let path = std::path::Path::new(&attachment.path);

        if !path.is_absolute() || !path.is_file() {
            return emit_error(
                app_handle,
                &format!("Attachment is unavailable: {}", attachment.name),
            );
        }
    }


    let mut contents: Vec<Value> = history
        .into_iter()
        .map(|message| {
            let mut text = message.content;

            if !message.attachments.is_empty() {
                text.push_str("\n\nAttachments in this message:\n");

                for attachment in &message.attachments {
                    text.push_str(&format!(
                        "- {} (Attachment Path: {})\n",
                        attachment.name, attachment.path
                    ));
                }
            }

            json!({
                "role": message.role,
                "parts": [
                    { "text": text }
                ]
            })
        })
        .collect();


    let mut user_prompt = prompt.to_string();

    if !attachments.is_empty() {
        user_prompt.push_str("\n\nAttached files:\n");

        for attachment in attachments {
            user_prompt.push_str(&format!(
                "- {} (Path: {})\n",
                attachment.name, attachment.path
            ));
        }
    }

    contents.push(json!({
        "role": "user",
        "parts": [
            { "text": user_prompt }
        ]
    }));

    let answer = match crate::agent::r#loop::run(contents, &api_key, &model, app_handle).await {
        Ok(answer) => answer,

        Err(error) => {
            eprintln!("Agent execution failed: {error}");
            let _ = save_log(
                &app_handle,
                "Error".to_string(),
                "Agent send message failed".to_string(),
                error.to_string(),
            );
            return emit_error(
                app_handle,
                "Agent execution failed. Check application logs.",
            );
        }
    };

    if answer.trim().is_empty() {
        eprintln!("Gemini returned no generated text.");

        return emit_error(app_handle, "Gemini returned an empty response.");
    }

    if let Err(error) =
        storage::save_exchange(&mut db, conversation_id, prompt, &answer, attachments)
    {
        eprintln!("Failed to save conversation exchange: {error}");

        return emit_error(app_handle, "Could not save conversation history.");
    }

    answer
}

fn emit_error(app_handle: &AppHandle, message: &str) -> String {
    let display_message = format!("Error: {message}");

    if let Err(error) = app_handle.emit("agent-token", display_message.as_str()) {
        eprintln!("Failed to emit error event: {error}");
    }

    display_message
}
