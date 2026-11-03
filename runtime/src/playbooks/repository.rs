
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{Playbook, PlaybookContent, PlaybookStatus};

const PLAYBOOK_EXTENSION: &str = "md";

pub struct PlaybookRepository {
    directory: PathBuf,
}

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

impl PlaybookRepository {
    pub fn new(directory: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&directory)
            .map_err(|e| format!("Failed to create playbooks directory: {e}"))?;

        Ok(Self { directory })
    }

    fn file_path(&self, id: &str) -> Result<PathBuf, String> {
        let uuid = Uuid::parse_str(id)
            .map_err(|_| "Invalid playbook ID".to_string())?;

        Ok(self.directory.join(format!("{uuid}.{PLAYBOOK_EXTENSION}")))
    }

    fn parse_markdown(
        &self,
        content: &str,
    ) -> Result<(PlaybookFrontmatter, String), String> {
        let content = content.trim_start();

        if !content.starts_with("---") {
            return Err("Invalid playbook: missing YAML frontmatter".into());
        }

        let remaining = &content[3..];

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
            .map_err(|e| format!("Failed to serialize metadata: {e}"))?;

        Ok(format!("---\n{yaml}---\n\n{body}"))
    }

    fn to_playbook(metadata: PlaybookFrontmatter, file_path: PathBuf) -> Playbook {
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

    pub fn create(
        &self,
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

        let now = Utc::now().to_rfc3339_opts(
            SecondsFormat::Secs,
            true,
        );

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

        let markdown = Self::serialize_markdown(&metadata, &content)?;

        let path = self.file_path(&id)?;

        fs::write(&path, markdown)
            .map_err(|e| format!("Failed to save playbook: {e}"))?;

        Ok(Self::to_playbook(metadata, path))
    }

    pub fn list(&self) -> Result<Vec<Playbook>, String> {
        let mut playbooks = Vec::new();

        let entries = fs::read_dir(&self.directory)
            .map_err(|e| format!("Failed to read playbooks directory: {e}"))?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().and_then(|e| e.to_str())
                != Some(PLAYBOOK_EXTENSION)
            {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read playbook: {e}"))?;

            let (metadata, _) = self.parse_markdown(&content)?;

            playbooks.push(Self::to_playbook(metadata, path));
        }

        playbooks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(playbooks)
    }

    pub fn get(&self, id: &str) -> Result<PlaybookContent, String> {
        let path = self.file_path(id)?;

        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read playbook: {e}"))?;

        let (metadata, body) = self.parse_markdown(&content)?;

        if metadata.id != id {
            return Err("Playbook ID mismatch".into());
        }

        Ok(PlaybookContent {
            metadata: Self::to_playbook(metadata, path),
            content: body,
        })
    }

    pub fn update(
        &self,
        id: &str,
        name: String,
        content: String,
    ) -> Result<Playbook, String> {
        let existing = self.get(id)?;

        let name = name.trim();

        if name.is_empty() {
            return Err("Playbook name cannot be empty".into());
        }

        if content.trim().is_empty() {
            return Err("Playbook content cannot be empty".into());
        }

        let mut metadata = PlaybookFrontmatter {
            id: existing.metadata.id,
            name: name.to_string(),
            description: existing.metadata.description,
            version: existing.metadata.version,
            author: existing.metadata.author,
            tags: existing.metadata.tags,
            status: existing.metadata.status,
            created_at: existing.metadata.created_at,
            updated_at: Utc::now().to_rfc3339_opts(
                SecondsFormat::Secs,
                true,
            ),
        };

        let markdown = Self::serialize_markdown(&metadata, &content)?;

        let path = self.file_path(id)?;

        fs::write(&path, markdown)
            .map_err(|e| format!("Failed to update playbook: {e}"))?;

        metadata.id = id.to_string();

        Ok(Self::to_playbook(metadata, path))
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let path = self.file_path(id)?;

        // Ensure the file is a valid playbook before deletion.
        self.get(id)?;

        fs::remove_file(path)
            .map_err(|e| format!("Failed to delete playbook: {e}"))
    }
}
