use anydoc::to_markdown_bytes;
use rmcp::handler::server::wrapper::Json;
use rmcp::{
    ServiceExt, handler::server::wrapper::Parameters, schemars, serde::Serialize, tool,
    tool_router, transport::stdio,
};
use serde::Deserialize;
use std::fs;
use std::path::Path;
mod report;
mod playbook;


#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UnzipArgs {
    /// Absolute path to the ZIP archive.
    zip_path: String,

    /// Absolute destination directory.
    output_dir: String,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
pub struct ConvertArgs {
    /// The absolute path to the local file (e.g. .docx, .pptx, .xlsx, .html, .csv) to convert.
    pub file_path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FileTicketArgs {
    /// Path to the local file (image, audio, video, document, etc.).
    pub file_path: String,
}

#[derive(Clone)]
struct FilesystemHandler;
#[tool_router(server_handler)]
impl FilesystemHandler {
    #[tool(
        description = "Generate a professional investigation report in PDF format. Provide case ID, title, status, executive summary, findings and recommendations. Separate multiple findings and recommendations using new lines."
    )]
    fn generate_report(
        &self,
        Parameters(args): Parameters<report::models::GenerateReportArgs>,
    ) -> Result<String, String> {
        report::generate_report_pdf(args)
    }
    #[tool(description = "Extracts a local ZIP archive into a target destination directory.")]
    fn unzip_file(&self, Parameters(args): Parameters<UnzipArgs>) -> Result<String, String> {
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
  
    #[tool(
        description = "Validate a local file and generate a lightweight reference ticket ('local-file://...'). Pass this exact reference string directly into any tool argument expecting file contents, media data, or Base64-encoded bytes—the host runtime automatically resolves and encodes the file during execution."
    )]
    fn create_file_ticket(
        &self,
        Parameters(args): Parameters<FileTicketArgs>,
    ) -> Result<String, String> {
        let path = std::path::Path::new(&args.file_path);

        // Ensure the file exists and is a regular file (not a directory)
        let metadata = fs::metadata(path)
            .map_err(|e| format!("Failed to access file '{}': {}", args.file_path, e))?;

        if !metadata.is_file() {
            return Err("Provided path is not a valid file.".to_string());
        }

        if metadata.len() == 0 {
            return Err("File is empty.".to_string());
        }

        // Enforce maximum file size (e.g., 50 MB for images, audio, video, or docs)
        const MAX_FILE_SIZE: u64 = 50 * 1024 * 1024;

        if metadata.len() > MAX_FILE_SIZE {
            return Err("File exceeds the 50 MB file size limit.".to_string());
        }

        // Canonicalize to resolve relative paths (../) and get a clean absolute path
        let canonical_path = fs::canonicalize(path)
            .map_err(|e| format!("Failed to resolve absolute file path: {}", e))?;

        let path_str = canonical_path
            .to_str()
            .ok_or_else(|| "File path contains invalid UTF-8 characters.".to_string())?;

        // On Windows, fs::canonicalize prepends "\\?\", which we strip for clean URIs
        let clean_path = path_str.strip_prefix(r"\\?\").unwrap_or(path_str);

        Ok(format!("local-file://{}", clean_path))
    }

    #[tool(description = "Read any file by converting it to markdown")]
    async fn read_any_file(
        &self,
        Parameters(args): Parameters<ConvertArgs>,
    ) -> Result<String, String> {
        {
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
            log("========== CONVERT DOCUMENT REQUEST ==========");

            let path = Path::new(&args.file_path);

            if !path.exists() {
                log(&format!(
                    "The specified file path could not be found {:?}",
                    path
                ));
                return Err("The specified file path could not be found:".to_string());
            }
            // Read the file bytes directly (Anydoc parses by checking magic bytes)
            let file_bytes = match std::fs::read(&args.file_path) {
                Ok(bytes) => bytes,
                Err(e) => return Err(format!("Failed to read file bytes: {}", e)),
            };

            // Convert the bytes natively - handles PDFs effortlessly
            match to_markdown_bytes(&file_bytes, None) {
                Ok(markdown_output) => {
                    log(&format!("Sucessful {:?}", markdown_output));
                    Ok(markdown_output)
                }
                Err(e) => {
                    log(&format!("Failed to parse document {:?}", e));

                    Err(format!("Failed to parse document: {}", e))
                }
            }
        }
    }

    #[tool(description = "Creates a new Markdown playbook with metadata.")]
    fn create_playbook(
        &self,
        Parameters(args): Parameters<playbook::models::CreatePlaybookArgs>,
    ) -> Result<String, String> {
       playbook::create_playbook(args)
    }

    #[tool(
        description = "Retrieves a list of all existing Markdown playbooks with their metadata."
    )]
    fn list_playbooks(&self) -> Result<Json<playbook::models::GetPlaybooksResponse>, String> {
        playbook::list_playbooks()
    }
    
    #[tool(description = "Retrieves the full Markdown content of a playbook using its ID.")]
    fn get_playbook_content(
        &self,
        Parameters(args): Parameters<playbook::models::GetPlaybookContentArgs>,
    ) -> Result<String, String> {
        playbook::get_playbook_content(args)
    }

}


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let service = FilesystemHandler.serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}
