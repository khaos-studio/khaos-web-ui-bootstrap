// Unix Domain Socket Transport
// Implements IPC transport for macOS and Linux

use crate::wfl_client::protocol::{Request, Response};
use crate::wfl_client::Transport;
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

pub struct UnixTransport {
    stream: UnixStream,
}

impl UnixTransport {
    pub async fn connect(socket_path: &str) -> Result<Self> {
        let stream = UnixStream::connect(socket_path)
            .context(format!("Failed to connect to {}", socket_path))?;

        stream
            .set_nonblocking(false)
            .context("Failed to configure stream")?;

        Ok(UnixTransport { stream })
    }
}

impl Transport for UnixTransport {
    fn send_request(&mut self, req: &Request) -> Result<Response> {
        // Serialize request to NDJSON
        let request_json = serde_json::to_string(req)?;
        self.stream.write_all(request_json.as_bytes())?;
        self.stream.write_all(b"\n")?;
        self.stream.flush()?;

        // Read response line
        let reader = BufReader::new(&self.stream);
        let mut lines = reader.lines();

        if let Some(Ok(line)) = lines.next() {
            let response: Response = serde_json::from_str(&line)?;
            Ok(response)
        } else {
            Err(anyhow::anyhow!("No response from daemon"))
        }
    }

    fn is_connected(&self) -> bool {
        // Check if stream is still valid
        self.stream.peer_addr().is_ok()
    }
}
