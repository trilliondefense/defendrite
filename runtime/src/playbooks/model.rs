
use serde::{Deserialize, Serialize};

/// Represents the lifecycle state of a playbook.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaybookStatus {
    Active,
    Draft,
    Archived,
}

/// Playbook metadata extracted from Markdown frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playbook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub file_path: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: PlaybookStatus,
}

/// Complete playbook including its Markdown body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookContent {
    pub metadata: Playbook,
    pub content: String,
}
