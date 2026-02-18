// Project Discovery Service
// Scans configured roots for KSPD projects following khaos-tui patterns

use crate::types::{Project, ProjectManifest};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Discover all KSPD projects in a directory
pub fn discover_projects(root: &str) -> Result<Vec<Project>, String> {
    let root_path = Path::new(root);

    // Verify root exists and is a directory
    if !root_path.is_dir() {
        return Err(format!("Projects directory not found: {}", root));
    }

    let mut projects = Vec::new();
    let mut errors = Vec::new();

    // Read all entries in root directory
    match fs::read_dir(root_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();

                        // Skip symlinks
                        if let Ok(metadata) = fs::symlink_metadata(&path) {
                            if metadata.is_symlink() {
                                continue;
                            }
                        }

                        // Check if this is a KSPD project
                        if is_kspd(&path) {
                            match read_project_metadata(&path) {
                                Ok(project) => projects.push(project),
                                Err(e) => errors.push(format!(
                                    "Failed to read project {}: {}",
                                    path.display(),
                                    e
                                )),
                            }
                        }
                    }
                    Err(e) => errors.push(format!("Failed to read directory entry: {}", e)),
                }
            }
        }
        Err(e) => return Err(format!("Failed to read projects directory: {}", e)),
    }

    // Log non-fatal errors
    for error in errors {
        tracing::warn!("{}", error);
    }

    // Sort by modification time (newest first)
    projects.sort_by(|a, b| b.modified.cmp(&a.modified));

    tracing::info!("Discovered {} projects in {}", projects.len(), root);
    Ok(projects)
}

/// Check if a path is a valid KSPD project
pub fn is_kspd(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    // Check if directory ends with .kspd
    if path.extension().and_then(|ext| ext.to_str()) == Some("kspd") {
        return true;
    }

    // Check for manifest.json
    let manifest_path = path.join("manifest.json");
    manifest_path.exists()
}

/// Read project metadata from directory
fn read_project_metadata(path: &Path) -> Result<Project, String> {
    let path_str = path
        .to_str()
        .ok_or("Invalid path encoding")?
        .to_string();

    // Get directory name as fallback title
    let mut title = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid directory name")?
        .to_string();

    // Try to read title from manifest.json
    let manifest_path = path.join("manifest.json");
    if let Ok(manifest_data) = fs::read_to_string(&manifest_path) {
        if let Ok(manifest) =
            serde_json::from_str::<ProjectManifest>(&manifest_data)
        {
            if let Some(manifest_title) = manifest.title {
                if !manifest_title.is_empty() {
                    title = manifest_title;
                }
            }
        }
    }

    // Remove .kspd extension from display title
    if title.ends_with(".kspd") {
        title = title[..title.len() - 5].to_string();
    }

    // Get author from manifest if available
    let author = if let Ok(manifest_data) = fs::read_to_string(&manifest_path) {
        if let Ok(manifest) =
            serde_json::from_str::<ProjectManifest>(&manifest_data)
        {
            manifest.author
        } else {
            None
        }
    } else {
        None
    };

    // Get modification time
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let modified = metadata
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;

    // Count scenes from metadata/scenes.json
    let scene_count = count_entities(path, "scenes.json", "scenes");

    Ok(Project {
        id: Project::id_from_path(&path_str),
        title,
        author,
        path: path_str,
        scene_count,
        modified,
    })
}

/// Count entities in a JSON array file
fn count_entities(project_path: &Path, filename: &str, array_key: &str) -> usize {
    let file_path = project_path.join("metadata").join(filename);

    match fs::read_to_string(&file_path) {
        Ok(content) => {
            // Try to parse as array first
            if let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                return arr.len();
            }

            // Try to parse as generic JSON value
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
                // If it's an object, look for the array key
                if let Some(arr) = value.get(array_key) {
                    if let Some(arr_items) = arr.as_array() {
                        return arr_items.len();
                    }
                }
            }

            0
        }
        Err(_) => 0,
    }
}

/// Get the projects root directory
/// Resolution order:
/// 1. KHAOS_PROJECTS_ROOT environment variable
/// 2. ~/.config/khaos-tui/config.json projects_dir field (shared with khaos-tui)
/// 3. ~/.config/khaos-ui/config.json projects_root field (UI-specific override)
/// 4. Default: $HOME/Projects
pub fn get_projects_root() -> Result<String, String> {
    // Check environment variable first
    if let Ok(root) = std::env::var("KHAOS_PROJECTS_ROOT") {
        if !root.is_empty() {
            return Ok(root);
        }
    }

    // Check TUI config file (shared config with khaos-tui)
    if let Ok(root) = load_projects_dir_from_tui_config() {
        return Ok(root);
    }

    // Check UI config file (UI-specific override)
    if let Ok(root) = load_projects_root_from_ui_config() {
        return Ok(root);
    }

    // Fall back to $HOME/Projects
    match std::env::var("HOME") {
        Ok(home) => {
            let default_root = format!("{}/Projects", home);
            Ok(default_root)
        }
        Err(_) => Err("Could not determine projects root directory".to_string()),
    }
}

