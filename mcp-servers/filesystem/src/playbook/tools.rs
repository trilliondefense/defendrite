use std::fs;
use rmcp::Json;
use uuid::Uuid;
use super::models::{CreatePlaybookArgs,GetPlaybooksResponse,
    PlaybookMetadata,PlaybookInfo,GetPlaybookContentArgs};

pub fn list_playbooks() -> Result<Json<GetPlaybooksResponse>, String> {
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



pub fn get_playbook_content(args: GetPlaybookContentArgs) -> Result<String, String> {
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

pub fn create_playbook(args: CreatePlaybookArgs) -> Result<String, String> {
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
            version: "1.1.0".into(),
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