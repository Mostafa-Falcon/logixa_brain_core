use crate::{app_state::AppState, dto::StatusResponse, error::AppResult};
use axum::{extract::State, Json};

pub async fn status(State(state): State<AppState>) -> AppResult<Json<StatusResponse>> {
    let cfg = state.config.read().expect("config lock poisoned").clone();
    let model_status = state.runtime.status_label().await;
    Ok(Json(StatusResponse {
        ok: true,
        service: "logixa-brain-core".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_ready: state.db.ping()?,
        model_status,
        active_model: cfg.model.name,
    }))
}
