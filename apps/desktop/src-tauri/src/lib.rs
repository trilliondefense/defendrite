use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn ask_agent(prompt: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    // 1. Pre-flight check: Verify the API key exists before triggering the runtime agent
    if let Some(store) = app_handle.get_store("settings.json") {
        let key_is_missing = match store.get("HARNESS_MODEL_API_KEY") {
            Some(value) => match value.as_str() {
                Some(s) => s.trim().is_empty(),
                None => true,
            },
            None => true,
        };

        if key_is_missing {
            // This error matches the Result type and goes straight to the frontend's catch block
            return Err("SETUP_REQUIRED: Google AI Studio API key is missing. Please add your key in the Workspace settings.".into());
        }
    } else {
        return Err("Error: Configuration store failed to initialize.".into());
    }

    // 2. Key is present; proceed to run the background agent execution loop
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
