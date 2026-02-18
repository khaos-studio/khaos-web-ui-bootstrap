// Projects Commands
// Handles project discovery, listing, and navigation

use crate::types::Project;
use tauri::command;

/// Discover projects from configured roots
#[command]
pub async fn discover_projects() -> Result<Vec<Project>, String> {
    // TODO: Implement filesystem scanning via services::discovery
    Ok(vec![])
}

/// Search projects by title or author
#[command]
pub async fn search_projects(query: String) -> Result<Vec<Project>, String> {
    // TODO: Implement search filtering
    Ok(vec![])
}

/// Get single project details
#[command]
pub async fn get_project(project_id: String) -> Result<Project, String> {
    // TODO: Implement project loading
    Err("Not implemented".to_string())
}
