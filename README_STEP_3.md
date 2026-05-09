# Step 3 — Conversation History Context

This step makes Logixa Brain Core continue an existing conversation when the client sends `conversation_id`.

## What changed

- `/chat` now validates `conversation_id` before saving the new message.
- Existing conversation project scope is respected.
- Project mismatch returns a clear request error.
- Recent conversation messages are injected into the prompt.
- `/chat` response now includes:
  - `history_used`
  - `history_messages_used`
- Event list includes `conversation_history_loaded` when previous messages are used.

## Files changed

- `src/dto.rs`
- `src/brain/engine.rs`
- `src/memory/db.rs`
- `README.md`
- `did.md`
- `todo.md`

## Checks

```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

Two-message test:

```bash
FIRST=$(curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"اسمي مصطفى. افتكري الاسم في المحادثة دي فقط.","project_id":"logixa_brain","mode":"chat","use_memory":true}')

CID=$(printf '%s' "$FIRST" | python3 -c 'import json,sys; print(json.load(sys.stdin)["conversation_id"])')

echo "$FIRST"
echo "Conversation: $CID"

curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d "{\"message\":\"أنا اسمي إيه؟\",\"conversation_id\":\"$CID\",\"project_id\":\"logixa_brain\",\"mode\":\"chat\",\"use_memory\":true}"
```

Expected: second response uses the previous message, `history_used` is true, and `history_messages_used` is greater than 0.
