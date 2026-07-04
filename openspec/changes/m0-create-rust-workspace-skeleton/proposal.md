## Why

Form is still docs-only, so M0/M1 implementation needs a compile-first Rust workspace before contract, session, provider, or tool work can land safely.
This change establishes the smallest Cargo structure that matches the accepted architecture and unblocks the first Wayfinder frontier ticket.

## What Changes

- Add a Cargo workspace with exactly two crates: `form-core` and `form-cli`.
- Add the minimum `form-cli` command/module shape needed for M0/M1 follow-up work.
- Add local Rust check paths for format, clippy, and tests.
- Exclude WIT, Wasmtime, mutation tools, plugin execution, and future-only placeholder crates.

## Capabilities

### New Capabilities
- `rust-workspace-skeleton`: Cargo workspace structure and baseline CLI/core crate behavior for the Form M0/M1 architecture spine.

### Modified Capabilities

## Impact

- Affected code: root Cargo workspace, `crates/form-core`, `crates/form-cli`, and minimal tests.
- APIs: introduces initial internal Rust crate/module boundaries only; no stable public plugin or provider API yet.
- Dependencies: Rust toolchain only, plus the smallest crate dependencies needed by the generated skeleton.
- Systems: local development and CI check commands gain runnable Cargo targets.
