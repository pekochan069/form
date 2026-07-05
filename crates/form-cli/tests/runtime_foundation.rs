use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use form_cli::{
    config::{DEFAULT_MODEL, load_from},
    paths::{CONFIG_FILE_NAME, CONTEXT_FILE_NAMES},
    resources::load_context,
    session::{SessionJsonlError, SessionStore, read_jsonl_path, render_timeline, workspace_hash},
    workspace::{Workspace, is_secret_path},
};
use form_core::{ContentBlock, SessionEntry, SessionEntryKind};
use serde_json::{Value, json};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let id = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "form-cli-{label}-{}-{nonce}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn workspace_detects_git_root_from_nested_directory() {
    let temp = TempDir::new("git-root");
    fs::create_dir(temp.path().join(".git")).unwrap();
    let nested = temp.path().join("crates/form-cli");
    fs::create_dir_all(&nested).unwrap();

    let workspace = Workspace::detect_from(&nested).unwrap();

    assert_eq!(workspace.root(), fs::canonicalize(temp.path()).unwrap());
}

#[test]
fn workspace_falls_back_to_current_directory_without_git() {
    let temp = TempDir::new("no-git");

    let workspace = Workspace::detect_from(temp.path()).unwrap();

    assert_eq!(workspace.root(), fs::canonicalize(temp.path()).unwrap());
}

#[test]
fn config_uses_default_model_and_openai_api_key_without_file() {
    let config = load_from(None, Some("test-key".to_owned())).unwrap();

    assert_eq!(config.model, DEFAULT_MODEL);
    assert_eq!(config.openai_api_key.as_deref(), Some("test-key"));
}

#[test]
fn config_file_overrides_default_model() {
    let temp = TempDir::new("config");
    let config_path = temp.path().join(CONFIG_FILE_NAME);
    fs::write(&config_path, "model = 'gpt-test-model' # comment\n").unwrap();

    let config = load_from(Some(&config_path), None).unwrap();

    assert_eq!(config.model, "gpt-test-model");
    assert_eq!(config.openai_api_key, None);
}

#[test]
fn context_loads_form_then_agents_and_ignores_extra_files() {
    let temp = TempDir::new("context");
    fs::write(temp.path().join(CONTEXT_FILE_NAMES[0]), "form context").unwrap();
    fs::write(temp.path().join(CONTEXT_FILE_NAMES[1]), "agents context").unwrap();
    fs::write(temp.path().join("OTHER_AGENT.md"), "other agent context").unwrap();

    let files = load_context(temp.path()).unwrap();

    assert_eq!(files.len(), 2);
    assert_eq!(files[0].name, CONTEXT_FILE_NAMES[0]);
    assert_eq!(files[0].content, "form context");
    assert_eq!(files[1].name, CONTEXT_FILE_NAMES[1]);
    assert_eq!(files[1].content, "agents context");
}

#[test]
fn missing_context_files_are_non_fatal() {
    let temp = TempDir::new("missing-context");

    let files = load_context(temp.path()).unwrap();

    assert!(files.is_empty());
}

#[test]
fn existing_path_resolution_denies_parent_traversal() {
    let temp = TempDir::new("traversal");
    let workspace = Workspace::detect_from(temp.path()).unwrap();
    fs::write(temp.path().join("allowed.txt"), "ok").unwrap();

    assert!(workspace.resolve_existing("allowed.txt").is_ok());
    assert!(workspace.resolve_existing("../outside.txt").is_err());
    assert!(workspace.resolve_existing("..\\outside.txt").is_err());
}

#[test]
fn new_file_resolution_validates_parent_inside_workspace() {
    let temp = TempDir::new("new-file");
    fs::create_dir(temp.path().join("safe")).unwrap();
    let workspace = Workspace::detect_from(temp.path()).unwrap();

    let path = workspace.resolve_new_file("safe/new.txt").unwrap();

    assert!(path.ends_with("safe/new.txt"));
    assert!(workspace.resolve_new_file("../outside.txt").is_err());
}

