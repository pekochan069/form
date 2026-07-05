use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

const SECRET_DIR_NAMES: &[&str] = &[".ssh", ".config"];
const SECRET_FILE_NAMES: &[&str] = &[
    "id_rsa",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    ".npmrc",
    ".pypirc",
    ".netrc",
    "credentials",
    "credentials.json",
    "token",
    "token.json",
    "secrets.toml",
    "secrets.json",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn detect_from(start: impl AsRef<Path>) -> io::Result<Self> {
        let start = fs::canonicalize(start.as_ref())?;
        let root = start
            .ancestors()
            .find(|ancestor| ancestor.join(".git").exists())
            .unwrap_or(&start);

        Self::new(root)
    }

    pub fn new(root: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            root: fs::canonicalize(root.as_ref())?,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resolve_existing(&self, requested: impl AsRef<Path>) -> io::Result<PathBuf> {
        let candidate = self.candidate(requested.as_ref())?;
        let resolved = fs::canonicalize(candidate)?;
        self.require_inside(resolved)
    }

    pub fn resolve_new_file(&self, requested: impl AsRef<Path>) -> io::Result<PathBuf> {
        let candidate = self.candidate(requested.as_ref())?;
        let parent = nearest_existing_parent(&candidate)?;
        let canonical_parent = self.require_inside(fs::canonicalize(parent)?)?;
        let tail = candidate.strip_prefix(parent).unwrap_or(Path::new(""));

        Ok(canonical_parent.join(tail))
    }

    fn candidate(&self, requested: &Path) -> io::Result<PathBuf> {
        if requested.as_os_str().is_empty() {
            return Err(invalid_input("empty workspace path"));
        }
        if has_parent_segment(requested) {
            return Err(outside_workspace(requested));
        }

        if requested.is_absolute() {
            Ok(requested.to_path_buf())
        } else {
            Ok(self.root.join(requested))
        }
    }

    fn require_inside(&self, path: PathBuf) -> io::Result<PathBuf> {
        if path.starts_with(&self.root) {
            Ok(path)
        } else {
            Err(outside_workspace(path))
        }
    }
}

pub fn is_secret_path(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    let components = path.components().filter_map(component_text);

    for component in components {
        if SECRET_DIR_NAMES.contains(&component) {
            return true;
        }
        if component.starts_with(".env") || SECRET_FILE_NAMES.contains(&component) {
            return true;
        }
    }

    false
}

fn nearest_existing_parent(path: &Path) -> io::Result<&Path> {
    let mut parent = path
        .parent()
        .ok_or_else(|| invalid_input("path has no parent directory"))?;

    loop {
        if parent.exists() {
            return Ok(parent);
        }
        parent = parent
            .parent()
            .ok_or_else(|| invalid_input("path has no existing parent directory"))?;
    }
}

fn has_parent_segment(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::ParentDir))
        || path
            .to_string_lossy()
            .split(['/', '\\'])
            .any(|part| part == "..")
}

fn component_text(component: Component<'_>) -> Option<&str> {
    match component {
        Component::Normal(value) => value.to_str(),
        _ => None,
    }
}

fn invalid_input(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn outside_workspace(path: impl AsRef<Path>) -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("path escapes workspace: {}", path.as_ref().display()),
    )
}
