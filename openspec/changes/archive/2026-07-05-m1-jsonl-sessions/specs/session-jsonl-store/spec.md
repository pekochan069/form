## ADDED Requirements

### Requirement: Workspace-scoped session files
The system SHALL store M1 session JSONL files outside the repository under `~/.form/sessions/<workspace-hash>/` using timestamped `YYYYMMDD-HHMMSS.jsonl` filenames.

#### Scenario: POSIX workspace gets stable session directory
- **WHEN** a session store is created for the same canonical POSIX workspace path across two runs
- **THEN** both runs use the same workspace-hash directory under the Form sessions root
- **AND** new session files use a `YYYYMMDD-HHMMSS.jsonl` filename

#### Scenario: Windows workspace gets stable session directory
- **WHEN** a session store is created for the same canonical Windows workspace path across two runs
- **THEN** both runs use the same workspace-hash directory under the Form sessions root
- **AND** new session files use a `YYYYMMDD-HHMMSS.jsonl` filename

### Requirement: Append-only session writing
The system SHALL append each serialized `SessionEntry` as one JSON object per line and flush the session writer before returning success to the caller.

#### Scenario: Appended entry is readable before close
- **WHEN** a user session entry is appended to a new session writer
- **THEN** the JSONL file contains exactly one valid JSON line before the writer is dropped
- **AND** deserializing that line preserves the entry IDs, kind, timestamp, and content

#### Scenario: Multiple entry kinds append in order
- **WHEN** user, assistant, tool_call, and tool_result entries are appended to one session
- **THEN** reading the JSONL file returns those entries in append order

### Requirement: Resume latest workspace session
The system SHALL select the latest existing `*.jsonl` session file for the current workspace when M1 resume-latest is requested.

#### Scenario: Latest timestamped session is selected
- **WHEN** multiple session JSONL files exist for one workspace
- **THEN** resume-latest selects the file with the latest timestamped filename
- **AND** appending through the resumed writer adds entries to that file

#### Scenario: Empty session directory has no latest session
- **WHEN** no JSONL session files exist for the workspace
- **THEN** resume-latest reports that no session exists instead of creating one silently

### Requirement: Corrupt JSONL is surfaced
The system SHALL return a session error when a JSONL file contains an invalid line and MUST NOT silently drop corrupt data.

#### Scenario: Corrupt trailing line reports path and line
- **WHEN** a session JSONL file ends with invalid JSON on a trailing line
- **THEN** reading the file returns a session JSONL error
- **AND** the error identifies the file path and line number of the invalid data

### Requirement: Timeline rendering fixtures
The system SHALL provide inspection rendering inputs and helpers for M1 session timeline shapes, including future unknown entry kinds.

#### Scenario: Known M1 entry kinds render visibly
- **WHEN** fixture JSONL contains user, assistant, tool_call, tool_result, approval, and audit entries
- **THEN** timeline rendering emits a visible line for each entry kind
- **AND** tool result status and approval/audit details remain distinguishable

#### Scenario: Unknown future kind renders gracefully
- **WHEN** fixture JSONL contains valid JSON with an unrecognized `kind` value
- **THEN** timeline rendering emits an unknown-entry line for that item
- **AND** reading continues for later valid lines

#### Scenario: Missing optional fields do not block rendering
- **WHEN** fixture JSONL contains a valid entry with only the required session IDs, timestamp, kind, and metadata fields
- **THEN** timeline rendering emits a bounded placeholder instead of failing
