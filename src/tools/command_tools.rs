use crate::error::{AppError, AppResult};
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::{
    process::Command,
    time::{timeout, Duration},
};

pub async fn run_command(input: Value) -> AppResult<Value> {
    let command = input
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Request("tool input requires `command` string".to_string()))?;
    let cwd = input.get("cwd").and_then(|v| v.as_str()).unwrap_or(".");
    let timeout_seconds = input
        .get("timeout_seconds")
        .and_then(|v| v.as_u64())
        .unwrap_or(20)
        .min(120);

    let fut = Command::new("bash")
        .arg("-lc")
        .arg(command)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let output = timeout(Duration::from_secs(timeout_seconds), fut)
        .await
        .map_err(|_| AppError::Request("command timed out".to_string()))?
        .map_err(|e| AppError::Request(e.to_string()))?;

    Ok(json!({
        "success": output.status.success(),
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
        "code": output.status.code()
    }))
}
