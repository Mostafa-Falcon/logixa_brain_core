# Step 6 — Tool Approval Flow v1

## Goal

Add a safe approval flow before Logixa Brain Core can execute tools.

This step keeps the brain headless and safe:

```txt
Any UI / CLI / app requests a tool
→ Brain creates pending approval
→ User/client approves or rejects
→ Brain executes only after approval
→ Result is logged in tool_runs
```

## Scope

Included tools in v1:

```txt
list_files
git_status
git_diff
```

All three require approval in this step.

Not included yet:

```txt
read_file
run_command
write_file
git_commit
git_push
```

## New endpoints

```txt
GET  /tools
POST /tools/request
GET  /tools/pending
POST /tools/approve/:id
POST /tools/reject/:id
GET  /tools/runs
```

`POST /tools/run` is intentionally disabled in Step 6. Use request → approve instead.

## Apply

```bash
cd ~/logixa_ai/logixa_brain_core

ZIP="$HOME/Downloads/logixa_brain_step6_tool_approval_flow.zip"
WORK="/tmp/logixa_brain_step6_tool_approval_flow"

rm -rf "$WORK"
mkdir -p "$WORK"

unzip -o "$ZIP" -d "$WORK"
bash "$WORK/logixa_brain_step6_tool_approval_flow_pkg/apply_step6_tool_approval_flow.sh"

cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

## Test list tools

```bash
curl -s http://127.0.0.1:8787/tools
```

## Test git_status approval flow

Request the tool:

```bash
REQ=$(curl -s -X POST http://127.0.0.1:8787/tools/request \
  -H "Content-Type: application/json" \
  -d '{"tool_name":"git_status","project_id":"logixa_brain","input":{"path":"/home/logixa/logixa_ai/logixa_brain_core"}}')

APPROVAL_ID=$(printf '%s' "$REQ" | python3 -c 'import json,sys; print(json.load(sys.stdin)["approval_id"])')

echo "$REQ"
echo "Approval: $APPROVAL_ID"
```

Check pending calls:

```bash
curl -s http://127.0.0.1:8787/tools/pending
```

Approve and execute:

```bash
curl -s -X POST "http://127.0.0.1:8787/tools/approve/$APPROVAL_ID" \
  -H "Content-Type: application/json" \
  -d '{"note":"approved git_status read-only test"}'
```

Check runs:

```bash
curl -s http://127.0.0.1:8787/tools/runs
```

## Test reject flow

```bash
REQ=$(curl -s -X POST http://127.0.0.1:8787/tools/request \
  -H "Content-Type: application/json" \
  -d '{"tool_name":"git_diff","project_id":"logixa_brain","input":{"path":"/home/logixa/logixa_ai/logixa_brain_core"}}')

APPROVAL_ID=$(printf '%s' "$REQ" | python3 -c 'import json,sys; print(json.load(sys.stdin)["approval_id"])')

curl -s -X POST "http://127.0.0.1:8787/tools/reject/$APPROVAL_ID" \
  -H "Content-Type: application/json" \
  -d '{"note":"testing rejection"}'
```

## Expected result

- `/tools/request` returns `approval_id`.
- `/tools/pending` shows pending tool calls.
- `/tools/approve/:id` executes the approved tool.
- `/tools/reject/:id` rejects without execution.
- `/tools/runs` shows executed tool runs.

## Commit after success

```bash
git status --short
git add .
git commit -m "step 6 add tool approval flow v1"
git tag brain-step6-tool-approval-flow
git push
git push origin brain-step6-tool-approval-flow

git status --short
git log --oneline --decorate -8
```
