use crate::error::{AppError, AppResult};
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::process::Command;

fn repo_path(input: &Value) -> AppResult<String> {
    input
        .get("path")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Request("tool input requires `path` string".to_string()))
}

async fn run_git(path: String, args: &[&str]) -> AppResult<Value> {
    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| AppError::Request(e.to_string()))?;
    Ok(json!({
        "success": output.status.success(),
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
        "code": output.status.code()
    }))
}

pub async fn git_status(input: Value) -> AppResult<Value> {
    run_git(repo_path(&input)?, &["status", "--short"]).await
}

pub async fn git_diff(input: Value) -> AppResult<Value> {
    run_git(repo_path(&input)?, &["diff", "--stat"]).await
}
