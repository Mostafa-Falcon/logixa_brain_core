# todo.md — Logixa Brain Core Roadmap

## Current status
- Step 1 complete: `logixa-brain-daemon` builds, runs, answers `/status`, initializes SQLite, and answers `/chat` through Qwen with `mocked=false`.
- Step 2 complete: prompt policy files added, Gigi persona activated, and system prompt moved out of hardcoded Rust text.
- Step 3 active: conversation history context.

## Active step

### Step 3 — Conversation History Context
- [x] Validate provided `conversation_id` before saving messages.
- [x] Continue an existing conversation when `conversation_id` is provided.
- [x] Keep project scope stable when continuing a conversation.
- [x] Load recent messages using `max_recent_messages`.
- [x] Inject recent conversation messages into the prompt.
- [x] Return `history_used` and `history_messages_used` in `/chat` response.
- [x] Add `conversation_history_loaded` event when previous messages are used.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo check`.
- [ ] Run daemon and test a two-message conversation.
- [ ] Commit/tag after successful checks.

## Next steps

### Step 4 — Conversation Management API
- Add `GET /conversations`.
- Add `GET /conversations/:id/messages`.
- Add rename conversation endpoint.
- Add delete conversation endpoint.
- Keep this API UI-neutral so Flutter/Tauri/Web clients can all use it.

### Step 5 — Memory polish
- Improve FTS query normalization.
- Add memory update/delete.
- Add memory scopes API contract.
- Add auto-save decision memory later, only after approval policy is clearer.

### Step 6 — SSE streaming
- Add `/chat/stream`.
- Stream lifecycle events.
- Stream generated tokens if llama-server streaming is enabled.

### Step 7 — Tool safety
- Add allowed paths config.
- Add stronger timeout rules.
- Add write_file only after approval.
- Add tool run audit details.

### Step 8 — Connectors
- Add CLI client.
- Add EDL connector contract.
- Add StoreOS connector contract.
- Add MCP adapter later.

### Step 9 — Packaging
- Build release binary.
- Add systemd user service template for Linux.
- Add backup/export commands.
