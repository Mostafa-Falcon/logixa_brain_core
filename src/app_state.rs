use crate::{
    config::BrainConfig, memory::db::BrainDb, model::runtime_manager::ModelRuntimeManager,
    tools::registry::ToolRegistry,
};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<BrainConfig>>,
    pub db: Arc<BrainDb>,
    pub runtime: Arc<ModelRuntimeManager>,
    pub tools: Arc<ToolRegistry>,
}
