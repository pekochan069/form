## Why

M1 needs a small runtime foundation before provider, session, and tool behavior can run safely. Workspace root detection, config loading, context resources, and path safety establish the defaults later M1 tools will rely on.

## What Changes

- Add `form-cli` internal modules for workspace detection and path safety helpers.
- Add config loading from `~/.form/config.toml`, `OPENAI_API_KEY`, and the default model `gpt5.5`.
- Add context loading from `FORM.md` then `AGENTS.md` at the workspace root, with missing files ignored and `OTHER_AGENT.md` ignored by default.
- Add focused tests for workspace root detection, config merge behavior, context order, containment, symlink escape, new-file parent validation, and secret-deny patterns.

## Capabilities

### New Capabilities
- `workspace-config-context`: Workspace detection, config loading, context loading, and reusable path-safety rules for M1 host behavior.

### Modified Capabilities

## Impact

- Affects `crates/form-cli` internals and tests.
- Adds only boring local parsing/path logic; no provider calls, sessions, read/search tools, mutation tools, WIT, Wasmtime, or plugin execution.
