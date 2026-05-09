use crate::{
    dto::ProjectUpsertRequest,
    error::AppResult,
    memory::db::{BrainDb, ProjectRecord},
};
use chrono::Utc;
use std::sync::Arc;

pub struct ProjectManager {
    db: Arc<BrainDb>,
}

impl ProjectManager {
    pub fn new(db: Arc<BrainDb>) -> Self {
        Self { db }
    }

    pub fn list(&self) -> AppResult<Vec<ProjectRecord>> {
        self.db.list_projects()
    }

    pub fn upsert(&self, req: ProjectUpsertRequest) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let record = ProjectRecord {
            id: req.id,
            name: req.name,
            path: req.path,
            kind: req.kind,
            rules: req.rules,
            status: "active".to_string(),
            active_model_profile: None,
            created_at: now.clone(),
            updated_at: now,
        };
        self.db.upsert_project(&record)
    }

    pub fn activate(&self, id: &str) -> AppResult<()> {
        self.db.activate_project(id)
    }
}
