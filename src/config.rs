use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    pub server: ServerConfig,
    pub paths: PathsConfig,
    pub model: ModelConfig,
    pub brain: BrainPolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub data_dir: String,
    pub database_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub model_id: String,
    pub model_path: String,
    pub llama_server_binary: String,
    pub llama_host: String,
    pub llama_port: u16,
    pub context_size: u32,
    pub threads: u32,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub auto_start: bool,
    pub keep_loaded: bool,
    pub idle_timeout_seconds: u64,
    pub mock_when_unavailable: bool,
    pub extra_args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainPolicyConfig {
    pub default_project_id: String,
    pub default_language: String,
    pub max_recent_messages: usize,
    pub memory_search_limit: usize,
}

impl BrainConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = std::env::var("LOGIXA_BRAIN_CONFIG")
            .unwrap_or_else(|_| "brain_config.toml".to_string());
        if Path::new(&path).exists() {
            let raw = fs::read_to_string(path)?;
            Ok(toml::from_str(&raw)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        Ok(format!("{}:{}", self.server.host, self.server.port).parse()?)
    }

    pub fn llama_base_url(&self) -> String {
        format!("http://{}:{}", self.model.llama_host, self.model.llama_port)
    }
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8787,
            },
            paths: PathsConfig {
                data_dir: "./data".to_string(),
                database_path: "./data/brain.sqlite".to_string(),
            },
            model: ModelConfig {
                name: "Qwen3.5-4B Brain Fast".to_string(),
                model_id: "qwen3.5-4b-brain-fast".to_string(),
                model_path: "./models/Qwen3.5-4B-UD-Q4_K_XL.gguf".to_string(),
                llama_server_binary: "llama-server".to_string(),
                llama_host: "127.0.0.1".to_string(),
                llama_port: 8788,
                context_size: 8192,
                threads: 4,
                max_tokens: 1024,
                temperature: 0.7,
                top_p: 0.9,
                auto_start: true,
                keep_loaded: false,
                idle_timeout_seconds: 180,
                mock_when_unavailable: true,
                extra_args: vec![],
            },
            brain: BrainPolicyConfig {
                default_project_id: "logixa_brain".to_string(),
                default_language: "ar-EG".to_string(),
                max_recent_messages: 12,
                memory_search_limit: 6,
            },
        }
    }
}
