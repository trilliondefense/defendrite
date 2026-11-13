use std::fs;
use std::path::Path;
use anydoc::to_markdown_bytes;
use super::models::{UnzipArgs,FileTicketArgs,ConvertArgs};


pub fn read_file(args: ConvertArgs) -> Result<String, String> {
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

pub fn create_file_ticket(args: FileTicketArgs) -> Result<String, String> {
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

pub fn unzip_file(args: UnzipArgs) -> Result<String, String> {
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
  