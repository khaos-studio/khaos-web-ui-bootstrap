// Import Commands
// Handles screenplay file parsing and project registration

use tauri::command;

/// Start parse operation for screenplay file
#[command]
pub async fn start_parse(
    file_path: String,
    title: String,
    slug: String,
) -> Result<String, String> {
    // TODO: Implement wfl.parser.parse invocation
    Ok("request-id".to_string())
}

/// Get parse progress
#[command]
pub async fn get_parse_progress(request_id: String) -> Result<serde_json::Value, String> {
    // TODO: Implement progress tracking
    Ok(serde_json::json!({"progress": 0.0}))
}

/// Cancel ongoing parse operation
#[command]
pub async fn cancel_parse(request_id: String) -> Result<(), String> {
    // TODO: Implement cancellation
    Ok(())
}
