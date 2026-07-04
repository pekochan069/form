# form-core-contracts Specification

## Purpose
TBD - created by archiving change m0-implement-form-core-contracts. Update Purpose after archive.
## Requirements
### Requirement: Public form-core contract surface
The system SHALL expose public Rust data contracts from `form-core` for messages, tool calls, tool results, structured errors, session entries, provider requests, provider results, approvals, patch proposals, audit entries, limits, and plugin manifest constraints.

#### Scenario: Downstream crate imports contracts
- **WHEN** `form-cli` or a test imports the M0 contract types from `form-core`
- **THEN** the types are available through public module paths or top-level re-exports
- **AND** no additional Form crate is required

#### Scenario: Contracts use JSON-compatible serde shapes
- **WHEN** a contract value is serialized with serde
- **THEN** the JSON uses the field names and enum values defined by the M0 data contracts
- **AND** deserializing that JSON recreates the same value

### Requirement: Session entry JSONL contract
The system SHALL define a serializable session entry contract for append-only JSONL conversations with tree-capable identifiers from day one.

#### Scenario: Session entry contains tree identifiers
- **WHEN** a session entry is serialized
- **THEN** the JSON includes `schema_version`, `session_id`, `branch_id`, `entry_id`, and `parent_entry_id`
- **AND** `parent_entry_id` can be null for a root entry

#### Scenario: Session entry supports M1 entry kinds
- **WHEN** a session entry is deserialized
- **THEN** it can represent `user`, `assistant`, `tool_call`, `tool_result`, `approval`, `summary`, and `audit` entry kinds
- **AND** only fields relevant to that entry kind need non-null values

#### Scenario: Session content preserves text blocks
- **WHEN** a session entry contains content blocks such as `{ "type": "text", "text": "hello" }`
- **THEN** the content round-trips without losing block type or text

### Requirement: Structured error envelope contract
The system SHALL define one structured error envelope shared by provider, tool, workspace, session, approval, audit, plugin, and CLI failures.

#### Scenario: Structured error JSON round-trips
- **WHEN** an error envelope is serialized and deserialized
- **THEN** it preserves `kind`, `source`, `retryable`, `user_message`, `audit_level`, `exit_code`, `details`, `cause`, and `redact_details`

#### Scenario: Error source and audit level are typed
- **WHEN** an error envelope uses a known source or audit level
- **THEN** serde accepts the documented lower_snake_case values
- **AND** unknown internal string constants are not required for known M0 values

### Requirement: Tool and provider contracts
The system SHALL define serde contracts for host tool inputs/results and provider request/result exchange without implementing tool execution or provider calls.

#### Scenario: Tool call contract round-trips
- **WHEN** a tool call with `id`, `name`, and JSON `input` is serialized and deserialized
- **THEN** the call ID, tool name, and input JSON are preserved

#### Scenario: Tool result contract round-trips
- **WHEN** a tool result with `status`, `content`, `details`, metrics, patches, or error data is serialized and deserialized
- **THEN** the result preserves `ok`, `error`, and `denied` status values
- **AND** model-visible content remains separate from host details

#### Scenario: Provider request contract round-trips
- **WHEN** a provider request is serialized
- **THEN** it can include `model`, `system`, `messages`, `tools`, `max_tokens`, and `temperature`
- **AND** deserialization preserves message and tool schema payloads

#### Scenario: Provider result contract round-trips
- **WHEN** a provider result is serialized
- **THEN** it can include assistant content, tool calls, stop reason, usage, raw provider ID, and a structured error
- **AND** deserialization preserves usage token counts

### Requirement: Approval, patch, audit, and limit contracts
The system SHALL define serde contracts for later approval, mutation evidence, patch proposal, audit, and bounded-output behavior without performing mutations.

#### Scenario: Approval contracts round-trip
- **WHEN** an approval request/result pair is serialized and deserialized
- **THEN** request fields such as `id`, `kind`, `summary`, `path`, `command`, `diff`, `risk`, `tool_call_id`, and `created_at` are preserved
- **AND** result fields such as `request_id`, `decision`, `scope`, `reason`, and `decided_at` are preserved

#### Scenario: Patch proposal preserves POSIX path strings
- **WHEN** a patch proposal target path is `src/main.rs`
- **THEN** serialization and deserialization preserve the exact path string
- **AND** `form-core` does not canonicalize, normalize, or validate the path

#### Scenario: Patch proposal preserves Windows path strings
- **WHEN** a patch proposal target path is `src\\main.rs`
- **THEN** serialization and deserialization preserve the exact path string
- **AND** platform-specific containment validation remains outside the contract layer

#### Scenario: Audit contract round-trips mutation evidence
- **WHEN** an audit entry is serialized with schema version, ID, timestamp, event kind, level, subject IDs, and JSON details
- **THEN** deserialization preserves the event and details for later audit writers

#### Scenario: Limits contract round-trips default caps
- **WHEN** limits are serialized for file bytes, output bytes, output lines, shell timeout, plugin timeout, and plugin memory
- **THEN** deserialization preserves the numeric caps without starting any runtime enforcement

### Requirement: Plugin manifest constraint contract
The system SHALL define a serializable plugin manifest constraint contract for M4 planning while leaving plugin execution unimplemented.

#### Scenario: Plugin manifest contract round-trips
- **WHEN** a plugin manifest with `id`, `name`, `version`, `api_version`, `component`, `permissions`, and commands is serialized and deserialized
- **THEN** all manifest fields and command definitions are preserved

#### Scenario: Plugin component POSIX path is preserved
- **WHEN** a plugin manifest component path is `plugins/example/plugin.wasm`
- **THEN** serialization and deserialization preserve the exact path string

#### Scenario: Plugin component Windows path is preserved
- **WHEN** a plugin manifest component path is `plugins\\example\\plugin.wasm`
- **THEN** serialization and deserialization preserve the exact path string

### Requirement: Contract serialization tests
The system SHALL include tests that prove the public M0 contract JSON shapes can round-trip through serde.

#### Scenario: Rust test suite covers contract families
- **WHEN** `cargo test --all-targets --all-features` runs from the repository root
- **THEN** tests cover session entries, structured errors, tool contracts, provider contracts, approvals, patch proposals, audit entries, limits, and plugin manifest constraints
- **AND** the command exits successfully

#### Scenario: Standard Rust checks remain green
- **WHEN** `cargo fmt --check` and `cargo clippy --all-targets --all-features` run from the repository root
- **THEN** both commands exit successfully

### Requirement: Runtime behavior remains out of scope
The system MUST NOT implement provider calls, session storage, context loading, host tools, mutation tools, approval prompts, audit writing, shell execution, WIT, Wasmtime, plugin registration, or plugin execution in this M0 contract change.

#### Scenario: Workspace stays two-crate only
- **WHEN** the Cargo workspace is inspected after this change
- **THEN** the only workspace packages remain `form-core` and `form-cli`

#### Scenario: Later milestone concepts are inert contracts
- **WHEN** the implementation is reviewed
- **THEN** M2 approval, patch, shell, and audit concepts exist only as serializable data shapes
- **AND** M4 plugin concepts exist only as manifest and permission data shapes

