use crate::{
    app_state::AppState,
    brain::engine::BrainEngine,
    dto::{ChatRequest, ChatResponse},
    error::AppResult,
};
use axum::{extract::State, Json};

pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> AppResult<Json<ChatResponse>> {
    let engine = BrainEngine::new(state);
    let response = engine.handle_chat(req).await?;
    Ok(Json(response))
}
