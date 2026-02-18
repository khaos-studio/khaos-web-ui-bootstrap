// Library utilities and initialization

use tracing_subscriber::fmt;

pub fn setup_logging() {
    let log_level = std::env::var("KHAOS_WEB_UI_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .init();
}
