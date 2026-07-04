use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PluginPermission {
    #[serde(rename = "workspace.search")]
    WorkspaceSearch,
    #[serde(rename = "workspace.read")]
    WorkspaceRead,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "patch.propose")]
    PatchPropose,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginCommand {
    pub id: String,
    pub description: String,
    pub input: String,
    pub output: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub component: String,
    pub permissions: Vec<PluginPermission>,
    pub commands: Vec<PluginCommand>,
}
