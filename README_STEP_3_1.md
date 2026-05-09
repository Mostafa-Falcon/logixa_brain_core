# Step 3.1 — Gigi Egyptian Style Guard Hotfix

This hotfix stabilizes Gigi's Egyptian feminine style before adding more memory or connector features.

## Scope

Included:
- Shorter identity prompt.
- Stronger Egyptian style guard prompt.
- Shorter Chat Mode prompt.
- Minimal deterministic response cleaner for repeated style leakage.

Not included:
- No UI.
- No model runtime changes.
- No memory schema changes.
- No tool changes.
- No MCP/connector work.

## Apply

```bash
cd ~/logixa_ai/logixa_brain_core

ZIP="$HOME/Downloads/logixa_brain_step3_1_gigi_style_guard.zip"
WORK="/tmp/logixa_brain_step3_1_gigi_style_guard"

rm -rf "$WORK"
mkdir -p "$WORK"

unzip -o "$ZIP" -d "$WORK"
bash "$WORK/step3_1_gigi_style_guard_pkg/apply_step3_1_gigi_style_guard.sh"

cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

## Test

In another terminal:

```bash
curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"إنتِ مين؟","project_id":"logixa_brain","mode":"chat","use_memory":true}'

curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"قوليلي نعمل إيه النهارده من غير رغي","project_id":"logixa_brain","mode":"chat","use_memory":true}'
```

Expected:
- Gigi identifies herself naturally.
- No "إيش" / "تبيه" / "وش".
- Feminine wording is preserved.
- Chat Mode remains concise.

## Commit

```bash
git status --short
git add .
git commit -m "step 3.1 add gigi egyptian style guard"
git tag brain-step3-1-gigi-style-guard
git push
git push origin brain-step3-1-gigi-style-guard
```
