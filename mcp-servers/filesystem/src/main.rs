use rmcp::handler::server::wrapper::Json;
use rmcp::{
    ServiceExt, handler::server::wrapper::Parameters, schemars, serde::Serialize, tool,
    tool_router, transport::stdio,
};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UnzipArgs {
    /// Absolute path to the ZIP archive.
    zip_path: String,

    /// Absolute destination directory.
    output_dir: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaybookMetadata {
    id: String,
    name: String,
    description: String,
    version: String,
    author: String,
    tags: Vec<String>,
    status: String,
    created_at: String,
    updated_at: String,
}
#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GetPlaybookContentArgs {
    /// Playbook id.
    pub id: String,
}

#[derive(serde::Serialize, schemars::JsonSchema)]
pub struct GetPlaybooksResponse {
    pub playbooks: Vec<PlaybookInfo>,
}

#[derive(serde::Serialize, schemars::JsonSchema)]
pub struct PlaybookInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub updated_at: String,
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
    #[tool(description = "Creates a new Markdown playbook with metadata.")]
    fn create_playbook(
        &self,
        Parameters(args): Parameters<CreatePlaybookArgs>,
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

        log("========== CREATE PLAYBOOK REQUEST ==========");
        // 1. Validate input
        let name = args.name.trim();

        if name.is_empty() {
            return Err("Playbook name cannot be empty".into());
        }

        if args.content.trim().is_empty() {
            return Err("Playbook content cannot be empty".into());
        }

        // 2. Resolve playbooks directory
        let base_directory = dirs::data_local_dir()
            .ok_or("Could not resolve application data directory")?
            .join("com.defendrite.desktop-app")
            .join("playbooks");

        fs::create_dir_all(&base_directory)
            .map_err(|e| format!("Could not create playbooks directory: {e}"))?;

        let base_directory = base_directory
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize playbooks directory: {e}"))?;

        // 3. Generate metadata
        let id = Uuid::new_v4().to_string();

        let now = chrono::Utc::now().to_rfc3339();

        let metadata = PlaybookMetadata {
            id: id.clone(),
            name: name.to_string(),
            description: String::new(),
            version: "1.0.0".into(),
            author: "Agent".into(),
            tags: Vec::new(),
            status: "active".into(),
            created_at: now.clone(),
            updated_at: now,
        };

        // 4. Serialize metadata into YAML frontmatter
        let frontmatter = serde_yaml::to_string(&metadata)
            .map_err(|e| format!("Could not serialize playbook metadata: {e}"))?;

        let markdown = format!("---\n{}---\n\n{}", frontmatter, args.content.trim());

        // 5. Generate UUID-based filename
        let target_path = base_directory.join(format!("{}.md", id));

        if !target_path.starts_with(&base_directory) {
            return Err("Security Violation: Invalid playbook path.".into());
        }

        // 6. Save playbook
        fs::write(&target_path, markdown).map_err(|e| format!("Could not create playbook: {e}"))?;

        log(&format!("Saved file to path: {:?}", target_path));

        Ok(format!(
            "Playbook '{}' created successfully. ID: {}",
            name, id
        ))
    }

    #[tool(
        description = "Retrieves a list of all existing Markdown playbooks with their metadata."
    )]
    fn list_playbooks(&self) -> Result<Json<GetPlaybooksResponse>, String> {
        let directory = dirs::data_local_dir()
            .ok_or("Could not resolve application data directory")?
            .join("com.defendrite.desktop-app")
            .join("playbooks");

        let entries = fs::read_dir(directory)
            .map_err(|e| format!("Could not read playbooks directory: {e}"))?;

        let mut playbooks = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| format!("Could not read directory entry: {e}"))?;

            let path = entry.path();

            // Only process Markdown files
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Could not read playbook file: {e}"))?;

            // Extract YAML frontmatter
            let content = content.trim_start();

            let remaining = content.strip_prefix("---").ok_or_else(|| {
                format!(
                    "Invalid playbook '{}': missing YAML frontmatter",
                    path.display()
                )
            })?;

            let end = remaining.find("\n---").ok_or_else(|| {
                format!(
                    "Invalid playbook '{}': unterminated frontmatter",
                    path.display()
                )
            })?;

            let yaml = &remaining[..end];

            let metadata: PlaybookMetadata = serde_yaml::from_str(yaml)
                .map_err(|e| format!("Invalid playbook metadata in '{}': {e}", path.display()))?;

            playbooks.push(PlaybookInfo {
                id: metadata.id,
                name: metadata.name,
                description: metadata.description,
                tags: metadata.tags,
                updated_at: metadata.updated_at,
            });
        }

        // Sort alphabetically by title
        playbooks.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));

        Ok(Json(GetPlaybooksResponse { playbooks }))
    }
    #[tool(description = "Retrieves the full Markdown content of a playbook using its ID.")]
    fn get_playbook_content(
        &self,
        Parameters(args): Parameters<GetPlaybookContentArgs>,
    ) -> Result<String, String> {
        use std::fs;

        let directory = dirs::data_local_dir()
            .ok_or("Could not resolve application data directory")?
            .join("com.defendrite.desktop-app")
            .join("playbooks");

        let entries = fs::read_dir(directory)
            .map_err(|e| format!("Could not read playbooks directory: {e}"))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Could not read directory entry: {e}"))?;

            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }

            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Could not read playbook file: {e}"))?;

            let trimmed = content.trim_start();

            let remaining = trimmed.strip_prefix("---").ok_or_else(|| {
                format!(
                    "Invalid playbook '{}': missing YAML frontmatter",
                    path.display()
                )
            })?;

            let end = remaining.find("\n---").ok_or_else(|| {
                format!(
                    "Invalid playbook '{}': unterminated frontmatter",
                    path.display()
                )
            })?;

            let yaml = &remaining[..end];

            let metadata: PlaybookMetadata = serde_yaml::from_str(yaml)
                .map_err(|e| format!("Invalid playbook metadata: {e}"))?;

            if metadata.id == args.id {
                let body_start = end + 4;
                let body = remaining[body_start..].trim_start();

                return Ok(body.to_string());
            }
        }

        Err(format!("Playbook with ID '{}' not found", args.id))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = FilesystemHandler.serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}
