// Import Commands
// Handles screenplay file parsing and project creation

use crate::services;
use crate::types::CollisionInfo;
use tauri::command;

/// Validate an import file path
#[command]
pub async fn validate_import_file(file_path: String) -> Result<(), String> {
    services::import::validate_import_file(&file_path)
}

/// Check if importing a project with this title would cause a collision.
/// Returns Some(CollisionInfo) if collision exists, None otherwise.
#[command]
pub async fn check_import_collision(title: String) -> Result<Option<CollisionInfo>, String> {
    let root = services::discovery::get_projects_root()?;

    services::import::validate_title(&title)?;

    let target_path = services::import::resolve_target_path(&root, &title)?;

    if services::import::check_collision(&target_path) {
        let suggested_names = services::import::generate_suggested_names(&root, &title, 5);
        Ok(Some(CollisionInfo {
            existing_path: target_path,
            suggested_names,
        }))
    } else {
        Ok(None)
    }
}

/// Resolve the target output path for a given title (without creating anything).
#[command]
pub async fn resolve_import_path(title: String) -> Result<String, String> {
    let root = services::discovery::get_projects_root()?;
    services::import::validate_title(&title)?;
    services::import::resolve_target_path(&root, &title)
}

/// Start a parse operation for a screenplay file.
/// Returns a request ID for tracking. Progress is streamed via Tauri events.
#[command]
pub async fn start_parse(
    app_handle: tauri::AppHandle,
    file_path: String,
    title: String,
    output_path: String,
    overwrite: bool,
) -> Result<String, String> {
    services::import::validate_import_file(&file_path)?;
    services::import::validate_title(&title)?;

    if !overwrite && services::import::check_collision(&output_path) {
        return Err(
            "Project already exists at target path. Confirm overwrite or choose a different name."
                .to_string(),
        );
    }

    let request_id = uuid::Uuid::new_v4().to_string();
    let req_id = request_id.clone();

    // Spawn the parse task asynchronously
    tokio::spawn(async move {
        match services::import::execute_parse(app_handle, &req_id, &file_path, &output_path).await
        {
            Ok(result) => {
                tracing::info!(
                    "Parse completed: success={}, request={}",
                    result.success,
                    req_id
                );
            }
            Err(e) => {
                tracing::error!("Parse failed for request {}: {}", req_id, e);
            }
        }
    });

    tracing::info!("Parse started: {} (request: {})", title, request_id);
    Ok(request_id)
}

/// Get parse progress (kept for compatibility; progress is event-driven)
#[command]
pub async fn get_parse_progress(request_id: String) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "request_id": request_id,
    }))
}

/// Cancel an ongoing parse operation
#[command]
pub async fn cancel_parse(_request_id: String) -> Result<(), String> {
    // TODO: Implement process-level cancellation via stored child PID
    Ok(())
}
