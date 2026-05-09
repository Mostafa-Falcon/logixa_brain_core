use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub id: String,
    pub name: String,
    pub model_path: String,
    pub context_size: u32,
    pub threads: u32,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
