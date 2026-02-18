# IPC Protocol Specification

## Overview

Khaos Web UI communicates with the local `khaos-wfl` daemon via **NDJSON** (Newline-Delimited JSON) envelopes over platform-specific IPC transport.

Protocol Version: **1.2** (per Daemon IPC spec)

## Transport

### macOS / Linux: Unix Domain Socket

**Environment Variable**: `KHAOS_WFL_SOCKET`
**Default Path**: `/tmp/khaos-wfl.sock`
**Example**:
```bash
export KHAOS_WFL_SOCKET=/var/run/khaos/wfl.sock
```

**Connection**:
```rust
let stream = UnixStream::connect(&socket_path)?;
```

### Windows: Named Pipe

**Environment Variable**: `KHAOS_WFL_PIPE`
**Default Path**: `\\.\pipe\khaos-wfl`
**Example**:
```bash
set KHAOS_WFL_PIPE=\\.\pipe\custom-wfl
```

**Connection**: Uses Windows named pipe API via `tokio-windows-util` or `winapi` crate.

---

## Message Format

All messages are **NDJSON**: one JSON object per line, terminated with `\n`.

### Request

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "method": "wfl.ping",
  "params": {}
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string (UUID v4) | Yes | Request ID for correlation |
| `method` | string | Yes | Canonical method name |
| `params` | object | Yes | Method parameters (can be empty `{}`) |

### Response

