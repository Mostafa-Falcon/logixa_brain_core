use crate::{
    error::{AppError, AppResult},
    memory::db::BrainDb,
    tools::{command_tools, file_tools, git_tools},
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
                    description: "List files inside an allowed path".to_string(),
                    requires_approval: false,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "read_file".to_string(),
                    description: "Read a UTF-8 text file".to_string(),
                    requires_approval: false,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "git_status".to_string(),
                    description: "Run git status --short in a repo".to_string(),
                    requires_approval: false,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "git_diff".to_string(),
                    description: "Run git diff --stat in a repo".to_string(),
                    requires_approval: false,
                    danger_level: "low".to_string(),
                },
                ToolDefinition {
                    name: "run_command".to_string(),
                    description: "Run a shell command with timeout. Requires approval.".to_string(),
                    requires_approval: true,
                    danger_level: "high".to_string(),
                },
            ],
        }
    }

    pub fn list(&self) -> Vec<ToolDefinition> {
        self.definitions.clone()
    }

    pub async fn run(
        &self,
        name: String,
        input: Value,
        approved: bool,
        db: Arc<BrainDb>,
    ) -> AppResult<Value> {
        let requires_approval = self
            .definitions
            .iter()
            .find(|tool| tool.name == name)
            .map(|tool| tool.requires_approval)
            .unwrap_or(true);
        if requires_approval && !approved {
            return Err(AppError::Request(format!(
                "tool `{name}` requires approval=true"
            )));
        }

        let input_json = input.to_string();
        let result = match name.as_str() {
            "list_files" => file_tools::list_files(input),
            "read_file" => file_tools::read_file(input),
            "git_status" => git_tools::git_status(input).await,
            "git_diff" => git_tools::git_diff(input).await,
            "run_command" => command_tools::run_command(input).await,
            _ => Err(AppError::NotFound(format!("unknown tool `{name}`"))),
        };

        match &result {
            Ok(value) => {
                let _ = db.insert_tool_run(&name, &input_json, Some(&value.to_string()), true);
            }
            Err(err) => {
                let _ = db.insert_tool_run(
                    &name,
                    &input_json,
                    Some(&serde_json::json!({"error": err.to_string()}).to_string()),
                    false,
                );
            }
        }
        result
    }
}