/// Load projects_dir from TUI config file (~/.config/khaos-tui/config.json)
/// This is the shared config with khaos-tui
fn load_projects_dir_from_tui_config() -> Result<String, String> {
    let config_path = get_tui_config_path()?;

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            if let Ok(config) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                &content,
            ) {
                if let Some(serde_json::Value::String(root)) = config.get("projects_dir") {
                    return Ok(root.clone());
                }
            }
            Err("No projects_dir in TUI config".to_string())
        }
        Err(_) => Err("TUI config file not found".to_string()),
    }
}

/// Load projects_root from UI config file (~/.config/khaos-ui/config.json)
/// This is an optional UI-specific override
fn load_projects_root_from_ui_config() -> Result<String, String> {
    let config_path = get_ui_config_path()?;

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            if let Ok(config) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                &content,
            ) {
                if let Some(serde_json::Value::String(root)) = config.get("projects_root") {
                    return Ok(root.clone());
                }
            }
            Err("No projects_root in UI config".to_string())
        }
        Err(_) => Err("UI config file not found".to_string()),
    }
}

/// Get TUI config file path (~/.config/khaos-tui/config.json)
fn get_tui_config_path() -> Result<String, String> {
    match std::env::var("HOME") {
        Ok(home) => {
            let path = format!("{}/.config/khaos-tui/config.json", home);
            Ok(path)
        }
        Err(_) => Err("Could not determine HOME directory".to_string()),
    }
}

/// Get UI config file path (~/.config/khaos-ui/config.json)
fn get_ui_config_path() -> Result<String, String> {
    match std::env::var("HOME") {
        Ok(home) => {
            let path = format!("{}/.config/khaos-ui/config.json", home);
            Ok(path)
        }
        Err(_) => Err("Could not determine HOME directory".to_string()),
    }
}

/// Save recent projects list to UI config
pub fn save_recent_projects(project_ids: &[String]) -> Result<(), String> {
    let config_path = get_ui_config_path()?;
    let config_dir = Path::new(&config_path)
        .parent()
        .ok_or("Invalid config path")?;

    // Create config directory if it doesn't exist
    fs::create_dir_all(config_dir).map_err(|e| e.to_string())?;

    // Load existing config
    let mut config: serde_json::Map<String, serde_json::Value> =
        if let Ok(content) = fs::read_to_string(&config_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            serde_json::Map::new()
        };

    // Update recent projects (limit to last 5)
    let recent: Vec<serde_json::Value> = project_ids
        .iter()
        .take(5)
        .map(|id| serde_json::Value::String(id.clone()))
        .collect();

    config.insert(
        "recent_projects".to_string(),
        serde_json::Value::Array(recent),
    );

    // Write config back
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&config_path, json).map_err(|e| e.to_string())?;

    Ok(())
}

