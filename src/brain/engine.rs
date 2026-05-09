use crate::{
    app_state::AppState,
    brain::{intent_router::IntentRouter, modes::BrainMode, response_pipeline::ResponsePipeline},
    context::{assembler::ContextAssembler, prompt_builder::PromptBuilder},
    dto::{ChatRequest, ChatResponse},
    error::{AppError, AppResult},
    logs::audit::AuditLog,
};

pub struct BrainEngine {
    state: AppState,
}

impl BrainEngine {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn handle_chat(&self, req: ChatRequest) -> AppResult<ChatResponse> {
        if req.message.trim().is_empty() {
            return Err(AppError::Request("message cannot be empty".to_string()));
        }

        let cfg = self
            .state
            .config
            .read()
            .expect("config lock poisoned")
            .clone();
        let mode = BrainMode::from_optional(req.mode.as_deref())
            .unwrap_or_else(|| IntentRouter::classify(&req.message));
        let project_id = req
            .project_id
            .clone()
            .or_else(|| self.state.db.active_project_id().ok());
        let title = req.message.chars().take(42).collect::<String>();
        let conversation_id = match req.conversation_id.clone() {
            Some(id) => id,
            None => self
                .state
                .db
                .create_conversation(&title, project_id.as_deref())?,
        };

        self.state
            .db
            .save_message(&conversation_id, "user", &req.message, Some(mode.as_str()))?;

        let use_memory = req.use_memory.unwrap_or(true);
        let assembler = ContextAssembler::new(self.state.db.clone(), cfg.clone());
        let assembled = assembler.assemble(
            &conversation_id,
            project_id,
            &req.message,
            &mode,
            use_memory,
        )?;
        let messages = PromptBuilder::build(&assembled, &req.message);

        let mut events = vec!["preparing_context".to_string()];
        let mut mocked = false;
        let raw_answer = match self.state.runtime.ensure_ready().await {
            Ok(_) => {
                events.push("model_ready".to_string());
                self.state.runtime.touch();
                match self.state.runtime.client().chat(messages).await {
                    Ok(answer) => answer,
                    Err(err) if self.state.runtime.should_mock() => {
                        mocked = true;
                        events.push("model_error_mock_fallback".to_string());
                        format!(
                            "{}\n\nسبب fallback: {}",
                            self.state.runtime.mock_answer(&req.message),
                            err
                        )
                    }
                    Err(err) => return Err(err),
                }
            }
            Err(err) if self.state.runtime.should_mock() => {
                mocked = true;
                events.push("model_unavailable_mock_fallback".to_string());
                format!(
                    "{}\n\nسبب fallback: {}",
                    self.state.runtime.mock_answer(&req.message),
                    err
                )
            }
            Err(err) => return Err(err),
        };

        events.push("generating_done".to_string());
        let answer = ResponsePipeline::clean(&raw_answer);
        self.state
            .db
            .save_message(&conversation_id, "assistant", &answer, Some(mode.as_str()))?;
        events.push("saving_memory".to_string());

        let audit = AuditLog::new(self.state.db.clone());
        let _ = audit.info(
            "chat_completed",
            Some(serde_json::json!({
                "conversation_id": conversation_id,
                "mode": mode.as_str(),
                "mocked": mocked
            })),
        );
        events.push("done".to_string());

        Ok(ChatResponse {
            answer,
            conversation_id,
            mode_used: mode.as_str().to_string(),
            model_used: cfg.model.name,
            memory_used: use_memory && !assembled.memories.is_empty(),
            events,
            tool_calls: vec![],
            mocked,
        })
    }
}
