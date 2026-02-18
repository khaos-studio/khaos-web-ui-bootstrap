// Dashboard Service Layer
// Two-channel architecture:
//   - khaos-tools CLI for reads (parser query, bot query-analysis)
//   - khaos-wfl daemon via wfl_client for analysis dispatch + real-time events

use crate::events::{
    app_events, AnalysisCompletedEvent, AnalysisProgressEvent, AnalysisStartedEvent,
};
use crate::types::*;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::process::Command;
use tokio::sync::{oneshot, Mutex};

use std::sync::{Arc, OnceLock};

// ---------------------------------------------------------------------------
// CLI binary resolution (shared with import service)
// ---------------------------------------------------------------------------

fn find_khaos_tools() -> Result<String, String> {
    if let Ok(path) = load_khaos_tools_from_config() {
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }

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

fn find_khaos_wfl() -> Result<String, String> {
    if let Ok(output) = std::process::Command::new("which")
        .arg("khaos-wfl")
        .output()
    {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(path);
            }
        }
    }

    Err("khaos-wfl not found. Ensure it is installed and available on PATH.".to_string())
}

// ---------------------------------------------------------------------------
// CLI Adapter — reads via khaos-tools subprocess
// ---------------------------------------------------------------------------

/// Run khaos-tools with args and parse JSON output
async fn run_khaos_tools(args: &[&str]) -> Result<serde_json::Value, String> {
    let cli_path = find_khaos_tools()?;

    tracing::debug!("Running: {} {}", cli_path, args.join(" "));

    let output = Command::new(&cli_path)
        .args(args)
        .output()
        .await
        .map_err(|e| format!("Failed to run khaos-tools: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "khaos-tools exited with code {}: {}",
            output.status.code().unwrap_or(-1),
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse khaos-tools output: {}", e))
}

fn extract_array_field(data: serde_json::Value, key: &str) -> serde_json::Value {
    use serde_json::Value;

    match data {
        Value::Array(_) => data,
        Value::Object(map) => match map.get(key) {
            Some(Value::Array(_)) => map.get(key).cloned().unwrap_or(Value::Array(vec![])),
            Some(Value::Null) | None => Value::Array(vec![]),
            Some(_) => Value::Array(vec![]),
        },
        Value::Null => Value::Array(vec![]),
        _ => Value::Array(vec![]),
    }
}

fn as_string(v: Option<&serde_json::Value>) -> String {
    v.and_then(|x| x.as_str()).unwrap_or_default().to_string()
}

fn as_usize(v: Option<&serde_json::Value>) -> usize {
    v.and_then(|x| x.as_u64()).unwrap_or(0) as usize
}

fn as_string_vec(v: Option<&serde_json::Value>) -> Vec<String> {
    v.and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_scene_summaries(value: serde_json::Value) -> Result<Vec<SceneSummary>, String> {
    let Some(arr) = value.as_array() else {
        return Ok(vec![]);
    };

    let mut out = Vec::with_capacity(arr.len());
    for (idx, item) in arr.iter().enumerate() {
        let Some(obj) = item.as_object() else {
            continue;
        };

        out.push(SceneSummary {
            id: as_string(obj.get("id")),
            index: obj
                .get("index")
                .and_then(|v| v.as_u64())
                .map(|n| n as usize)
                .unwrap_or(idx),
            slugline: as_string(obj.get("slugline")),
            duration: as_string(obj.get("duration")),
            word_count: as_usize(obj.get("word_count")),
            line_count: as_usize(obj.get("line_count")),
            characters: as_string_vec(obj.get("characters")),
        });
    }
    Ok(out)
}

fn parse_character_summaries(value: serde_json::Value) -> Result<Vec<CharacterSummary>, String> {
    let Some(arr) = value.as_array() else {
        return Ok(vec![]);
    };

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let Some(obj) = item.as_object() else {
            continue;
        };

        out.push(CharacterSummary {
            id: as_string(obj.get("id")),
            name: as_string(obj.get("name")),
            dialogue_lines: as_usize(obj.get("dialogue_lines")),
            words: as_usize(obj.get("words")),
            scene_count: as_usize(obj.get("scene_count")),
            percentage: obj
                .get("percentage")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
        });
    }
    Ok(out)
}

fn parse_location_summaries(value: serde_json::Value) -> Result<Vec<LocationSummary>, String> {
    let Some(arr) = value.as_array() else {
        return Ok(vec![]);
    };

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let Some(obj) = item.as_object() else {
            continue;
        };

        out.push(LocationSummary {
            id: as_string(obj.get("id")),
            name: as_string(obj.get("name")),
            scene_count: as_usize(obj.get("scene_count")),
            page_count: as_usize(obj.get("page_count")),
        });
    }
    Ok(out)
}

/// Query project summary (scene/character/location counts)
pub async fn query_scenes(kspd_path: &str) -> Result<Vec<SceneSummary>, String> {
    let data = run_khaos_tools(&[
        "parser", "query", "--type", "scenes", "--format", "json", kspd_path,
    ])
    .await?;

    // khaos-tools may return { "scenes": [...] }, { "scenes": null }, or just [...]
    let scenes_val = extract_array_field(data, "scenes");

    parse_scene_summaries(scenes_val).map_err(|e| format!("Failed to parse scenes: {}", e))
}

pub async fn query_characters(kspd_path: &str) -> Result<Vec<CharacterSummary>, String> {
    let data = run_khaos_tools(&[
        "parser", "query", "--type", "characters", "--format", "json", kspd_path,
    ])
    .await?;

    let chars_val = extract_array_field(data, "characters");

    parse_character_summaries(chars_val)
        .map_err(|e| format!("Failed to parse characters: {}", e))
}

pub async fn query_locations(kspd_path: &str) -> Result<Vec<LocationSummary>, String> {
    let data = run_khaos_tools(&[
        "parser", "query", "--type", "locations", "--format", "json", kspd_path,
    ])
    .await?;

    let locs_val = extract_array_field(data, "locations");

    parse_location_summaries(locs_val)
        .map_err(|e| format!("Failed to parse locations: {}", e))
}

pub async fn query_project_summary(kspd_path: &str) -> Result<ProjectSummary, String> {
    let data = run_khaos_tools(&[
        "parser", "query", "--type", "statistics", "--format", "json", kspd_path,
    ])
    .await?;

    // Statistics output varies; extract counts
    let stats = data.get("statistics");

    let scenes = data
        .get("scene_count")
        .or(data.get("scenes"))
        .or_else(|| stats.and_then(|s| s.get("scenes")))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let characters = data
        .get("character_count")
        .or(data.get("characters"))
        .or_else(|| stats.and_then(|s| s.get("characters")))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let locations = data
        .get("location_count")
        .or(data.get("locations"))
        .or_else(|| stats.and_then(|s| s.get("locations")))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let compositions = data
        .get("composition_count")
        .or(data.get("compositions"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    Ok(ProjectSummary {
        scenes,
        characters,
        locations,
        compositions,
    })
}

/// Query previously-saved analysis for a scene
pub async fn query_scene_analysis(
    kspd_path: &str,
    scene_id: &str,
) -> Result<Option<SceneAnalysis>, String> {
    match run_khaos_tools(&[
        "bot", "query-analysis", "--json", kspd_path, "scene", scene_id,
    ])
    .await
    {
        Ok(data) => {
            let analysis: SceneAnalysis = serde_json::from_value(data)
                .map_err(|e| format!("Failed to parse scene analysis: {}", e))?;
            Ok(Some(analysis))
        }
        Err(e) => {
            if e.contains("not found") || e.contains("no analysis") || e.contains("exit") {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

pub async fn query_character_analysis(
    kspd_path: &str,
    character_id: &str,
) -> Result<Option<CharacterAnalysis>, String> {
    match run_khaos_tools(&[
        "bot",
        "query-analysis",
        "--json",
        kspd_path,
        "character",
        character_id,
    ])
    .await
    {
        Ok(data) => {
            let analysis: CharacterAnalysis = serde_json::from_value(data)
                .map_err(|e| format!("Failed to parse character analysis: {}", e))?;
            Ok(Some(analysis))
        }
        Err(e) => {
            if e.contains("not found") || e.contains("no analysis") || e.contains("exit") {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

pub async fn query_location_analysis(
    kspd_path: &str,
    location_id: &str,
) -> Result<Option<LocationAnalysis>, String> {
    match run_khaos_tools(&[
        "bot",
        "query-analysis",
        "--json",
        kspd_path,
        "location",
        location_id,
    ])
    .await
    {
        Ok(data) => {
            let analysis: LocationAnalysis = serde_json::from_value(data)
                .map_err(|e| format!("Failed to parse location analysis: {}", e))?;
            Ok(Some(analysis))
        }
        Err(e) => {
            if e.contains("not found") || e.contains("no analysis") || e.contains("exit") {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Analysis Index — scan metadata/analysis/ for existing results
// ---------------------------------------------------------------------------

pub fn scan_analysis_index(kspd_path: &str) -> AnalysisIndex {
    let metadata_dir = Path::new(kspd_path).join("metadata").join("analysis");
    let mut index = AnalysisIndex {
        scenes: vec![],
        characters: vec![],
        locations: vec![],
    };

    if !metadata_dir.exists() {
        return index;
    }

    if let Ok(entries) = std::fs::read_dir(&metadata_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.ends_with(".json") {
                continue;
            }
            let id = name.trim_end_matches(".json").to_string();
            if id.starts_with("scn_") {
                index.scenes.push(id);
            } else if id.starts_with("chr_") {
                index.characters.push(id);
            } else if id.starts_with("loc_") {
                index.locations.push(id);
            }
        }
    }

    index
}

// ---------------------------------------------------------------------------
// Analysis Dispatch — prefers wfl daemon, falls back to direct CLI
// ---------------------------------------------------------------------------

/// Check if khaos-wfl daemon is running for the given project
pub async fn check_daemon_status(kspd_path: &str) -> DaemonStatus {
    // Try connecting to daemon via wfl_client
    match crate::wfl_client::DaemonClient::connect().await {
        Ok(client) => {
            // Ping and get status
            match client.request("wfl.getStatus", serde_json::json!({})).await {
                Ok(status) => {
                    let project_path = status
                        .get("projectPath")
                        .or(status.get("project"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let watching = status
                        .get("watching")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let busy = status
                        .get("busy")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let queue_depth = status
                        .get("queueDepth")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as usize;

                    // Verify project path matches
                    let running = project_path
                        .as_ref()
                        .map(|p| {
                            let daemon_canonical =
                                std::fs::canonicalize(p).unwrap_or_else(|_| p.into());
                            let requested_canonical = std::fs::canonicalize(kspd_path)
                                .unwrap_or_else(|_| kspd_path.into());
                            daemon_canonical == requested_canonical
                        })
                        .unwrap_or(false);

                    DaemonStatus {
                        running,
                        project_path,
                        watching,
                        busy,
                        queue_depth,
                    }
                }
                Err(_) => DaemonStatus {
                    running: false,
                    project_path: None,
                    watching: false,
                    busy: false,
                    queue_depth: 0,
                },
            }
        }
        Err(_) => DaemonStatus {
            running: false,
            project_path: None,
            watching: false,
            busy: false,
            queue_depth: 0,
        },
    }
}

pub async fn get_daemon_status_with_bridge(app: &AppHandle, kspd_path: &str) -> DaemonStatus {
    let status = check_daemon_status(kspd_path).await;
    if status.running {
        ensure_daemon_event_bridge(app, kspd_path).await;
    }
    status
}

/// Analyze a scene — tries daemon first, falls back to CLI
pub async fn analyze_scene(
    app: &AppHandle,
    kspd_path: &str,
    scene_id: &str,
) -> Result<AnalysisResult, String> {
    let _ = app.emit(
        app_events::DAEMON_ANALYSIS_STARTED,
        AnalysisStartedEvent {
            item_type: "scene".to_string(),
            item_id: scene_id.to_string(),
            operation: "analyze-scene".to_string(),
        },
    );

    // Try daemon first
    let daemon_status = check_daemon_status(kspd_path).await;
    if daemon_status.running {
        return analyze_via_daemon(app, kspd_path, "analyze-scene", scene_id).await;
    }

    // Fall back to direct CLI
    analyze_via_cli(app, kspd_path, "scene", scene_id).await
}

pub async fn analyze_character(
    app: &AppHandle,
    kspd_path: &str,
    character_id: &str,
) -> Result<AnalysisResult, String> {
    let _ = app.emit(
        app_events::DAEMON_ANALYSIS_STARTED,
        AnalysisStartedEvent {
            item_type: "character".to_string(),
            item_id: character_id.to_string(),
            operation: "analyze-character".to_string(),
        },
    );

    let daemon_status = check_daemon_status(kspd_path).await;
    if daemon_status.running {
        return analyze_via_daemon(app, kspd_path, "analyze-character", character_id).await;
    }

    analyze_via_cli(app, kspd_path, "character", character_id).await
}

pub async fn analyze_location(
    app: &AppHandle,
    kspd_path: &str,
    location_id: &str,
) -> Result<AnalysisResult, String> {
    let _ = app.emit(
        app_events::DAEMON_ANALYSIS_STARTED,
        AnalysisStartedEvent {
            item_type: "location".to_string(),
            item_id: location_id.to_string(),
            operation: "analyze-location".to_string(),
        },
    );

    let daemon_status = check_daemon_status(kspd_path).await;
    if daemon_status.running {
        return analyze_via_daemon(app, kspd_path, "analyze-location", location_id).await;
    }

    analyze_via_cli(app, kspd_path, "location", location_id).await
}

/// Analyze all items for a section — prefers daemon, falls back to CLI
pub async fn analyze_all(
    app: &AppHandle,
    kspd_path: &str,
    section: &str,
) -> Result<AnalysisResult, String> {
    let operation = match section {
        "scenes" => "analyze-all-scenes",
        "characters" => "analyze-all-characters",
        "locations" => "analyze-all-locations",
        _ => return Err(format!("Unknown section: {}", section)),
    };

    let _ = app.emit(
        app_events::DAEMON_ANALYSIS_STARTED,
        AnalysisStartedEvent {
            item_type: section.to_string(),
            item_id: "all".to_string(),
            operation: operation.to_string(),
        },
    );

    let daemon_status = check_daemon_status(kspd_path).await;
    if daemon_status.running {
        return analyze_via_daemon(app, kspd_path, operation, "all").await;
    }

    // Fall back to CLI — analyze-screenplay does all at once
    let cli_cmd = match section {
        "scenes" => "analyze-screenplay",
        "characters" => "analyze-screenplay",
        "locations" => "analyze-screenplay",
        _ => return Err(format!("Unknown section: {}", section)),
    };
    analyze_via_cli(app, kspd_path, cli_cmd, "all").await
}

// ---------------------------------------------------------------------------
// Daemon path — wfl.analyze via wfl_client UDS
// ---------------------------------------------------------------------------

async fn analyze_via_daemon(
    app: &AppHandle,
    kspd_path: &str,
    operation: &str,
    item_id: &str,
) -> Result<AnalysisResult, String> {
    ensure_daemon_event_bridge(app, kspd_path).await;

    let client = crate::wfl_client::DaemonClient::connect()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    // Load provider/model from settings
    let settings = crate::services::config::load_settings().unwrap_or_default();

    let mut params = serde_json::json!({
        "projectPath": kspd_path,
        "operation": operation,
        "wait": false,
        "verbose": true,
    });

    // Add provider if configured
    if !settings.provider.is_empty() {
        params["provider"] = serde_json::json!(settings.provider);
    }

    // Add entity ID params
    let item_type = if operation.contains("scene") {
        "scene"
    } else if operation.contains("character") {
        "character"
    } else {
        "location"
    };

    if item_id != "all" {
        match item_type {
            "scene" => params["sceneId"] = serde_json::json!(item_id),
            "character" => params["characterId"] = serde_json::json!(item_id),
            "location" => params["locationId"] = serde_json::json!(item_id),
            _ => {}
        }
    }

    match client.request("wfl.analyze", params).await {
        Ok(result) => {
            let queued = result
                .get("queued")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            tracing::info!(
                "Analysis dispatched via daemon: operation={}, queued={}",
                operation,
                queued
            );

            Ok(AnalysisResult {
                success: queued,
                item_type: item_type.to_string(),
                item_id: item_id.to_string(),
                error: if !queued {
                    Some("Analysis was not queued".to_string())
                } else {
                    None
                },
            })
        }
        Err(e) => {
            let _ = app.emit(
                app_events::DAEMON_ANALYSIS_COMPLETED,
                AnalysisCompletedEvent {
                    item_type: item_type.to_string(),
                    item_id: item_id.to_string(),
                    success: false,
                    error: Some(e.to_string()),
                },
            );
            Err(format!("Daemon analysis failed: {}", e))
        }
    }
}

// ---------------------------------------------------------------------------
// CLI path — direct khaos-tools invocation (fallback when no daemon)
// ---------------------------------------------------------------------------

async fn analyze_via_cli(
    app: &AppHandle,
    kspd_path: &str,
    entity_type: &str,
    entity_id: &str,
) -> Result<AnalysisResult, String> {
    let cli_path = find_khaos_tools()?;

    // Load provider/model from settings
    let settings = crate::services::config::load_settings().unwrap_or_default();

    let mut args: Vec<String> = vec!["bot".to_string()];

    // Build command based on entity type
    match entity_type {
        "scene" => {
            args.push("analyze-scene".to_string());
            args.push("--save".to_string());
            if !settings.provider.is_empty() {
                args.push("--provider".to_string());
                args.push(settings.provider.clone());
            }
            args.push(kspd_path.to_string());
            args.push(entity_id.to_string());
        }
        "character" => {
            args.push("analyze-character".to_string());
            args.push("--save".to_string());
            if !settings.provider.is_empty() {
                args.push("--provider".to_string());
                args.push(settings.provider.clone());
            }
            args.push(kspd_path.to_string());
            args.push(entity_id.to_string());
        }
        "location" => {
            args.push("analyze-location".to_string());
            args.push("--save".to_string());
            if !settings.provider.is_empty() {
                args.push("--provider".to_string());
                args.push(settings.provider.clone());
            }
            args.push(kspd_path.to_string());
            args.push(entity_id.to_string());
        }
        "analyze-screenplay" => {
            args.push("analyze-screenplay".to_string());
            args.push("--save".to_string());
            if !settings.provider.is_empty() {
                args.push("--provider".to_string());
                args.push(settings.provider.clone());
            }
            args.push(kspd_path.to_string());
        }
        _ => return Err(format!("Unknown entity type: {}", entity_type)),
    }

    tracing::info!("Running analysis via CLI: {} {}", cli_path, args.join(" "));

    let app_handle = app.clone();
    let item_type = entity_type.to_string();
    let item_id_owned = entity_id.to_string();

    // Spawn and stream output
    let mut child = Command::new(&cli_path)
        .args(&args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn khaos-tools: {}", e))?;

    let stdout = child.stdout.take();
    let app_clone = app_handle.clone();
    let it = item_type.clone();
    let ii = item_id_owned.clone();

    if let Some(stdout) = stdout {
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                tracing::debug!("khaos-tools stdout: {}", line);
                let _ = app_clone.emit(
                    app_events::DAEMON_ANALYSIS_PROGRESS,
                    AnalysisProgressEvent {
                        item_type: it.clone(),
                        item_id: ii.clone(),
                        progress: -1.0,
                        status: "running".to_string(),
                        completed: 0,
                        total: 0,
                    },
                );
            }
        });
    }

    let status = child
        .wait()
        .await
        .map_err(|e| format!("Failed to wait for khaos-tools: {}", e))?;

    let success = status.success();

    let _ = app_handle.emit(
        app_events::DAEMON_ANALYSIS_COMPLETED,
        AnalysisCompletedEvent {
            item_type: item_type.clone(),
            item_id: item_id_owned.clone(),
            success,
            error: if success {
                None
            } else {
                Some(format!(
                    "khaos-tools exited with code {}",
                    status.code().unwrap_or(-1)
                ))
            },
        },
    );

    Ok(AnalysisResult {
        success,
        item_type,
        item_id: item_id_owned,
        error: if success {
            None
        } else {
            Some(format!(
                "Analysis failed with exit code {}",
                status.code().unwrap_or(-1)
            ))
        },
    })
}

// ---------------------------------------------------------------------------
// Daemon event bridge — subscribe to wfl events and re-emit to frontend
// ---------------------------------------------------------------------------

struct DaemonBridgeHandle {
    project_path: String,
    stop_tx: Option<oneshot::Sender<()>>,
    task: tokio::task::JoinHandle<()>,
}

static DAEMON_BRIDGE: OnceLock<Arc<Mutex<Option<DaemonBridgeHandle>>>> = OnceLock::new();

fn daemon_bridge_state() -> Arc<Mutex<Option<DaemonBridgeHandle>>> {
    DAEMON_BRIDGE
        .get_or_init(|| Arc::new(Mutex::new(None)))
        .clone()
}

pub async fn ensure_daemon_event_bridge(app: &AppHandle, kspd_path: &str) {
    let state = daemon_bridge_state();
    let mut guard = state.lock().await;

    if let Some(existing) = guard.as_ref() {
        if existing.project_path == kspd_path && !existing.task.is_finished() {
            return;
        }
    }

    if let Some(mut existing) = guard.take() {
        if let Some(stop_tx) = existing.stop_tx.take() {
            let _ = stop_tx.send(());
        }
        existing.task.abort();
    }

    let (stop_tx, stop_rx) = oneshot::channel();
    let app_clone = app.clone();
    let project_path = kspd_path.to_string();
    let task = tokio::spawn(async move {
        run_daemon_event_bridge(app_clone, project_path, stop_rx).await;
    });

    *guard = Some(DaemonBridgeHandle {
        project_path: kspd_path.to_string(),
        stop_tx: Some(stop_tx),
        task,
    });
}

async fn run_daemon_event_bridge(
    app: AppHandle,
    kspd_path: String,
    mut stop_rx: oneshot::Receiver<()>,
) {
    let socket_path = std::env::var("KHAOS_WFL_SOCKET").unwrap_or_else(|_| "/tmp/khaos-wfl.sock".to_string());

    loop {
        let connect = UnixStream::connect(&socket_path);
        tokio::select! {
            _ = &mut stop_rx => return,
            result = connect => {
                let stream = match result {
                    Ok(s) => s,
                    Err(err) => {
                        tracing::debug!("Daemon bridge connect failed: {}", err);
                        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
                        continue;
                    }
                };

                if let Err(err) = bridge_session(&app, &kspd_path, stream, &mut stop_rx).await {
                    tracing::debug!("Daemon bridge session ended: {}", err);
                }
            }
        }
    }
}

async fn bridge_session(
    app: &AppHandle,
    kspd_path: &str,
    mut stream: UnixStream,
    stop_rx: &mut oneshot::Receiver<()>,
) -> Result<(), String> {
    let subscribe = serde_json::json!({
        "v": 1,
        "id": "webui-wfl-subscribe",
        "type": "req",
        "method": "wfl.subscribe",
        "params": {
            "topics": [
                "wfl.analysis.started",
                "wfl.analysis.queued",
                "wfl.analysis.progress",
                "wfl.analysis.active",
                "wfl.analysis.completed"
            ]
        }
    });

    let payload = serde_json::to_vec(&subscribe).map_err(|e| e.to_string())?;
    stream
        .write_all(&payload)
        .await
        .map_err(|e| format!("failed to write subscribe request: {}", e))?;
    stream
        .write_all(b"\n")
        .await
        .map_err(|e| format!("failed to write newline: {}", e))?;
    stream
        .flush()
        .await
        .map_err(|e| format!("failed to flush subscribe request: {}", e))?;

    let mut lines = BufReader::new(stream).lines();

    loop {
        tokio::select! {
            _ = &mut *stop_rx => return Ok(()),
            read = lines.next_line() => {
                let line = match read {
                    Ok(Some(line)) => line,
                    Ok(None) => return Err("daemon closed stream".to_string()),
                    Err(e) => return Err(format!("failed reading daemon stream: {}", e)),
                };

                if let Ok(envelope) = serde_json::from_str::<serde_json::Value>(&line) {
                    consume_daemon_envelope(app, kspd_path, &envelope);
                }
            }
        }
    }
}

fn consume_daemon_envelope(app: &AppHandle, kspd_path: &str, envelope: &serde_json::Value) {
    let typ = envelope.get("type").and_then(|v| v.as_str()).unwrap_or("");
    if typ != "evt" {
        return;
    }

    let topic = envelope.get("topic").and_then(|v| v.as_str()).unwrap_or("");
    let data = envelope
        .get("data")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    match topic {
        "wfl.analysis.started" | "wfl.analysis.queued" => {
            if let Some(run) = parse_wfl_run(&data) {
                if !event_matches_project(&run.project_path, kspd_path) {
                    return;
                }
                let _ = app.emit(
                    app_events::DAEMON_ANALYSIS_STARTED,
                    AnalysisStartedEvent {
                        item_type: run.target_kind.unwrap_or_else(|| "unknown".to_string()),
                        item_id: run.target_id.unwrap_or_else(|| "all".to_string()),
                        operation: run.op.unwrap_or_else(|| "analysis".to_string()),
                    },
                );
            }
        }
        "wfl.analysis.progress" | "wfl.analysis.active" => {
            if let Some(run) = parse_wfl_run(&data) {
                if !event_matches_project(&run.project_path, kspd_path) {
                    return;
                }
                let total = run.total.unwrap_or(0);
                let completed = run.completed.unwrap_or(0);
                let progress = run.percent.map(|p| (p as f32) / 100.0).unwrap_or_else(|| {
                    if total > 0 {
                        (completed as f32) / (total as f32)
                    } else {
                        0.0
                    }
                });

                let _ = app.emit(
                    app_events::DAEMON_ANALYSIS_PROGRESS,
                    AnalysisProgressEvent {
                        item_type: run.target_kind.unwrap_or_else(|| "unknown".to_string()),
                        item_id: run.target_id.unwrap_or_else(|| "all".to_string()),
                        progress,
                        status: run.status.unwrap_or_else(|| "running".to_string()),
                        completed: completed as usize,
                        total: total as usize,
                    },
                );
            }
        }
        "wfl.analysis.completed" => {
            if let Some(run) = parse_wfl_run(&data) {
                if !event_matches_project(&run.project_path, kspd_path) {
                    return;
                }
                let status = run.status.unwrap_or_else(|| "completed".to_string());
                let success = !matches!(
                    status.to_lowercase().as_str(),
                    "failed" | "error" | "cancelled" | "canceled"
                );

                let _ = app.emit(
                    app_events::DAEMON_ANALYSIS_COMPLETED,
                    AnalysisCompletedEvent {
                        item_type: run.target_kind.unwrap_or_else(|| "unknown".to_string()),
                        item_id: run.target_id.unwrap_or_else(|| "all".to_string()),
                        success,
                        error: if success { None } else { Some(status) },
                    },
                );
            }
        }
        _ => {}
    }
}

#[derive(Default)]
struct ParsedWFLRun {
    op: Option<String>,
    status: Option<String>,
    completed: Option<i64>,
    total: Option<i64>,
    percent: Option<i64>,
    target_kind: Option<String>,
    target_id: Option<String>,
    project_path: Option<String>,
}

fn parse_wfl_run(data: &serde_json::Map<String, serde_json::Value>) -> Option<ParsedWFLRun> {
    let base = if let Some(run) = data.get("run").and_then(|v| v.as_object()) {
        run
    } else {
        data
    };

    let run_id = as_str(base.get("runId").or_else(|| data.get("runId")));
    if run_id.is_none() {
        return None;
    }

    let target_obj = base
        .get("target")
        .and_then(|v| v.as_object())
        .or_else(|| data.get("target").and_then(|v| v.as_object()));

    let project_path = as_str(base.get("projectPath"))
        .or_else(|| as_str(base.get("project")))
        .or_else(|| as_str(data.get("projectPath")))
        .or_else(|| as_str(data.get("project")));

    Some(ParsedWFLRun {
        op: as_str(base.get("op")).or_else(|| as_str(data.get("op"))),
        status: as_str(base.get("status")).or_else(|| as_str(data.get("status"))),
        completed: as_i64(base.get("completed")).or_else(|| as_i64(data.get("completed"))),
        total: as_i64(base.get("total")).or_else(|| as_i64(data.get("total"))),
        percent: as_i64(base.get("percent")).or_else(|| as_i64(data.get("percent"))),
        target_kind: target_obj.and_then(|t| as_str(t.get("kind"))),
        target_id: target_obj.and_then(|t| as_str(t.get("id"))),
        project_path,
    })
}

fn as_str(value: Option<&serde_json::Value>) -> Option<String> {
    value.and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn as_i64(value: Option<&serde_json::Value>) -> Option<i64> {
    value.and_then(|v| v.as_i64())
}

fn event_matches_project(event_project: &Option<String>, requested_path: &str) -> bool {
    let Some(event_project) = event_project.as_ref() else {
        return true;
    };

    let daemon_canonical =
        std::fs::canonicalize(event_project).unwrap_or_else(|_| event_project.clone().into());
    let requested_canonical =
        std::fs::canonicalize(requested_path).unwrap_or_else(|_| requested_path.into());

    daemon_canonical == requested_canonical
}

// ---------------------------------------------------------------------------
// Daemon lifecycle — spawn khaos-wfl for a project
// ---------------------------------------------------------------------------

pub async fn start_daemon(kspd_path: &str) -> Result<DaemonStatus, String> {
    let wfl_path = find_khaos_wfl()?;

    // Generate socket path
    let socket_path = daemon_socket_path(kspd_path);

    tracing::info!(
        "Starting khaos-wfl daemon: project={}, socket={}",
        kspd_path,
        socket_path
    );

    // Spawn daemon process
    let _child = Command::new(&wfl_path)
        .args([
            "daemon",
            "--project",
            kspd_path,
            "-ipc",
            "uds",
            "-ipc-addr",
            &socket_path,
            "-watch=true",
            "-run-on-start=false",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start khaos-wfl: {}", e))?;

    // Set env vars for wfl_client to find the socket
    std::env::set_var("KHAOS_WFL_SOCKET", &socket_path);
    std::env::set_var("KHAOS_WFL_NETWORK", "unix");

    // Wait for socket to appear (poll every 100ms, timeout after 5s)
    let socket = Path::new(&socket_path);
    for _ in 0..50 {
        if socket.exists() {
            tracing::info!("Daemon socket ready: {}", socket_path);
            return check_daemon_status(kspd_path).await.into_ok();
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    Err("Daemon started but socket did not appear within 5 seconds".to_string())
}

/// Generate a deterministic socket path for a project
fn daemon_socket_path(kspd_path: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    kspd_path.hash(&mut hasher);
    let hash = hasher.finish();
    let pid = std::process::id();
    format!("/tmp/khaos-webui-wfl-{}-{:x}.sock", pid, hash)
}

trait IntoOk {
    fn into_ok(self) -> Result<Self, String>
    where
        Self: Sized;
}

impl IntoOk for DaemonStatus {
    fn into_ok(self) -> Result<Self, String> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_analysis_index_nonexistent() {
        let index = scan_analysis_index("/nonexistent/path.kspd");
        assert!(index.scenes.is_empty());
        assert!(index.characters.is_empty());
        assert!(index.locations.is_empty());
    }

    #[test]
    fn test_scan_analysis_index_with_files() {
        let dir = std::env::temp_dir().join("khaos_dashboard_test_index");
        let analysis_dir = dir.join("metadata").join("analysis");
        let _ = std::fs::create_dir_all(&analysis_dir);

        std::fs::write(analysis_dir.join("scn_001.json"), "{}").unwrap();
        std::fs::write(analysis_dir.join("scn_002.json"), "{}").unwrap();
        std::fs::write(analysis_dir.join("chr_001.json"), "{}").unwrap();
        std::fs::write(analysis_dir.join("loc_001.json"), "{}").unwrap();

        let index = scan_analysis_index(dir.to_str().unwrap());
        assert_eq!(index.scenes.len(), 2);
        assert_eq!(index.characters.len(), 1);
        assert_eq!(index.locations.len(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_daemon_socket_path() {
        let path = daemon_socket_path("/Users/k/projects/test.kspd");
        assert!(path.starts_with("/tmp/khaos-webui-wfl-"));
        assert!(path.ends_with(".sock"));
    }

    #[test]
    fn test_find_khaos_tools_available() {
        // This test verifies the binary exists on this system
        let result = find_khaos_tools();
        assert!(result.is_ok(), "khaos-tools should be available: {:?}", result);
    }

    #[test]
    fn test_find_khaos_wfl_available() {
        let result = find_khaos_wfl();
        assert!(result.is_ok(), "khaos-wfl should be available: {:?}", result);
    }
}
