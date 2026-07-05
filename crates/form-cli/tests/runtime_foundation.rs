use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use form_cli::{
    config::{DEFAULT_MODEL, load_from},
    resources::load_context,
    workspace::{Workspace, is_secret_path},
};

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
    let config_path = temp.path().join("config.toml");
    fs::write(&config_path, "model = 'gpt-test-model' # comment\n").unwrap();

    let config = load_from(Some(&config_path), None).unwrap();

    assert_eq!(config.model, "gpt-test-model");
    assert_eq!(config.openai_api_key, None);
}

#[test]
fn context_loads_form_then_agents_and_ignores_extra_files() {
    let temp = TempDir::new("context");
    fs::write(temp.path().join("FORM.md"), "form context").unwrap();
    fs::write(temp.path().join("AGENTS.md"), "agents context").unwrap();
    fs::write(temp.path().join("OTHER_AGENT.md"), "other agent context").unwrap();

    let files = load_context(temp.path()).unwrap();

    assert_eq!(files.len(), 2);
    assert_eq!(files[0].name, "FORM.md");
    assert_eq!(files[0].content, "form context");
    assert_eq!(files[1].name, "AGENTS.md");
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

#[cfg(unix)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
