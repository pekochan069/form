## ADDED Requirements

### Requirement: Workspace root detection
The system SHALL detect the workspace root as the canonical git root when a `.git` ancestor exists, otherwise the canonical current working directory.

#### Scenario: Git workspace root is selected
- **WHEN** Form starts from a nested directory inside a workspace containing a `.git` entry
- **THEN** the detected workspace root is the canonical directory containing that `.git` entry

#### Scenario: Current directory is selected outside git
- **WHEN** Form starts from a directory with no `.git` ancestor
- **THEN** the detected workspace root is the canonical current directory

### Requirement: Config loading
The system SHALL load runtime config from `~/.form/config.toml`, `OPENAI_API_KEY`, and default values.

#### Scenario: Default model is used without config file
- **WHEN** no config file provides a model
- **THEN** the config model is `gpt5.5`

#### Scenario: Config file overrides default model
- **WHEN** `~/.form/config.toml` contains a `model` value
- **THEN** the config model uses that value instead of the default model

#### Scenario: API key comes from environment
- **WHEN** `OPENAI_API_KEY` is set
- **THEN** the config exposes that API key value

### Requirement: Context resource loading
The system SHALL load project context files from the workspace root in the order `FORM.md` then `AGENTS.md`.

#### Scenario: Context files load in fixed order
- **WHEN** both `FORM.md` and `AGENTS.md` exist at the workspace root
- **THEN** the loaded context contains `FORM.md` before `AGENTS.md`

#### Scenario: Missing context files are non-fatal
- **WHEN** one or both M1 context files are absent
- **THEN** context loading succeeds with only the files that exist

#### Scenario: Extra context is ignored
- **WHEN** `OTHER_AGENT.md` exists at the workspace root
- **THEN** context loading does not include `OTHER_AGENT.md` by default

### Requirement: Path safety helpers
The system SHALL provide path helpers that later host tools can use to reject workspace escapes and secret paths.

#### Scenario: POSIX traversal is denied
- **WHEN** a requested relative path uses `..` segments to leave the workspace
- **THEN** path resolution denies the request as outside the workspace

#### Scenario: Windows traversal is denied
- **WHEN** a requested relative path uses Windows-style parent traversal such as `..\\secret.txt`
- **THEN** path resolution denies the request as outside the workspace on platforms where that path would escape

#### Scenario: Symlink escape is denied
- **WHEN** a requested existing path points through a symlink outside the workspace
- **THEN** path resolution denies the request as outside the workspace

#### Scenario: New-file parent is validated
- **WHEN** a requested new file does not exist
- **THEN** path resolution validates the nearest existing parent remains inside the workspace before accepting the path

#### Scenario: Secret paths are denied
- **WHEN** a requested path matches M1 secret-deny patterns such as `.env`, `.env.local`, `.ssh/id_rsa`, or `.config`
- **THEN** the helper marks the path as denied before model-visible content can be returned
