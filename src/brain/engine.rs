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

        let (conversation_id, project_id) = self.resolve_conversation_scope(&req)?;

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
        let history_messages_used = assembled.recent_messages.len().saturating_sub(1);
        let history_used = history_messages_used > 0;
        let memory_items_used = assembled.memories.len();
        let memory_used = use_memory && memory_items_used > 0;
        let messages = PromptBuilder::build(&assembled, &req.message);

        let mut events = vec!["preparing_context".to_string()];
        if history_used {
            events.push("conversation_history_loaded".to_string());
        }
        if memory_used {
            events.push("long_term_memory_loaded".to_string());
        }

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
                "mocked": mocked,
                "history_used": history_used,
                "history_messages_used": history_messages_used,
                "memory_used": memory_used,
                "memory_items_used": memory_items_used
            })),
        );
        events.push("done".to_string());

        Ok(ChatResponse {
            answer,
            conversation_id,
            mode_used: mode.as_str().to_string(),
            model_used: cfg.model.name,
            memory_used,
            memory_items_used,
            history_used,
            history_messages_used,
            events,
            tool_calls: vec![],
            mocked,
        })
    }

    fn resolve_conversation_scope(&self, req: &ChatRequest) -> AppResult<(String, Option<String>)> {
        let requested_project_id = req.project_id.clone();

        if let Some(raw_id) = req.conversation_id.as_deref() {
            let conversation_id = raw_id.trim();
            if conversation_id.is_empty() {
                return Err(AppError::Request(
                    "conversation_id cannot be empty".to_string(),
                ));
            }

            let existing_project_id = self
                .state
                .db
                .conversation_project_id(conversation_id)?
                .ok_or_else(|| {
                    AppError::Request(format!("conversation_id not found: {}", conversation_id))
                })?;

            if let (Some(requested), Some(existing)) = (
                requested_project_id.as_deref(),
                existing_project_id.as_deref(),
            ) {
                if requested != existing {
                    return Err(AppError::Request(format!(
                        "conversation_id belongs to project '{}' but request used project '{}'",
                        existing, requested
                    )));
                }
            }

            let project_id = existing_project_id.or(requested_project_id);
            return Ok((conversation_id.to_string(), project_id));
        }

        let project_id = requested_project_id.or_else(|| self.state.db.active_project_id().ok());
        let title = req.message.chars().take(42).collect::<String>();
        let conversation_id = self
            .state
            .db
            .create_conversation(&title, project_id.as_deref())?;

        Ok((conversation_id, project_id))
    }
}
