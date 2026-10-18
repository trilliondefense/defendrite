
use std::fs;
use std::path::PathBuf;
use tauri::{Emitter, Manager};

pub async fn handle_agent_request(
    prompt: &str,
    app_handle: &tauri::AppHandle,
) -> String {
    // Resolve the application-specific data directory.
    let mut storage_path = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("./"));

    storage_path.push("storage");

    // Ensure the storage directory exists.
    if let Err(error) = fs::create_dir_all(&storage_path) {
        eprintln!("Failed to create storage directory: {error}");
    }

    // TODO: Replace mock chunks with actual model-generated tokens.
    let chunks = [
        "Agent",
        " Loop",
        " Stream:",
        " Processing",
        " your",
        " request",
        " live...",
        " [Done]",
    ];

    for chunk in chunks {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        if let Err(error) = app_handle.emit("agent-token", chunk) {
            eprintln!("Failed to emit agent token: {error}");
        }
    }

    // Return the completed response.
    format!(
        "Processed prompt ({} characters).",
        prompt.chars().count()
    )
}
