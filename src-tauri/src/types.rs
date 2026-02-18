// Shared types for command handlers and IPC

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "serde")]
pub struct Project {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub path: String,
    pub scene_count: usize,
    pub modified: i64,  // Unix timestamp
}

/// Manifest.json structure found in KSPD projects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProjectManifest {
    pub title: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

impl Project {
    /// Generate a unique ID from the project path
    pub fn id_from_path(path: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("proj_{:x}", hasher.finish())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub daemon_connected: bool,
}

// Import wizard types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct CollisionInfo {
    pub existing_path: String,
    pub suggested_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ImportResult {
    pub success: bool,
    pub project_id: Option<String>,
    pub output_path: Option<String>,
    pub error: Option<String>,
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
