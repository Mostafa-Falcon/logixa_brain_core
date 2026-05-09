use crate::{
    app_state::AppState, dto::ProjectUpsertRequest, error::AppResult,
    projects::project_manager::ProjectManager,
};
use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

pub async fn list_projects(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    Ok(Json(json!({"ok": true, "projects": manager.list()?})))
}

pub async fn upsert_project(
    State(state): State<AppState>,
    Json(req): Json<ProjectUpsertRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    manager.upsert(req)?;
    Ok(Json(json!({"ok": true})))
}

pub async fn activate_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    manager.activate(&id)?;
    Ok(Json(json!({"ok": true, "active_project_id": id})))
}
