# todo.md — Logixa Brain Core Roadmap

## Current status
Step 1 is complete: `logixa-brain-daemon` builds, runs, answers `/status`, initializes SQLite, and answers `/chat` through Qwen with `mocked=false`.

## Active step

### Step 2 — Prompt Policy Files
- [x] Add editable prompt policy files under `prompts/`.
- [x] Add Gigi identity/persona policy.
- [x] Add Egyptian feminine style policy.
- [x] Add execution, memory, and tool rules.
- [x] Add mode-specific prompt files.
- [x] Add `PromptLoader`.
- [x] Update `ContextAssembler` to load prompt policies.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo check`.
- [ ] Run daemon and test `/chat`.
- [ ] Commit/tag after successful checks.

## Next steps

### Step 3 — Prompt verification and response correction
- Test identity question: "إنتِ مين؟"
- Test Egyptian language stability.
- Test Task Mode response shape.
- Test Code Mode response shape.
- Adjust prompt files only if model drifts.

### Step 4 — SSE streaming
- Add `/chat/stream`.
- Stream lifecycle events.
- Stream generated tokens if llama-server streaming is enabled.

### Step 5 — Memory polish
- Improve FTS query normalization.
- Add memory update/delete.
- Add memory scopes API contract.

### Step 6 — Tool safety
- Add allowed paths config.
- Add stronger timeout rules.
- Add write_file only after approval.
- Add tool run audit details.

### Step 7 — Connectors
- Add CLI client.
- Add EDL connector contract.
- Add StoreOS connector contract.
- Add MCP adapter later.

### Step 8 — Packaging
- Build release binary.
- Add systemd user service template for Linux.
- Add backup/export commands.
