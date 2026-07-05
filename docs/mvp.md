# Form MVP Specification

## 1. Purpose

Form is a Rust-native replacement for Pi, built for daily coding-agent use.

The original idea was a Wasmtime/WIT plugin-host MVP. That is still part of the thesis, but it is no longer the first milestone. A plugin runtime that cannot run a real coding-agent session does not replace Pi.

The MVP now validates this product shape:

- Rust core owns the daily agent loop.
- Host-owned tools can read, search, edit, write, and run shell with explicit approval.
- Sessions are persistent, tree-capable, and auditable from day one.
- Project context files are loaded automatically.
- WASM Component plugins arrive after the host loop works, as a sandboxed extension firewall.

## 2. One-Line MVP Definition

Build a Rust CLI that can run one real coding-agent session with OpenAI Responses, project context files, persistent JSONL sessions, host-owned read/search/edit/write/shell tools, approval gates, and an architecture-ready WASM plugin boundary to implement after the core loop works.

## 3. Core Product Hypothesis

A Pi replacement is valuable when the user can trust it as the daily terminal agent harness.

The durable architecture is:

- Rust owns core state and performance-sensitive workflow.
- The host owns model calls, tools, sessions, approvals, patch application, and audit logs.
- Plugins are untrusted extensions. They request bounded host APIs and propose changes, but they do not mutate the workspace directly.
- WASM is for sandboxing, resource control, stable API boundaries, and language-neutral extension, not a magic speed path for TypeScript.

## 4. MVP Scope

### 4.1 In Scope

The first complete MVP includes:

- Rust CLI application.
- Line-mode `form chat` command.
- OpenAI Responses provider first.
- Project context loading from `FORM.md` and `AGENTS.md`.
- Persistent JSONL session store with tree-capable IDs.
- Host-owned tools:
  - read file,
  - search workspace,
  - propose/apply approved edits,
  - write approved files,
  - run approved shell with timeout.
- Approval pipeline for mutations and shell.
- Workspace root detection and path containment.
- Local audit events.
- Structured errors.
- Data contracts for sessions, tools, approvals, providers, patches, and plugin manifest constraints.
- M4 WASM Component plugin milestone:
  - manifest parsing,
  - register/list/run,
  - bounded search/read/log/propose-patch host APIs,
  - default-deny permissions.

### 4.2 Later Scope

These matter, but must not block the daily-driver MVP:

- Full TUI polish.
- Pi command parity.
- Skills and prompt templates with exact Pi semantics.
- TypeScript plugin SDK.
- Plugin marketplace.
- Signed plugin distribution.
- Package manager.
- Multi-agent orchestration.
- Cloud sync.
- Persistent plugin daemons.
- Full LSP integration.
- Node sidecar bridge.
- Advanced prompt-injection taint tracking.
- Cross-machine plugin cache.

### 4.3 Out of Scope for M1/M2

The early MVP must not attempt to:

- Implement Wasmtime before the host loop and mutation approval work.
- Let plugins run shell commands.
- Let plugins directly access arbitrary filesystem paths.
- Let plugins directly access environment variables or secrets.
- Let plugins open network connections.
- Let plugins directly apply patches.
- Build a marketplace-grade security model.
- Treat TypeScript-to-WASM as a performance optimization.

## 5. Design Principles

### 5.1 Daily Driver First

If Form cannot replace one real Pi coding session, the architecture has not proven itself.

M1 must support:

- asking a model for help,
- letting the model call read/search tools,
- storing the session,
- loading project context,
- recovering cleanly from tool errors.

M2 adds approved mutation tools.

### 5.2 Rust Owns Core State

The Rust host owns:

- model loop,
- session tree,
- workspace index,
- file snapshots,
- tool registry,
- approval decisions,
- patch proposals and application,
- provider adapters,
- runtime budgets,
- audit logs,
- plugin registry.

Plugins must not own or mutate global state directly.

### 5.3 Host-Mediated Everything

The host mediates:

- file access,
- search,
- edits and writes,
- shell execution,
- patch application,
- LLM calls,
- secrets access,
- plugin permissions,
- git operations.

### 5.4 Default Deny for Plugins

Plugin permissions start disabled.

A plugin manifest can request permissions, but the host decides what is granted. Every plugin host API call checks permissions again.

### 5.5 Handles and Bounded Results

