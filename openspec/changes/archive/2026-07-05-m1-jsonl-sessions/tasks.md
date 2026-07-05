## 1. Session Store

- [x] 1.1 Add `form-cli::session` module and export it from `lib.rs`.
- [x] 1.2 Add minimal date/time support for `YYYYMMDD-HHMMSS.jsonl` filenames.
- [x] 1.3 Implement stable workspace-hash directory selection for canonical POSIX and Windows-style workspace paths.
- [x] 1.4 Implement session creation under `~/.form/sessions/<workspace-hash>/` with timestamped `.jsonl` files.

## 2. JSONL Write and Resume

- [x] 2.1 Implement append-only `SessionEntry` writing as one JSON object per line.
- [x] 2.2 Flush appended entries before returning success.
- [x] 2.3 Implement latest-session lookup by sorted timestamped `.jsonl` filenames.
- [x] 2.4 Implement resume-latest append behavior and no-session reporting.

## 3. JSONL Read and Rendering

- [x] 3.1 Implement JSONL reading that returns path + line session errors for invalid JSON.
- [x] 3.2 Render user, assistant, tool_call, tool_result, approval, and audit entries from raw JSON values.
- [x] 3.3 Render valid unknown future `kind` values as unknown entries without stopping later valid lines.
- [x] 3.4 Render missing optional fields as bounded placeholders.

## 4. Tests and Checks

- [x] 4.1 Add runtime foundation tests for session path shape, stable workspace hash, append + flush, resume latest, and no-session behavior.
- [x] 4.2 Add fixture tests for user, assistant, tool_call, tool_result, approval/audit placeholders, corrupt trailing line, and unknown future kind rendering.
- [x] 4.3 Run `cargo fmt --check`.
- [x] 4.4 Run `cargo clippy --all-targets --all-features`.
- [x] 4.5 Run `cargo test --all-targets --all-features`.
