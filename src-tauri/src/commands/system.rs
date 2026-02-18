// System Commands
// Handles platform and runtime information

use crate::types::SystemInfo;
use tauri::command;

/// Get system information
#[command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        daemon_connected: false,
    })
}

/// Log message from frontend (for debugging)
#[command]
pub async fn log_message(level: String, message: String) -> Result<(), String> {
    match level.as_str() {
        "error" => tracing::error!("{}", message),
        "warn" => tracing::warn!("{}", message),
        "info" => tracing::info!("{}", message),
        "debug" => tracing::debug!("{}", message),
        _ => tracing::trace!("{}", message),
    }
    Ok(())
}
