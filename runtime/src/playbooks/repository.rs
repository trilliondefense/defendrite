
use std::fs;
use std::path::PathBuf;

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use super::models::{
    Playbook,
    PlaybookContent,
    PlaybookStatus,
};

// --------------------------------------------------
// Internal Metadata
// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaybookFrontmatter {
    id: String,
    name: String,
    description: String,
    version: String,

    #[serde(default)]
    author: Option<String>,

    #[serde(default)]
    tags: Vec<String>,

    #[serde(default = "default_status")]
    status: PlaybookStatus,

    created_at: String,
    updated_at: String,
}

fn default_status() -> PlaybookStatus {
    PlaybookStatus::Draft
}

// --------------------------------------------------
// Storage Helpers
// --------------------------------------------------

fn get_playbooks_directory(
    app_handle: &AppHandle,
) -> Result<PathBuf, String> {
    let directory = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data directory: {e}"))?
        .join("playbooks");

    fs::create_dir_all(&directory)
        .map_err(|e| format!("Could not create playbooks directory: {e}"))?;

    Ok(directory)
}

fn get_playbook_path(
    app_handle: &AppHandle,
    id: &str,
) -> Result<PathBuf, String> {
    let uuid = Uuid::parse_str(id)
        .map_err(|_| "Invalid playbook ID".to_string())?;

    Ok(get_playbooks_directory(app_handle)?
        .join(format!("{uuid}.md")))
}

fn current_timestamp() -> String {
    Utc::now().to_rfc3339_opts(
        SecondsFormat::Secs,
        true,
    )
}

// --------------------------------------------------
// Markdown Helpers
// --------------------------------------------------

fn parse_markdown(
    content: &str,
) -> Result<(PlaybookFrontmatter, String), String> {
    let content = content.trim_start();

    let remaining = content
        .strip_prefix("---")
        .ok_or("Invalid playbook: missing YAML frontmatter")?;

    let end = remaining
        .find("\n---")
        .ok_or("Invalid playbook: unterminated frontmatter")?;

    let yaml = &remaining[..end];

    let body_start = end + 4;
    let body = remaining[body_start..].trim_start();

    let metadata: PlaybookFrontmatter =
        serde_yaml::from_str(yaml)
            .map_err(|e| format!("Invalid playbook metadata: {e}"))?;

    Ok((metadata, body.to_string()))
}

fn serialize_markdown(
    metadata: &PlaybookFrontmatter,
    body: &str,
) -> Result<String, String> {
    let yaml = serde_yaml::to_string(metadata)
        .map_err(|e| format!("Could not serialize playbook metadata: {e}"))?;

    Ok(format!("---\n{yaml}---\n\n{body}"))
}

fn to_playbook(
    metadata: PlaybookFrontmatter,
    file_path: PathBuf,
) -> Playbook {
    Playbook {
        id: metadata.id,
        name: metadata.name,
        description: metadata.description,
        version: metadata.version,
        author: metadata.author,
        tags: metadata.tags,
        file_path: file_path.to_string_lossy().into_owned(),
        created_at: metadata.created_at,
        updated_at: metadata.updated_at,
        status: metadata.status,
    }
}

// --------------------------------------------------
// Create Playbook
// --------------------------------------------------

pub fn create_playbook(
    app_handle: &AppHandle,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    let name = name.trim();

    if name.is_empty() {
        return Err("Playbook name cannot be empty".into());
    }

    if content.trim().is_empty() {
        return Err("Playbook content cannot be empty".into());
    }

    let id = Uuid::new_v4().to_string();
    let now = current_timestamp();

    let metadata = PlaybookFrontmatter {
        id: id.clone(),
        name: name.to_string(),
        description: String::new(),
        version: "1.0.0".into(),
        author: Some("Local".into()),
        tags: Vec::new(),
        status: PlaybookStatus::Active,
        created_at: now.clone(),
        updated_at: now,
    };

    let markdown = serialize_markdown(&metadata, &content)?;

    let path = get_playbook_path(app_handle, &id)?;

    fs::write(&path, markdown)
        .map_err(|e| format!("Could not save playbook: {e}"))?;

    Ok(to_playbook(metadata, path))
}

