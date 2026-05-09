use crate::brain::modes::BrainMode;

pub struct IntentRouter;

impl IntentRouter {
    pub fn classify(message: &str) -> BrainMode {
        let lower = message.to_lowercase();
        let code_markers = [
            "code",
            "كود",
            "rust",
            "flutter",
            "dart",
            "sql",
            "cargo",
            "function",
            "class",
            "error:",
            "stacktrace",
        ];
        let task_markers = [
            "نفذ", "اعمل", "ابني", "صلح", "step", "خطوة", "شغل", "commit", "tag",
        ];
        let plan_markers = ["خطة", "roadmap", "architecture", "معمارية", "حلل", "تصميم"];
        let memory_markers = ["افتكر", "ذاكرة", "remember", "memory", "احفظ"];
        let tool_markers = ["git status", "ls ", "run command", "terminal", "شغل الأمر"];

        if tool_markers.iter().any(|m| lower.contains(m)) {
            return BrainMode::Tool;
        }
        if memory_markers.iter().any(|m| lower.contains(m)) {
            return BrainMode::Memory;
        }
        if code_markers.iter().any(|m| lower.contains(m)) {
            return BrainMode::Code;
        }
        if task_markers.iter().any(|m| lower.contains(m)) {
            return BrainMode::Task;
        }
        if plan_markers.iter().any(|m| lower.contains(m)) {
            return BrainMode::Planning;
        }
        BrainMode::Chat
    }
}
