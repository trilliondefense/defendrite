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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreatePlaybookArgs {
    /// Playbook title.
    name: String,

    /// Markdown instructions for the playbook.
    content: String,
}

#[derive(Clone)]
struct FilesystemHandler;
#[tool_router(server_handler)]
impl FilesystemHandler {
    #[tool(description = "Extracts a local ZIP archive into a target destination directory.")]
    fn unzip_file(
        &self,
        Parameters(args): Parameters<UnzipArgs>,
    ) -> Result<String, String> {

        let log_path = "/tmp/filesystem-mcp.log";

        let log = |message: &str| {
            use std::io::Write;

            if let Ok(mut file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
            {
                let _ = writeln!(file, "{message}");
            }
        };

        log("========== UNZIP REQUEST ==========");
        log(&format!("ZIP path: {}", args.zip_path));
        log(&format!("Output path: {}", args.output_dir));
        log(&format!("Current directory: {:?}", std::env::current_dir()));

        let zip_path = Path::new(&args.zip_path);

        log(&format!("ZIP exists: {}", zip_path.exists()));
        log(&format!("ZIP is file: {}", zip_path.is_file()));

        let zip_file = match fs::File::open(zip_path) {
            Ok(file) => file,
            Err(error) => {
                log(&format!("OPEN ERROR: {error}"));
                return Err(format!("Failed to open ZIP archive: {error}"));
            }
        };

        log("ZIP opened successfully");

        let mut archive = match zip::ZipArchive::new(zip_file) {
            Ok(archive) => archive,
            Err(error) => {
                log(&format!("ZIP ERROR: {error}"));
                return Err(format!("Invalid ZIP archive: {error}"));
            }
        };

        log(&format!("Archive entries: {}", archive.len()));

        let output_path = Path::new(&args.output_dir);

        if let Err(error) = fs::create_dir_all(output_path) {
            log(&format!("OUTPUT DIRECTORY ERROR: {error}"));
            return Err(format!("Failed to create output directory: {error}"));
        }

        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(file) => file,
                Err(error) => {
                    log(&format!("ENTRY ERROR: {error}"));
                    return Err(format!("Failed to read ZIP entry: {error}"));
                }
            };

            log(&format!("Extracting: {:?}", file.name()));

            let enclosed_name = match file.enclosed_name() {
                Some(name) => name.to_owned(),
                None => {
                    let error = format!("Blocked unsafe archive entry: {:?}", file.name());
                    log(&error);
                    return Err(error);
                }
            };

            let file_out_path = output_path.join(enclosed_name);

            if file.is_dir() {
                fs::create_dir_all(&file_out_path)
                    .map_err(|e| format!("Directory creation failed: {e}"))?;
            } else {
                if let Some(parent) = file_out_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Parent creation failed: {e}"))?;
                }

                let mut outfile = fs::File::create(&file_out_path)
                    .map_err(|e| format!("File creation failed: {e}"))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Extraction failed: {e}"))?;
            }
        }

        log("EXTRACTION SUCCESS");

        Ok(format!(
            "Successfully extracted ZIP archive into: {}",
            args.output_dir
        ))
    }
  #[tool(description = "Creates a new Markdown playbook for the AI agent.")]
fn create_playbook(
    &self,
    Parameters(args): Parameters<CreatePlaybookArgs>,
) -> Result<String, String> {
    // 1. Resolve the base directory safely using standard dirs
    let base_directory = dirs::data_local_dir()
        .ok_or("Could not resolve application data directory")?
        .join("Defendrite")
        .join("playbooks");

    // Ensure the base directory exists
    fs::create_dir_all(&base_directory)
        .map_err(|e| format!("Could not create playbooks directory: {e}"))?;

    // 2. Canonicalize the base directory to resolve any symlinks for strict comparison
    let base_directory_canonical = base_directory
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize base directory: {e}"))?;

    // 3. Prevent arbitrary filenames/extensions (Use UUID to completely decouple user input from file path)
    let safe_filename = format!("{}.md", uuid::Uuid::new_v4());

    // 4. Join paths and canonicalize the final target path
    let target_path = base_directory_canonical.join(safe_filename);

    // 5. Explicitly verify the final path is strictly inside the sandbox directory
    if !target_path.starts_with(&base_directory_canonical) {
        return Err("Security Violation: Target path is outside the designated playbooks directory.".to_string());
    }

    // 6. Safely write the content to disk
    fs::write(&target_path, args.content)
        .map_err(|e| format!("Could not create playbook: {e}"))?;

    Ok(format!(
        "Playbook '{}' created successfully.",
        args.name
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