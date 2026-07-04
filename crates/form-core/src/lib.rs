pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_crate_version() {
        assert_eq!(version(), "0.1.0");
    }
}
