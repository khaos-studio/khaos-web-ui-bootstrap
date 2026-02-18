// Window Commands
// Opens secondary windows (settings, dashboard) with correct URLs per environment

use tauri::{command, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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

/// Open or focus the Dashboard window
#[command]
pub async fn open_dashboard_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("dashboard") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    #[cfg(debug_assertions)]
    let url = WebviewUrl::External("http://localhost:5175".parse().unwrap());
    #[cfg(not(debug_assertions))]
    let url = WebviewUrl::App("dashboard/index.html".into());

    WebviewWindowBuilder::new(&app, "dashboard", url)
        .title("Khaos Dashboard")
        .inner_size(1400.0, 900.0)
        .min_inner_size(1000.0, 700.0)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
