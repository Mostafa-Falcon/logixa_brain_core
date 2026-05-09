use crate::{
    app_state::AppState,
    dto::{ToolDecisionRequest, ToolRequestRequest, ToolRunRequest},
    error::{AppError, AppResult},
    memory::db::PendingToolCallRecord,
};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

pub async fn list_tools(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    Ok(Json(json!({"ok": true, "tools": state.tools.list()})))
}

pub async fn request_tool(
    State(state): State<AppState>,
    Json(req): Json<ToolRequestRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let definition = state.tools.validate_request(&req.tool_name)?;
    let now = Utc::now().to_rfc3339();
    let pending = PendingToolCallRecord {
        id: Uuid::new_v4().to_string(),
        tool_name: req.tool_name,
        project_id: req.project_id,
        input_json: req.input.to_string(),
        status: "pending".to_string(),
        danger_level: definition.danger_level,
        created_at: now.clone(),
        updated_at: now,
        decided_at: None,
        decision_note: None,
    };

    state.db.insert_pending_tool_call(&pending)?;

    Ok(Json(json!({
        "ok": true,
        "approval_required": true,
        "approval_id": pending.id,
        "pending_tool_call": pending
    })))
}

pub async fn pending_tools(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let items = state.db.list_pending_tool_calls(50)?;
    Ok(Json(
        json!({"ok": true, "count": items.len(), "items": items}),
    ))
}

pub async fn approve_tool(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ToolDecisionRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let pending = state
        .db
        .get_pending_tool_call(&id)?
        .ok_or_else(|| AppError::NotFound(format!("pending tool call `{id}` not found")))?;

    if pending.status != "pending" {
        return Err(AppError::Request(format!(
            "tool call `{id}` is not pending; current status is `{}`",
            pending.status
        )));
    }

    state
        .db
        .update_pending_tool_status(&id, "approved", req.note.as_deref())?;

    let input: serde_json::Value = serde_json::from_str(&pending.input_json)
        .map_err(|e| AppError::Request(format!("invalid pending tool input json: {e}")))?;

    let execution = state
        .tools
        .execute_approved(&pending.tool_name, input, state.db.clone())
        .await;

    match execution {
        Ok(output) => {
            state
                .db
                .update_pending_tool_status(&id, "executed", req.note.as_deref())?;
            Ok(Json(json!({
                "ok": true,
                "approval_id": id,
                "status": "executed",
                "output": output
            })))
        }
        Err(err) => {
            let note = format!("{}; execution error: {}", req.note.unwrap_or_default(), err);
            let _ = state
                .db
                .update_pending_tool_status(&id, "failed", Some(&note));
            Err(err)
        }
    }
}

pub async fn reject_tool(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ToolDecisionRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let pending = state
        .db
        .get_pending_tool_call(&id)?
        .ok_or_else(|| AppError::NotFound(format!("pending tool call `{id}` not found")))?;

    if pending.status != "pending" {
        return Err(AppError::Request(format!(
            "tool call `{id}` is not pending; current status is `{}`",
            pending.status
        )));
    }

    state
        .db
        .update_pending_tool_status(&id, "rejected", req.note.as_deref())?;

    Ok(Json(json!({
        "ok": true,
        "approval_id": id,
        "status": "rejected"
    })))
}

pub async fn recent_tool_runs(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let items = state.db.recent_tool_runs(50)?;
    Ok(Json(
        json!({"ok": true, "count": items.len(), "items": items}),
    ))
}

pub async fn run_tool(
    State(_state): State<AppState>,
    Json(_req): Json<ToolRunRequest>,
) -> AppResult<Json<serde_json::Value>> {
    Err(AppError::Request(
        "direct /tools/run is disabled in Step 6; use /tools/request then /tools/approve/:id"
            .to_string(),
    ))
}
