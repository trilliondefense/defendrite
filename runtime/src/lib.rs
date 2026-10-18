use models::run_hf_inference;
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, Emitter}; // Added Emitter for event broadcasting

pub async fn handle_agent_request(prompt: &str, app_handle: &tauri::AppHandle) -> String {
    let mut storage_path = app_handle.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("./"));
    storage_path.push("storage");
    let _ = fs::create_dir_all(&storage_path);
    
    // Simulate streaming by emitting chunks over the Tauri IPC channel
    let words = vec!["Agent", " Loop", " Stream:", " Processing", " your", " request", " live..."];
    for word in words {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        // Broadcast the specific token chunk directly to the window listener
        let _ = app_handle.emit("agent-token", word);
    }

    // Return final confirmation token
    " [Done]".to_string()
}
