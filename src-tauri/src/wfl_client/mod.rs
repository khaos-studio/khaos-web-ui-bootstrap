// Daemon IPC Client
// Handles NDJSON communication over platform-specific transports

pub mod protocol;
#[cfg(unix)]
pub mod transport_unix;
#[cfg(windows)]
pub mod transport_windows;

use anyhow::{Context, Result};
use protocol::{Request, Response};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Daemon client facade
pub struct DaemonClient {
    transport: Arc<Mutex<DynamicTransport>>,
}

pub trait Transport: Send {
    fn send_request(&mut self, req: &Request) -> Result<Response>;
    fn is_connected(&self) -> bool;
}

/// Platform-independent transport abstraction
enum DynamicTransport {
    #[cfg(unix)]
    Unix(Box<dyn Transport + Send>),
    #[cfg(windows)]
    Windows(Box<dyn Transport + Send>),
}

impl Transport for DynamicTransport {
    fn send_request(&mut self, req: &Request) -> Result<Response> {
        match self {
            #[cfg(unix)]
            DynamicTransport::Unix(t) => t.send_request(req),
            #[cfg(windows)]
            DynamicTransport::Windows(t) => t.send_request(req),
        }
    }

    fn is_connected(&self) -> bool {
        match self {
            #[cfg(unix)]
            DynamicTransport::Unix(t) => t.is_connected(),
            #[cfg(windows)]
            DynamicTransport::Windows(t) => t.is_connected(),
        }
    }
}

impl DaemonClient {
    /// Connect to daemon via platform-specific transport
    pub async fn connect() -> Result<Self> {
        let transport = connect_transport().await?;
        Ok(DaemonClient {
            transport: Arc::new(Mutex::new(transport)),
        })
    }

    /// Send request and wait for response
    pub async fn request(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        let req = Request {
            id: Uuid::new_v4().to_string(),
            method: method.to_string(),
            params,
        };

        let mut transport = self.transport.lock().await;
        let response = transport.send_request(&req)?;

        if let Some(error) = response.error {
            Err(anyhow::anyhow!("Daemon error: {}", error))
        } else {
            Ok(response.result.unwrap_or(serde_json::json!({})))
        }
    }

    /// Test connectivity
    pub async fn ping(&self) -> Result<bool> {
        match self.request("wfl.ping", serde_json::json!({})).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(unix)]
async fn connect_transport() -> Result<DynamicTransport> {
    let socket_path = std::env::var("KHAOS_WFL_SOCKET")
        .unwrap_or_else(|_| format!("/tmp/khaos-wfl.sock"));

    let transport = transport_unix::UnixTransport::connect(&socket_path)
        .await
        .context("Failed to connect to Unix domain socket")?;

    Ok(DynamicTransport::Unix(Box::new(transport)))
}

#[cfg(windows)]
async fn connect_transport() -> Result<DynamicTransport> {
    let pipe_path = std::env::var("KHAOS_WFL_PIPE")
        .unwrap_or_else(|_| "\\\\.\\pipe\\khaos-wfl".to_string());

    let transport = transport_windows::WindowsTransport::connect(&pipe_path)
        .await
        .context("Failed to connect to Windows named pipe")?;

    Ok(DynamicTransport::Windows(Box::new(transport)))
}
