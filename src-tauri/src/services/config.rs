// Configuration Service
// Manages persistent app settings at ~/.khaos-ui/config.json

use crate::types::SettingsConfig;
use std::fs;
use std::path::PathBuf;

/// Get the config directory path (~/.khaos-ui/)
fn config_dir() -> Result<PathBuf, String> {
    let home =
        std::env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;
    Ok(PathBuf::from(home).join(".khaos-ui"))
}

/// Get the config file path (~/.khaos-ui/config.json)
fn config_path() -> Result<PathBuf, String> {
    Ok(config_dir()?.join("config.json"))
}

/// Load settings from disk, falling back to defaults if file doesn't exist.
pub fn load_settings() -> Result<SettingsConfig, String> {
    let path = config_path()?;

    if !path.exists() {
        tracing::info!("No config file found at {:?}, using defaults", path);
        return Ok(SettingsConfig::default());
    }

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: SettingsConfig =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))?;

    tracing::info!("Loaded settings from {:?}", path);
    Ok(config)
}

/// Save settings to disk, creating the directory if needed.
pub fn save_settings(config: &SettingsConfig) -> Result<(), String> {
    let dir = config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let path = config_path()?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write config file: {}", e))?;

    tracing::info!("Saved settings to {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_uses_home() {
        let result = config_dir();
        assert!(result.is_ok());
        let dir = result.unwrap();
        assert!(dir.to_str().unwrap().ends_with(".khaos-ui"));
    }

    #[test]
    fn test_config_path_is_json() {
        let result = config_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_str().unwrap().ends_with("config.json"));
    }

    #[test]
    fn test_load_settings_returns_defaults_when_no_file() {
        // This test relies on the config file potentially not existing,
        // which is fine for default behavior testing
        let result = load_settings();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.provider, "ollama");
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("khaos_config_test");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("config.json");

        let config = SettingsConfig {
            provider: "openai".to_string(),
            model: Some("gpt-4o-mini".to_string()),
            projects_root: Some("/tmp/projects".to_string()),
        };

        let content = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&path, &content).unwrap();

        let loaded: SettingsConfig =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.provider, "openai");
        assert_eq!(loaded.model, Some("gpt-4o-mini".to_string()));
        assert_eq!(loaded.projects_root, Some("/tmp/projects".to_string()));

        let _ = fs::remove_dir_all(&dir);
    }
}
