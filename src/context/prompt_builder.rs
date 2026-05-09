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

        out.push_str("\n# Active Project Context\n\n");
        out.push_str(&format!("- project_id: {}\n", context.project_id));

        if let Some(project) = &context.project {
            out.push_str(&format!("- name: {}\n", project.name));
            if let Some(kind) = &project.kind {
                out.push_str(&format!("- kind: {}\n", kind));
            }
            if let Some(path) = &project.path {
                out.push_str(&format!("- path: {}\n", path));
            }
            out.push_str(&format!("- status: {}\n", project.status));
            if let Some(rules) = &project.rules {
                out.push_str("- project_rules:\n");
                out.push_str(rules.trim());
                out.push('\n');
            }
        }

        if let Some(state) = &context.project_state {
            out.push_str("\n# Project State Memory\n\n");
            if let Some(step) = &state.current_step {
                out.push_str(&format!("- current_step: {}\n", step));
            }
            if let Some(commit) = &state.last_commit {
                out.push_str(&format!("- last_commit: {}\n", commit));
            }
            if let Some(tag) = &state.last_tag {
                out.push_str(&format!("- last_tag: {}\n", tag));
            }
            if let Some(summary) = &state.summary {
                out.push_str("- summary: ");
                out.push_str(summary.trim());
                out.push('\n');
            }
            if let Some(next_step) = &state.next_step {
                out.push_str(&format!("- next_step: {}\n", next_step));
            }
            out.push_str(&format!("- project_state_status: {}\n", state.status));
        }

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
