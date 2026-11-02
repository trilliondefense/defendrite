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
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = FilesystemHandler
        .serve(stdio())
        .await?;

    service.waiting().await?;

    Ok(())
}