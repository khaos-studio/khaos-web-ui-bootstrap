// NDJSON Envelope Protocol
// Implements Daemon IPC spec v1.2 for request/response communication

use serde::{Deserialize, Serialize};

/// Canonical method names per daemon spec
pub mod methods {
    pub const PING: &str = "wfl.ping";
    pub const GET_STATUS: &str = "wfl.getStatus";
    pub const GET_CAPABILITIES: &str = "wfl.getCapabilities";
    pub const ANALYZE: &str = "wfl.analyze";
    pub const PARSER_PARSE: &str = "wfl.parser.parse";
    pub const PARSER_QUERY: &str = "wfl.parser.query";
    pub const GET_SIGNALS: &str = "wfl.getSignals";
}

/// Canonical topic names per daemon spec
pub mod topics {
    pub const ANALYSIS_PROGRESS: &str = "wfl.analysis.progress";
    pub const ANALYSIS_COMPLETED: &str = "wfl.analysis.completed";
    pub const PARSER_PROGRESS: &str = "wfl.parser.progress";
    pub const PARSER_COMPLETED: &str = "wfl.parser.completed";
}

/// NDJSON Request envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Request {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

/// NDJSON Response envelope
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Response {
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Event published by daemon on subscription topic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Event {
    pub topic: String,
    pub data: serde_json::Value,
}

/// Error translation: daemon errors -> user-facing messages
#[derive(Debug, Clone)]
pub enum DaemonError {
    Unreachable,
    InvalidMethod(String),
    InvalidParams,
    Internal(String),
    Timeout,
    Transport(String),
}

impl std::fmt::Display for DaemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(f, "Daemon unreachable"),
            Self::InvalidMethod(m) => write!(f, "Unknown method: {}", m),
            Self::InvalidParams => write!(f, "Invalid parameters"),
            Self::Internal(msg) => write!(f, "Daemon error: {}", msg),
            Self::Timeout => write!(f, "Request timeout"),
            Self::Transport(msg) => write!(f, "Transport error: {}", msg),
        }
    }
}

impl std::error::Error for DaemonError {}
