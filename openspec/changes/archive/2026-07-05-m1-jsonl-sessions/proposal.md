## Why

M1 needs persisted conversation state before the host loop can survive process restarts or support future branching. JSONL storage now avoids a later migration by writing tree-capable session entries from the start.

## What Changes

- Add a `form-cli` session module that stores workspace-scoped session JSONL under `~/.form/sessions/<workspace-hash>/YYYYMMDD-HHMMSS.jsonl`.
- Implement append + flush behavior for session entries and resume-latest lookup for M1.
- Surface corrupt JSONL lines as session errors instead of silently dropping them.
- Add rendering/inspection fixtures for user, assistant, tool call, tool result, approval, audit, corrupt trailing line, and unknown future entry kind inputs.

## Capabilities

### New Capabilities
- `session-jsonl-store`: Workspace-scoped append-only session JSONL storage, latest-session resume, strict corrupt-line handling, and inspection-rendering fixture coverage.

### Modified Capabilities

## Impact

- Affects `crates/form-cli` session internals and runtime foundation tests.
- Reuses existing `form-core` session entry, tool result, approval, and audit contracts without changing their JSON schema.
- Adds small local-storage helpers only; no provider calls, read/search tools, mutation tools, WIT, Wasmtime, or plugin execution.