// --------------------------------------------------
// List Playbooks
// --------------------------------------------------

pub fn list_playbooks(
    app_handle: &AppHandle,
) -> Result<Vec<Playbook>, String> {
    let directory = get_playbooks_directory(app_handle)?;

    let entries = fs::read_dir(&directory)
        .map_err(|e| format!("Could not read playbooks directory: {e}"))?;

    let mut playbooks = Vec::new();

    for entry in entries {
        let entry = entry
            .map_err(|e| format!("Could not read directory entry: {e}"))?;

        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Could not read playbook file: {e}"))?;

        let (metadata, _) = parse_markdown(&content)?;

        playbooks.push(to_playbook(metadata, path));
    }

    playbooks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(playbooks)
}

// --------------------------------------------------
// Get Playbook
// --------------------------------------------------

pub fn get_playbook(
    app_handle: &AppHandle,
    id: &str,
) -> Result<PlaybookContent, String> {
    let path = get_playbook_path(app_handle, id)?;

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Could not read playbook: {e}"))?;

    let (metadata, body) = parse_markdown(&content)?;

    if metadata.id != id {
        return Err("Playbook ID mismatch".into());
    }

    Ok(PlaybookContent {
        metadata: to_playbook(metadata, path),
        content: body,
    })
}

// --------------------------------------------------
// Update Playbook
// --------------------------------------------------

pub fn update_playbook(
    app_handle: &AppHandle,
    id: &str,
    name: String,
    content: String,
) -> Result<Playbook, String> {
    let name = name.trim();

    if name.is_empty() {
        return Err("Playbook name cannot be empty".into());
    }

    if content.trim().is_empty() {
        return Err("Playbook content cannot be empty".into());
    }

    let existing = get_playbook(app_handle, id)?;

    let metadata = PlaybookFrontmatter {
        id: existing.metadata.id,
        name: name.to_string(),
        description: existing.metadata.description,
        version: existing.metadata.version,
        author: existing.metadata.author,
        tags: existing.metadata.tags,
        status: existing.metadata.status,
        created_at: existing.metadata.created_at,
        updated_at: current_timestamp(),
    };

    let markdown = serialize_markdown(&metadata, &content)?;

    let path = get_playbook_path(app_handle, id)?;

    fs::write(&path, markdown)
        .map_err(|e| format!("Could not update playbook: {e}"))?;

    Ok(to_playbook(metadata, path))
}

// --------------------------------------------------
// Delete Playbook
// --------------------------------------------------

pub fn delete_playbook(
    app_handle: &AppHandle,
    id: &str,
) -> Result<(), String> {
    // Verify that the playbook exists and its ID matches.
    get_playbook(app_handle, id)?;

    let path = get_playbook_path(app_handle, id)?;

    fs::remove_file(&path)
        .map_err(|e| format!("Could not delete playbook: {e}"))?;

    Ok(())
}

pub fn export_playbook(
    app_handle: &AppHandle,
    id: &str,
    destination: PathBuf,
) -> Result<(), String> {
    let playbook = get_playbook(app_handle, id)?;

    let metadata = PlaybookFrontmatter {
        id: playbook.metadata.id.clone(),
        name: playbook.metadata.name.clone(),
        description: playbook.metadata.description.clone(),
        version: playbook.metadata.version.clone(),
        author: playbook.metadata.author.clone(),
        tags: playbook.metadata.tags.clone(),
        status: playbook.metadata.status.clone(),
        created_at: playbook.metadata.created_at.clone(),
        updated_at: playbook.metadata.updated_at.clone(),
    };

    let markdown = serialize_markdown(&metadata, &playbook.content)?;

    fs::write(&destination, markdown)
        .map_err(|e| format!("Could not export playbook: {e}"))?;

    Ok(())
}