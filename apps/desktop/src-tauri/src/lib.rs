
#[tauri::command]
async fn ask_agent(
    prompt: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let result =
        runtime::handle_agent_request(&prompt, &app_handle).await;

    if result.starts_with("Error:") {
        return Err(result);
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ask_agent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
