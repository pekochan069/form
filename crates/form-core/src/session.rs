use crate::{ApprovalSummary, AuditEntry, ContentBlock, ToolCall, ToolResult, Usage};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionEntryKind {
    User,
    Assistant,
    ToolCall,
    ToolResult,
    Approval,
    Summary,
    Audit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionEntry {
    pub schema_version: u32,
    pub session_id: String,
    pub branch_id: String,
    pub entry_id: String,
    pub parent_entry_id: Option<String>,
    pub ts: String,
    pub kind: SessionEntryKind,
    #[serde(default)]
    pub content: Vec<ContentBlock>,
    pub tool_call: Option<ToolCall>,
    pub tool_result: Option<ToolResult>,
    pub approval: Option<ApprovalSummary>,
    pub audit: Option<AuditEntry>,
    pub usage: Option<Usage>,
    #[serde(default)]
    pub meta: Value,
}
