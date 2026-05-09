# Logixa Brain Core v0.1

**Logixa Brain Core** is a lightweight headless local AI brain service.

It is not a UI app. It is a local service that any UI or product can connect to.

```txt
Any UI / App
Flutter / Tauri / Web / EDL / StoreOS
        │
        │ HTTP JSON API
        ▼
Logixa Brain Core
Rust daemon + SQLite + llama.cpp / llama-server
```

## Core decisions

- Brain is independent from EDL.
- UI is only a client.
- Model runtime is managed by the Brain, not the UI.
- Memory lives in SQLite.
- No Vector DB in v0.1.
- No Firebase, no Python runtime, no UI framework inside the core.
- `llama-server` is started on demand and can be stopped after idle timeout.

## What this MVP includes

- `logixa-brain-daemon`
- Local HTTP API
- SQLite database migrations
- Conversations/messages persistence
- Memory save/search using SQLite FTS5
- Project registry and active project
- Model runtime manager for `llama-server`
- OpenAI-compatible `/v1/chat/completions` call to llama-server
- Mock fallback for testing the daemon before model setup
- Tool registry with safe starter tools
- Audit/system events

## Requirements

- Rust toolchain
- llama.cpp build with `llama-server`
- Qwen GGUF model, for example:
  - `Qwen3.5-4B-UD-Q4_K_XL.gguf`

## Setup

```bash
cd logixa_brain_core
cp brain_config.example.toml brain_config.toml
nano brain_config.toml
```

Edit:

```toml
model_path = "/absolute/path/to/Qwen3.5-4B-UD-Q4_K_XL.gguf"
llama_server_binary = "/absolute/path/to/llama-server"
```

Then run:

```bash
cargo run --bin logixa-brain-daemon
```

Or:

```bash
./scripts/start_dev.sh
```

## Default ports

```txt
Brain API:    http://127.0.0.1:8787
llama-server: http://127.0.0.1:8788
```

## API examples

### Status

```bash
curl http://127.0.0.1:8787/status
```

### Chat

```bash
curl -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "إنت مين؟",
    "project_id": "logixa_brain",
    "mode": "auto",
    "use_memory": true
  }'
```

### Save memory

```bash
curl -X POST http://127.0.0.1:8787/memory/save \
  -H "Content-Type: application/json" \
  -d '{
    "scope": "project",
    "project_id": "logixa_brain",
    "kind": "decision",
    "content": "Brain Core is a headless local service. UI is external client only.",
    "tags": "architecture,brain,core",
    "importance": 5
  }'
```

### Search memory

```bash
curl -X POST http://127.0.0.1:8787/memory/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "headless local service",
    "project_id": "logixa_brain",
    "limit": 5
  }'
```

### Model start/stop

```bash
curl -X POST http://127.0.0.1:8787/model/start
curl -X POST http://127.0.0.1:8787/model/stop
```

### Tools

```bash
curl http://127.0.0.1:8787/tools
```

```bash
curl -X POST http://127.0.0.1:8787/tools/run \
  -H "Content-Type: application/json" \
  -d '{
    "name": "git_status",
    "input": {"path": "/path/to/repo"}
  }'
```

Dangerous tool example requires approval:

```bash
curl -X POST http://127.0.0.1:8787/tools/run \
  -H "Content-Type: application/json" \
  -d '{
    "name": "run_command",
    "approved": true,
    "input": {
      "cwd": "/path/to/repo",
      "command": "git status --short",
      "timeout_seconds": 20
    }
  }'
```

## First checks

```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
curl http://127.0.0.1:8787/status
```

## Important note

If `mock_when_unavailable = true`, `/chat` can return a mock response when llama-server or the GGUF model is not ready. This lets you verify the Brain API, database, logging, and integration contracts before real model inference.

Set it to false when you want strict real-model-only behavior.