Large state stays in the host. Plugins and tools pass handles or small JSON payloads, not giant session/workspace dumps.

All host APIs return bounded results.

### 5.6 Behavioral Compatibility Before Full Parity

Form should match the Pi workflows the user actually depends on before copying every Pi command.

## 6. High-Level Architecture

Initial crates/modules:

1. `form-core` crate — domain types: messages, tool calls, approvals, errors, limits.
2. `form-cli` crate — command parsing, config path resolution, top-level commands, and these internal modules until M1/M2 behavior proves a split is worth it:
   - `agent` — model loop, tool dispatch, turn lifecycle, retry boundaries.
   - `provider` — OpenAI Responses adapter first, behind a narrow trait.
   - `tools` — host-owned read/search/write/edit/shell tools.
   - `workspace` — root detection, path containment, text detection, search.
   - `session` — JSONL session store with tree-capable IDs.
   - `resources` — context file loading, later skills/prompts.
   - `approval` — unified approval pipeline.
   - `plugin` — WIT/Wasmtime boundary, implemented in M4.
   - `audit` — global append-only logs for security and operations.

Split an internal module into its own crate only after it has a stable contract, a real caller, and a test that proves the boundary.

## 7. Milestones

### M0: Architecture Freeze

Deliverables:

- Module boundaries.
- Error model.
- Session entry schema.
- Tool trait.
- Approval request/result model.
- Patch proposal format.
- Resource loading rules.
- OpenAI provider request/result contract.
- Plugin manifest constraints and host API constraints. WIT waits until M4, after M1/M2 prove the host APIs.

No WIT or Wasmtime implementation in M0.

Exit criteria:

- An agent can scaffold the Rust workspace without major architecture questions.

### M1: Host Loop Skeleton

Deliverables:

- `form chat` line-mode CLI.
- OpenAI Responses provider.
- Config loading from `~/.form/config.toml` and `OPENAI_API_KEY`.
- Context file loading.
- JSONL session append and resume latest.
- Minimal session inspection: `form sessions` and `form inspect --latest`.
- Read/search tools.
- Structured tool errors.

Exit criteria:

- CI proves the loop with a scripted provider: context load, read/search tool call, tool result, and session append.
- With `OPENAI_API_KEY` set, a manual M1 smoke test proves Form can answer a real coding question in this repo, read files, search files, and persist the session.

### M2: Approved Mutation Tools

Deliverables:

- Edit tool with diff preview and approval.
- Write tool with approval.
- Shell tool with approval, timeout, output caps, and cancellation.
- Patch proposal validation and application.
- Audit entries for approvals, denials, mutations, and timeouts.

Exit criteria:

- Form can complete a small docs/code edit with approved host-owned tools.

### M3: Session Branching

Deliverables:

- Resume specific session.
- Basic fork from a prior entry.
- Basic tree navigation.
- Tree-compatible storage already exists from M1.

Exit criteria:

- User can branch a session without losing prior history.

Replay, diff, and compaction stay deferred until confirmed daily-critical.

### M4: Minimal WASM Plugin

Deliverables:

- `form plugin check <manifest>` no-exec dry-run.
- Plugin manifest parsing.
- Register/list/inspect plugins.
- Wasmtime Component loading.
- Run one plugin command.
- Host imports for search/read/log/propose-patch.
- Permission checks and resource limits.

Exit criteria:

- A sample plugin can search/read bounded workspace data and propose a patch that the host can inspect but not auto-apply.

### M5: Extra Hardening

Core safety tests do not wait for M5:

- M1 owns read/search path traversal, symlink escape, secret-deny, binary/oversized file, and output cap tests.
- M2 owns edit/write/shell approval, timeout, cancellation, malformed patch, patch conflict, stale snapshot, and atomicity tests.
- M4 owns plugin permission, trap, timeout, memory, and output-cap tests.

M5 adds stress and malicious-input depth after those gates pass.

Exit criteria:

- Broken or malicious plugins cannot crash the host or escape basic constraints under stress.

## 8. M1 Defaults

These are fixed defaults for the first implementation.

### Provider

Use OpenAI Responses first.

- Env: `OPENAI_API_KEY`.
- Config file: `~/.form/config.toml`.
- Default model: `gpt5.5` unless config overrides `model`.
- Streaming: off in M1. Print final assistant text after each turn.
- Tool calls: map OpenAI tool call blocks into internal `ToolCall { id, name, input_json }`.
- Retries: one retry on 429/5xx with exponential backoff. No retry after a mutation.
- Tests use a scripted provider by default. Real OpenAI smoke tests are manual or ignored by default so CI does not need network or credentials.

