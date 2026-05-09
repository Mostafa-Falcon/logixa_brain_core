use crate::{
    dto::{MemorySaveRequest, MemorySearchRequest},
    error::{AppError, AppResult},
    memory::db::{BrainDb, MemoryRecord},
};
use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct MemoryManager {
    db: Arc<BrainDb>,
}

impl MemoryManager {
    pub fn new(db: Arc<BrainDb>) -> Self {
        Self { db }
    }

    pub fn save_memory(&self, req: MemorySaveRequest) -> AppResult<MemoryRecord> {
        let scope = normalize_required("scope", &req.scope)?;
        let kind = normalize_kind(req.kind, req.key.as_deref());
        let content = resolve_content(req.content, req.key.as_deref(), req.value.as_deref())?;
        let tags = normalize_tags(req.tags);
        let importance = req
            .importance
            .unwrap_or_else(|| default_importance(&scope, &kind))
            .clamp(1, 5);
        let project_id = req.project_id.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });
        let now = Utc::now().to_rfc3339();

        let record = MemoryRecord {
            id: Uuid::new_v4().to_string(),
            scope,
            project_id,
            kind,
            content,
            tags,
            importance,
            created_at: now.clone(),
            updated_at: now,
        };
        self.db.insert_memory(&record)?;
        Ok(record)
    }

    pub fn search(&self, req: MemorySearchRequest) -> AppResult<Vec<MemoryRecord>> {
        self.db.search_memories_filtered(
            &req.query,
            req.project_id.as_deref(),
            req.scope.as_deref(),
            req.kind.as_deref(),
            req.limit.unwrap_or(6),
        )
    }
}

fn normalize_required(field: &str, value: &str) -> AppResult<String> {
    let trimmed = value.trim().to_lowercase();
    if trimmed.is_empty() {
        return Err(AppError::Request(format!("{} cannot be empty", field)));
    }
    Ok(trimmed)
}

fn normalize_kind(kind: Option<String>, key: Option<&str>) -> String {
    let candidate = kind
        .and_then(|value| {
            let trimmed = value.trim().to_lowercase();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .or_else(|| {
            key.map(|value| value.trim().to_lowercase())
                .filter(|value| !value.is_empty())
        });

    candidate.unwrap_or_else(|| "note".to_string())
}

fn resolve_content(
    content: Option<String>,
    key: Option<&str>,
    value: Option<&str>,
) -> AppResult<String> {
    if let Some(raw_content) = content {
        let trimmed = raw_content.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    match (key.map(str::trim), value.map(str::trim)) {
        (Some(key), Some(value)) if !key.is_empty() && !value.is_empty() => {
            Ok(format!("{}: {}", key, value))
        }
        (None, Some(value)) if !value.is_empty() => Ok(value.to_string()),
        _ => Err(AppError::Request(
            "memory requires either non-empty content or key/value".to_string(),
        )),
    }
}

fn normalize_tags(tags: Option<Value>) -> Option<String> {
    match tags {
        Some(Value::String(value)) => {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }
        Some(Value::Array(values)) => {
            let joined = values
                .into_iter()
                .filter_map(|item| match item {
                    Value::String(value) => {
                        let trimmed = value.trim().to_string();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed)
                        }
                    }
                    other => Some(other.to_string()),
                })
                .collect::<Vec<_>>()
                .join(",");
            if joined.is_empty() {
                None
            } else {
                Some(joined)
            }
        }
        Some(other) => Some(other.to_string()),
        None => None,
    }
}

fn default_importance(scope: &str, kind: &str) -> i64 {
    if matches!(kind, "preferred_name" | "name" | "style" | "preference") {
        5
    } else if matches!(scope, "user" | "style") {
        4
    } else {
        3
    }
}
