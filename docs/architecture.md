# Form Architecture

Form starts with two crates:

- `form-core` — shared domain types: messages, tool calls/results, approvals, errors, limits.
- `form-cli` — CLI plus internal modules for the host loop.

Split more crates only after a module has a stable API, a real caller, and a boundary test.

## Module graph

```text
form-cli
  ├─▶ agent ─▶ provider ─▶ form-core
  │     ├─▶ tools ─▶ workspace ─▶ form-core
  │     ├─▶ session ─▶ workspace
  │     ├─▶ resources ─▶ workspace
  │     ├─▶ approval ─▶ audit ─▶ form-core
  │     └─▶ form-core
  ├─▶ session
  ├─▶ resources
  ├─▶ approval
  ├─▶ audit
  └─▶ form-core

plugin (M4)
  ├─▶ approval
  ├─▶ audit
  ├─▶ workspace
  └─▶ form-core
```

Rules:

- `form-core` depends on no Form crate.
- `provider` never depends on `tools`, `session`, `resources`, `approval`, or `plugin`.
- `tools` never call `provider`; `agent` owns the next model turn.
- `resources` owns `FORM.md` / `AGENTS.md` loading. `CLAUDE.md` is not loaded by default.
- `plugin` never mutates workspace directly. It can request bounded host APIs and propose patches only. WIT design waits until M4, after host APIs are proven.

## Error envelope

All failures use a common envelope: `kind`, `source`, `retryable`, `user_message`, `audit_level`, `exit_code`, `details`, `cause`, and `redact_details`.

## Safety defaults

- Read and search both apply workspace containment, deterministic traversal, default skip dirs, nested-repo boundaries, size caps, early-stop search limits, and secret-deny patterns before returning model-visible content.
- Writes and edits revalidate path containment after approval and before mutation; new files validate nearest existing parent and create temp files inside the target directory.
- Patch apply is atomic: stale snapshots, failed hunks, binary files, mode changes, renames, deletes, unclear line endings, and malformed diffs do not mutate files.
- Mutations and shell require approval. Default answer is deny.
- Approved shell is the user's shell, not a sandbox; Form prevents accidental model-requested mutation through approval, risk display, timeouts, and output caps.
- Approval/audit entries for mutation-critical events use `sync_data` or stronger before returning success; creates/renames also sync parent directories where supported.
- Session/audit JSONL never silently drops corrupt data.
- Session entries include `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id` from M1.

## M4 plugin sandbox defaults

- Timeout: 2 seconds per plugin command.
- Memory: 64 MiB per plugin instance.
- Output: 50 KB or 2000 lines.
- Use Wasmtime fuel or epoch interruption when available.
- Timeout, memory, output-cap, trap, and denied-permission failures return structured errors and audit entries.

## Line-mode UX rule

`form chat` shows provider requests/retries, each tool call, inline approvals, cancellations, malformed tool-output errors, and final assistant text in turn order. M1/M2 execute multiple tool calls sequentially.

See `docs/mvp.md` for full milestones and data contracts.
