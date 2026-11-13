use super::consts::REPORT_TEMPLATE;
use super::models::GenerateReportArgs;
use chrono::Local;
use std::fs;
use typst::foundations::{Dict, Str, Value};
use typst_as_lib::TypstEngine;
use uuid::Uuid;

pub fn generate_report_pdf(args: GenerateReportArgs) -> Result<String, String> {
    let case_id = args.case_id.trim();

    if case_id.is_empty() {
        return Err("Case ID cannot be empty".to_string());
    }

    if args.title.trim().is_empty() {
        return Err("Report title cannot be empty".to_string());
    }

    let current_date = Local::now().format("%d_%b_%Y").to_string();
    let unique_id = Uuid::new_v4();
    let short_id = &unique_id.simple().to_string()[..8];

    let report_name = format!("{}_{}", current_date, short_id);

    let base_directory = dirs::data_local_dir()
        .ok_or_else(|| "Could not resolve application data directory".to_string())?
        .join("com.defendrite.desktop-app");

    let reports_directory = base_directory.join("reports");

    fs::create_dir_all(&reports_directory)
        .map_err(|e| format!("Failed to create reports directory: {e}"))?;

    let json_payload = serde_json::to_string(&args)
        .map_err(|e| format!("Failed to serialize report data: {e}"))?;

    let mut inputs = Dict::new();

    inputs.insert(Str::from("data"), Value::Str(Str::from(json_payload)));

    let engine = TypstEngine::builder()
        .main_file(REPORT_TEMPLATE)
        .search_fonts_with(Default::default())
        .build();

    let compilation = engine.compile_with_input(inputs);

    let document = compilation
        .output
        .map_err(|err| format!("Typst compilation failed: {err:#?}"))?;
    let pdf_bytes = typst_pdf::pdf(&document, &Default::default())
        .map_err(|err| format!("PDF generation failed: {err:#?}"))?;
    let output_path = reports_directory.join(format!("{report_name}_report.pdf"));

    fs::write(&output_path, pdf_bytes).map_err(|e| format!("Failed to save PDF: {e}"))?;

    Ok(format!(
        "Report generated successfully!\n\n\
         Case ID: {}\n\
         Title: {}\n\
         Status: {}\n\
         PDF location: {}",
        case_id,
        args.title,
        args.status,
        output_path.display()
    ))
}
