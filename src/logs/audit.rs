use crate::{error::AppResult, memory::db::BrainDb};
use std::sync::Arc;

pub struct AuditLog {
    db: Arc<BrainDb>,
}

impl AuditLog {
    pub fn new(db: Arc<BrainDb>) -> Self {
        Self { db }
    }

    pub fn info(&self, event: &str, details: Option<serde_json::Value>) -> AppResult<String> {
        let details_string = details.map(|v| v.to_string());
        self.db
            .insert_event("info", event, details_string.as_deref())
    }

    pub fn error(&self, event: &str, details: Option<serde_json::Value>) -> AppResult<String> {
        let details_string = details.map(|v| v.to_string());
        self.db
            .insert_event("error", event, details_string.as_deref())
    }

    pub fn recent(&self, limit: usize) -> AppResult<Vec<serde_json::Value>> {
        self.db.recent_events(limit)
    }
}
