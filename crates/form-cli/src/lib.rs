pub mod config;
pub mod resources;
pub mod workspace;

use clap::{Command, error::ErrorKind};

pub struct CliOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run<I, S>(args: I) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| arg.as_ref().to_owned())
        .collect();
    let mut command = command();

    if args.is_empty() {
        return success(command.render_help().to_string());
    }

    match command.try_get_matches_from(std::iter::once("form".to_owned()).chain(args)) {
        Ok(_) => success(String::new()),
        Err(error) if error.kind() == ErrorKind::DisplayHelp => success(error.to_string()),
        Err(error) if error.kind() == ErrorKind::DisplayVersion => success(error.to_string()),
        Err(error) => CliOutput {
            exit_code: error.exit_code(),
            stdout: String::new(),
            stderr: error.to_string(),
        },
    }
}

fn success(stdout: String) -> CliOutput {
    CliOutput {
        exit_code: 0,
        stdout,
        stderr: String::new(),
    }
}

fn command() -> Command {
    Command::new("form")
        .about("Form CLI")
        .version(version())
        .disable_help_subcommand(true)
}

fn version() -> String {
    format!(
        "{} (form-core {})",
        env!("CARGO_PKG_VERSION"),
        form_core::version()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_succeeds() {
        let output = run(["--help"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("Form CLI"));
        assert!(output.stdout.contains("--version"));
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn version_succeeds() {
        let output = run(["--version"]);

        assert_eq!(output.exit_code, 0);
        assert!(output.stdout.contains("form 0.1.0"));
        assert!(output.stdout.contains("form-core 0.1.0"));
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn unknown_argument_fails() {
        let output = run(["chat"]);

        assert_eq!(output.exit_code, 2);
        assert!(output.stdout.is_empty());
        assert!(output.stderr.contains("unexpected argument 'chat'"));
        assert!(output.stderr.contains("Usage: form"));
    }
}
