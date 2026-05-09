# todo.md — Logixa Brain Core Roadmap

## Current status
- Step 1 complete: `logixa-brain-daemon` builds, runs, answers `/status`, initializes SQLite, and answers `/chat` through Qwen with `mocked=false`.
- Step 2 complete: prompt policy files added, Gigi persona activated, and system prompt moved out of hardcoded Rust text.
- Step 3 complete: `/chat` supports `conversation_id`, injects recent conversation history, and returns `history_used` / `history_messages_used`.
- Step 3.1 active: Gigi Egyptian style guard hotfix.

## Active step

### Step 3.1 — Gigi Egyptian Style Guard Hotfix
- [x] Keep Gigi identity short and clear.
- [x] Strengthen Egyptian feminine style policy.
- [x] Reduce Chat Mode verbosity.
- [x] Add minimal response cleaner for repeated non-Egyptian leaks.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo check`.
- [ ] Run daemon and test Gigi identity/style.
- [ ] Commit/tag after successful checks.

## Next step candidates — choose after Step 3.1

### Candidate A — Step 4: Long-Term Memory v1
- Improve manual memory save/search behavior.
- Inject relevant long-term memories into prompt when useful.
- Keep `memory_used=true` only when memories are actually injected.
- Keep memory lightweight with SQLite/FTS only.

### Candidate B — Step 4: Conversation Management API
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
