// Project Discovery Service
// Scans configured roots for KSPD projects

use crate::types::Project;

pub fn discover_projects() -> Result<Vec<Project>, String> {
    // TODO: Implement filesystem scanning
    Ok(vec![])
}

pub fn get_recent_projects() -> Result<Vec<Project>, String> {
    // TODO: Load from recent-projects cache
    Ok(vec![])
}
