use crate::error::{AppError, AppResult};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn get_path(input: &Value) -> AppResult<PathBuf> {
    let path = input
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Request("tool input requires `path` string".to_string()))?;
    Ok(PathBuf::from(path))
}

fn ensure_safe(path: &Path) -> AppResult<()> {
    if path.to_string_lossy().contains("..") {
        return Err(AppError::Request(
            "parent path traversal is not allowed".to_string(),
        ));
    }
    Ok(())
}

pub fn list_files(input: Value) -> AppResult<Value> {
    let path = get_path(&input)?;
    ensure_safe(&path)?;
    let mut items = vec![];
    for entry in fs::read_dir(&path).map_err(|e| AppError::Request(e.to_string()))? {
        let entry = entry.map_err(|e| AppError::Request(e.to_string()))?;
        let meta = entry
            .metadata()
            .map_err(|e| AppError::Request(e.to_string()))?;
        items.push(json!({
            "name": entry.file_name().to_string_lossy(),
            "path": entry.path().to_string_lossy(),
            "is_dir": meta.is_dir(),
            "size": meta.len()
        }));
    }
    Ok(json!({"items": items}))
}

pub fn read_file(input: Value) -> AppResult<Value> {
    let path = get_path(&input)?;
    ensure_safe(&path)?;
    let max_bytes = input
        .get("max_bytes")
        .and_then(|v| v.as_u64())
        .unwrap_or(200_000) as usize;
    let bytes = fs::read(&path).map_err(|e| AppError::Request(e.to_string()))?;
    let truncated = bytes.len() > max_bytes;
    let content_bytes = if truncated {
        &bytes[..max_bytes]
    } else {
        &bytes[..]
    };
    let content = String::from_utf8_lossy(content_bytes).to_string();
    Ok(json!({"path": path.to_string_lossy(), "content": content, "truncated": truncated}))
}
