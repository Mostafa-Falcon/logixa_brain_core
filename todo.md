# Logixa Brain Core TODO

## Current stable status

Completed:
- Step 1 — Brain Core Boot
- Step 2 — Gigi Prompt Policy
- Step 3 — Conversation History Context
- Step 3.1 — Gigi Egyptian Style Guard
- Step 4 — Long-Term Memory v1
- Step 5 — Project State Memory
- Step 6 — Tool Approval Flow v1

## Next recommended steps

### Step 7 — CLI Client v1

Goal:
- Add a small `logixa-brain` CLI client for:
  - status
  - chat
  - memory save/search
  - project state get/set
  - tool request/pending/approve/reject

Why:
- The brain is headless.
- A CLI makes testing and future UI integration easier without adding a UI framework.

### Step 8 — Safe Command Runner v1

Goal:
- Add approval-protected `run_command` with tight controls:
  - command allow/deny policy
  - cwd validation
  - timeout
  - output limits
  - audit logging

### Step 9 — Patch Proposal System

Goal:
- Let Gigi propose file edits as patches, not write files directly.
- Apply patches only after explicit approval.

## Deferred

- UI client
- MCP adapter
- embeddings/vector search
- multiple model profile switching
- automatic tool planning from chat
