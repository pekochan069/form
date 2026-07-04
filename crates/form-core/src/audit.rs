use crate::AuditLevel;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditEntry {
    pub schema_version: u32,
    pub id: String,
    pub ts: String,
    pub event_kind: String,
    pub level: AuditLevel,
    pub session_id: Option<String>,
    pub branch_id: Option<String>,
    pub entry_id: Option<String>,
    pub tool_call_id: Option<String>,
    pub details: Value,
}
