use crate::{app_state::AppState, error::AppResult, logs::audit::AuditLog};
use axum::{extract::State, Json};
use serde_json::json;

pub async fn recent_logs(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let logs = AuditLog::new(state.db.clone()).recent(100)?;
    Ok(Json(json!({"ok": true, "logs": logs})))
}
