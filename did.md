# did.md — Logixa Brain Core

## Step 1 — Build headless Brain Core MVP

### Goal
Create the first independent Logixa Brain Core service, focused on the brain itself rather than any UI.

### Decisions implemented
- Brain is a headless Rust local service.
- UI is external client only.
- SQLite is the only database in v0.1.
- llama.cpp / llama-server is the model runtime.
- Model starts on demand.
- Memory uses SQLite FTS5 first, not a Vector DB.
- Tools are routed through a controlled registry.

### Files added
- Cargo.toml
- brain_config.example.toml
- README.md
- did.md
- todo.md
- scripts/start_dev.sh
- src/main.rs
- src/config.rs
- src/error.rs
- src/dto.rs
- src/app_state.rs
- src/api/*
- src/brain/*
- src/context/*
- src/model/*
- src/memory/*
- src/projects/*
- src/tools/*
- src/logs/*

### Implemented modules
- API Gateway
- Brain Engine
- Intent Router
- Context Assembler
- Prompt Builder
- Runtime Manager
- llama-server Client
- SQLite DB + migrations
- Memory Manager
- Project Manager
- Tool Registry
- Audit Log

### Checks run locally
```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
curl http://127.0.0.1:8787/status
curl -X POST http://127.0.0.1:8787/chat ...
```

### Result
- Daemon started successfully.
- `/status` returned ok=true and database_ready=true.
- `/chat` returned a real Qwen response with `mocked=false`.
- Commit/tag: `42f409a`, `brain-step1-core-boot`.

### Deferred
- Streaming SSE endpoint implementation
- MCP adapter
- Strong approval workflow UI/client contract
- Embeddings/vector memory
- Advanced project indexing
- Real tool schema validation
- Packaging/systemd service

---

## Step 2 — Add prompt policy files for Gigi persona

### Goal
Move the system prompt policy out of hardcoded Rust text into editable Markdown files, and establish the assistant personality as **Gigi**: Egyptian, feminine, practical, and consistent with Logixa Brain Core.

### Decisions implemented
- Persona name is **Gigi / جيجي**.
- Persona speaks as feminine Egyptian Arabic.
- Persona must not present herself as a male assistant.
- Persona should not start by saying "أنا AI" or "أنا نموذج".
- The brain remains a headless local service; persona is the conversational layer.
- Prompt policies are loaded from disk when available, with compile-time fallback using `include_str!`.

### Files added
- prompts/identity.md
- prompts/style_egyptian.md
- prompts/execution_rules.md
- prompts/memory_rules.md
- prompts/tool_rules.md
- prompts/modes/chat.md
- prompts/modes/task.md
- prompts/modes/code.md
- prompts/modes/planning.md
- prompts/modes/memory.md
- prompts/modes/tool.md
- src/context/prompt_loader.rs

### Files changed
- src/context/assembler.rs
- src/context/prompt_builder.rs
- src/context/mod.rs
- did.md
- todo.md

### Checks to run locally
```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
curl http://127.0.0.1:8787/status
curl -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"إنتِ مين؟","project_id":"logixa_brain","mode":"chat","use_memory":true}'
```

### Expected result
- Response should identify as **جيجي**.
- Response should use feminine Egyptian language.
- Response should avoid Gulf phrasing like "إيش" and "تبيه".
- Response should preserve Brain Core architecture: service independent, UI is only client.

### Deferred
- Prompt versioning in database.
- Prompt editing API.
- Prompt reload endpoint.
- Per-project prompt overrides.

---

## Step 3 — Add conversation history context

### Goal
Make `/chat` continue an existing conversation when `conversation_id` is provided, and inject the latest messages from that conversation into the model prompt so Gigi can answer with short-term conversational context.

### Decisions implemented
- Conversation continuation is explicit through `conversation_id` in `ChatRequest`.
- New conversations are still created automatically when `conversation_id` is omitted.
- Provided `conversation_id` is validated before saving the new user message.
- If the conversation belongs to a different project than the requested `project_id`, the request returns a clear error instead of mixing project scopes.
- Recent conversation messages are loaded using `max_recent_messages` from `brain_config.toml`.
- The response now reports:
  - `history_used`
  - `history_messages_used`
- Event stream now includes `conversation_history_loaded` when prior messages are used.
- No UI, model runtime, tool execution, or MCP work was added in this step.

### Files changed
- src/dto.rs
- src/brain/engine.rs
- src/memory/db.rs
- README.md
- did.md
- todo.md

### Checks to run locally
```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

In another terminal:
```bash
FIRST=$(curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"اسمي مصطفى. افتكري الاسم في المحادثة دي فقط.","project_id":"logixa_brain","mode":"chat","use_memory":true}')

CID=$(printf '%s' "$FIRST" | python3 -c 'import json,sys; print(json.load(sys.stdin)["conversation_id"])')

echo "$FIRST"

echo "Conversation: $CID"

curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d "{\"message\":\"أنا اسمي إيه؟\",\"conversation_id\":\"$CID\",\"project_id\":\"logixa_brain\",\"mode\":\"chat\",\"use_memory\":true}"
```

### Expected result
- The second response should answer using the previous message context.
- Response should include `history_used:true`.
- Response should include `history_messages_used` greater than 0.
- Response events should include `conversation_history_loaded`.
- Response should still include `mocked:false` when the real model is working.

### Deferred
- `/conversations` listing endpoint.
- Conversation rename/delete API.
- Conversation pagination.
- Summarized long-history compression.
- Per-project chat history browser UI.
