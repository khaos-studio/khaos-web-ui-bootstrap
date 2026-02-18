// Settings Commands
// Handles configuration, theme, and provider validation

use crate::types::{DaemonCheckResult, ProviderValidationResult};
use serde_json::json;
use tauri::command;

/// Load current settings
#[command]
pub async fn load_settings() -> Result<serde_json::Value, String> {
    // TODO: Implement settings loading from config service
    Ok(json!({}))
}

/// Save settings and notify all windows
#[command]
pub async fn save_settings(settings: serde_json::Value) -> Result<(), String> {
    // TODO: Implement settings saving and app:settings-changed event emission
    Ok(())
}

/// Check if daemon is reachable
#[command]
pub async fn check_daemon_connection() -> Result<DaemonCheckResult, String> {
    // TODO: Implement daemon ping via wfl_client
    Ok(DaemonCheckResult {
        reachable: false,
        version: None,
        error: Some("Not implemented".to_string()),
    })
}

/// Validate provider configuration and credentials
#[command]
pub async fn validate_provider_config(
    provider: String,
    config: serde_json::Value,
) -> Result<ProviderValidationResult, String> {
    // TODO: Implement capability-driven validation
    Ok(ProviderValidationResult {
        valid: false,
        checks_run: vec![],
        errors: vec!["Not implemented".to_string()],
    })
}
