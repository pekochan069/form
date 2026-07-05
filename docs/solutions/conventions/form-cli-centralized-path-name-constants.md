---
title: Centralized path and file name constants for Form CLI
date: 2026-07-05
category: conventions
module: form-cli
problem_type: convention
component: tooling
severity: low
applies_when:
  - CLI code or tests reference shared filesystem layout names.
  - Tests assert the same conventional file names used by production code.
  - A path or file name is part of Form's persisted local storage contract.
tags: [form-cli, paths, constants, convention, tests]
---

# Centralized path and file name constants for Form CLI

## Context

Form CLI had stable filesystem names embedded directly in implementation and tests. Examples included `.form`, `.form/sessions`, `config.toml`, `FORM.md`, and `AGENTS.md` appearing inside path joins or filesystem assertions.

The fix moved those names into `crates/form-cli/src/paths.rs` and updated config loading, session storage, context resource loading, and runtime foundation tests to import the same constants.

## Guidance

Keep shared Form CLI filesystem names in one boring module near the code that owns them. For this repo, `paths.rs` is the home for stable path segments and conventional file names.

Before:

```rust
let sessions_dir = home.join(".form/sessions");
let config_path = home.join(".form").join("config.toml");
```

After:

```rust
use crate::paths::{CONFIG_FILE_NAME, FORM_DIR_NAME, SESSIONS_DIR_NAME};

let sessions_dir = home.join(FORM_DIR_NAME).join(SESSIONS_DIR_NAME);
let config_path = home.join(FORM_DIR_NAME).join(CONFIG_FILE_NAME);
```

For context resources, reuse the shared list instead of copying names into discovery logic or tests:

```rust
use crate::paths::CONTEXT_FILE_NAMES;

for file_name in CONTEXT_FILE_NAMES {
    let path = workspace_root.join(file_name);
    // load context file if present
}
```

Keep this convention intentionally small. These names are project conventions, not a configuration framework. Do not introduce a registry, path service, or user setting unless runtime behavior actually needs one.

## Why This Matters

Repeated path literals make filesystem conventions harder to change and easier to mistype. A typo in one caller can silently create a second directory, miss an existing file, or make tests assert a layout that production no longer uses.

Central constants make the convention visible, searchable, and consistent across production code and tests. When future work touches Form CLI filesystem layout, reviewers can inspect the constants first instead of hunting scattered string joins.

## When to Apply

- A path segment or conventional file name is used by more than one module.
- Tests assert the same filesystem names used by production code.
- A name is part of the CLI's documented or persisted layout.
- Changing the name later would require coordinated edits.

Do not apply this to one-off local strings with no shared meaning. A constant used once usually adds noise unless it documents a public convention.

## Examples

Session path construction should use shared directory names:

```rust
use crate::paths::{FORM_DIR_NAME, SESSIONS_DIR_NAME};

let sessions_dir = home.join(FORM_DIR_NAME).join(SESSIONS_DIR_NAME);
```

Config path construction should use the same Form directory and config file constants:

```rust
use crate::paths::{CONFIG_FILE_NAME, FORM_DIR_NAME};

let config_path = home.join(FORM_DIR_NAME).join(CONFIG_FILE_NAME);
```

Tests should import the same constants when asserting layout:

```rust
use form_cli::paths::{FORM_DIR_NAME, SESSIONS_DIR_NAME};

assert!(temp.path().join(FORM_DIR_NAME).join(SESSIONS_DIR_NAME).exists());
```

The rule is simple: if the string names a Form CLI filesystem convention, put it in `paths.rs` and reuse it. If it is just a local value, leave it local.

## Related

- `docs/mvp.md` documents the persisted `~/.form` session/config layout and context file names that these constants represent.
- `docs/architecture.md` assigns context loading to resources and session persistence to the session module, which now share named filesystem conventions.
- Related: https://github.com/pekochan069/form/issues/4
- Related: https://github.com/pekochan069/form/issues/5
- Related: https://github.com/pekochan069/form/pull/15
