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
    pub project_id: String,
    pub mode_used: String,
    pub model_used: String,
    pub memory_used: bool,
    pub memory_items_used: usize,
    pub history_used: bool,
    pub history_messages_used: usize,
    pub project_state_used: bool,
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
    pub kind: Option<String>,
    pub content: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub importance: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchRequest {
    pub query: String,
    pub project_id: Option<String>,
    pub scope: Option<String>,
    pub kind: Option<String>,
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
pub struct ProjectStateUpdateRequest {
    pub current_step: Option<String>,
    pub last_commit: Option<String>,
    pub last_tag: Option<String>,
    pub summary: Option<String>,
    pub next_step: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRunRequest {
    pub name: String,
    pub input: serde_json::Value,
    pub approved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequestRequest {
    pub tool_name: String,
    pub project_id: Option<String>,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDecisionRequest {
    pub note: Option<String>,
}