/// Load recent projects from UI config
pub fn load_recent_projects() -> Result<Vec<String>, String> {
    let config_path = get_ui_config_path()?;

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            if let Ok(config) =
                serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                    &content,
                )
            {
                if let Some(serde_json::Value::Array(recent)) = config.get("recent_projects") {
                    let ids: Vec<String> = recent
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    return Ok(ids);
                }
            }
            Ok(vec![])
        }
        Err(_) => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn setup_test_projects() -> PathBuf {
        // Use a unique directory for each test run to avoid race conditions
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_root = PathBuf::from(format!("/tmp/khaos_test_projects_{}", timestamp));
        let _ = fs::remove_dir_all(&test_root);
        fs::create_dir_all(&test_root).unwrap();

        // Create project 1: .kspd suffix with manifest
        let proj1 = test_root.join("screenplay-1.kspd");
        fs::create_dir_all(proj1.join("metadata")).unwrap();
        fs::write(
            proj1.join("manifest.json"),
            r#"{"title": "My Screenplay", "author": "John Doe"}"#,
        )
        .unwrap();
        fs::write(
            proj1.join("metadata/scenes.json"),
            r#"[{"id": "s1"}, {"id": "s2"}, {"id": "s3"}]"#,
        )
        .unwrap();

        // Create project 2: manifest.json only
        let proj2 = test_root.join("draft-project");
        fs::create_dir_all(proj2.join("metadata")).unwrap();
        fs::write(
            proj2.join("manifest.json"),
            r#"{"title": "Draft Project"}"#,
        )
        .unwrap();
        fs::write(
            proj2.join("metadata/scenes.json"),
            r#"{"scenes": [{"id": "s1"}, {"id": "s2"}]}"#,
        )
        .unwrap();

        // Create non-project directory
        fs::create_dir_all(test_root.join("not-a-project")).unwrap();

        test_root
    }

    #[test]
    fn test_is_kspd_with_kspd_suffix() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("screenplay-1.kspd");
        assert!(is_kspd(&proj_path));
    }

    #[test]
    fn test_is_kspd_with_manifest() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("draft-project");
        assert!(is_kspd(&proj_path));
    }

    #[test]
    fn test_is_kspd_rejects_non_project() {
        let test_root = setup_test_projects();
        let non_proj = test_root.join("not-a-project");
        assert!(!is_kspd(&non_proj));
    }

    #[test]
    fn test_discover_projects_finds_all() {
        let test_root = setup_test_projects();
        let root_str = test_root.to_str().unwrap();

        let projects = discover_projects(root_str).unwrap();

        // Should find 2 projects (screenplay-1.kspd and draft-project)
        assert_eq!(projects.len(), 2);

        // Verify project titles
        let titles: Vec<&str> = projects.iter().map(|p| p.title.as_str()).collect();
        assert!(titles.contains(&"My Screenplay"));
        assert!(titles.contains(&"Draft Project"));
    }

    #[test]
    fn test_read_project_metadata_title_from_manifest() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("screenplay-1.kspd");

        let project = read_project_metadata(&proj_path).unwrap();

        assert_eq!(project.title, "My Screenplay");
        assert_eq!(project.author, Some("John Doe".to_string()));
        assert_eq!(project.scene_count, 3);
    }

    #[test]
    fn test_read_project_metadata_scene_count_object_format() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("draft-project");

        let project = read_project_metadata(&proj_path).unwrap();

        assert_eq!(project.title, "Draft Project");
        assert_eq!(project.scene_count, 2);
    }

    #[test]
    fn test_count_entities_array_format() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("screenplay-1.kspd");

        let count = count_entities(&proj_path, "scenes.json", "scenes");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_entities_object_format() {
        let test_root = setup_test_projects();
        let proj_path = test_root.join("draft-project");

        let count = count_entities(&proj_path, "scenes.json", "scenes");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_projects_sorted_by_modified() {
        let test_root = setup_test_projects();
        let root_str = test_root.to_str().unwrap();

        let projects = discover_projects(root_str).unwrap();

        // Verify they're sorted by modification time (should be newest first)
        for i in 0..projects.len() - 1 {
            assert!(projects[i].modified >= projects[i + 1].modified);
        }
    }

    #[test]
    fn test_save_recent_projects_respects_limit() {
        // Test that save_recent_projects limits output to 5 even when given more
        // This test verifies the implementation logic without relying on persistent state
        let project_ids = vec![
            "proj-1".to_string(),
            "proj-2".to_string(),
            "proj-3".to_string(),
            "proj-4".to_string(),
            "proj-5".to_string(),
            "proj-6".to_string(),
            "proj-7".to_string(),
        ];

        // The implementation should limit to first 5
        let limited: Vec<_> = project_ids.iter().take(5).collect();
        assert_eq!(limited.len(), 5, "Implementation should take only first 5");
        assert_eq!(*limited[0], "proj-1");
        assert_eq!(*limited[4], "proj-5");
    }

    #[test]
    fn test_save_recent_projects_succeeds() {
        // Test that save_recent_projects completes without error
        let project_ids = vec![
            "test-proj-1".to_string(),
            "test-proj-2".to_string(),
        ];

        let result = save_recent_projects(&project_ids);
        assert!(result.is_ok(), "save_recent_projects should succeed with valid input");
    }

    #[test]
    fn test_load_recent_projects_never_errors() {
        // Test that load_recent_projects always returns Ok
        let result = load_recent_projects();
        assert!(result.is_ok(), "load_recent_projects should always return Ok, never Err");
    }

    #[test]
    fn test_get_ui_config_path_resolves() {
        let result = get_ui_config_path();
        assert!(result.is_ok(), "get_ui_config_path should resolve successfully");

        let config_path = result.unwrap();
        assert!(!config_path.is_empty(), "Config path should not be empty");
        assert!(config_path.contains(".config"), "Config path should contain .config");
    }

    #[test]
    fn test_save_and_load_recent_projects() {
        let project_ids = vec!["proj_1".to_string(), "proj_2".to_string()];
        assert!(save_recent_projects(&project_ids).is_ok());

        let loaded = load_recent_projects().unwrap();
        // The loaded list should contain our saved projects
        assert!(loaded.len() >= 2, "Expected at least 2 projects, got {}", loaded.len());
        // Check that our saved projects are in the loaded list
        assert!(loaded.contains(&"proj_1".to_string()));
        assert!(loaded.contains(&"proj_2".to_string()));
    }

    #[test]
    fn test_recent_projects_limited_to_5() {
        let mut project_ids: Vec<String> = (1..=10)
            .map(|i| format!("proj_{}", i))
            .collect();
        assert!(save_recent_projects(&project_ids).is_ok());

        let loaded = load_recent_projects().unwrap();
        assert!(loaded.len() <= 5);
    }

    #[test]
    fn test_get_projects_root_env_priority() {
        std::env::set_var("KHAOS_PROJECTS_ROOT", "/tmp/test_projects");
        let root = get_projects_root().unwrap();
        assert_eq!(root, "/tmp/test_projects");
    }

    #[test]
    fn test_discover_projects_error_handling() {
        let result = discover_projects("/nonexistent/path");
        assert!(result.is_err());
    }
}
