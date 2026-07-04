use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    pub file_read_bytes: u64,
    pub tool_output_bytes: u64,
    pub tool_output_lines: u64,
    pub shell_timeout_seconds: u64,
    pub plugin_timeout_seconds: u64,
    pub plugin_memory_bytes: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            file_read_bytes: 256 * 1024,
            tool_output_bytes: 50 * 1024,
            tool_output_lines: 2_000,
            shell_timeout_seconds: 30,
            plugin_timeout_seconds: 2,
            plugin_memory_bytes: 64 * 1024 * 1024,
        }
    }
}
