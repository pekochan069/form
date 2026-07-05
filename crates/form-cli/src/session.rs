use std::{
    error::Error,
    fmt, fs,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};

use form_core::SessionEntry;
use serde_json::Value;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub struct SessionStore {
    dir: PathBuf,
}

impl SessionStore {
    pub fn for_workspace(
        sessions_root: impl AsRef<Path>,
        workspace_root: impl AsRef<Path>,
    ) -> Self {
        Self {
            dir: sessions_root
                .as_ref()
                .join(workspace_hash(workspace_root.as_ref())),
        }
    }

    pub fn default_for_workspace(workspace_root: impl AsRef<Path>) -> io::Result<Self> {
        let home = home_dir().ok_or_else(|| invalid_input("home directory not found"))?;
        Ok(Self::for_workspace(
            home.join(".form/sessions"),
            workspace_root,
        ))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn create(&self) -> Result<SessionWriter, SessionJsonlError> {
        self.create_at(SystemTime::now())
    }

    pub fn create_at(&self, time: SystemTime) -> Result<SessionWriter, SessionJsonlError> {
        fs::create_dir_all(&self.dir).map_err(|source| SessionJsonlError::Io {
            path: self.dir.clone(),
            source,
        })?;
        SessionWriter::open(self.dir.join(session_file_name(time)?))
    }

    pub fn resume_latest(&self) -> Result<Option<SessionWriter>, SessionJsonlError> {
        let Some(path) = latest_jsonl_path(&self.dir)? else {
            return Ok(None);
        };
        SessionWriter::open(path).map(Some)
    }
}

pub struct SessionWriter {
    path: PathBuf,
    writer: BufWriter<fs::File>,
}

impl SessionWriter {
    fn open(path: PathBuf) -> Result<Self, SessionJsonlError> {
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|source| SessionJsonlError::Io {
                path: path.clone(),
                source,
            })?;
        Ok(Self {
            path,
            writer: BufWriter::new(file),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(&mut self, entry: &SessionEntry) -> Result<(), SessionJsonlError> {
        serde_json::to_writer(&mut self.writer, entry).map_err(|source| {
            SessionJsonlError::Encode {
                path: self.path.clone(),
                source,
            }
        })?;
        self.writer
            .write_all(b"\n")
            .and_then(|_| self.writer.flush())
            .map_err(|source| SessionJsonlError::Io {
                path: self.path.clone(),
                source,
            })
    }
}

#[derive(Debug)]
pub enum SessionJsonlError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Json {
        path: PathBuf,
        line: usize,
        source: serde_json::Error,
    },
    Encode {
        path: PathBuf,
        source: serde_json::Error,
    },
    Time {
        source: time::error::Format,
    },
}

impl fmt::Display for SessionJsonlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Json { path, line, source } => {
                write!(f, "{} line {line}: {source}", path.display())
            }
            Self::Encode { path, source } => write!(f, "{}: {source}", path.display()),
            Self::Time { source } => write!(f, "failed to format session timestamp: {source}"),
        }
    }
}

impl Error for SessionJsonlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Encode { source, .. } => Some(source),
            Self::Time { source } => Some(source),
        }
    }
}

pub fn read_jsonl_path(path: impl AsRef<Path>) -> Result<Vec<Value>, SessionJsonlError> {
    let path = path.as_ref();
    let file = fs::File::open(path).map_err(|source| SessionJsonlError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut values = Vec::new();

    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|source| SessionJsonlError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        values.push(
            serde_json::from_str(&line).map_err(|source| SessionJsonlError::Json {
                path: path.to_path_buf(),
                line: index + 1,
                source,
            })?,
        );
    }

    Ok(values)
}

