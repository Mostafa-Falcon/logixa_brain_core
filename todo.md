# todo.md — Logixa Brain Core Roadmap

## Current status
- Step 1 complete: `logixa-brain-daemon` builds, runs, answers `/status`, initializes SQLite, and answers `/chat` through Qwen with `mocked=false`.
- Step 2 complete: prompt policy files added, Gigi persona activated, and system prompt moved out of hardcoded Rust text.
- Step 3 complete: `/chat` supports `conversation_id`, injects recent conversation history, and returns `history_used` / `history_messages_used`.
- Step 3.1 complete: Gigi Egyptian style guard hotfix added.
- Step 4 active: Long-Term Memory v1.

## Active step

### Step 4 — Long-Term Memory v1
- [x] Keep memory lightweight with SQLite/FTS only.
- [x] Support flexible manual memory save payloads.
- [x] Improve memory search with FTS normalization and fallbacks.
- [x] Inject long-term memories into chat context.
- [x] Add `memory_items_used` and `long_term_memory_loaded`.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo check`.
- [ ] Run daemon and test memory save/search/chat injection.
- [ ] Commit/tag after successful checks.

## Next step candidates — choose after Step 4

### Candidate A — Step 5: Project State Memory
- Store per-project current step/status.
- Track last committed tag per project.
- Add project-specific rules/context injection.
- Make Gigi know the active project state without relying only on chat history.

### Candidate B — Step 5: Conversation Management API
- Add `GET /conversations`.
- Add `GET /conversations/:id/messages`.
- Add rename conversation endpoint.
- Add delete/archive conversation endpoint.
- Keep this API UI-neutral so Flutter/Tauri/Web clients can all use it.

### Later
- SSE streaming.
- Stronger tool approval flow.
- CLI client.
- EDL connector contract.
- StoreOS connector contract.
- MCP adapter.
- Release packaging.
