# todo.md — Logixa Brain Core Roadmap

## Current MVP target
Make `logixa-brain-daemon` run locally, respond to `/status`, create SQLite database, and answer `/chat` using Qwen through llama-server or mock fallback.

## Next steps

### Step 2 — Local compile and config fix
- Run `cargo fmt`
- Run `cargo check`
- Fix any Rust compile issues from local environment
- Set absolute `model_path`
- Set absolute `llama_server_binary`

### Step 3 — Real Qwen inference
- Start `/chat` with real Qwen response
- Verify llama-server health
- Verify idle shutdown
- Save successful response in `messages`

### Step 4 — SSE streaming
- Add `/chat/stream`
- Stream lifecycle events
- Stream generated tokens if llama-server streaming is enabled

### Step 5 — Memory polish
- Improve FTS query normalization
- Add memory update/delete
- Add memory scopes UI/API contract

### Step 6 — Tool safety
- Add allowed paths config
- Add stronger timeout rules
- Add write_file only after approval
- Add tool run audit details

### Step 7 — Connectors
- Add CLI client
- Add EDL connector contract
- Add StoreOS connector contract
- Add MCP adapter later

### Step 8 — Packaging
- Build release binary
- Add systemd user service template for Linux
- Add backup/export commands
