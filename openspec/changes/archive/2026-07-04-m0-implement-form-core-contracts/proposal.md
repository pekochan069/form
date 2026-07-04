## Why

Form now has a compiling Rust workspace, but `form-core` still exposes only a version helper. The next M0 step is to define the shared JSON contracts that later session, provider, tool, approval, audit, and plugin work can reuse without storage migrations or premature crate splits.

## What Changes

- Add minimal `form-core` serde data contracts for messages, tool calls/results, structured errors, session entries, provider request/result, approvals, patch proposals, audit entries, limits, and plugin manifest constraints.
- Add serialization round-trip tests for the public JSON shapes described in `docs/mvp.md`.
- Ensure session entries include `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id` from day one.
- Keep M2 and M4 work as data contracts only: no mutation tools, shell execution, WIT, Wasmtime, plugin runtime, provider calls, or CLI behavior beyond existing skeleton.

## Capabilities

### New Capabilities
- `form-core-contracts`: Shared Rust domain contracts and serde JSON shapes for M0/M1 session, tool, provider, approval, audit, limits, patch, and plugin-manifest data.

### Modified Capabilities

## Impact

- Affected code: `crates/form-core`, its Cargo manifest, and contract serialization tests.
- APIs: introduces public Rust structs/enums used by later `form-cli` modules; no stable plugin or provider runtime API yet.
- Dependencies: add only serde-related crates needed for data contracts and tests.
- Systems: local Rust checks must still pass with exactly the existing two-crate workspace.
