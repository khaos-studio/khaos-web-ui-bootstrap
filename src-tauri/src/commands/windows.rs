// Window Commands
// Opens secondary windows (settings, dashboard) with correct URLs per environment

use crate::events::{app_events, ProjectSelectedEvent};
use tauri::{command, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};

fn encode_query_component(input: &str) -> String {
    // RFC 3986 unreserved set: ALPHA / DIGIT / "-" / "." / "_" / "~"
    input
        .bytes()
        .flat_map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![b as char]
            }
            _ => format!("%{:02X}", b).chars().collect(),
        })
        .collect()
}

/// Open or focus the Settings window
#[command]
pub async fn open_settings_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External("http://localhost:5174".parse().unwrap());
    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("settings/index.html".into());

    WebviewWindowBuilder::new(&app, "settings", url)
        .title("Khaos Settings")
        .inner_size(600.0, 700.0)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Open the Dashboard window with a selected project.
/// If the window already exists, focus it and emit the project-selected event.
/// If not, create it and emit after a short delay to let the frontend mount.
#[command]
pub async fn open_dashboard_window(
    app: AppHandle,
    project_title: String,
    project_path: String,
) -> Result<(), String> {
    let encoded_project_path = encode_query_component(&project_path);
    let encoded_project_title = encode_query_component(&project_title);
    let dashboard_query = format!(
        "project={}&title={}",
        encoded_project_path, encoded_project_title
    );

    let event_payload = ProjectSelectedEvent {
        project_title: project_title.clone(),
        project_path: project_path.clone(),
    };

    if let Some(window) = app.get_webview_window("dashboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        let window_title = format!("Khaos — {}", project_title);
        let _ = window.set_title(&window_title);
        // Window already mounted — emit immediately
        let _ = window.emit(app_events::APP_PROJECT_SELECTED, &event_payload);
        return Ok(());
    }

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External(
        format!("http://localhost:5175/?{}", dashboard_query)
            .parse()
            .unwrap(),
    );
    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App(format!("dashboard/index.html?{}", dashboard_query).into());

    let window_title = format!("Khaos — {}", project_title);

    let _window = WebviewWindowBuilder::new(&app, "dashboard", url)
        .title(&window_title)
        .inner_size(1400.0, 900.0)
        .min_inner_size(1000.0, 700.0)
        .build()
        .map_err(|e| e.to_string())?;

    // Give the new window time to mount its Vue app before emitting the event
    let app_clone = app.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        if let Some(dashboard_window) = app_clone.get_webview_window("dashboard") {
            let _ = dashboard_window.emit(app_events::APP_PROJECT_SELECTED, &event_payload);
        }
    });

    Ok(())
}
