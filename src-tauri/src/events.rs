// Event Bridge
// Maps daemon topics to Tauri app events for frontend subscription

use serde::Serialize;

/// Normalized Tauri event names (frontend-facing)
pub mod app_events {
    pub const DAEMON_ANALYSIS_PROGRESS: &str = "daemon:analysis-progress";
    pub const DAEMON_ANALYSIS_COMPLETED: &str = "daemon:analysis-completed";
    pub const DAEMON_ANALYSIS_STARTED: &str = "daemon:analysis-started";
    pub const DAEMON_PARSER_PROGRESS: &str = "daemon:parser-progress";
    pub const DAEMON_PARSER_COMPLETED: &str = "daemon:parser-completed";
    pub const DAEMON_STATUS: &str = "daemon:status";
    pub const APP_SETTINGS_CHANGED: &str = "app:settings-changed";
    pub const APP_PROJECT_SELECTED: &str = "app:project-selected";
}

/// Payload emitted when analysis progresses
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisProgressEvent {
    pub item_type: String,
    pub item_id: String,
    pub progress: f32,
    pub status: String,
    pub completed: usize,
    pub total: usize,
}

/// Payload emitted when analysis starts
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisStartedEvent {
    pub item_type: String,
    pub item_id: String,
    pub operation: String,
}

/// Payload emitted when analysis completes
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisCompletedEvent {
    pub item_type: String,
    pub item_id: String,
    pub success: bool,
    pub error: Option<String>,
}

/// Payload emitted when parser progresses
#[derive(Debug, Clone, Serialize)]
pub struct ParserProgressEvent {
    pub request_id: String,
    pub phase: String,
    pub progress: f32,
    pub line: Option<String>,
}

/// Payload emitted when parser completes
#[derive(Debug, Clone, Serialize)]
pub struct ParserCompletedEvent {
    pub request_id: String,
    pub success: bool,
    pub project_id: Option<String>,
    pub error: Option<String>,
}

/// Payload emitted when settings change
#[derive(Debug, Clone, Serialize)]
pub struct SettingsChangedEvent {
    pub provider: String,
    pub model: Option<String>,
}

/// Payload emitted when a project is selected
#[derive(Debug, Clone, Serialize)]
pub struct ProjectSelectedEvent {
    pub project_title: String,
    pub project_path: String,
}
