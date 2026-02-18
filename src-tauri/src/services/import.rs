// Import Service Layer
// Handles screenplay file validation, title normalization, collision detection,
// and khaos-tools CLI execution for project creation.

use crate::events::{app_events, ParserCompletedEvent, ParserProgressEvent};
use crate::services::discovery;
use crate::types::{ImportResult, Project};
use regex::Regex;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

const ALLOWED_EXTENSIONS: &[&str] = &[".fountain", ".fdx", ".sbx", ".md"];

/// Validate a project title (non-empty, max 255 chars).
/// Ported from khaos-tui domain/models.go:244-253
pub fn validate_title(title: &str) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    if title.len() > 255 {
        return Err("Title cannot exceed 255 characters".to_string());
    }
    Ok(())
}

/// Normalize a title to a filesystem-safe filename.
/// Ported from khaos-tui domain/models.go:255-273
pub fn normalize_project_filename(title: &str) -> String {
    // Replace problematic characters with underscores
    let re = Regex::new(r#"[<>:"/\\|?*]"#).unwrap();
    let normalized = re.replace_all(title, "_").to_string();

    // Replace multiple whitespace with single underscore
    let ws_re = Regex::new(r"\s+").unwrap();
    let normalized = ws_re.replace_all(&normalized, "_").to_string();

    // Trim leading/trailing underscores
    let normalized = normalized.trim_matches('_').to_string();

    // Fallback if empty
    if normalized.is_empty() {
        "project".to_string()
    } else {
        normalized
    }
}

/// Resolve a target .kspd path from projects dir and title.
/// Ported from khaos-tui cli/adapter.go:ResolveTargetPath
pub fn resolve_target_path(projects_dir: &str, title: &str) -> Result<String, String> {
    let normalized = normalize_project_filename(title);
    if normalized.is_empty() {
        return Err("Title resulted in empty filename".to_string());
    }

    let filename = format!("{}.kspd", normalized);
    let target = Path::new(projects_dir).join(filename);
    target
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid path encoding".to_string())
}

/// Validate an import file path (exists, not directory, allowed extension).
/// Ported from khaos-tui import/model.go:261-290
pub fn validate_import_file(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("File path cannot be empty".to_string());
    }

    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err(format!("File not found: {}", path));
    }

    if file_path.is_dir() {
        return Err(format!(
            "Please select a file, not a directory: {}",
            path
        ));
    }

    // Check extension
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e.to_lowercase()))
        .unwrap_or_default();

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(format!(
            "Unsupported file type \"{}\". Allowed: .fountain, .fdx, .sbx, .md",
            ext
        ));
    }

    Ok(())
}

/// Check if a collision exists at the target path.
pub fn check_collision(target_path: &str) -> bool {
    let path = Path::new(target_path);
    discovery::is_kspd(path)
}

/// Generate suggested alternative filenames for collision resolution.
/// Ported from khaos-tui import/model.go:611-623
pub fn generate_suggested_names(projects_dir: &str, title: &str, max: usize) -> Vec<String> {
    let normalized = normalize_project_filename(title);
    let mut suggestions = Vec::new();

    for i in 1..=(max + 5) {
        if suggestions.len() >= max {
            break;
        }
        let suggested = format!("{}_{}", normalized, i);
        let candidate_path = Path::new(projects_dir).join(format!("{}.kspd", suggested));
        if !discovery::is_kspd(&candidate_path) {
            suggestions.push(suggested);
        }
    }

    suggestions
}

/// Execute khaos-tools parser parse, streaming output via Tauri events.
pub async fn execute_parse(
    app_handle: AppHandle,
    request_id: &str,
    input_path: &str,
    output_path: &str,
) -> Result<ImportResult, String> {
    // Locate khaos-tools binary
    let cli_path = find_khaos_tools()?;

    tracing::info!(
        "Starting parse: {} -> {} (request: {})",
        input_path,
        output_path,
        request_id
    );

    // Spawn the child process
    let mut child = Command::new(&cli_path)
        .args(["parser", "parse", "--output", output_path, input_path])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn khaos-tools: {}", e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let req_id = request_id.to_string();
    let handle = app_handle.clone();

    // Stream stdout
    if let Some(stdout) = stdout {
        let req_id = req_id.clone();
        let handle = handle.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = handle.emit(
                    app_events::DAEMON_PARSER_PROGRESS,
                    ParserProgressEvent {
                        request_id: req_id.clone(),
                        phase: "parsing".to_string(),
                        progress: -1.0,
                        line: Some(line),
                    },
                );
            }
        });
    }

    // Stream stderr
    if let Some(stderr) = stderr {
        let req_id = req_id.clone();
        let handle = handle.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = handle.emit(
                    app_events::DAEMON_PARSER_PROGRESS,
                    ParserProgressEvent {
                        request_id: req_id.clone(),
                        phase: "parsing".to_string(),
                        progress: -1.0,
                        line: Some(line),
                    },
                );
            }
        });
    }

    // Wait for the process to finish
    let status = child
        .wait()
        .await
        .map_err(|e| format!("Failed to wait for khaos-tools: {}", e))?;

    let success = status.success();
    let project_id = if success {
        Some(Project::id_from_path(output_path))
    } else {
        None
    };

    let result = ImportResult {
        success,
        project_id: project_id.clone(),
        output_path: Some(output_path.to_string()),
        error: if success {
            None
        } else {
            Some(format!(
                "khaos-tools exited with code {}",
                status.code().unwrap_or(-1)
            ))
        },
    };

    // Emit completion event
    let _ = app_handle.emit(
        app_events::DAEMON_PARSER_COMPLETED,
        ParserCompletedEvent {
            request_id: req_id,
            success,
            project_id,
            error: result.error.clone(),
        },
    );

    tracing::info!(
        "Parse completed: success={}, output={}",
        success,
        output_path
    );

    Ok(result)
}

