---
title: Inert Serde Contracts for Form Core M0
date: 2026-07-05
category: architecture-patterns
module: form-core
problem_type: architecture_pattern
component: development_workflow
severity: low
applies_when:
  - "Adding or changing boundary contracts before runtime behavior exists."
  - "Session JSON entries need sparse, kind-specific payload fields."
  - "Later milestone concepts need stable JSON shapes without implementation."
related_components:
  - testing_framework
  - assistant
tags: [form-core, serde, contracts, m0, openspec]
---

# Inert Serde Contracts for Form Core M0

## Context

Issue #3 asked which M0 data contracts `form-core` should expose first, and whether they could round-trip without forcing later crate splits. The solved change added serde contracts for messages, session entries, tool calls/results, structured errors, provider request/result, approvals, patch proposals, audit entries, limits, and plugin manifests.

Session history showed this work was intentionally sequenced after the Rust workspace skeleton: issue #2 created `form-core` and `form-cli`, then issue #3 layered contracts onto the existing core crate instead of adding new crates. Earlier architecture discussion also pushed against premature WIT/Wasmtime design, reinforcing the data-only boundary for M0. (session history)

## Guidance

Keep M0 contract work as plain serde structs and enums plus direct JSON round-trip tests. Do not start provider calls, session storage, host tool execution, approval prompts, audit writing, shell execution, WIT, Wasmtime, plugin registration, or plugin execution just because the data shape names those concepts.

Use strings for IDs, timestamps, and paths until runtime code needs generation, parsing, validation, or platform-specific containment checks. Use typed enums for closed vocabularies such as entry kinds, tool result status, approval decisions, error sources, and audit levels. Use `serde_json::Value` only for intentionally open payloads.

For fields irrelevant to some session entry kinds, accept omitted JSON with serde defaults instead of forcing every entry to carry empty arrays or objects:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionEntry {
    pub schema_version: u32,
    pub session_id: String,
    pub branch_id: String,
    pub entry_id: String,
    pub parent_entry_id: Option<String>,
    pub kind: SessionEntryKind,
    #[serde(default)]
    pub content: Vec<ContentBlock>,
    pub tool_call: Option<ToolCall>,
    pub tool_result: Option<ToolResult>,
    pub approval: Option<ApprovalSummary>,
    pub audit: Option<AuditEntry>,
    pub usage: Option<Usage>,
    #[serde(default)]
    pub meta: serde_json::Value,
}
```

Add tests that assert the serialized contract, not just Rust construction. The review blocker on this change came from `SessionEntry` requiring `content` and `meta` for every entry even though the spec allowed only kind-relevant fields. The fix was a serde default plus a regression test:

```rust
let entry: SessionEntry = serde_json::from_value(json!({
    "schema_version": 1,
    "session_id": "session-1",
    "branch_id": "branch-1",
    "entry_id": "entry-2",
    "parent_entry_id": "entry-1",
    "ts": "2026-07-02T00:00:00Z",
    "kind": "approval",
    "tool_call": null,
    "tool_result": null,
    "approval": { "request_id": "approval-1", "decision": "deny", "reason": "no" },
    "audit": null,
    "usage": null
}))
.unwrap();

assert!(entry.content.is_empty());
assert_eq!(entry.meta, serde_json::Value::Null);
```

Cover every documented enum value that future consumers may branch on. For tool results, that means testing all three status strings:

```rust
assert_eq!(serde_json::to_value(&ToolResultStatus::Ok).unwrap(), "ok");
assert_eq!(serde_json::to_value(&ToolResultStatus::Error).unwrap(), "error");
assert_eq!(serde_json::to_value(&ToolResultStatus::Denied).unwrap(), "denied");
```

## Why This Matters

These contracts become the shared vocabulary between later Form runtime pieces. If they are too strict, valid persisted session JSON fails to deserialize. If they perform runtime work too early, later milestones inherit behavior before the host loop, approvals, audit, and plugin boundary are ready.

The sparse session-entry shape is especially important. User messages, assistant messages, tool calls, tool results, approvals, summaries, and audit entries do not all need the same payload fields. Required identity fields stay strict, but kind-specific payloads should not require fake empty content just to satisfy the type.

## When to Apply

- Apply this when adding boundary data contracts before the behavior module exists.
- Apply this when a JSONL record has shared identity fields plus kind-specific payloads.
- Apply this when later M2/M4 concepts need names and serialized shape, not execution.
- Do not apply this to required identity, schema version, parent linkage, explicit statuses, or trust-boundary decisions unless the spec says they may be absent.

## Examples

Good M0 contract work:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStopReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    StopSequence,
    Error,
}
```

Good M0 plugin boundary work:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub component: String,
    pub permissions: Vec<PluginPermission>,
    pub commands: Vec<PluginCommand>,
}
```

Avoid this in M0:

```rust
// This crosses from contract into runtime behavior.
host_tools.execute(call).await?;
```

## Related

- GitHub issue #3: M0 form-core contracts and serialization tests.
- GitHub issue #1: Wayfinder map tracking M0/M1 sequencing.
- `openspec/changes/m0-implement-form-core-contracts/`: proposal, design, spec, and tasks for this contract layer.
- `docs/mvp.md`: source data-contract shapes for sessions, tools, approvals, providers, patches, and plugin manifests.
- `docs/architecture.md`: source module boundaries and milestone constraints.
- `docs/solutions/developer-experience/native-git-hooks-rust-quality-gates.md`: related only for Rust quality gates (`cargo fmt`, clippy, tests), not for contract shape.
