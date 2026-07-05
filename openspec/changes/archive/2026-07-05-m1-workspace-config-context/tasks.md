## 1. Workspace and path safety

- [x] 1.1 Add `form-cli` workspace module with canonical git-root-or-cwd detection.
- [x] 1.2 Add existing-path containment checks, including traversal and symlink escape tests.
- [x] 1.3 Add new-file parent validation and secret-deny helpers with POSIX and Windows-aware tests.

## 2. Config and context resources

- [x] 2.1 Add config module for default model, `~/.form/config.toml` model override, and `OPENAI_API_KEY` lookup.
- [x] 2.2 Add resources module that loads `FORM.md` then `AGENTS.md`, ignores missing files, and never loads `OTHER_AGENT.md` by default.

## 3. Verification and closure

- [x] 3.1 Run `cargo fmt --check`.
- [x] 3.2 Run `cargo clippy --all-targets --all-features`.
- [x] 3.3 Run `cargo test --all-targets --all-features`.
- [x] 3.4 Validate the OpenSpec change and prepare the Wayfinder resolution comment.
