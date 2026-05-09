# README_STEP_5 — Project State Memory

## Goal

Step 5 adds lightweight per-project state to Logixa Brain Core.

The goal is not UI and not project file indexing. The goal is for Gigi to know the active project's current state from the Brain database:

- current step
- last commit
- last tag
- summary
- next step
- status

## New API

### Get active project

```bash
curl -s http://127.0.0.1:8787/projects/active
```

### Read project state

```bash
curl -s http://127.0.0.1:8787/projects/logixa_brain/state
```

### Update project state

```bash
curl -s -X POST http://127.0.0.1:8787/projects/logixa_brain/state \
  -H "Content-Type: application/json" \
  -d '{
    "current_step":"Step 5 — Project State Memory",
    "last_commit":"af8a814",
    "last_tag":"brain-step4-long-term-memory",
    "summary":"Logixa Brain Core has boot, Gigi prompt policy, conversation history, Egyptian style guard, and long-term memory v1.",
    "next_step":"Validate project state injection, then commit/tag Step 5.",
    "status":"active"
  }'
```

## Chat test

```bash
curl -s -X POST http://127.0.0.1:8787/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"إحنا واقفين فين في مشروع العقل؟","project_id":"logixa_brain","mode":"chat","use_memory":true}'
```

Expected response fields:

```json
"project_state_used": true
```

Expected event:

```json
"project_state_loaded"
```

## Checks

```bash
cargo fmt
cargo check
cargo run --bin logixa-brain-daemon
```

## Commit after success

```bash
git status --short
git add .
git commit -m "step 5 add project state memory"
git tag brain-step5-project-state-memory
git push
git push origin brain-step5-project-state-memory
```
