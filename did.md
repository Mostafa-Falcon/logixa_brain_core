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

### Checks to run locally
```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
curl http://127.0.0.1:8787/status
```

### Deferred
- Streaming SSE endpoint implementation
- MCP adapter
- Strong approval workflow UI/client contract
- Embeddings/vector memory
- Advanced project indexing
- Real tool schema validation
- Packaging/systemd service
