# rust-workspace-skeleton Specification

## Purpose
TBD - created by archiving change m0-create-rust-workspace-skeleton. Update Purpose after archive.
## Requirements
### Requirement: Two-crate Cargo workspace
The system SHALL define a Cargo workspace with exactly the `form-core` and `form-cli` packages for the initial M0 skeleton.

#### Scenario: Workspace members are minimal
- **WHEN** Cargo metadata is read from the repository root
- **THEN** the workspace members include `form-core` and `form-cli` only
- **AND** no WIT, Wasmtime, plugin-runtime, or future-only placeholder crate is present

#### Scenario: CLI depends on core crate
- **WHEN** the `form-cli` package is built
- **THEN** it uses the local `form-core` package through a normal Cargo path dependency

### Requirement: Runnable minimal CLI
The system SHALL provide a `form` binary from the `form-cli` package with minimal help and version behavior.

#### Scenario: Help output succeeds
- **WHEN** a user runs the `form` binary with `--help`
- **THEN** the command exits successfully
- **AND** the output names the Form CLI and available skeleton flags

#### Scenario: Version output succeeds
- **WHEN** a user runs the `form` binary with `--version`
- **THEN** the command exits successfully
- **AND** the output includes the Form CLI version

### Requirement: No out-of-scope implementation
The system MUST NOT implement provider calls, session storage, context loading, read/search tools, mutation tools, approvals, audit logs, WIT, Wasmtime, or plugin execution in this skeleton change.

#### Scenario: Skeleton stays behavior-limited
- **WHEN** the M0 skeleton change is reviewed
- **THEN** it contains only workspace, crate, binary entrypoint, and baseline test code
- **AND** M1/M2/M4 behavior remains for later changes

### Requirement: Local Rust checks pass
The system SHALL provide a repository state where the standard Rust checks pass from the repository root.

#### Scenario: Formatting passes
- **WHEN** `cargo fmt --check` runs from the repository root
- **THEN** it exits successfully

#### Scenario: Clippy passes
- **WHEN** `cargo clippy --all-targets --all-features` runs from the repository root
- **THEN** it exits successfully

#### Scenario: Tests pass
- **WHEN** `cargo test --all-targets --all-features` runs from the repository root
- **THEN** it exits successfully

