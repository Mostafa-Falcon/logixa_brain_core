use crate::{app_state::AppState, dto::ToolRunRequest, error::AppResult};
use axum::{extract::State, Json};
use serde_json::json;

pub async fn list_tools(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(json!({"ok": true, "tools": state.tools.list()})))
}

pub async fn run_tool(
    State(state): State<AppState>,
    Json(req): Json<ToolRunRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let output = state
        .tools
        .run(
            req.name,
            req.input,
            req.approved.unwrap_or(false),
            state.db.clone(),
        )
        .await?;
    Ok(Json(json!({"ok": true, "output": output})))
}
