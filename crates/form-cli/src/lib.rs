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
    let mut args = args.into_iter();
    match args.next().map(|arg| arg.as_ref().to_owned()).as_deref() {
        None | Some("--help" | "-h") => success(help()),
        Some("--version" | "-V") => success(version()),
        Some(arg) => CliOutput {
            exit_code: 2,
            stdout: String::new(),
            stderr: format!("unknown argument: {arg}\n\n{}", help()),
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

fn help() -> String {
    "Form CLI\n\nUsage: form [--help] [--version]\n\nOptions:\n  -h, --help       Print help\n  -V, --version    Print version\n"
        .to_owned()
}

fn version() -> String {
    format!(
        "form {} (form-core {})\n",
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
        assert!(output.stderr.contains("unknown argument: chat"));
        assert!(output.stderr.contains("Usage: form"));
    }
}
