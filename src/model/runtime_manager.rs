use crate::{
    config::BrainConfig,
    error::{AppError, AppResult},
    model::llama_client::LlamaClient,
};
use serde::Serialize;
use std::{
    path::Path,
    process::Stdio,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    process::{Child, Command},
    sync::Mutex as AsyncMutex,
};

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    pub running: bool,
    pub mock_enabled: bool,
    pub model_path: String,
    pub llama_server_binary: String,
    pub base_url: String,
}

pub struct ModelRuntimeManager {
    config: BrainConfig,
    child: Arc<AsyncMutex<Option<Child>>>,
    last_used: Arc<Mutex<Instant>>,
}

impl ModelRuntimeManager {
    pub fn new(config: BrainConfig) -> Self {
        Self {
            config,
            child: Arc::new(AsyncMutex::new(None)),
            last_used: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn client(&self) -> LlamaClient {
        LlamaClient::new(self.config.clone())
    }

    pub async fn status(&self) -> RuntimeStatus {
        RuntimeStatus {
            running: self.is_running().await,
            mock_enabled: self.config.model.mock_when_unavailable,
            model_path: self.config.model.model_path.clone(),
            llama_server_binary: self.config.model.llama_server_binary.clone(),
            base_url: self.config.llama_base_url(),
        }
    }

    pub async fn status_label(&self) -> String {
        if self.client().health().await {
            "ready".to_string()
        } else if self.is_running().await {
            "starting_or_unhealthy".to_string()
        } else {
            "stopped".to_string()
        }
    }

    pub async fn is_running(&self) -> bool {
        let mut guard = self.child.lock().await;
        if let Some(child) = guard.as_mut() {
            match child.try_wait() {
                Ok(Some(_)) => {
                    *guard = None;
                    false
                }
                Ok(None) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub async fn ensure_ready(&self) -> AppResult<String> {
        self.touch();
        if self.client().health().await {
            return Ok("llama-server already healthy".to_string());
        }
        if !self.config.model.auto_start {
            return Err(AppError::ModelRuntime(
                "model is not running and auto_start=false".to_string(),
            ));
        }
        self.start().await?;
        for _ in 0..60 {
            if self.client().health().await {
                return Ok("llama-server started and healthy".to_string());
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        Err(AppError::ModelRuntime(
            "llama-server did not become healthy in time".to_string(),
        ))
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.is_running().await {
            return Ok(());
        }
        if !Path::new(&self.config.model.model_path).exists() {
            return Err(AppError::ModelRuntime(format!(
                "model file not found: {}",
                self.config.model.model_path
            )));
        }
        if self.config.model.llama_server_binary != "llama-server"
            && !Path::new(&self.config.model.llama_server_binary).exists()
        {
            return Err(AppError::ModelRuntime(format!(
                "llama-server binary not found: {}",
                self.config.model.llama_server_binary
            )));
        }

        let mut cmd = Command::new(&self.config.model.llama_server_binary);
        cmd.arg("-m")
            .arg(&self.config.model.model_path)
            .arg("--host")
            .arg(&self.config.model.llama_host)
            .arg("--port")
            .arg(self.config.model.llama_port.to_string())
            .arg("-c")
            .arg(self.config.model.context_size.to_string())
            .arg("-t")
            .arg(self.config.model.threads.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        for extra in &self.config.model.extra_args {
            cmd.arg(extra);
        }

        let child = cmd
            .spawn()
            .map_err(|e| AppError::ModelRuntime(format!("failed to start llama-server: {e}")))?;
        let mut guard = self.child.lock().await;
        *guard = Some(child);
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        let mut guard = self.child.lock().await;
        if let Some(child) = guard.as_mut() {
            let _ = child.kill().await;
        }
        *guard = None;
        Ok(())
    }

    pub fn touch(&self) {
        if let Ok(mut guard) = self.last_used.lock() {
            *guard = Instant::now();
        }
    }

    pub fn should_mock(&self) -> bool {
        self.config.model.mock_when_unavailable
    }

    pub fn mock_answer(&self, message: &str) -> String {
        format!("[Mock Brain Response] استلمت رسالتك: {message}\n\nالـ Brain Core شغال، لكن llama-server غير متاح حاليًا. ظبط `brain_config.toml` بمسار الموديل و `llama_server_binary` عشان الرد يبقى من Qwen فعليًا.")
    }

    pub fn start_idle_watcher(&self) {
        if self.config.model.keep_loaded {
            return;
        }
        let child = self.child.clone();
        let last_used = self.last_used.clone();
        let timeout = self.config.model.idle_timeout_seconds;
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let elapsed = last_used.lock().map(|t| t.elapsed().as_secs()).unwrap_or(0);
                if elapsed >= timeout {
                    let mut guard = child.lock().await;
                    if let Some(process) = guard.as_mut() {
                        let _ = process.kill().await;
                    }
                    *guard = None;
                }
            }
        });
    }
}
