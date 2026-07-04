use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalKind {
    Write,
    Edit,
    Shell,
    PluginPatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub kind: ApprovalKind,
    pub summary: String,
    pub path: Option<String>,
    pub command: Option<String>,
    pub diff: Option<String>,
    pub risk: String,
    pub tool_call_id: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Allow,
    Deny,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalScope {
    Once,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalResult {
    pub request_id: String,
    pub decision: ApprovalDecision,
    pub scope: ApprovalScope,
    pub reason: Option<String>,
    pub decided_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalSummary {
    pub request_id: String,
    pub decision: ApprovalDecision,
    pub reason: Option<String>,
}