### Sessions

Store sessions outside the repo by default:

```text
~/.form/sessions/<workspace-hash>/YYYYMMDD-HHMMSS.jsonl
```

### Interface

Use line-mode `form chat` first.

No rich TUI until the host loop is useful.

### Context Files

Load from the workspace root in this order:

1. `FORM.md`
2. `AGENTS.md`

Concatenate in that order. Missing files are ignored. Do not load `OTHER_AGENT.md` by default; it belongs to another agent ecosystem and can conflict with Form's safety model.

### Resources

Skills, prompt templates, packages, and themes are later. Do not block M1 on them.

### Plugin API

M0 sketches only plugin manifest constraints and host API constraints. M4 designs and implements WIT after M1/M2 prove the host APIs.

The first WIT can use a JSON command/result envelope to avoid overdesign.

## 9. Data Contracts

### 9.1 Session Entry JSONL

Each line is one append-only JSON object:

```json
{
  "schema_version": 1,
  "session_id": "uuid",
  "branch_id": "uuid",
  "entry_id": "uuid",
  "parent_entry_id": "uuid-or-null",
  "ts": "2026-07-02T00:00:00Z",
  "kind": "user|assistant|tool_call|tool_result|approval|summary|audit",
  "content": [{ "type": "text", "text": "..." }],
  "tool_call": { "id": "toolu_...", "name": "read", "input": {} },
  "tool_result": {
    "tool_call_id": "toolu_...",
    "status": "ok|error|denied",
    "content": [],
    "details": {}
  },
  "approval": { "request_id": "uuid", "decision": "allow|deny", "reason": "..." },
  "usage": { "input_tokens": 0, "output_tokens": 0 },
  "meta": {}
}
```

Only fields relevant to `kind` are required.

`entry_id`, `parent_entry_id`, `session_id`, `branch_id`, and `schema_version` exist from M1 so `/fork` and `/tree` do not require a storage migration. Branch-head metadata can live in a small sidecar index derived from JSONL, but JSONL remains the source of truth.

Session JSONL is the conversation-scoped source of truth. `form-audit` writes a separate global append-only log for cross-session security and event inspection.

Write policy:

- Normal turn/session entries are append + flush before the next model turn.
- Approval decisions, mutation results, shell results, and audit entries that justify a mutation call `sync_data` or stronger before the mutation-dependent result is returned.
- File creates/renames that matter for session/audit/mutation evidence sync the file and then the parent directory where supported.
- Non-critical chat turns may batch/flush without fsync; mutation-critical evidence may not be batched past the mutation result.
- If sync fails, stop the active turn and show a structured session/audit write error. Do not continue as if evidence was persisted.

### 9.1.1 Structured Error Envelope

All provider, tool, workspace, session, approval, audit, and plugin errors use a common envelope:

```json
{
  "kind": "WorkspaceEscape",
  "source": "workspace|provider|tool|session|approval|audit|plugin|cli",
  "retryable": false,
  "user_message": "Path is outside workspace",
  "audit_level": "none|log|audit",
  "exit_code": 1,
  "details": {},
  "cause": "optional lower-level error summary",
  "redact_details": false
}
```

Rules:

- `user_message` is safe for terminal display.
- `details` may be model-visible only when `redact_details` is false.
- `retryable` controls provider/tool retry behavior.
- `audit_level` controls whether the event is local log only or audit JSONL.
- CLI commands map terminal failures to stable `exit_code` values.

### 9.2 Tool Trait

```rust
trait HostTool {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn input_schema(&self) -> serde_json::Value;
    fn risk(&self, input: &serde_json::Value) -> ToolRisk;
    async fn execute(&self, ctx: ToolContext, input: serde_json::Value) -> ToolResult;
}
```

`ToolResult` fields:

- `status`: `ok|error|denied`
- `content`: model-visible content
- `details`: host/UI details
- `patches`: patch proposal IDs or embedded proposals
- `metrics`: duration, bytes read, output bytes, etc.
- `error`: structured error, if any

Mutation tools must call the approval pipeline before writing or running shell.

Async boundary default:

