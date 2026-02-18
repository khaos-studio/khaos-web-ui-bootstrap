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

// Settings types

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SettingsConfig {
    pub provider: String,
    pub model: Option<String>,
    pub projects_root: Option<String>,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            model: None,
            projects_root: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ProviderInfo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub default_model: String,
    pub requires_key: bool,
    pub env_var: Option<String>,
}

impl ProviderInfo {
    pub fn all() -> Vec<ProviderInfo> {
        vec![
            ProviderInfo {
                id: "ollama".to_string(),
                title: "Ollama".to_string(),
                description: "Local LLM inference via Ollama".to_string(),
                default_model: "qwen3".to_string(),
                requires_key: false,
                env_var: None,
            },
            ProviderInfo {
                id: "openai".to_string(),
                title: "OpenAI".to_string(),
                description: "GPT models via OpenAI API".to_string(),
                default_model: "gpt-4o".to_string(),
                requires_key: true,
                env_var: Some("OPENAI_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "mistralai".to_string(),
                title: "Mistral AI".to_string(),
                description: "Mistral models via Mistral API".to_string(),
                default_model: "mistral-large-latest".to_string(),
                requires_key: true,
                env_var: Some("MISTRALAI_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "anthropic".to_string(),
                title: "Anthropic".to_string(),
                description: "Claude models via Anthropic API".to_string(),
                default_model: "claude-sonnet-4-20250514".to_string(),
                requires_key: true,
                env_var: Some("ANTHROPIC_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "groq".to_string(),
                title: "Groq".to_string(),
                description: "Fast inference via Groq API".to_string(),
                default_model: "llama-3.3-70b-versatile".to_string(),
                requires_key: true,
                env_var: Some("GROQ_API_KEY".to_string()),
            },
            ProviderInfo {
                id: "mock".to_string(),
                title: "Mock".to_string(),
                description: "Mock provider for testing (no API calls)".to_string(),
                default_model: "mock-model".to_string(),
                requires_key: false,
                env_var: None,
            },
        ]
    }

    pub fn find(id: &str) -> Option<ProviderInfo> {
        Self::all().into_iter().find(|p| p.id == id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct OllamaStatus {
    pub installed: bool,
    pub model_available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct SettingsLoadResult {
    pub config: SettingsConfig,
    pub providers: Vec<ProviderInfo>,
}
