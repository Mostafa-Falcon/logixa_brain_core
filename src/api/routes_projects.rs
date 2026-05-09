use crate::{
    app_state::AppState,
    dto::{ProjectStateUpdateRequest, ProjectUpsertRequest},
    error::AppResult,
    projects::project_manager::ProjectManager,
};
use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;

pub async fn list_projects(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    let active_project_id = manager.active_project_id().ok();
    Ok(Json(json!({
        "ok": true,
        "active_project_id": active_project_id,
        "projects": manager.list()?
    })))
}

pub async fn active_project(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    let project = manager.active_project()?;
    let state = manager.get_state(&project.id).ok();
    Ok(Json(json!({
        "ok": true,
        "project": project,
        "state": state
    })))
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

pub async fn get_project_state(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    let project = manager.get(&id)?;
    let project_state = manager.get_state(&id).ok();
    Ok(Json(json!({
        "ok": true,
        "project": project,
        "state": project_state
    })))
}

pub async fn update_project_state(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ProjectStateUpdateRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let manager = ProjectManager::new(state.db.clone());
    let project_state = manager.update_state(&id, req)?;
    Ok(Json(json!({"ok": true, "state": project_state})))
}
