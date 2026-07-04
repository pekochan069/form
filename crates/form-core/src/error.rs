use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSource {
    Workspace,
    Provider,
    Tool,
    Session,
    Approval,
    Audit,
    Plugin,
    Cli,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditLevel {
    None,
    Log,
    Audit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub kind: String,
    pub source: ErrorSource,
    pub retryable: bool,
    pub user_message: String,
    pub audit_level: AuditLevel,
    pub exit_code: i32,
    pub details: Value,
    pub cause: Option<String>,
    pub redact_details: bool,
}
