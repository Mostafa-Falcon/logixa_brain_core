use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
    pub project_id: Option<String>,
    pub mode: Option<String>,
    pub use_memory: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub answer: String,
    pub conversation_id: String,
    pub mode_used: String,
    pub model_used: String,
    pub memory_used: bool,
    pub events: Vec<String>,
    pub tool_calls: Vec<String>,
    pub mocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub ok: bool,
    pub service: String,
    pub version: String,
    pub database_ready: bool,
    pub model_status: String,
    pub active_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySaveRequest {
    pub scope: String,
    pub project_id: Option<String>,
    pub kind: String,
    pub content: String,
    pub tags: Option<String>,
    pub importance: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchRequest {
    pub query: String,
    pub project_id: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpsertRequest {
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub kind: Option<String>,
    pub rules: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRunRequest {
    pub name: String,
    pub input: serde_json::Value,
    pub approved: Option<bool>,
}
