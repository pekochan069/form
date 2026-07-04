## Context

Form is a docs-only Rust-native coding-agent project. The Wayfinder frontier ticket is issue #2, which asks for the smallest compiling Rust workspace skeleton that matches `docs/architecture.md` without placeholder crates.

Constraints:

- Start with `form-core` and `form-cli` only.
- Keep `agent`, `provider`, `tools`, `workspace`, `session`, `resources`, `approval`, `audit`, and plugin work out until each has real behavior or a boundary test.
- Do not implement WIT, Wasmtime, mutation tools, or plugin execution.
- Leave M1/M2 behavior for later changes; this change only creates the compile/test spine.

## Goals / Non-Goals

**Goals:**

- Create a root Cargo workspace with exactly `crates/form-core` and `crates/form-cli`.
- Make `form-cli` build a `form` binary and depend on `form-core`.
- Provide a tiny, tested CLI entry point (`--help` / `--version`) so the binary is runnable and future command work has a place to attach.
- Make `cargo fmt --check`, `cargo clippy --all-targets --all-features`, and `cargo test --all-targets --all-features` pass locally.

**Non-Goals:**

- No provider adapter, session JSONL store, context loading, read/search tools, approval flow, audit log, WIT, Wasmtime, plugin runtime, or mutation tools.
- No extra crates for future seams.
- No new command parser dependency until real subcommands need it.

## Decisions

- **Use one Cargo workspace with two packages.** This matches the accepted architecture while avoiding the rejected 11-crate split. Alternative considered: one crate only; rejected because `form-core` needs to become the shared contract crate and issue #2 explicitly requires both crates.
- **Make `form-cli` own a small library entry point plus `form` binary.** The binary stays thin and tests can call library functions without subprocess setup. Alternative considered: binary-only crate; rejected because it makes even trivial behavior harder to test.
- **Use standard library argument parsing for the skeleton.** `--help` and `--version` do not need `clap`. Alternative considered: add `clap` immediately; rejected as dependency ceremony before real subcommands exist.
- **Defer internal modules until behavior lands.** Empty module files look architectural but add no proof. Future changes should add `session`, `resources`, `provider`, and other modules with their first real API/test.
- **Keep `form-core` minimal but used.** It should expose a tiny version/build-info function or constant consumed by `form-cli`, proving the crate boundary is wired without inventing domain types early.

## Risks / Trade-offs

- **Later command parsing may replace the stdlib parser** → acceptable; swap in `clap` when real subcommands make it cheaper than hand parsing.
- **Few modules may look less like the architecture diagram** → intentional; empty modules are deferred until they have behavior or tests.
- **Skeleton can become busywork if it adds domain types too early** → keep domain contracts for the next ticket, where serialization tests define them.

## Migration Plan

1. Add root workspace manifest and the two crate directories.
2. Add minimal `form-core` library API and tests.
3. Add `form-cli` library/binary, wire it to `form-core`, and test help/version output.
4. Run fmt, clippy, and tests.
5. Rollback by deleting the new Cargo manifests/crates if the skeleton blocks later work.

## Open Questions

None. Later tickets decide core domain contracts and M1 module internals.
