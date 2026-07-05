use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub const DEFAULT_MODEL: &str = "gpt5.5";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub model: String,
    pub openai_api_key: Option<String>,
}

pub fn load() -> io::Result<Config> {
    let config_path = default_config_path();
    load_from(config_path.as_deref(), env::var("OPENAI_API_KEY").ok())
}

pub fn load_from(config_path: Option<&Path>, openai_api_key: Option<String>) -> io::Result<Config> {
    let mut model = DEFAULT_MODEL.to_owned();

    if let Some(path) = config_path {
        match fs::read_to_string(path) {
            Ok(contents) => {
                if let Some(config_model) = parse_model(&contents) {
                    model = config_model;
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }

    Ok(Config {
        model,
        openai_api_key: openai_api_key.filter(|key| !key.is_empty()),
    })
}

pub fn default_config_path() -> Option<PathBuf> {
    home_dir().map(|home| home.join(".form").join("config.toml"))
}

fn parse_model(contents: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        if key.trim() != "model" {
            return None;
        }

        let value = value.split('#').next().unwrap_or_default().trim();
        parse_quoted(value)
    })
}

fn parse_quoted(value: &str) -> Option<String> {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .map(str::to_owned)
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}
