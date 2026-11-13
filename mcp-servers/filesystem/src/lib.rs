use rmcp::handler::server::wrapper::Json;
use rmcp::{
    ServiceExt, handler::server::wrapper::Parameters, tool,
    tool_router, transport::stdio,
};
mod report;
mod playbook;
mod file;

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
    fn unzip_file(&self, Parameters(args): Parameters<file::models::UnzipArgs>) -> Result<String, String> {
        file::unzip_file(args)
    }
  
    #[tool(
        description = "Validate a local file and generate a lightweight reference ticket ('local-file://...'). Pass this exact reference string directly into any tool argument expecting file contents, media data, or Base64-encoded bytes—the host runtime automatically resolves and encodes the file during execution."
    )]
    fn create_file_ticket(
        &self,
        Parameters(args): Parameters<file::models::FileTicketArgs>,
    ) -> Result<String, String> {
        file::create_file_ticket(args)
    }

    #[tool(description = "Read any file by converting it to markdown")]
    async fn read_any_file(
        &self,
        Parameters(args): Parameters<file::models::ConvertArgs>,
    ) -> Result<String, String> {
        file::read_file(args)
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