**Success:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "result": {"version": "0.1.0"}
}
```

**Error:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "error": "Invalid method: wfl.unknown"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | No | Echo of request ID (correlate response) |
| `result` | object | Conditional | Method result (present if success) |
| `error` | string | Conditional | Error message (present if error) |

**Rule**: Response contains either `result` or `error`, never both.

---

## Canonical Methods (Phase 2)

### wfl.ping

Test daemon connectivity.

**Request:**
```json
{"id": "...", "method": "wfl.ping", "params": {}}
```

**Response:**
```json
{"id": "...", "result": {}}
```

**Use Case**: Check daemon is running and responding.

---

### wfl.getStatus

Get daemon status and health.

**Request:**
```json
{"id": "...", "method": "wfl.getStatus", "params": {}}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "ready": true,
    "uptime_secs": 3600,
    "active_requests": 2
  }
}
```

---

### wfl.getCapabilities

Get available providers, methods, and topics.

**Request:**
```json
{"id": "...", "method": "wfl.getCapabilities", "params": {}}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "providers": ["mock", "ollama", "openai", "anthropic"],
    "methods": ["wfl.ping", "wfl.analyze", "wfl.parser.parse", ...],
    "topics": ["wfl.analysis.progress", "wfl.analysis.completed", ...]
  }
}
```

**Use Case**: Enumerate available providers for Settings window; validate daemon capabilities.

---

### wfl.analyze

Trigger analysis of scene(s).

**Request (Single Scene):**
```json
{
  "id": "...",
  "method": "wfl.analyze",
  "params": {
    "project_id": "proj-123",
    "scene_id": "scene-456"
  }
}
```

**Request (All Scenes):**
```json
{
  "id": "...",
  "method": "wfl.analyze",
  "params": {
    "project_id": "proj-123"
  }
}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "request_id": "analysis-789",
    "scene_count": 1
  }
}
```

**Behavior**:
- Returns immediately with request ID
- Daemon publishes progress on `wfl.analysis.progress` topic
- Daemon publishes completion on `wfl.analysis.completed` topic

---

### wfl.parser.parse

Start screenplay file parsing.

**Request:**
```json
{
  "id": "...",
  "method": "wfl.parser.parse",
  "params": {
    "file_path": "/path/to/screenplay.fountain",
    "title": "My Screenplay",
    "slug": "my-screenplay"
  }
}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "request_id": "parse-123",
    "format": "fountain"
  }
}
```

**Behavior**:
- Returns immediately with request ID
- Daemon publishes progress on `wfl.parser.progress` topic
- Daemon publishes completion on `wfl.parser.completed` topic

---

### wfl.parser.query

Query parsing results (after completion).

**Request:**
```json
{
  "id": "...",
  "method": "wfl.parser.query",
  "params": {
    "request_id": "parse-123"
  }
}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "status": "completed",
    "scene_count": 5,
    "project_id": "proj-123"
  }
}
```

---

### wfl.getSignals

Get analysis results for project/scene.

**Request:**
```json
{
  "id": "...",
  "method": "wfl.getSignals",
  "params": {
    "project_id": "proj-123",
    "scene_id": "scene-456"
  }
}
```

**Response:**
```json
{
  "id": "...",
  "result": {
    "scenes": [
      {
        "id": "scene-456",
        "signals": [...analysis results...]
      }
    ]
  }
}
```

---

## Canonical Topics (Phase 2)

Topics are used for long-running operation updates. Frontend must subscribe to receive events.

### wfl.analysis.progress

Emitted periodically during analysis.

**Payload:**
```json
{
  "request_id": "analysis-789",
  "progress": 0.35,
  "current": 3,
  "total": 9,
  "timestamp": 1708123456
}
```

### wfl.analysis.completed

Emitted when analysis finishes.

**Payload:**
```json
{
  "request_id": "analysis-789",
  "success": true,
  "project_id": "proj-123",
  "timestamp": 1708123460
}
```

**Or on error:**
```json
{
  "request_id": "analysis-789",
  "success": false,
  "error": "Provider unreachable",
  "timestamp": 1708123461
}
```

### wfl.parser.progress

Emitted during parsing.

**Payload:**
```json
{
  "request_id": "parse-123",
  "phase": "lexing",
  "progress": 0.5,
  "timestamp": 1708123456
}
```

### wfl.parser.completed

Emitted when parsing finishes.

**Payload:**
```json
{
  "request_id": "parse-123",
  "success": true,
  "project_id": "proj-456",
  "scene_count": 5,
  "timestamp": 1708123470
}
```

---

## Tauri App Events (Frontend-Facing)

Backend re-emits daemon topics as normalized Tauri app events for frontend subscription.
This decouples frontend components from daemon protocol details.

### daemon:analysis-progress
**Emitted when**: Daemon publishes `wfl.analysis.progress`
**Frontend receives**: Via `onTauriEvent('daemon:analysis-progress', ...)`

### daemon:analysis-completed
**Emitted when**: Daemon publishes `wfl.analysis.completed`

### daemon:parser-progress
**Emitted when**: Daemon publishes `wfl.parser.progress`

### daemon:parser-completed
**Emitted when**: Daemon publishes `wfl.parser.completed`

### daemon:status
**Emitted when**: Daemon status changes (optional)

### app:settings-changed
**Emitted when**: Settings are modified and saved
**Payload**: Full settings object including theme, active project, etc.

---

## Error Handling

### Daemon Errors (returned in response)

```json
{
  "id": "...",
  "error": "Unknown method: wfl.invalid"
}
```

**Common errors**:
- `"Unknown method: {method}"` — Method not implemented
- `"Invalid params: {reason}"` — Parameter validation failed
- `"Daemon error: {details}"` — Internal daemon error
- `"Timeout: request did not complete in time"` — Request timed out

### Transport Errors (connection layer)

- Socket/pipe connection refused → "Daemon unreachable"
- Socket/pipe closed unexpectedly → Reconnect with backoff
- NDJSON parse error → Log and skip malformed line

### Retry Strategy (in client)

For transient errors (timeout, connection reset):
1. Wait with exponential backoff (50ms, 100ms, 200ms, 500ms, 1s max)
2. Retry up to 5 times
3. If all retries exhausted, return user-facing error

---

## Example: End-to-End Request

**Frontend (Vue):**
```typescript
const result = await invoke('analyze_scene', {
  projectId: 'proj-123',
  sceneId: 'scene-456'
})
// Returns: { requestId: 'analysis-789' }

// Listen for progress
onTauriEvent('daemon:analysis-progress', (event) => {
  console.log(`Progress: ${event.payload.progress}%`)
})
```

**Backend (Rust):**
1. Command handler receives `analyze_scene` with projectId, sceneId
2. Creates NDJSON request: `{"id": "...", "method": "wfl.analyze", "params": {"project_id": ..., "scene_id": ...}}`
3. Sends via `wfl_client` (Unix socket or Windows pipe)
4. Receives response with `request_id`
5. Returns request_id to frontend
6. Subscribes to daemon's `wfl.analysis.progress` topic
7. For each progress event, re-emits as Tauri app event: `daemon:analysis-progress`
8. Frontend receives and updates UI

---

## Testing

### Mock Transport
For unit/integration tests, provide mock transport that:
- Accepts NDJSON requests
- Returns canned responses
- Allows assertion on request bodies

### Integration Tests
Run against actual daemon (or test fixture):
- Verify request/response round-trips
- Test error paths
- Test timeout and reconnection logic

### E2E Tests
Run full app against real daemon:
- Import → Parse → Analyze → Export flow
- Multi-window event propagation
- Settings changes and persistence
