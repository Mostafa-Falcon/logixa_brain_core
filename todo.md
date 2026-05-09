# todo.md — Logixa Brain Core Roadmap

## Current status
- Step 1 complete: `logixa-brain-daemon` builds, runs, answers `/status`, initializes SQLite, and answers `/chat` through Qwen with `mocked=false`.
- Step 2 complete: prompt policy files added, Gigi persona activated, and system prompt moved out of hardcoded Rust text.
- Step 3 complete: `/chat` supports `conversation_id`, injects recent conversation history, and returns `history_used` / `history_messages_used`.
- Step 3.1 complete: Gigi Egyptian style guard hotfix added.
- Step 4 complete: Long-Term Memory v1 works with SQLite/FTS and injects relevant memory into chat context.
- Step 5 active: Project State Memory.

## Active step

### Step 5 — Project State Memory
- [x] Add `project_states` SQLite table.
- [x] Add `ProjectStateRecord` and update request DTO.
- [x] Add project state read/update endpoints.
- [x] Inject active project and project state into prompt context.
- [x] Add `project_state_used` and `project_state_loaded` signals.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo check`.
- [ ] Run daemon and test project state save/read/chat injection.
- [ ] Commit/tag after successful checks.

## Next step candidates — choose after Step 5

### Candidate A — Step 6: Conversation Management API
- Add `GET /conversations`.
- Add `GET /conversations/:id/messages`.
- Add rename conversation endpoint.
- Add delete/archive conversation endpoint.
- Keep this API UI-neutral so Flutter/Tauri/Web clients can all use it.

### Candidate B — Step 6: Tool Approval Flow v1
- Make tool approvals explicit and logged.
- Add safer structured output for proposed tool calls.
- Keep dangerous tools disabled without approval.

### Later
- SSE streaming.
- CLI client.
- EDL connector contract.
- StoreOS connector contract.
- MCP adapter.
- Release packaging.