- Write provider/tool traits as native `async fn` traits.
- Do not add a proc macro just for static dispatch.
- When the first runtime registry needs dynamic dispatch, use `dynosaur` wrappers (`DynProvider`, `DynHostTool`) rather than rewriting traits with `async-trait`.

### 9.3 Approval Request/Result

`ApprovalRequest` fields:

- `id`
- `kind`: `write|edit|shell|plugin_patch`
- `summary`
- `path`
- `command`
- `diff`
- `risk`
- `tool_call_id`
- `created_at`

`ApprovalResult` fields:

- `request_id`
- `decision`: `allow|deny`
- `scope`: `once` in M2. Session-scoped approvals require a later design with exact matching semantics.
- `reason`
- `decided_at`

Interactive CLI behavior:

- Show summary plus diff or command.
- Prompt `[y/N]`.
- Default deny.
- Denials return a tool result with `status = "denied"`.
- Append approval and audit entries before the next model turn.

Non-interactive behavior:

- Reads/search are allowed.
- write/edit/shell/plugin_patch are denied unless a future explicit `--allow-mutations` flag exists.
- No silent mutation in headless mode.

### 9.4 Patch Proposal

Patch proposals use unified diff for M2/M4.

Fields:

- `id`
- `source`: `tool|plugin`
- `target_path`
- `unified_diff`
- `summary`
- `risk`
- `created_at`

Validation rules:

- Existing `target_path` must resolve inside the workspace.
- New files validate the nearest existing parent inside the workspace.
- Parent directories must not route through symlinks that escape the workspace.
- Revalidate the target path and parent immediately after approval and before mutation.
- Atomic temp files are created inside the target directory, not a global temp dir.
- `target_path` must not match secret-deny patterns.
- The diff must apply cleanly to the current file snapshot captured before approval.
- Re-check the file snapshot after approval and before mutation; stale snapshots return `patch_conflict`.
- Apply multi-hunk patches atomically: all hunks apply or no file changes.
- Preserve existing line endings where possible. Mixed or unclear line endings return `patch_invalid` unless the whole file is being written through the write tool.
- Preserve missing/present trailing newline semantics exactly.
- Binary files are denied for edit patches.
- File mode changes, renames, and deletes are denied until a later milestone.
- Conflicts return `status = "error"` with `error.kind = "patch_conflict"` and do not mutate files.
- Malformed diffs return `error.kind = "patch_invalid"` and are audited.
- New files are allowed only through explicit approval.
- Plugins may create proposals only. The host approval pipeline applies approved patches.

### 9.5 Provider Request/Result

`ProviderRequest` fields:

- `model`
- `system`
- `messages`
- `tools`
- `max_tokens`
- `temperature`

`ProviderResult` fields:

- `assistant_content`
- `tool_calls`
- `stop_reason`
- `usage`
- `raw_provider_id`
- `error`

M1 supports OpenAI Responses only. Other providers implement this internal request/result later.

### 9.6 Plugin Manifest Sketch

```toml
id = "workspace-summary"
name = "Workspace Summary"
version = "0.1.0"
api_version = "form-plugin-0.1"
component = "plugin.wasm"
permissions = ["workspace.search", "workspace.read", "log", "patch.propose"]

[[commands]]
id = "summarize"
description = "Summarize the workspace"
input = "json"
output = "json"
```

M4 only. Plugins get search/read/log/propose-patch host imports.

No plugin shell, env, network, direct writes, direct patch apply, or secrets.

## 10. Approval and Security Defaults

| Area | M1/M2 default |
| --- | --- |
| Workspace root | Canonical git root if present, else cwd. |
| Symlinks and new paths | Existing paths resolve with `realpath`; deny if target escapes workspace. New files validate the nearest existing parent, deny symlink parent escapes, create temp files inside the target directory, and revalidate immediately before mutation. |
| Ignored files | Search respects `.gitignore`; explicit read may read ignored files unless denied by secret patterns. |
| Secret patterns | Read and search both apply secret-deny patterns before returning model-visible content or snippets. Deny `.env*`, SSH keys, known token files, and `~/.config` unless explicitly allowlisted later. |
| File reads | Text only, max 256 KB per read in M1. Larger reads return a bounded error. |
| Tool output | Return max 50 KB or 2000 lines to the model. Search stops work once caps are hit instead of scanning the full workspace first. Save full local log when available. |
| Shell | Host-only, approval required, cwd = workspace root, timeout = 30s default, user env inherited, stdout/stderr capped. Approved shell is the user's shell, not a sandbox. Form protects against accidental model-requested mutation through approval, risk display, timeout, and output caps. |
| Shell cancel | Ctrl-C sends kill to child process group, records `cancelled` result. |
| Writes/edits | Approval required, diff displayed before mutation, atomic write via temp file + rename where possible. |
| Plugin access | No shell, env, network, direct filesystem write, direct patch apply, or secrets. |
| Audit timing | Append audit entry immediately after every approval decision, denied operation, mutation, timeout, or plugin permission error. |

