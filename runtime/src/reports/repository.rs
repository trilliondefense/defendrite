
use std::{
    fs,
    path::PathBuf,
    time::UNIX_EPOCH,
};

use tauri::{AppHandle, Manager};

use super::models::Report;

fn get_reports_directory(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let directory = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data directory: {e}"))?
        .join("reports");

    fs::create_dir_all(&directory)
        .map_err(|e| format!("Could not create reports directory: {e}"))?;

    Ok(directory)
}

pub fn list_reports(app_handle: &AppHandle) -> Result<Vec<Report>, String> {
    let directory = get_reports_directory(app_handle)?;

    let entries = fs::read_dir(&directory)
        .map_err(|e| format!("Failed to read reports directory: {e}"))?;

    let mut reports = Vec::new();

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let path = entry.path();

        // Only consider PDF files.
        if !path.is_file() {
            continue;
        }

        let is_pdf = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("pdf"))
            .unwrap_or(false);

        if !is_pdf {
            continue;
        }

        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        // Extract case ID from "{case_id}_report.pdf".
        let case_id = match file_name.strip_suffix("_report.pdf") {
            Some(id) => id.to_string(),
            None => continue,
        };

        let created_at = metadata
            .created()
            .or_else(|_| metadata.modified())
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or(0);

        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or(0);

        let report = Report {
            case_id,
            file_name,
            title: None,
            file_path: path.to_string_lossy().to_string(),
            file_size: metadata.len(),
            created_at,
            modified_at,
        };

        reports.push(report);
    }

    // Newest reports first.
    reports.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(reports)
}
