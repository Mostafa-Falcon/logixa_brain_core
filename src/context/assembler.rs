use crate::{
    brain::modes::BrainMode,
    config::BrainConfig,
    context::prompt_loader::PromptLoader,
    error::AppResult,
    memory::db::{BrainDb, MemoryRecord, MessageRecord},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssembledContext {
    pub project_id: String,
    pub recent_messages: Vec<MessageRecord>,
    pub memories: Vec<MemoryRecord>,
    pub system_prompt: String,
}

pub struct ContextAssembler {
    db: Arc<BrainDb>,
    config: BrainConfig,
}

impl ContextAssembler {
    pub fn new(db: Arc<BrainDb>, config: BrainConfig) -> Self {
        Self { db, config }
    }

    pub fn assemble(
        &self,
        conversation_id: &str,
        project_id: Option<String>,
        user_message: &str,
        mode: &BrainMode,
        use_memory: bool,
    ) -> AppResult<AssembledContext> {
        let active_project_id = match project_id {
            Some(id) => id,
            None => self
                .db
                .active_project_id()
                .unwrap_or_else(|_| self.config.brain.default_project_id.clone()),
        };

        let recent = self
            .db
            .recent_messages(conversation_id, self.config.brain.max_recent_messages)?;

        let memories = if use_memory {
            self.db
                .search_memories(
                    user_message,
                    Some(&active_project_id),
                    self.config.brain.memory_search_limit,
                )
                .unwrap_or_default()
        } else {
            vec![]
        };

        let system_prompt = PromptLoader::load_system_prompt(mode, &active_project_id)?;

        Ok(AssembledContext {
            project_id: active_project_id,
            recent_messages: recent,
            memories,
            system_prompt,
        })
    }
}
