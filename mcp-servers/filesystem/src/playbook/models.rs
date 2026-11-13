use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(serde::Deserialize, JsonSchema)]
pub struct GetPlaybookContentArgs {
    /// Playbook id.
    pub id: String,
}

#[derive(serde::Serialize, JsonSchema)]
pub struct GetPlaybooksResponse {
    pub playbooks: Vec<PlaybookInfo>,
}

#[derive(serde::Serialize, JsonSchema)]
pub struct PlaybookInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub updated_at: String,
}

#[derive(Debug, serde::Deserialize, JsonSchema)]
pub struct CreatePlaybookArgs {
    /// Playbook title.
    pub name: String,

    /// Markdown instructions for the playbook.
    pub content: String,
}
