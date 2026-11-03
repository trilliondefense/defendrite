
pub mod model;
pub mod repository;

use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub use model::{Playbook, PlaybookContent, PlaybookStatus};

use repository::PlaybookRepository;

/// Initialize the playbook repository.
fn get_repository(
    app_handle: &AppHandle,
) -> Result<PlaybookRepository, String> {
    let directory: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data directory: {e}"))?
        .join("playbooks");

    PlaybookRepository::new(directory)
}

/// Create a new Markdown playbook.
pub fn create_new_playbook(
    app_handle: &AppHandle,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    get_repository(app_handle)?.create(name, content)
}

/// Retrieve all playbooks.
pub fn get_playbooks(
    app_handle: &AppHandle,
) -> Result<Vec<Playbook>, String> {
    get_repository(app_handle)?.list()
}

/// Retrieve a playbook and its Markdown content.
pub fn get_playbook(
    app_handle: &AppHandle,
    id: &str,
) -> Result<PlaybookContent, String> {
    get_repository(app_handle)?.get(id)
}

/// Update an existing playbook.
pub fn update_playbook(
    app_handle: &AppHandle,
    id: &str,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    get_repository(app_handle)?.update(id, name, content)
}

/// Delete a playbook.
pub fn delete_playbook(
    app_handle: &AppHandle,
    id: &str,
) -> Result<(), String> {
    get_repository(app_handle)?.delete(id)
}
