use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn ask_agent(prompt: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    // 1. Pre-flight check: In Tauri v2, .store() automatically loads the file safely if it's not already in memory
    let store = app_handle
        .store("settings.json")
        .map_err(|e| format!("Error: Configuration store failed to access or initialize: {e}"))?;

    // 2. Inspect the verified store object for the credential token
    let key_is_missing = match store.get("HARNESS_MODEL_API_KEY") {
        Some(value) => match value.as_str() {
            Some(s) => s.trim().is_empty(),
            None => true,
        },
        None => true,
    };

    if key_is_missing {
        return Err("SETUP_REQUIRED: Google AI Studio API key is missing. Please add your key in the Workspace settings.".into());
    }

    // 3. Dispatch to runtime worker pool
    let result = runtime::handle_agent_request(&prompt, &app_handle).await;

    if result.starts_with("Error:") {
        return Err(result);
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask_agent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