## 11. Host Tools

### 11.1 Read Tool

Purpose:

- Read bounded text files inside the workspace.

Rules:

- Canonicalize path.
- Deny workspace escape.
- Deny secret patterns.
- Deny binary files.
- Cap output.

### 11.2 Search Tool

Purpose:

- Search workspace file names and text content.

Rules:

- Respect `.gitignore`.
- Traverse in deterministic path order for stable tests and repeatable output.
- Always skip `.git`, common build/cache directories, and vendored dependency directories unless explicitly allowlisted later.
- Treat nested git repos and submodules as boundaries unless explicitly included later.
- Traverse incrementally and stop once max matches or output bytes are reached.
- Apply secret-deny before reading contents or returning snippets.
- Skip binary files and files above the search per-file size cap before reading contents.
- Cap result count and bytes.
- Return path, line, snippet, and match metadata for allowed paths only.

### 11.3 Edit Tool

Purpose:

- Apply exact replacements or unified-diff patches after approval.

Rules:

- Show diff before approval.
- Revalidate path containment after approval and before mutation.
- Deny conflicts.
- Atomic write where possible, using a temp file in the target directory.
- Record audit entry.

### 11.4 Write Tool

Purpose:

- Create or replace files after approval.

Rules:

- Show path and content summary or diff.
- Deny secret paths.
- Validate nearest existing parent for new files.
- Revalidate path containment after approval and before mutation.
- Create parent directories only after approval.
- Atomic write where possible, using a temp file in the target directory.

### 11.5 Shell Tool

Purpose:

- Run host-owned shell commands after approval.

Rules:

- cwd is workspace root.
- approved shell runs with the user's normal environment and network access; it is not sandboxed.
- timeout defaults to 30 seconds.
- stdout/stderr capped.
- Ctrl-C kills child process group.
- Result includes exit code, killed/cancelled status, duration, and capped output.

## 12. Plugin Model (M4)

### 12.1 Plugin Unit

A plugin consists of:

- manifest,
- WASM Component artifact,
- optional README,
- optional plugin config schema.

### 12.2 Plugin Lifecycle

1. User registers a local plugin manifest.
2. Host validates manifest and API version.
3. Host validates requested permissions.
4. Host loads the WASM Component through Wasmtime.
5. Host lists plugin commands.
6. User invokes a plugin command.
7. Host creates a bounded execution context.
8. Plugin calls host APIs.
9. Host checks permission on every call.
10. Plugin returns structured result and optional patch proposals.
11. Host stores logs/proposals and shows them to the user.

### 12.3 Plugin Permissions

M4 permissions:

- `workspace.search`
- `workspace.read`
- `log`
- `patch.propose`

Denied in M4:

- shell,
- network,
- environment variables,
- secrets,
- arbitrary filesystem reads,
- filesystem writes,
- direct patch apply,
- git mutation.

### 12.4 Plugin Failure Behavior

Plugin failure must not crash the host.

Host handles:

- invalid manifest,
- incompatible API version,
- missing component,
- invalid component,
- instantiation failure,
- trap during execution,
- timeout,
- memory limit exceeded,
- denied permission,
- invalid output,
- malformed patch proposal.

Failures return structured errors and audit entries.

### 12.5 Plugin Resource Limits

M4 default sandbox limits:

- Wall-clock timeout: 2 seconds per plugin command by default.
- Memory cap: 64 MiB per plugin instance by default.
- Model-visible output cap: same tool output cap as host tools, 50 KB or 2000 lines.
- Host API result caps: every search/read/log/propose-patch response remains bounded by the host, not plugin choice.
- Interruption: use Wasmtime fuel or epoch interruption when available; otherwise enforce wall-clock timeout from the host task and drop the instance on timeout.