/// Find khaos-tools binary via PATH or config override.
fn find_khaos_tools() -> Result<String, String> {
    // Check config override first (shared TUI config)
    if let Ok(path) = load_khaos_tools_from_config() {
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }

    // Try PATH lookup
    if let Ok(output) = std::process::Command::new("which")
        .arg("khaos-tools")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(path);
            }
        }
    }

    Err("khaos-tools not found. Ensure it is installed and available on PATH.".to_string())
}

/// Load khaos_tools_path from shared TUI config
fn load_khaos_tools_from_config() -> Result<String, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    let config_path = format!("{}/.config/khaos-tui/config.json", home);

    let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let config: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(&content).map_err(|e| e.to_string())?;

    if let Some(serde_json::Value::String(path)) = config.get("khaos_tools_path") {
        Ok(path.clone())
    } else {
        Err("No khaos_tools_path in config".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_validate_title_empty() {
        assert!(validate_title("").is_err());
        assert!(validate_title("   ").is_err());
    }

    #[test]
    fn test_validate_title_too_long() {
        let long_title = "a".repeat(256);
        assert!(validate_title(&long_title).is_err());
    }

    #[test]
    fn test_validate_title_valid() {
        assert!(validate_title("My Screenplay").is_ok());
        assert!(validate_title("a").is_ok());
        assert!(validate_title(&"a".repeat(255)).is_ok());
    }

    #[test]
    fn test_normalize_project_filename() {
        assert_eq!(normalize_project_filename("My Project"), "My_Project");
        assert_eq!(
            normalize_project_filename("My Project!"),
            "My_Project!"
        );
        assert_eq!(
            normalize_project_filename("Test <file>"),
            "Test__file"
        );
        assert_eq!(
            normalize_project_filename("a/b\\c:d"),
            "a_b_c_d"
        );
        assert_eq!(
            normalize_project_filename("hello   world"),
            "hello_world"
        );
    }

    #[test]
    fn test_normalize_project_filename_special_chars() {
        assert_eq!(
            normalize_project_filename("test<>:\"/\\|?*end"),
            "test_________end"
        );
    }

    #[test]
    fn test_normalize_project_filename_trimming() {
        assert_eq!(
            normalize_project_filename("  *hello*  "),
            "hello"
        );
    }

    #[test]
    fn test_normalize_project_filename_empty() {
        assert_eq!(normalize_project_filename(""), "project");
        assert_eq!(normalize_project_filename("***"), "project");
    }

    #[test]
    fn test_resolve_target_path() {
        let result = resolve_target_path("/tmp/projects", "My Screenplay").unwrap();
        assert_eq!(result, "/tmp/projects/My_Screenplay.kspd");
    }

    #[test]
    fn test_resolve_target_path_normalizes() {
        let result = resolve_target_path("/home/user/projects", "Test: File").unwrap();
        assert_eq!(result, "/home/user/projects/Test__File.kspd");
    }

    #[test]
    fn test_validate_import_file_nonexistent() {
        let result = validate_import_file("/nonexistent/file.fountain");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File not found"));
    }

    #[test]
    fn test_validate_import_file_wrong_extension() {
        // Create a temp file with wrong extension
        let dir = std::env::temp_dir().join("khaos_import_test");
        let _ = fs::create_dir_all(&dir);
        let file_path = dir.join("test.txt");
        fs::write(&file_path, "content").unwrap();

        let result = validate_import_file(file_path.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file type"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_validate_import_file_directory() {
        let dir = std::env::temp_dir().join("khaos_import_test_dir");
        let _ = fs::create_dir_all(&dir);

        let result = validate_import_file(dir.to_str().unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a directory"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_validate_import_file_valid() {
        let dir = std::env::temp_dir().join("khaos_import_test_valid");
        let _ = fs::create_dir_all(&dir);

        for ext in &["fountain", "fdx", "sbx", "md"] {
            let file_path = dir.join(format!("test.{}", ext));
            fs::write(&file_path, "content").unwrap();
            assert!(
                validate_import_file(file_path.to_str().unwrap()).is_ok(),
                "Should accept .{} files",
                ext
            );
        }

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_validate_import_file_empty_path() {
        assert!(validate_import_file("").is_err());
    }

    #[test]
    fn test_check_collision_no_collision() {
        assert!(!check_collision("/nonexistent/path.kspd"));
    }

    #[test]
    fn test_generate_suggested_names() {
        let dir = std::env::temp_dir().join("khaos_suggest_test");
        let _ = fs::create_dir_all(&dir);

        let suggestions = generate_suggested_names(dir.to_str().unwrap(), "My Project", 3);
        assert_eq!(suggestions.len(), 3);
        assert_eq!(suggestions[0], "My_Project_1");
        assert_eq!(suggestions[1], "My_Project_2");
        assert_eq!(suggestions[2], "My_Project_3");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_generate_suggested_names_skips_existing() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = PathBuf::from(format!("/tmp/khaos_suggest_skip_{}", timestamp));
        let _ = fs::create_dir_all(&dir);

        // Create a colliding path (My_Project_1.kspd with manifest.json)
        let colliding = dir.join("My_Project_1.kspd");
        fs::create_dir_all(&colliding).unwrap();

        let suggestions = generate_suggested_names(dir.to_str().unwrap(), "My Project", 3);
        assert_eq!(suggestions.len(), 3);
        // Should skip _1 since it exists as .kspd dir
        assert_eq!(suggestions[0], "My_Project_2");
        assert_eq!(suggestions[1], "My_Project_3");
        assert_eq!(suggestions[2], "My_Project_4");

        let _ = fs::remove_dir_all(&dir);
    }
}
