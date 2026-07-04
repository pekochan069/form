use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchSource {
    Tool,
    Plugin,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchProposal {
    pub id: String,
    pub source: PatchSource,
    pub target_path: String,
    pub unified_diff: String,
    pub summary: String,
    pub risk: String,
    pub created_at: String,
}
