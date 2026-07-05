## Context

`form-core` already defines tree-capable `SessionEntry` JSON contracts, but `form-cli` does not persist sessions yet. M1 needs a small local session store before the provider loop, read/search tools, and later `form inspect` can share one source of truth.

## Goals / Non-Goals

**Goals:**
- Store session JSONL outside the repo under `~/.form/sessions/<workspace-hash>/`.
- Create timestamped session files named `YYYYMMDD-HHMMSS.jsonl`.
- Append serialized `SessionEntry` values and flush before callers continue.
- Resume the latest workspace session for M1.
- Read/render fixture JSONL while surfacing corrupt lines and preserving unknown future kinds.

**Non-Goals:**
- No `form chat`, provider calls, read/search tool execution, mutation tools, approvals, global audit log, WIT, Wasmtime, or plugin execution.
- No full session tree UI, branch selection, compaction, replay, or diff.
- No `form sessions` or `form inspect` command wiring; this change only creates the storage and rendering helpers they will use.

## Decisions

- Keep the session store as `form-cli::session`. The repo architecture keeps runtime modules internal to `form-cli` until a crate boundary has multiple real callers.
- Use the existing `form_core::SessionEntry` type for writes. This avoids duplicating the session schema and preserves `session_id`, `branch_id`, `entry_id`, and `parent_entry_id` from M1.
- Read timeline JSONL as raw `serde_json::Value` for rendering. Valid JSON with an unknown `kind` renders as an unknown entry; invalid JSON returns a `SessionJsonlError` with the path and line number.
- Derive the workspace directory name from a stable FNV-1a hash of the canonical workspace root string. This uses no new hash dependency and stays stable across runs on Linux and Windows.
- Add a small date/time dependency only for filename formatting. `std::time` cannot format `YYYYMMDD-HHMMSS`; using a maintained formatter is less code and safer than handwritten calendar math.
- Pick resume-latest by sorting `*.jsonl` filenames and choosing the last timestamped name. Timestamp names make this deterministic and avoid platform-specific mtime behavior.
- Flush every appended line with `BufWriter::flush`. M1 chat turns need visibility before the next step; mutation-critical `sync_data` remains for M2 audit/mutation evidence.
- Render fixture lines into compact human-readable strings for each M1 shape: user, assistant, tool call, tool result, approval, audit, error/malformed, and unknown kind.

## Risks / Trade-offs

- Filename collisions are possible if two sessions start in the same second → add a numeric suffix later if concurrent session creation becomes real.
- FNV-1a is not cryptographic → acceptable because the hash is only a local directory key, not a security boundary.
- Raw-value rendering can miss malformed field types inside otherwise valid JSON → acceptable for M1 inspection; strict typed validation remains on write paths.
- `flush` does not guarantee durable disk persistence after power loss → acceptable for non-mutation M1 chat entries; M2 will add `sync_data` for approval and mutation evidence.
- Home directory lookup differs by platform → reuse the existing `HOME`/`USERPROFILE` approach from config and keep tests path-injected where possible.

## Migration Plan

- Add the new module and tests without changing existing runtime behavior.
- Future `form chat`, `form resume`, `form sessions`, and `form inspect` commands will call the new store APIs.
- Rollback is deleting the new module, dependency, and tests; no repo data migration is needed because session files live outside the repo.

## Open Questions

- Whether concurrent same-second session creation needs suffixes now or can wait until parallel sessions exist.
- Whether M3 branch indexes should be derived on demand from JSONL or cached in a sidecar file.