All defaults are configurable later, but M4 tests must prove timeout, memory-limit, and output-cap failures return structured errors and audit entries.

## 13. CLI Behaviors

### 13.1 `form chat`

Starts a line-mode interactive coding-agent session.

Expected behavior:

- Load config.
- Detect workspace root.
- Load context files.
- Create or resume a session.
- Send user turns to the provider.
- Execute allowed tools.
- Ask approval for mutations.
- Append all turns/tool results to session JSONL.

Line-mode turn UX contract:

- Show one compact status line when a provider request starts, retries, or fails.
- Show each tool call before execution with tool name and bounded summary.
- Execute multiple tool calls sequentially in M1/M2 unless a later design adds parallel tool execution.
- Show approval prompts inline at the point the mutation or shell command is requested.
- Ctrl-C during provider/tool execution cancels the active turn where possible and writes a cancelled session entry.
- Malformed model tool output becomes a visible tool error and session entry, not a silent retry loop.
- Print the final assistant text only after all required tool calls for that turn finish or fail safely.

### 13.2 `form resume`

Resume latest or selected session.

M1 can resume latest only. M3 adds selection/tree behavior.

### 13.3 `form sessions`

List local sessions for the workspace.

M1 lists sessions with time, branch ID, entry count, and latest status. M3 adds richer tree navigation.

### 13.4 `form inspect`

Inspect a session timeline.

M1 supports `form inspect --latest` for read-only JSONL inspection with user, assistant, tool_call, tool_result, approval, audit, error, and unknown future entry rendering. Replay/diff remain later.

### 13.5 `form plugins` (M4)

Plugin subcommands:

- `check <manifest>` — validate manifest and permissions without executing Wasmtime.
- `register <manifest>`
- `list`
- `inspect <id>`
- `run <id> <command> [json-input]`

## 14. Storage Strategy

Use simple local files first.

Suggested state:

```text
~/.form/config.toml
~/.form/sessions/<workspace-hash>/*.jsonl
~/.form/audit/*.jsonl
~/.form/plugins/registry.toml
~/.form/plugins/permissions.toml
```

SQLite can be introduced later if local state becomes complex.

## 15. Testing Strategy

### 15.1 Unit Tests

Test:

- config parsing,
- session entry serialization,
- provider request mapping,
- tool schema validation,
- approval request/result handling,
- path containment,
- patch validation,
- output truncation,
- plugin manifest constraints.

### 15.2 Integration Tests

Test:

- `form chat` can run one provider turn with a mocked provider.
- Context files load in the right order.
- Read/search tools return bounded results and do not return secret-pattern snippets.
- Edit/write/shell require approval.
- Denied approvals produce denied tool results.
- Sessions append and resume.
- Tool errors do not lose the session.

### 15.3 Security Tests

Test:

- `../` path traversal denial.
- Symlink escape denial.
- Secret path denial for both read and search.
- Binary/oversized read denial.
- Shell timeout and output caps.
- Malformed patch denial.
- Patch conflict denial.
- Plugin permission denial in M4.

### 15.4 Manual Smoke Tests

Run in this repo:

1. Ask Form to summarize `docs/mvp.md`.
2. Ask Form to search for `Wasmtime`.
3. Ask Form to propose a docs edit and approve it.
4. Ask Form to run a harmless command such as `git status --short` and approve it.
5. Resume the session and verify history remains.

## 16. Acceptance Criteria

The daily-driver MVP is complete when:

1. A Rust CLI binary runs `form chat`.
2. The CLI loads OpenAI config and sends a model request.
3. The CLI loads context files in the defined order.
4. The CLI creates and appends a JSONL session.
5. The CLI can resume the latest session.
6. The CLI can list sessions and inspect the latest session timeline.
7. The model can call read/search tools.
8. Read/search enforce workspace containment, size limits, and secret-deny patterns.
9. Edit/write/shell require explicit approval.
10. Denied approvals are represented as tool results.
11. Approved edits/writes apply through host-owned code only.
12. Approved shell runs with timeout and output caps.
13. Tool errors do not crash the host or lose session state.
14. Audit events are written for approvals, denials, mutations, and timeouts.
15. Tests cover valid tool execution and denial cases.
16. The architecture leaves a clear M4 path for WASM plugins without needing to rewrite sessions/tools/approvals.

The WASM milestone is complete when:

1. A local plugin manifest can be registered.
2. The CLI can list plugin commands.
3. The host can run one WASM Component command through Wasmtime.
4. The plugin can call bounded search/read/log/propose-patch host APIs.
5. Unauthorized plugin access is denied.
6. Plugin patch proposals are inspectable but not directly applied.
7. Plugin traps/timeouts return structured errors.

## 17. Key Risks and Mitigations

### 17.1 Risk: Plugin Architecture Distracts from Daily Use

Problem:

Building Wasmtime first delays the actual Pi replacement.

Mitigation:

- M1/M2 focus on host loop and approved mutation tools.
- M4 implements plugins only after the host workflow is usable.

### 17.2 Risk: Provider Abstraction Hides Real Tool-Call Complexity

Problem:

Providers differ in tool call formats, streaming, retry behavior, and auth.

Mitigation:

- Use OpenAI Responses only in M1.
- Define internal provider request/result early.
- Add providers later by adapting into the same internal types.

### 17.3 Risk: Session Tree Retrofits Are Expensive

Problem:

Linear logs are easy but make `/fork` and `/tree` harder later.

Mitigation:

- Include `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id` in M1 entries.
- Implement full tree UI later.

### 17.4 Risk: Approval UX Blocks Flow

Problem:

Too many prompts make daily usage painful.

Mitigation:

- Start with safe default-deny.
- Add scoped approvals after the basic approval log is correct and exact matching semantics are designed. M2 supports `once` only.

### 17.5 Risk: Patch Application Is Fragile

Problem:

LLM-generated diffs can conflict or be malformed.

Mitigation:

- Validate unified diffs against current snapshots.
- Return structured `patch_conflict` and `patch_invalid` errors.
- Never partially apply failed patches.

### 17.6 Risk: WASM Boundary Overhead

Problem:

Frequent host/plugin calls or giant serialized payloads can be slow.

Mitigation:

- Use handles and bounded results.
- Keep large state host-owned.
- Batch calls only when real usage shows it matters.

## 18. Recommended First Implementation Order

Rule: every contract/module step lands with its smallest useful test in the same change. Do not build the spine first and add tests later.

1. Keep `docs/architecture.md` concise and current with module graph, dependency rules, safety defaults, and plugin sandbox budgets.
2. Create Rust workspace skeleton with `form-core` and `form-cli`; keep the rest as internal modules first.
3. Define core domain types: messages, session entries, tool calls, tool results, errors.
4. Implement config loading.
5. Implement workspace root detection and context file loading.
6. Implement session append and resume latest.
7. Before coding the OpenAI adapter, verify current OpenAI Responses/tool-use docs: model ID, tool schema, tool_result formatting, stop reasons, usage, error taxonomy, retry rules, and max token defaults.
8. Implement OpenAI request/result mapping with mocked tests.
9. Implement tool trait and tool registry.
10. Implement read/search tools with path containment and caps.
11. Implement `form chat` loop with read/search tool dispatch.
12. Implement approval request/result and audit writes.
13. Implement patch proposal validation.
14. Implement edit/write tools.
15. Implement shell tool with approval, timeout, cancellation, and caps.
16. Add session listing and basic resume selection.
17. Add M3 fork/tree basics.
18. Sketch plugin manifest constraints only; defer WIT design to M4.
19. Add WIT, Wasmtime, and one sample plugin in M4.
20. Harden permission, path, timeout, and patch tests.

## 19. Handoff Prompt for the Next AI Coding Agent

You are implementing Form, a Rust-native daily-driver replacement for Pi.

Do not start with the WASM plugin runtime. First build the host agent loop:

- line-mode `form chat`,
- OpenAI Responses provider,
- `FORM.md` / `AGENTS.md` context loading,
- JSONL sessions with `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id`,
- host-owned read/search tools,
- approval pipeline,
- host-owned edit/write/shell tools.

Keep all mutation inside the host. Plugins are M4 and may propose patches only.

Use boring local storage first. Use explicit approval for every write/edit/shell action. Deny unsafe paths and plugin privileges by default. Prioritize a clean architecture spine that can grow into full Pi replacement without pretending to match every Pi command on day one. Start with `form-core` + `form-cli` and internal modules; split more crates only after behavior proves the seam.

The MVP is done when Form can complete a small real coding task in this repo, persist the session, resume it, and safely mediate host tool mutations.
