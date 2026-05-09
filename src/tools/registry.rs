use crate::{
    error::{AppError, AppResult},
    memory::db::BrainDb,
    tools::{file_tools, git_tools},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub requires_approval: bool,
    pub danger_level: String,
}

pub struct ToolRegistry {
    definitions: Vec<ToolDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            definitions: vec![
                ToolDefinition {
                    name: "list_files".to_string(),
                    description:
                        "List files inside an allowed path. Read-only. Requires approval in v1."
                            .to_string(),
                    requires_approval: true,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "git_status".to_string(),
                    description:
                        "Run git status --short in a repo. Read-only. Requires approval in v1."
                            .to_string(),
                    requires_approval: true,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "git_diff".to_string(),
                    description:
                        "Run git diff --stat in a repo. Read-only. Requires approval in v1."
                            .to_string(),
                    requires_approval: true,
                    danger_level: "low".to_string(),
                },
            ],
        }
    }

    pub fn list(&self) -> Vec<ToolDefinition> {
        self.definitions.clone()
    }

    pub fn definition_for(&self, name: &str) -> Option<ToolDefinition> {
        self.definitions
            .iter()
            .find(|tool| tool.name == name)
            .cloned()
    }

    pub fn validate_request(&self, name: &str) -> AppResult<ToolDefinition> {
        self.definition_for(name)
            .ok_or_else(|| AppError::NotFound(format!("unknown or disabled tool `{name}`")))
    }

    pub async fn execute_approved(
        &self,
        name: &str,
        input: Value,
        db: Arc<BrainDb>,
    ) -> AppResult<Value> {
        self.validate_request(name)?;
        let input_json = input.to_string();

        let result = match name {
            "list_files" => file_tools::list_files(input),
            "git_status" => git_tools::git_status(input).await,
            "git_diff" => git_tools::git_diff(input).await,
            _ => Err(AppError::NotFound(format!(
                "unknown or disabled tool `{name}`"
            ))),
        };

        match &result {
            Ok(value) => {
                let _ = db.insert_tool_run(name, &input_json, Some(&value.to_string()), true);
            }
            Err(err) => {
                let _ = db.insert_tool_run(
                    name,
                    &input_json,
                    Some(&serde_json::json!({"error": err.to_string()}).to_string()),
                    false,
                );
            }
        }

        result
    }
}
