use crate::{
    config::BrainConfig,
    context::prompt_builder::ChatMessage,
    error::{AppError, AppResult},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct LlamaChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct LlamaChatResponse {
    choices: Vec<LlamaChoice>,
}

#[derive(Debug, Deserialize)]
struct LlamaChoice {
    message: LlamaMessage,
}

#[derive(Debug, Deserialize)]
struct LlamaMessage {
    content: String,
}

#[derive(Clone)]
pub struct LlamaClient {
    http: Client,
    config: BrainConfig,
}

impl LlamaClient {
    pub fn new(config: BrainConfig) -> Self {
        Self {
            http: Client::new(),
            config,
        }
    }

    pub async fn health(&self) -> bool {
        let url = format!("{}/health", self.config.llama_base_url());
        self.http
            .get(url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> AppResult<String> {
        let url = format!("{}/v1/chat/completions", self.config.llama_base_url());
        let body = LlamaChatRequest {
            model: self.config.model.model_id.clone(),
            messages,
            temperature: self.config.model.temperature,
            top_p: self.config.model.top_p,
            max_tokens: self.config.model.max_tokens,
            stream: false,
        };
        let response = self.http.post(url).json(&body).send().await?;
        if !response.status().is_success() {
            return Err(AppError::ModelRuntime(format!(
                "llama-server returned HTTP {}",
                response.status()
            )));
        }
        let parsed: LlamaChatResponse = response.json().await?;
        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| AppError::ModelRuntime("llama-server returned no choices".to_string()))
    }
}
