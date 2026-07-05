use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::paths::CONTEXT_FILE_NAMES;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextFile {
    pub name: &'static str,
    pub path: PathBuf,
    pub content: String,
}

pub fn load_context(root: impl AsRef<Path>) -> io::Result<Vec<ContextFile>> {
    let root = root.as_ref();
    let mut files = Vec::new();

    for name in CONTEXT_FILE_NAMES {
        let path = root.join(name);
        match fs::read_to_string(&path) {
            Ok(content) => files.push(ContextFile {
                name,
                path,
                content,
            }),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }

    Ok(files)
}
