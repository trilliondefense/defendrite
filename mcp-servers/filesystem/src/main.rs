use rmcp::{
    handler::server::wrapper::Parameters,
    schemars,
    tool,
    tool_router,
    ServiceExt,
    transport::stdio,
};

use std::fs;
use std::path::Path;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UnzipArgs {
    /// Absolute path to the ZIP archive.
    zip_path: String,

    /// Absolute destination directory.
    output_dir: String,
}

#[derive(Clone)]
struct FilesystemHandler;

#[tool_router(server_handler)]
impl FilesystemHandler {
    /// Extracts a ZIP archive into a destination directory.
    #[tool(description = "Extracts a local ZIP archive into a target destination directory.")]
    fn unzip_file(
        &self,
        Parameters(args): Parameters<UnzipArgs>,
    ) -> Result<String, String> {

        // Open the ZIP archive.
        let zip_file = fs::File::open(&args.zip_path)
            .map_err(|e| format!("Failed to open ZIP archive: {}", e))?;

        let mut archive = zip::ZipArchive::new(zip_file)
            .map_err(|e| format!("Invalid ZIP archive: {}", e))?;

        let output_path = Path::new(&args.output_dir);

        // Create destination directory if it doesn't exist.
        fs::create_dir_all(output_path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        // Extract archive entries.
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read ZIP entry: {}", e))?;

            // Prevent ZIP Slip path traversal.
            let enclosed_name = file
                .enclosed_name()
                .ok_or_else(|| {
                    format!("Blocked unsafe archive entry: {:?}", file.name())
                })?
                .to_owned();

            let file_out_path = output_path.join(enclosed_name);

            // Use is_dir() instead of checking file.name().ends_with('/').
            if file.is_dir() {
                fs::create_dir_all(&file_out_path)
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            } else {
                // Create parent directories.
                if let Some(parent) = file_out_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }

                // Write extracted file.
                let mut outfile = fs::File::create(&file_out_path)
                    .map_err(|e| format!("Failed to create output file: {}", e))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file: {}", e))?;
            }
        }

        Ok(format!(
            "Successfully extracted ZIP archive into: {}",
            args.output_dir
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = FilesystemHandler
        .serve(stdio())
        .await?;

    service.waiting().await?;

    Ok(())
}