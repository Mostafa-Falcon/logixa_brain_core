use crate::context::assembler::AssembledContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build(context: &AssembledContext, user_message: &str) -> Vec<ChatMessage> {
        let mut messages = vec![ChatMessage {
            role: "system".to_string(),
            content: Self::build_system_content(context),
        }];

        for msg in &context.recent_messages {
            if msg.role == "user" || msg.role == "assistant" || msg.role == "system" {
                messages.push(ChatMessage {
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                });
            }
        }

        if messages.last().map(|m| m.content.as_str()) != Some(user_message) {
            messages.push(ChatMessage {
                role: "user".to_string(),
                content: user_message.to_string(),
            });
        }
        messages
    }

    fn build_system_content(context: &AssembledContext) -> String {
        let mut out = context.system_prompt.clone();
        if !context.memories.is_empty() {
            out.push_str("\n# Relevant Long-Term Memory\n\n");
            for memory in &context.memories {
                out.push_str(&format!(
                    "- [{}:{}] {}\n",
                    memory.scope, memory.kind, memory.content
                ));
            }
        }
        out
    }
}
