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
            // Projects commands
            commands::projects::discover_projects,
            commands::projects::search_projects,
            commands::projects::get_project,
            // Settings commands
            commands::settings::load_settings,
            commands::settings::save_settings,
            commands::settings::check_daemon_connection,
            commands::settings::validate_provider_config,
            // Dashboard commands
            commands::dashboard::get_scenes,
            commands::dashboard::analyze_scene,
            commands::dashboard::analyze_all,
            commands::dashboard::get_analysis_results,
            // Import commands
            commands::import::start_parse,
            commands::import::get_parse_progress,
            commands::import::cancel_parse,
            // System commands
            commands::system::get_system_info,
            commands::system::log_message,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {
                // Cleanup on app exit
            }
        });
}
