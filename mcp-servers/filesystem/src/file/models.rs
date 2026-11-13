use serde::{Deserialize,Serialize};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnzipArgs {
    /// Absolute path to the ZIP archive.
    pub zip_path: String,

    /// Absolute destination directory.
    pub output_dir: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ConvertArgs {
    /// The absolute path to the local file (e.g. .docx, .pptx, .xlsx, .html, .csv) to convert.
    pub file_path: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FileTicketArgs {
    /// Path to the local file (image, audio, video, document, etc.).
    pub file_path: String,
}
