// Settings Commands
// Handles configuration loading/saving, provider validation, and daemon connectivity

use crate::events::{app_events, SettingsChangedEvent};
use crate::services;
use crate::types::{
    DaemonCheckResult, ProviderInfo, ProviderValidationResult, SettingsConfig, SettingsLoadResult,
};
use tauri::{command, AppHandle, Emitter};

/// Load current settings and provider list
#[command]
pub async fn load_settings() -> Result<SettingsLoadResult, String> {
    let config = services::config::load_settings()?;
    let providers = ProviderInfo::all();
    Ok(SettingsLoadResult { config, providers })
}

/// Save settings and notify all windows
#[command]
pub async fn save_settings(app: AppHandle, config: SettingsConfig) -> Result<(), String> {
    services::config::save_settings(&config)?;

    let _ = app.emit(
        app_events::APP_SETTINGS_CHANGED,
        SettingsChangedEvent {
            provider: config.provider.clone(),
            model: config.model.clone(),
        },
    );

    tracing::info!("Settings saved and event emitted");
    Ok(())
}

/// Check if daemon is reachable
#[command]
pub async fn check_daemon_connection() -> Result<DaemonCheckResult, String> {
    // TODO: Implement real daemon ping via wfl_client when daemon is available
    Ok(DaemonCheckResult {
        reachable: false,
        version: None,
        error: Some("Daemon connection not yet implemented".to_string()),
    })
}

/// Validate provider configuration and credentials
#[command]
pub async fn validate_provider_config(
    config: SettingsConfig,
) -> Result<ProviderValidationResult, String> {
    Ok(services::settings::validate_provider(&config).await)
}
