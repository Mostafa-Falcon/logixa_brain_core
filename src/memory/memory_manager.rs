use crate::{
    dto::{MemorySaveRequest, MemorySearchRequest},
    error::AppResult,
    memory::db::{BrainDb, MemoryRecord},
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct MemoryManager {
    db: Arc<BrainDb>,
}

impl MemoryManager {
    pub fn new(db: Arc<BrainDb>) -> Self {
        Self { db }
    }

    pub fn save_memory(&self, req: MemorySaveRequest) -> AppResult<String> {
        let now = Utc::now().to_rfc3339();
        let record = MemoryRecord {
            id: Uuid::new_v4().to_string(),
            scope: req.scope,
            project_id: req.project_id,
            kind: req.kind,
            content: req.content,
            tags: req.tags,
            importance: req.importance.unwrap_or(1),
            created_at: now.clone(),
            updated_at: now,
        };
        self.db.insert_memory(&record)?;
        Ok(record.id)
    }

    pub fn search(&self, req: MemorySearchRequest) -> AppResult<Vec<MemoryRecord>> {
        self.db.search_memories(
            &req.query,
            req.project_id.as_deref(),
            req.limit.unwrap_or(6),
        )
    }
}