#[test]
fn secret_deny_patterns_cover_m1_defaults() {
    for path in [
        ".env",
        ".env.local",
        ".ssh/id_rsa",
        ".config/form/config.toml",
        "nested/token.json",
        "nested/.npmrc",
    ] {
        assert!(is_secret_path(path), "expected {path} to be secret");
    }

    assert!(!is_secret_path("src/main.rs"));
}

#[test]
fn symlink_escape_is_denied() {
    let root = TempDir::new("symlink-root");
    let outside = TempDir::new("symlink-outside");
    fs::write(outside.path().join("secret.txt"), "secret").unwrap();
    symlink_dir(outside.path(), &root.path().join("link")).unwrap();
    let workspace = Workspace::detect_from(root.path()).unwrap();

    assert!(workspace.resolve_existing("link/secret.txt").is_err());
    assert!(workspace.resolve_new_file("link/new.txt").is_err());
}

#[test]
fn session_path_uses_workspace_hash_and_timestamp() {
    let sessions = TempDir::new("sessions-root");
    let workspace = TempDir::new("sessions-workspace");
    let store = SessionStore::for_workspace(sessions.path(), workspace.path());

    let writer = store.create_at(UNIX_EPOCH).unwrap();

    assert_eq!(
        store.dir(),
        sessions.path().join(workspace_hash(workspace.path()))
    );
    assert_eq!(writer.path(), store.dir().join("19700101-000000.jsonl"));
}

#[test]
fn workspace_hash_is_stable_for_posix_and_windows_paths() {
    assert_eq!(
        workspace_hash(Path::new("/tmp/form")),
        workspace_hash(Path::new("/tmp/form"))
    );
    assert_eq!(
        workspace_hash(Path::new(r"C:\Users\me\form")),
        workspace_hash(Path::new(r"C:\Users\me\form"))
    );
}

#[test]
fn session_append_flushes_jsonl_before_close() {
    let sessions = TempDir::new("append-root");
    let workspace = TempDir::new("append-workspace");
    let store = SessionStore::for_workspace(sessions.path(), workspace.path());
    let mut writer = store.create_at(UNIX_EPOCH).unwrap();

    writer
        .append(&session_entry("entry-1", SessionEntryKind::User, "hello"))
        .unwrap();

    let contents = fs::read_to_string(writer.path()).unwrap();
    let lines: Vec<_> = contents.lines().collect();
    assert_eq!(lines.len(), 1);
    let entry: SessionEntry = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(entry.entry_id, "entry-1");
    assert_eq!(entry.kind, SessionEntryKind::User);
    assert_eq!(entry.content, vec![ContentBlock::text("hello")]);
}

