// Dashboard Commands
// Two-channel architecture:
//   - khaos-tools CLI for reads (parser query, bot query-analysis)
//   - khaos-wfl daemon for analysis dispatch + progress events (falls back to CLI)

use crate::services;
use crate::types::*;
use tauri::{command, AppHandle};

/// Get scene list for a project
#[command]
pub async fn get_scenes(project_path: String) -> Result<Vec<SceneSummary>, String> {
    services::dashboard::query_scenes(&project_path).await
}

/// Get character list for a project
#[command]
pub async fn get_characters(project_path: String) -> Result<Vec<CharacterSummary>, String> {
    services::dashboard::query_characters(&project_path).await
}

/// Get location list for a project
#[command]
pub async fn get_locations(project_path: String) -> Result<Vec<LocationSummary>, String> {
    services::dashboard::query_locations(&project_path).await
}

/// Get project summary (scene/character/location counts)
#[command]
pub async fn get_project_summary(project_path: String) -> Result<ProjectSummary, String> {
    services::dashboard::query_project_summary(&project_path).await
}

/// Get scene detail (summary + analysis)
#[command]
pub async fn get_scene_detail(
    project_path: String,
    scene_id: String,
) -> Result<SceneDetail, String> {
    let scenes = services::dashboard::query_scenes(&project_path).await?;
    let summary = scenes
        .into_iter()
        .find(|s| s.id == scene_id)
        .ok_or_else(|| format!("Scene not found: {}", scene_id))?;
    let analysis = services::dashboard::query_scene_analysis(&project_path, &scene_id).await?;
    Ok(SceneDetail { summary, analysis })
}

/// Get character detail (summary + analysis)
#[command]
pub async fn get_character_detail(
    project_path: String,
    character_id: String,
) -> Result<CharacterDetail, String> {
    let characters = services::dashboard::query_characters(&project_path).await?;
    let summary = characters
        .into_iter()
        .find(|c| c.id == character_id)
        .ok_or_else(|| format!("Character not found: {}", character_id))?;
    let analysis =
        services::dashboard::query_character_analysis(&project_path, &character_id).await?;
    Ok(CharacterDetail { summary, analysis })
}

/// Get location detail (summary + analysis)
#[command]
pub async fn get_location_detail(
    project_path: String,
    location_id: String,
) -> Result<LocationDetail, String> {
    let locations = services::dashboard::query_locations(&project_path).await?;
    let summary = locations
        .into_iter()
        .find(|l| l.id == location_id)
        .ok_or_else(|| format!("Location not found: {}", location_id))?;
    let analysis =
        services::dashboard::query_location_analysis(&project_path, &location_id).await?;
    Ok(LocationDetail { summary, analysis })
}

/// Trigger analysis for a single scene — prefers daemon, falls back to CLI
#[command]
pub async fn analyze_scene(
    app: AppHandle,
    project_path: String,
    scene_id: String,
) -> Result<AnalysisResult, String> {
    services::dashboard::analyze_scene(&app, &project_path, &scene_id).await
}

/// Trigger analysis for a single character
#[command]
pub async fn analyze_character(
    app: AppHandle,
    project_path: String,
    character_id: String,
) -> Result<AnalysisResult, String> {
    services::dashboard::analyze_character(&app, &project_path, &character_id).await
}

/// Trigger analysis for a single location
#[command]
pub async fn analyze_location(
    app: AppHandle,
    project_path: String,
    location_id: String,
) -> Result<AnalysisResult, String> {
    services::dashboard::analyze_location(&app, &project_path, &location_id).await
}

/// Trigger analysis for all items in a section — prefers daemon, falls back to CLI
#[command]
pub async fn analyze_all(
    app: AppHandle,
    project_path: String,
    section: String,
) -> Result<AnalysisResult, String> {
    services::dashboard::analyze_all(&app, &project_path, &section).await
}

/// Scan analysis index for existing results
#[command]
pub async fn scan_analysis_index(project_path: String) -> Result<AnalysisIndex, String> {
    Ok(services::dashboard::scan_analysis_index(&project_path))
}

/// Get analysis results for a specific entity (reads saved analysis)
#[command]
pub async fn get_analysis_results(
    project_path: String,
    entity_type: String,
    entity_id: String,
) -> Result<serde_json::Value, String> {
    match entity_type.as_str() {
        "scene" => {
            let analysis =
                services::dashboard::query_scene_analysis(&project_path, &entity_id).await?;
            Ok(serde_json::to_value(analysis).unwrap_or(serde_json::json!(null)))
        }
        "character" => {
            let analysis =
                services::dashboard::query_character_analysis(&project_path, &entity_id).await?;
            Ok(serde_json::to_value(analysis).unwrap_or(serde_json::json!(null)))
        }
        "location" => {
            let analysis =
                services::dashboard::query_location_analysis(&project_path, &entity_id).await?;
            Ok(serde_json::to_value(analysis).unwrap_or(serde_json::json!(null)))
        }
        _ => Err(format!("Unknown entity type: {}", entity_type)),
    }
}

/// Check if khaos-wfl daemon is running for a project
#[command]
pub async fn get_daemon_status(project_path: String) -> Result<DaemonStatus, String> {
    Ok(services::dashboard::check_daemon_status(&project_path).await)
}

/// Start khaos-wfl daemon for a project
#[command]
pub async fn start_daemon(project_path: String) -> Result<DaemonStatus, String> {
    services::dashboard::start_daemon(&project_path).await
}
