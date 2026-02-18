// Shared types for command handlers and IPC

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Project {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub path: String,
    pub scene_count: usize,
    pub modified: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub daemon_connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct DaemonCheckResult {
    pub reachable: bool,
    pub version: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProviderValidationResult {
    pub valid: bool,
    pub checks_run: Vec<String>,
    pub errors: Vec<String>,
}
