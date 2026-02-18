// Projects Commands
// Handles project discovery, listing, and navigation

use crate::services;
use crate::types::Project;
use tauri::command;

/// Discover projects from configured roots
#[command]
pub async fn discover_projects() -> Result<Vec<Project>, String> {
    let root = services::discovery::get_projects_root()?;
    tracing::info!("Discovering projects from: {}", root);
    services::discovery::discover_projects(&root)
}

/// Search projects by title or author
#[command]
pub async fn search_projects(query: String) -> Result<Vec<Project>, String> {
    if query.trim().is_empty() {
        // Empty query returns all projects
        return discover_projects().await;
    }

    let root = services::discovery::get_projects_root()?;
    let mut projects = services::discovery::discover_projects(&root)?;

    let query_lower = query.to_lowercase();

    // Filter by title, author, or path matching
    projects.retain(|proj| {
        proj.title.to_lowercase().contains(&query_lower)
            || proj
                .author
                .as_ref()
                .map(|a| a.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
            || proj.path.to_lowercase().contains(&query_lower)
    });

    Ok(projects)
}

/// Get single project details by ID
#[command]
pub async fn get_project(project_id: String) -> Result<Project, String> {
    let root = services::discovery::get_projects_root()?;
    let projects = services::discovery::discover_projects(&root)?;

    projects
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Project not found: {}", project_id))
}

/// Set a project as active (adds to recent projects list)
/// Loads existing recent projects, inserts this project at the front,
/// limits to 5 most recent, and saves back to config
#[command]
pub async fn set_active_project(project_id: String) -> Result<(), String> {
    // Load current recent projects list
    let mut recent_ids = services::discovery::load_recent_projects()?;

    // Remove project_id if it's already in the list
    recent_ids.retain(|id| id != &project_id);

    // Insert at the front
    recent_ids.insert(0, project_id.clone());

    // Limit to 5 most recent
    recent_ids.truncate(5);

    // Save back to config
    services::discovery::save_recent_projects(&recent_ids)?;

    tracing::info!("Set project as active: {}", project_id);
    Ok(())
}

/// Delete a project by removing it from the filesystem
#[command]
pub async fn delete_project(project_id: String) -> Result<(), String> {
    let root = services::discovery::get_projects_root()?;
    let projects = services::discovery::discover_projects(&root)?;

    // Find project by ID
    let project = projects
        .into_iter()
        .find(|p| p.id == project_id)
        .ok_or_else(|| format!("Project not found: {}", project_id))?;

    // Delete the project directory
    std::fs::remove_dir_all(&project.path).map_err(|e| {
        format!("Failed to delete project directory: {}", e)
    })?;

    // Remove from recent projects if it's there
    let mut recent_ids = services::discovery::load_recent_projects().unwrap_or_default();
    recent_ids.retain(|id| id != &project_id);
    let _ = services::discovery::save_recent_projects(&recent_ids);

    tracing::info!("Deleted project: {} at {}", project_id, project.path);
    Ok(())
}
