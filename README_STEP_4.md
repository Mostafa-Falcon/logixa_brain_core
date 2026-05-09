# Step 4 — Long-Term Memory v1

## Goal
Add a lightweight long-term memory layer that is separate from conversation history and remains UI-neutral.

This step keeps the Brain Core headless and local:

- No UI work.
- No Vector DB.
- No embeddings.
- No model/runtime changes.
- SQLite + FTS5 only.

## What changed

### Memory API
`POST /memory/save` now supports both styles:

```json
{
  "scope": "user",
  "kind": "preferred_name",
  "content": "مصطفى",
  "tags": ["identity", "user"],
  "importance": 5
}
```

and:

```json
{
  "scope": "user",
  "key": "preferred_name",
  "value": "مصطفى",
  "tags": ["identity", "user"]
}
```

`POST /memory/search` supports optional filters:

```json
{
  "query": "مصطفى",
  "scope": "user",
  "project_id": "logixa_brain",
  "kind": "preferred_name",
  "limit": 6
}
```

### Retrieval behavior
Memory retrieval now uses:

1. SQLite FTS query.
2. LIKE fallback if FTS returns nothing.
3. Top filtered memories fallback if both return nothing.

This is intentional for v1 because small local models often need important user memories injected even when the user's wording is indirect.

### Chat response
`/chat` now returns:

```json
{
  "memory_used": true,
  "memory_items_used": 1,
  "events": ["long_term_memory_loaded"]
}
```

when long-term memories are injected into the prompt.

## Checks

```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

In another terminal:

```bash
curl -s -X POST http://127.0.0.1:8787/memory/save \
  -H "Content-Type: application/json" \
  -d '{"scope":"user","key":"preferred_name","value":"مصطفى","tags":["identity","user"],"importance":5}'
```

Search memory:

```bash
curl -s -X POST http://127.0.0.1:8787/memory/search \
  -H "Content-Type: application/json" \
  -d '{"query":"مصطفى","scope":"user","limit":6}'
```

Ask in a new chat:

```bash
curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"أنا اسمي إيه؟","project_id":"logixa_brain","mode":"chat","use_memory":true}'
```

Expected:

- `memory_used:true`
- `memory_items_used` greater than 0
- `events` includes `long_term_memory_loaded`
- Gigi answers using the saved memory.

## Deferred

- Memory delete/update API.
- Memory pinning endpoint.
- Memory source attribution in chat response.
- Embeddings/vector search.
- Automatic memory extraction from conversations.
