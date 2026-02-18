// Settings Service Layer
// Handles provider validation, Ollama detection, and API key checks.
// Delegates config persistence to the config service.

use crate::types::{OllamaStatus, ProviderInfo, ProviderValidationResult, SettingsConfig};

/// Check if an API key environment variable is set for a provider.
pub fn check_api_key(provider_id: &str) -> Result<bool, String> {
    let provider = ProviderInfo::find(provider_id)
        .ok_or_else(|| format!("Unknown provider: {}", provider_id))?;

    if !provider.requires_key {
        return Ok(true);
    }

    match provider.env_var {
        Some(ref var_name) => Ok(std::env::var(var_name).is_ok()),
        None => Ok(true),
    }
}

/// Check if Ollama is installed by looking for the binary.
pub fn check_ollama_installed() -> bool {
    std::process::Command::new("which")
        .arg("ollama")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if a specific model is available in Ollama.
pub async fn check_ollama_model(model: &str) -> Result<bool, String> {
    let output = tokio::process::Command::new("ollama")
        .arg("list")
        .output()
        .await
        .map_err(|e| format!("Failed to run ollama list: {}", e))?;

    if !output.status.success() {
        return Err("ollama list failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| {
        let name = line.split_whitespace().next().unwrap_or("");
        name == model || name.starts_with(&format!("{}:", model))
    }))
}

/// Get full Ollama status including installation and model availability.
pub async fn get_ollama_status(model: &str) -> OllamaStatus {
    let installed = check_ollama_installed();

    if !installed {
        return OllamaStatus {
            installed: false,
            model_available: false,
            error: Some("Ollama is not installed".to_string()),
        };
    }

    match check_ollama_model(model).await {
        Ok(available) => OllamaStatus {
            installed: true,
            model_available: available,
            error: if available {
                None
            } else {
                Some(format!(
                    "Model '{}' not found. Run: ollama pull {}",
                    model, model
                ))
            },
        },
        Err(e) => OllamaStatus {
            installed: true,
            model_available: false,
            error: Some(e),
        },
    }
}

/// Validate a provider's configuration completeness.
pub async fn validate_provider(config: &SettingsConfig) -> ProviderValidationResult {
    let provider_id = &config.provider;
    let mut checks_run = Vec::new();
    let mut errors = Vec::new();

    checks_run.push("provider_exists".to_string());
    let provider = match ProviderInfo::find(provider_id) {
        Some(p) => p,
        None => {
            errors.push(format!("Unknown provider: {}", provider_id));
            return ProviderValidationResult {
                valid: false,
                checks_run,
                errors,
            };
        }
    };

    if provider.requires_key {
        checks_run.push("api_key".to_string());
        match check_api_key(provider_id) {
            Ok(true) => {}
            Ok(false) => {
                let var_name = provider.env_var.as_deref().unwrap_or("UNKNOWN");
                errors.push(format!(
                    "Missing API key: {} environment variable not set",
                    var_name
                ));
            }
            Err(e) => errors.push(e),
        }
    }

    if provider_id == "ollama" {
        checks_run.push("ollama_installed".to_string());
        checks_run.push("ollama_model".to_string());

        let effective_model = config
            .model
            .as_deref()
            .unwrap_or(&provider.default_model);
        let status = get_ollama_status(effective_model).await;

        if !status.installed {
            errors.push("Ollama is not installed".to_string());
        } else if !status.model_available {
            if let Some(err) = status.error {
                errors.push(err);
            }
        }
    }

    ProviderValidationResult {
        valid: errors.is_empty(),
        checks_run,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_api_key_mock_provider() {
        let result = check_api_key("mock");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_api_key_ollama_provider() {
        let result = check_api_key("ollama");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_api_key_unknown_provider() {
        let result = check_api_key("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_provider_info_all_has_six_providers() {
        assert_eq!(ProviderInfo::all().len(), 6);
    }

    #[test]
    fn test_provider_info_find_valid() {
        assert!(ProviderInfo::find("ollama").is_some());
        assert!(ProviderInfo::find("openai").is_some());
        assert!(ProviderInfo::find("mock").is_some());
    }

    #[test]
    fn test_provider_info_find_invalid() {
        assert!(ProviderInfo::find("nonexistent").is_none());
    }

    #[test]
    fn test_settings_config_default() {
        let config = SettingsConfig::default();
        assert_eq!(config.provider, "ollama");
        assert!(config.model.is_none());
        assert!(config.projects_root.is_none());
    }

    #[tokio::test]
    async fn test_validate_provider_mock() {
        let config = SettingsConfig {
            provider: "mock".to_string(),
            model: None,
            projects_root: None,
        };
        let result = validate_provider(&config).await;
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validate_provider_unknown() {
        let config = SettingsConfig {
            provider: "nonexistent".to_string(),
            model: None,
            projects_root: None,
        };
        let result = validate_provider(&config).await;
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }
}
