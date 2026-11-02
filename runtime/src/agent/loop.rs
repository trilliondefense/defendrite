use super::context::AgentContext;

pub async fn run(
    context: AgentContext,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {

    // 1. Send conversation to LLM.
    // 2. Inspect response.
    // 3. If tool call exists, execute through MCP.
    // 4. Send tool result back to LLM.
    // 5. Repeat until final response.

    let _ = context;

    Ok(String::new())
}