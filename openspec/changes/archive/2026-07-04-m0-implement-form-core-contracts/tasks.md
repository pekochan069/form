## 1. Wayfinder Prep

- [x] 1.1 Read GitHub issues #1 and #3, then claim #3 with the `wayfinder:claimed` label before code edits.
- [x] 1.2 Re-read `docs/mvp.md`, `docs/architecture.md`, and this change's proposal/design/spec to confirm M0 contract scope.
- [x] 1.3 Confirm the workspace still has exactly `form-core` and `form-cli`, and `form-core` has no existing contract types to preserve.

## 2. Contract Module Setup

- [x] 2.1 Add only serde-related dependencies needed by `form-core` for data contracts and round-trip tests.
- [x] 2.2 Create the minimal `form-core` contract module layout for messages, sessions, tools, errors, providers, approvals, patches, audit, limits, and plugin manifests.
- [x] 2.3 Re-export public contract types from `form-core` while preserving the existing version helper used by `form-cli`.

## 3. Core Contract Types

- [x] 3.1 Implement message content, usage, tool call, and tool result contracts with lower_snake_case serde enum values.
- [x] 3.2 Implement session entry contracts with `schema_version`, `session_id`, `branch_id`, `entry_id`, and nullable `parent_entry_id`.
- [x] 3.3 Implement the shared structured error envelope with typed source and audit-level values.
- [x] 3.4 Implement provider request/result contracts for Anthropic-ready message, tool schema, stop reason, usage, and raw provider ID data.
- [x] 3.5 Implement approval request/result, patch proposal, audit entry, and limit contracts as inert data shapes.
- [x] 3.6 Implement plugin manifest constraint contracts with manifest fields, permissions, and command definitions only.

## 4. Serialization Tests

- [x] 4.1 Add session entry and content block serde round-trip tests using JSON shaped like `docs/mvp.md`.
- [x] 4.2 Add structured error, tool call/result, and provider request/result round-trip tests.
- [x] 4.3 Add approval, patch proposal, audit entry, and limits round-trip tests.
- [x] 4.4 Add plugin manifest constraint round-trip tests, including POSIX and Windows component path strings.
- [x] 4.5 Add patch proposal path round-trip tests for POSIX and Windows target path strings.
- [x] 4.6 Add a public import test proving downstream code can use the exported contract types without another Form crate.

## 5. Scope Guardrails

- [x] 5.1 Verify the implementation adds no provider calls, session storage, context loading, host tool execution, approval prompts, audit writing, shell execution, WIT, Wasmtime, plugin registration, or plugin execution.
- [x] 5.2 Verify no extra workspace crates are added beyond `form-core` and `form-cli`.
- [x] 5.3 Verify functions in Rust code stay under the documented 100-line guideline or split them before checks.

## 6. Checks and Closeout

- [x] 6.1 Run `cargo fmt --check` from the repository root.
- [x] 6.2 Run `cargo clippy --all-targets --all-features` from the repository root.
- [x] 6.3 Run `cargo test --all-targets --all-features` from the repository root.
- [x] 6.4 Resolve issue #3 with a summary comment, close it, and append a one-line context pointer to issue #1 Decisions so far.
