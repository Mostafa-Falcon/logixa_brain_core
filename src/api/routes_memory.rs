use crate::{
    app_state::AppState,
    dto::{MemorySaveRequest, MemorySearchRequest},
    error::AppResult,
    memory::memory_manager::MemoryManager,
};
use axum::{extract::State, Json};
use serde_json::json;

pub async fn save_memory(
    State(state): State<AppState>,
    Json(req): Json<MemorySaveRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = MemoryManager::new(state.db.clone());
    let item = manager.save_memory(req)?;
    Ok(Json(json!({"ok": true, "id": item.id, "item": item})))
}

pub async fn search_memory(
    State(state): State<AppState>,
    Json(req): Json<MemorySearchRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = MemoryManager::new(state.db.clone());
    let items = manager.search(req)?;
    Ok(Json(
        json!({"ok": true, "count": items.len(), "items": items}),
    ))
}
