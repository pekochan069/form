## Context

`form-core` currently exposes only a version helper. Issue #3 asks for the first real M0 contract layer: public JSON shapes that later `form-cli` modules can use for sessions, providers, tools, approvals, audit, patches, limits, and plugin manifest checks.

This change should keep the workspace at `form-core` + `form-cli` only. It should add contracts and tests, not behavior.

## Goals / Non-Goals

**Goals:**

- Define minimal serializable `form-core` types for the JSON contracts in `docs/mvp.md`.
- Keep session JSONL tree-ready from day one with `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id`.
- Cover public JSON shapes with serde round-trip tests.
- Keep later M2/M4 concepts as inert data contracts so future work has stable shapes without runtime side effects.

**Non-Goals:**

- No provider adapter, model calls, session file store, tools, approvals pipeline, audit writer, patch applier, shell execution, WIT, Wasmtime, plugin registry, or CLI command expansion.
- No new crates beyond the existing two workspace members.
- No validation framework beyond what the contracts need for serialization tests.

## Decisions

- **Use plain serde structs/enums in `form-core`.** Alternative: traits and registries now. Rejected because there is no runtime caller yet; data contracts are enough for M0.
- **Use explicit enums for closed vocabularies and `serde_json::Value` for open payloads.** `kind`, `status`, decisions, audit levels, and sources should be typed; tool inputs, provider metadata, error details, and session `meta` stay JSON values so contracts do not predict every future shape.
- **Represent IDs and timestamps as strings in M0.** Alternative: add `uuid` and time crates now. Rejected until behavior needs generation, parsing, or ordering; round-trip contracts only need stable JSON fields.
- **Keep modules grouped by contract area inside `form-core`.** A shallow module split such as `message`, `session`, `tool`, `error`, `provider`, `approval`, `patch`, `audit`, `limit`, and `plugin` keeps files readable without adding crates.
- **Re-export public contracts from `form_core::lib`.** Callers should not need to know every internal file path, and tests can use the same API future crates will use.
- **Use serde rename rules matching docs.** JSON uses snake_case field names and lower_snake_case enum strings such as `tool_call`, `tool_result`, `allow`, `deny`, `ok`, `error`, and `denied`.
- **Test round trips with representative examples, not exhaustive builders.** One focused test per contract family is enough to lock the public JSON shape without creating fixture machinery.

## Risks / Trade-offs

- String IDs/timestamps can accept invalid values → future behavior modules add validation when they generate or consume these values.
- Open JSON payload fields can hide malformed tool/provider details → runtime validation belongs with the caller that knows the schema.
- Many contract types can make `lib.rs` noisy → keep files short and re-export only public contract types.
- Enum names may drift from docs → round-trip tests use literal JSON from `docs/mvp.md` shapes to catch drift.

## Migration Plan

1. Add serde dependencies to `form-core` only.
2. Replace the version-only core module with contract modules plus the existing version export.
3. Add serde round-trip tests for session entries, structured errors, provider/tool contracts, approval/patch/audit contracts, limits, and plugin manifest constraints.
4. Run `cargo fmt --check`, `cargo clippy --all-targets --all-features`, and `cargo test --all-targets --all-features`.
5. Roll back by removing the new dependencies/modules if the contracts block later work.

## Open Questions

None for M0. Strong ID/timestamp types and validation rules wait until runtime code needs them.
