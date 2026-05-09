use crate::{app_state::AppState, error::AppResult};
use axum::{extract::State, Json};
use serde_json::json;

pub async fn model_status(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(json!({
        "ok": true,
        "status": state.runtime.status_label().await,
        "details": state.runtime.status().await
    })))
}

pub async fn model_start(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let result = state.runtime.ensure_ready().await?;
    Ok(Json(json!({"ok": true, "result": result})))
}

pub async fn model_stop(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    state.runtime.stop().await?;
    Ok(Json(json!({"ok": true, "stopped": true})))
}
