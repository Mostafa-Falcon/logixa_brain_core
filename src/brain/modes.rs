use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainMode {
    Chat,
    Task,
    Code,
    Planning,
    Memory,
    Tool,
}

impl BrainMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::Task => "task",
            Self::Code => "code",
            Self::Planning => "planning",
            Self::Memory => "memory",
            Self::Tool => "tool",
        }
    }

    pub fn from_optional(value: Option<&str>) -> Option<Self> {
        match value.unwrap_or("auto").to_lowercase().as_str() {
            "chat" => Some(Self::Chat),
            "task" => Some(Self::Task),
            "code" => Some(Self::Code),
            "planning" | "plan" => Some(Self::Planning),
            "memory" => Some(Self::Memory),
            "tool" => Some(Self::Tool),
            "auto" | "" => None,
            _ => None,
        }
    }
}
