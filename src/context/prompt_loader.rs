use crate::{brain::modes::BrainMode, error::AppResult};
use std::{fs, path::Path};

pub struct PromptLoader;

impl PromptLoader {
    pub fn load_system_prompt(mode: &BrainMode, project_id: &str) -> AppResult<String> {
        let identity = Self::read_prompt(
            "prompts/identity.md",
            include_str!("../../prompts/identity.md"),
        );
        let style = Self::read_prompt(
            "prompts/style_egyptian.md",
            include_str!("../../prompts/style_egyptian.md"),
        );
        let execution = Self::read_prompt(
            "prompts/execution_rules.md",
            include_str!("../../prompts/execution_rules.md"),
        );
        let memory = Self::read_prompt(
            "prompts/memory_rules.md",
            include_str!("../../prompts/memory_rules.md"),
        );
        let tools = Self::read_prompt(
            "prompts/tool_rules.md",
            include_str!("../../prompts/tool_rules.md"),
        );
        let mode_policy = Self::load_mode_prompt(mode);

        Ok(format!(
            r#"{identity}

{style}

{execution}

{memory}

{tools}

{mode_policy}

# Runtime Context

Project ID: {project_id}
Mode: {}

التزمي بالسياسات السابقة في الرد القادم.
"#,
            mode.as_str()
        ))
    }

    fn load_mode_prompt(mode: &BrainMode) -> String {
        match mode {
            BrainMode::Chat => Self::read_prompt(
                "prompts/modes/chat.md",
                include_str!("../../prompts/modes/chat.md"),
            ),
            BrainMode::Task => Self::read_prompt(
                "prompts/modes/task.md",
                include_str!("../../prompts/modes/task.md"),
            ),
            BrainMode::Code => Self::read_prompt(
                "prompts/modes/code.md",
                include_str!("../../prompts/modes/code.md"),
            ),
            BrainMode::Planning => Self::read_prompt(
                "prompts/modes/planning.md",
                include_str!("../../prompts/modes/planning.md"),
            ),
            BrainMode::Memory => Self::read_prompt(
                "prompts/modes/memory.md",
                include_str!("../../prompts/modes/memory.md"),
            ),
            BrainMode::Tool => Self::read_prompt(
                "prompts/modes/tool.md",
                include_str!("../../prompts/modes/tool.md"),
            ),
        }
    }

    fn read_prompt(path: &str, fallback: &str) -> String {
        let disk_path = Path::new(path);
        match fs::read_to_string(disk_path) {
            Ok(content) if !content.trim().is_empty() => content,
            _ => fallback.to_string(),
        }
    }
}
