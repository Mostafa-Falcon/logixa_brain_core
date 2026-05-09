pub mod routes_chat;
pub mod routes_logs;
pub mod routes_memory;
pub mod routes_model;
pub mod routes_projects;
pub mod routes_status;
pub mod routes_tools;

use crate::app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/status", get(routes_status::status))
        .route("/chat", post(routes_chat::chat))
        .route("/memory/save", post(routes_memory::save_memory))
        .route("/memory/search", post(routes_memory::search_memory))
        .route("/model/status", get(routes_model::model_status))
        .route("/model/start", post(routes_model::model_start))
        .route("/model/stop", post(routes_model::model_stop))
        .route(
            "/projects",
            get(routes_projects::list_projects).post(routes_projects::upsert_project),
        )
        .route("/projects/active", get(routes_projects::active_project))
        .route(
            "/projects/activate/:id",
            post(routes_projects::activate_project),
        )
        .route(
            "/projects/:id/state",
            get(routes_projects::get_project_state).post(routes_projects::update_project_state),
        )
        .route("/tools", get(routes_tools::list_tools))
        .route("/tools/run", post(routes_tools::run_tool))
        .route("/logs", get(routes_logs::recent_logs))
        .with_state(state)
}
