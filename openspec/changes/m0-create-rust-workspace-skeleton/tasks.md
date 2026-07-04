## 1. Wayfinder Prep

- [x] 1.1 Read GitHub issues #1 and #2, then claim #2 with the `wayfinder:claimed` label before code edits.
- [x] 1.2 Confirm the repo has no existing Cargo workspace or Rust crates that must be preserved.

## 2. Workspace Skeleton

- [x] 2.1 Add a root `Cargo.toml` workspace with members `crates/form-core` and `crates/form-cli` only.
- [x] 2.2 Add `target/` to `.gitignore` without changing existing ignored OpenSpec paths.
- [x] 2.3 Create `crates/form-core` with a minimal library API used by `form-cli` and a small unit test.
- [x] 2.4 Create `crates/form-cli` with a `form` binary, a testable library entry point, and no command-parser dependency.
- [x] 2.5 Wire `form-cli` to `form-core` through a local Cargo path dependency.

## 3. CLI Baseline Behavior

- [x] 3.1 Implement `form --help` so it exits successfully and prints the skeleton usage.
- [x] 3.2 Implement `form --version` so it exits successfully and includes the Form CLI version.
- [x] 3.3 Add tests for help, version, and unknown-argument behavior.

## 4. Scope Guardrails

- [x] 4.1 Verify the workspace contains no extra crates beyond `form-core` and `form-cli`.
- [x] 4.2 Verify the change adds no WIT, Wasmtime, provider calls, session storage, tools, approvals, audit log, mutation behavior, or plugin execution.

## 5. Checks and Closeout

- [x] 5.1 Run `cargo fmt --check` from the repository root.
- [x] 5.2 Run `cargo clippy --all-targets --all-features` from the repository root.
- [x] 5.3 Run `cargo test --all-targets --all-features` from the repository root.
- [x] 5.4 Resolve issue #2 with a summary comment, close it, and append a one-line context pointer to issue #1 Decisions so far.
