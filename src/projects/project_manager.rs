use crate::{
    dto::{ProjectStateUpdateRequest, ProjectUpsertRequest},
    error::{AppError, AppResult},
    memory::db::{BrainDb, ProjectRecord, ProjectStateRecord},
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

    pub fn get(&self, id: &str) -> AppResult<ProjectRecord> {
        self.db
            .get_project(id)?
            .ok_or_else(|| AppError::NotFound(format!("project not found: {}", id)))
    }

    pub fn active_project_id(&self) -> AppResult<String> {
        self.db.active_project_id()
    }

    pub fn active_project(&self) -> AppResult<ProjectRecord> {
        let id = self.active_project_id()?;
        self.get(&id)
    }

    pub fn upsert(&self, req: ProjectUpsertRequest) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let existing = self.db.get_project(&req.id)?;
        let created_at = existing
            .as_ref()
            .map(|p| p.created_at.clone())
            .unwrap_or_else(|| now.clone());
        let status = existing
            .as_ref()
            .map(|p| p.status.clone())
            .unwrap_or_else(|| "active".to_string());
        let active_model_profile = existing.and_then(|p| p.active_model_profile);

        let record = ProjectRecord {
            id: req.id.clone(),
            name: req.name,
            path: req.path,
            kind: req.kind,
            rules: req.rules,
            status,
            active_model_profile,
            created_at,
            updated_at: now.clone(),
        };
        self.db.upsert_project(&record)?;

        if self.db.get_project_state(&req.id)?.is_none() {
            let state = ProjectStateRecord {
                project_id: req.id,
                current_step: None,
                last_commit: None,
                last_tag: None,
                summary: None,
                next_step: None,
                status: "active".to_string(),
                created_at: now.clone(),
                updated_at: now,
            };
            self.db.upsert_project_state(&state)?;
        }

        Ok(())
    }

    pub fn activate(&self, id: &str) -> AppResult<()> {
        self.get(id)?;
        self.db.activate_project(id)
    }

    pub fn get_state(&self, project_id: &str) -> AppResult<ProjectStateRecord> {
        self.get(project_id)?;
        self.db
            .get_project_state(project_id)?
            .ok_or_else(|| AppError::NotFound(format!("project state not found: {}", project_id)))
    }

    pub fn update_state(
        &self,
        project_id: &str,
        req: ProjectStateUpdateRequest,
    ) -> AppResult<ProjectStateRecord> {
        self.get(project_id)?;
        let now = Utc::now().to_rfc3339();
        let existing = self.db.get_project_state(project_id)?;

        let state = ProjectStateRecord {
            project_id: project_id.to_string(),
            current_step: req
                .current_step
                .or_else(|| existing.as_ref().and_then(|s| s.current_step.clone())),
            last_commit: req
                .last_commit
                .or_else(|| existing.as_ref().and_then(|s| s.last_commit.clone())),
            last_tag: req
                .last_tag
                .or_else(|| existing.as_ref().and_then(|s| s.last_tag.clone())),
            summary: req
                .summary
                .or_else(|| existing.as_ref().and_then(|s| s.summary.clone())),
            next_step: req
                .next_step
                .or_else(|| existing.as_ref().and_then(|s| s.next_step.clone())),
            status: req
                .status
                .or_else(|| existing.as_ref().map(|s| s.status.clone()))
                .unwrap_or_else(|| "active".to_string()),
            created_at: existing
                .as_ref()
                .map(|s| s.created_at.clone())
                .unwrap_or_else(|| now.clone()),
            updated_at: now,
        };

        self.db.upsert_project_state(&state)?;
        Ok(state)
    }
}
