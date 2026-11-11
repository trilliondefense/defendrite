
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    /// Unique investigation case ID.
    pub case_id: String,

    /// Report filename.
    pub file_name: String,

    /// Report title (optional for now).
    pub title: Option<String>,

    /// Absolute path to the PDF file.
    pub file_path: String,

    /// File size in bytes.
    pub file_size: u64,

    /// File creation time (Unix timestamp).
    pub created_at: i64,

    /// File modification time (Unix timestamp).
    pub modified_at: i64,
}