#[test]
fn session_appends_multiple_kinds_in_order() {
    let temp = TempDir::new("append-order");
    let store = SessionStore::for_workspace(temp.path(), temp.path());
    let mut writer = store.create_at(UNIX_EPOCH).unwrap();

    for (id, kind) in [
        ("entry-1", SessionEntryKind::User),
        ("entry-2", SessionEntryKind::Assistant),
        ("entry-3", SessionEntryKind::ToolCall),
        ("entry-4", SessionEntryKind::ToolResult),
    ] {
        writer.append(&session_entry(id, kind, id)).unwrap();
    }

    let ids: Vec<_> = read_jsonl_path(writer.path())
        .unwrap()
        .iter()
        .map(|entry| entry["entry_id"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(ids, ["entry-1", "entry-2", "entry-3", "entry-4"]);
}

#[test]
fn resume_latest_reads_newest_session_and_appends_to_it() {
    let temp = TempDir::new("resume-latest");
    let store = SessionStore::for_workspace(temp.path(), temp.path());
    let older = UNIX_EPOCH + Duration::from_secs(1);
    let newer = UNIX_EPOCH + Duration::from_secs(86_400);
    store.create_at(older).unwrap();
    let newer_path = store.create_at(newer).unwrap().path().to_path_buf();

    let mut writer = store.resume_latest().unwrap().unwrap();
    writer
        .append(&session_entry(
            "entry-new",
            SessionEntryKind::User,
            "resume",
        ))
        .unwrap();

    assert_eq!(writer.path(), newer_path);
    let contents = fs::read_to_string(newer_path).unwrap();
    assert!(contents.contains("entry-new"));
}

#[test]
fn resume_latest_reports_none_without_sessions() {
    let temp = TempDir::new("resume-none");
    let store = SessionStore::for_workspace(temp.path(), temp.path());

    assert!(store.resume_latest().unwrap().is_none());
}

#[test]
fn corrupt_trailing_jsonl_is_reported() {
    let temp = TempDir::new("corrupt-jsonl");
    let path = temp.path().join("session.jsonl");
    fs::write(&path, "{\"kind\":\"user\"}\nnot-json\n").unwrap();

    let error = read_jsonl_path(&path).unwrap_err();

    match error {
        SessionJsonlError::Json {
            path: error_path,
            line,
            ..
        } => {
            assert_eq!(error_path, path);
            assert_eq!(line, 2);
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn inspection_fixture_renders_m1_shapes_and_future_kind() {
    let entries = vec![
        fixture_entry(
            "user",
            json!({ "content": [{ "type": "text", "text": "hi" }] }),
        ),
        fixture_entry(
            "assistant",
            json!({ "content": [{ "type": "text", "text": "hello" }] }),
        ),
        fixture_entry(
            "tool_call",
            json!({ "tool_call": { "id": "toolu_1", "name": "read", "input": { "path": "Cargo.toml" } } }),
        ),
        fixture_entry(
            "tool_result",
            json!({ "tool_result": { "tool_call_id": "toolu_1", "status": "ok", "content": [{ "type": "text", "text": "done" }] } }),
        ),
        fixture_entry(
            "approval",
            json!({ "approval": { "request_id": "approval-1", "decision": "deny", "reason": "no" } }),
        ),
        fixture_entry(
            "audit",
            json!({ "audit": { "event_kind": "approval_denied", "level": "audit" } }),
        ),
        fixture_entry("future_kind", json!({})),
        fixture_entry("user", json!({})),
    ];

    let rendered = render_timeline(&entries);

    for expected in [
        "user: hi",
        "assistant: hello",
        "tool_call toolu_1 read",
        "tool_result toolu_1 ok: done",
        "approval approval-1 deny no",
        "audit approval_denied audit",
        "unknown(future_kind): entry-1",
        "user: <no text>",
    ] {
        assert!(
            rendered.contains(expected),
            "missing {expected} in {rendered}"
        );
    }
}

fn session_entry(id: &str, kind: SessionEntryKind, text: &str) -> SessionEntry {
    SessionEntry {
        schema_version: 1,
        session_id: "session-1".to_owned(),
        branch_id: "branch-1".to_owned(),
        entry_id: id.to_owned(),
        parent_entry_id: None,
        ts: "2026-07-02T00:00:00Z".to_owned(),
        kind,
        content: vec![ContentBlock::text(text)],
        tool_call: None,
        tool_result: None,
        approval: None,
        audit: None,
        usage: None,
        meta: Value::Null,
    }
}

fn fixture_entry(kind: &str, fields: Value) -> Value {
    let mut entry = json!({
        "schema_version": 1,
        "session_id": "session-1",
        "branch_id": "branch-1",
        "entry_id": "entry-1",
        "parent_entry_id": null,
        "ts": "2026-07-02T00:00:00Z",
        "kind": kind,
        "meta": {}
    });

    let Value::Object(ref mut entry) = entry else {
        unreachable!();
    };
    let Value::Object(fields) = fields else {
        return Value::Object(entry.clone());
    };
    entry.extend(fields);
    Value::Object(entry.clone())
}

#[cfg(unix)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