pub fn render_timeline(entries: &[Value]) -> String {
    entries
        .iter()
        .map(render_line)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_line(entry: &Value) -> String {
    match entry
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
    {
        "user" => format!("user: {}", content_text(entry)),
        "assistant" => format!("assistant: {}", content_text(entry)),
        "tool_call" => render_tool_call(entry),
        "tool_result" => render_tool_result(entry),
        "approval" => render_approval(entry),
        "audit" => render_audit(entry),
        kind => format!("unknown({kind}): {}", entry_id(entry)),
    }
}

pub fn workspace_hash(path: &Path) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in path.to_string_lossy().as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn latest_jsonl_path(dir: &Path) -> Result<Option<PathBuf>, SessionJsonlError> {
    let mut paths = jsonl_paths(dir)?;
    paths.sort();
    Ok(paths.pop())
}

fn jsonl_paths(dir: &Path) -> Result<Vec<PathBuf>, SessionJsonlError> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .map(|entry| entry.map(|entry| entry.path()))
            .filter_map(|result| match result {
                Ok(path) if path.extension().and_then(|ext| ext.to_str()) == Some("jsonl") => {
                    Some(Ok(path))
                }
                Ok(_) => None,
                Err(source) => Some(Err(SessionJsonlError::Io {
                    path: dir.to_path_buf(),
                    source,
                })),
            })
            .collect(),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(source) => Err(SessionJsonlError::Io {
            path: dir.to_path_buf(),
            source,
        }),
    }
}

fn session_file_name(time: SystemTime) -> Result<String, SessionJsonlError> {
    let time = OffsetDateTime::from(time);
    let rfc3339 = time
        .format(&Rfc3339)
        .map_err(|source| SessionJsonlError::Time { source })?;
    Ok(format!(
        "{}{}{}-{}{}{}.jsonl",
        &rfc3339[0..4],
        &rfc3339[5..7],
        &rfc3339[8..10],
        &rfc3339[11..13],
        &rfc3339[14..16],
        &rfc3339[17..19]
    ))
}

fn render_tool_call(entry: &Value) -> String {
    let call = &entry["tool_call"];
    format!(
        "tool_call {} {} {}",
        call.get("id")
            .and_then(Value::as_str)
            .unwrap_or("<missing-id>"),
        call.get("name")
            .and_then(Value::as_str)
            .unwrap_or("<missing-name>"),
        call.get("input").unwrap_or(&Value::Null)
    )
}

fn render_tool_result(entry: &Value) -> String {
    let result = &entry["tool_result"];
    format!(
        "tool_result {} {}: {}",
        result
            .get("tool_call_id")
            .and_then(Value::as_str)
            .unwrap_or("<missing-call-id>"),
        result
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("<missing-status>"),
        value_text(result, "content")
    )
}

fn render_approval(entry: &Value) -> String {
    let approval = &entry["approval"];
    format!(
        "approval {} {} {}",
        approval
            .get("request_id")
            .and_then(Value::as_str)
            .unwrap_or("<missing-request-id>"),
        approval
            .get("decision")
            .and_then(Value::as_str)
            .unwrap_or("<missing-decision>"),
        approval
            .get("reason")
            .and_then(Value::as_str)
            .unwrap_or("<no-reason>")
    )
}

fn render_audit(entry: &Value) -> String {
    let audit = &entry["audit"];
    format!(
        "audit {} {}",
        audit
            .get("event_kind")
            .and_then(Value::as_str)
            .unwrap_or("<missing-event>"),
        audit
            .get("level")
            .and_then(Value::as_str)
            .unwrap_or("<missing-level>")
    )
}

fn content_text(entry: &Value) -> String {
    value_text(entry, "content")
}

fn value_text(value: &Value, field: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_array)
        .and_then(|items| items.iter().find_map(text_block))
        .unwrap_or("<no text>")
        .to_owned()
}

fn text_block(value: &Value) -> Option<&str> {
    match value.get("type").and_then(Value::as_str) {
        Some("text") => value.get("text").and_then(Value::as_str),
        _ => None,
    }
}

fn entry_id(entry: &Value) -> &str {
    entry
        .get("entry_id")
        .and_then(Value::as_str)
        .unwrap_or("<missing-entry-id>")
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn invalid_input(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
