use serde_json::Value;

pub struct AgentContext {
    pub conversation_id: String,
    pub contents: Vec<Value>,
}