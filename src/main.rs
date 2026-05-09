mod api;
mod app_state;
mod brain;
mod config;
mod context;
mod dto;
mod error;
mod logs;
mod memory;
mod model;
mod projects;
mod tools;

use app_state::AppState;
use config::BrainConfig;
use memory::db::BrainDb;
use model::runtime_manager::ModelRuntimeManager;
use std::{
    fs,
    sync::{Arc, RwLock},
};
use tools::registry::ToolRegistry;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "logixa_brain_core=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = BrainConfig::load()?;
    fs::create_dir_all(&config.paths.data_dir)?;

    let db = Arc::new(BrainDb::open(&config.paths.database_path)?);
    db.migrate()?;
    db.seed_defaults(&config)?;

    let runtime = Arc::new(ModelRuntimeManager::new(config.clone()));
    runtime.start_idle_watcher();

    let tools = Arc::new(ToolRegistry::new());

    let state = AppState {
        config: Arc::new(RwLock::new(config.clone())),
        db,
        runtime,
        tools,
    };

    let app = api::router(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = config.socket_addr()?;
    tracing::info!("Logixa Brain Core listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
