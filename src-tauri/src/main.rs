// Tauri v2 Application Entry Point
// Initializes multi-window desktop app with Projects, Settings, and Dashboard windows

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod events;
mod services;
mod types;
mod wfl_client;

use khaos_web_ui::setup_logging;

fn main() {
    setup_logging();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::projects::discover_projects,
            commands::projects::search_projects,
            commands::projects::get_project,
            commands::settings::check_daemon_connection,
            commands::settings::validate_provider_config,
            commands::system::get_system_info,
            commands::system::log_message,
        ])
        .setup(|app| {
            // Create main windows on startup
            create_windows(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                // Cleanup
            }
        });
}

/// Initialize three windows: Projects, Settings, and Dashboard
fn create_windows(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Projects window
    tauri::WebviewWindowBuilder::new(app, "projects", tauri::WebviewUrl::App("/".into()))
        .title("Khaos Projects")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .build()?;

    // Settings window (hidden by default, shown on demand)
    tauri::WebviewWindowBuilder::new(app, "settings", tauri::WebviewUrl::App("/".into()))
        .title("Khaos Settings")
        .inner_size(600.0, 700.0)
        .visible(false)
        .build()?;

    // Dashboard window (hidden by default, shown on demand)
    tauri::WebviewWindowBuilder::new(app, "dashboard", tauri::WebviewUrl::App("/".into()))
        .title("Khaos Dashboard")
        .inner_size(1400.0, 900.0)
        .min_inner_size(1000.0, 700.0)
        .visible(false)
        .build()?;

    Ok(())
}
