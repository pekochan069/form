pub mod approval;
pub mod audit;
pub mod error;
pub mod limits;
pub mod message;
pub mod patch;
pub mod plugin;
pub mod provider;
pub mod session;
pub mod tool;

pub use approval::*;
pub use audit::*;
pub use error::*;
pub use limits::*;
pub use message::*;
pub use patch::*;
pub use plugin::*;
pub use provider::*;
pub use session::*;
pub use tool::*;

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

    #[test]
    fn public_contract_types_are_reexported() {
        let _ = ContentBlock::text("hello");
        let _ = SessionEntryKind::User;
        let _ = ToolResultStatus::Ok;
        let _ = ErrorSource::Cli;
        let _ = ProviderStopReason::EndTurn;
        let _ = ApprovalDecision::Allow;
        let _ = PatchSource::Tool;
        let _ = AuditLevel::Audit;
        let _ = Limits::default();
        let _ = PluginPermission::WorkspaceRead;
    }
}
