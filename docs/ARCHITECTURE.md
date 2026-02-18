# Khaos Web UI Architecture

## Overview

Khaos Web UI is a Tauri v2 desktop application providing a graphical interface for screenwriting analysis. It consists of:

- **Tauri Host** (`src-tauri/`): Rust backend running the application shell
- **Three Frontend Windows**: Projects, Settings, Dashboard (Nuxt/Vue 3)
- **Shared Frontend Packages**: Common components, stores, utilities
- **Daemon Client**: NDJSON IPC communication with local `khaos-wfl` daemon

## Directory Structure

```
src-tauri/
  src/
    main.rs                    # Tauri app entry point, window setup
    lib.rs                     # Logging and utilities
    types.rs                   # Shared type definitions
    events.rs                  # Event bridge definitions
    wfl_client/
      mod.rs                   # Daemon client facade
      protocol.rs              # NDJSON protocol definitions
      transport_unix.rs        # Unix socket transport
      transport_windows.rs     # Windows named pipe transport
    commands/
      mod.rs                   # Command handler registry
      projects.rs              # Projects window commands
      settings.rs              # Settings window commands
      dashboard.rs             # Dashboard window commands
      import.rs                # Import/parse commands
      system.rs                # System info commands
    services/
      mod.rs                   # Service module exports
      config.rs                # Settings persistence
      keychain.rs              # Secure credential storage
      discovery.rs             # Project discovery
      export.rs                # Export formatting
  tauri.conf.json              # Tauri app configuration
  Cargo.toml                   # Rust dependencies

windows/
  projects/                    # Projects window (Nuxt app)
    app.vue                    # Root component
    nuxt.config.ts
    package.json
  settings/                    # Settings window (Nuxt app)
    app.vue
    nuxt.config.ts
    package.json
  dashboard/                   # Dashboard window (Nuxt app)
    app.vue
    nuxt.config.ts
    package.json

shared/
  composables/                 # Shared Vue composables
  stores/                      # Pinia stores (per-window domain + global UX)
  components/                  # Shared Vue components
  types/                       # Shared TypeScript types
  styles/                      # Shared global styles
```

## Data Flow

1. **Frontend → Backend**
   - Frontend window calls Tauri command: `invoke('discover_projects')`
   - Rust command handler receives and validates input
   - Service/client layer executes business logic
   - Response marshaled to JSON and returned

2. **Backend → Daemon**
   - Rust handler creates `wfl_client::Request` (NDJSON)
   - Platform-specific transport sends over IPC (Unix socket or Windows named pipe)
   - Daemon responds with `wfl_client::Response` (NDJSON)
   - Rust handler translates daemon response to frontend types

3. **Long-running Operations (Analysis, Parse)**
   - Frontend initiates: `invoke('analyze_scene', { projectId, sceneId })`
   - Backend sends `wfl.analyze` to daemon, stores request ID
   - Daemon emits progress events on subscription topic
   - Backend listens and re-emits to frontend via Tauri app event
   - Frontend receives via `onTauriEvent('daemon:analysis-progress')`

## State Model

### Per-Window Domain State
- Each window maintains its own Pinia stores for domain data
- Example (Projects window): project list, search filters, selection
- Stores are isolated and don't sync across windows

### Cross-Window Shared State
Only two global UX concerns are synchronized:

1. **Theme** (`light|dark|system`)
2. **Active Project ID** (optional convenience)

**Synchronization Mechanism:**
- Source of truth: persisted in config service (`services/config.rs`)
- Change flow: settings update → save to config → emit `app:settings-changed` event
- Each window listens and hydrates local store
- On window startup, Pinia store initializes from persisted config

## IPC Contract (Daemon Communication)

### Transport
- **macOS/Linux**: Unix domain socket (env: `KHAOS_WFL_SOCKET`, default: `/tmp/khaos-wfl.sock`)
- **Windows**: Named pipe (env: `KHAOS_WFL_PIPE`, default: `\\.\pipe\khaos-wfl`)

### Protocol: NDJSON Envelopes
All messages are newline-delimited JSON per Daemon IPC spec v1.2.

**Request:**
```json
{"id": "uuid", "method": "wfl.ping", "params": {}}
```

**Response:**
```json
{"id": "uuid", "result": {}} or {"id": "uuid", "error": "message"}
```

### Canonical Methods (Phase 2)
- `wfl.ping` — Test connectivity
- `wfl.getStatus` — Daemon status and health
- `wfl.getCapabilities` — Available providers and methods
- `wfl.analyze` — Trigger scene/project analysis
- `wfl.parser.parse` — Start screenplay parsing
- `wfl.parser.query` — Query parsing results
- `wfl.getSignals` — Get analysis results

### Canonical Topics (Phase 2)
- `wfl.analysis.progress` — Analysis progress updates
- `wfl.analysis.completed` — Analysis completion
- `wfl.parser.progress` — Parser progress updates
- `wfl.parser.completed` — Parser completion

### Frontend Event Names (Tauri App Events)
Backend re-emits normalized event names to decouple frontend from daemon protocol:
- `daemon:analysis-progress`
- `daemon:analysis-completed`
- `daemon:parser-progress`
- `daemon:parser-completed`
- `daemon:status`
- `app:settings-changed`

## Error Handling

**Daemon Errors**
- If daemon unreachable: Operations fail with user-visible message
- If daemon returns error: Translated to `DaemonError` enum, surfaced as command error
- Retry logic: Transient errors (timeout, connection reset) with exponential backoff

**Frontend Errors**
- Command invocation failures passed to caller as `Err(string)`
- UI should display user-friendly error messages with recovery suggestions

## Performance Targets (Phase 2)

- **Startup**: < 2s on reference dev machine
- **Scene list scrolling**: Smooth for 100+ rows (virtualization required)
- **Project discovery**: < 500ms for 100 projects on SSD
- **Analysis render**: < 100ms after data arrival
- **Per-window memory**: < 150MB (guideline, not hard fail in debug)

## Development Workflow

### Build & Run
```bash
just install       # Install all dependencies
just dev          # Start all three windows in dev mode
just build        # Build for production
just test         # Run tests
just lint         # Lint code
just format       # Format code
```

### Environment
Copy `.env.example` to `.env` and configure:
- `KHAOS_WFL_SOCKET` / `KHAOS_WFL_PIPE`: Daemon IPC paths
- `KHAOS_PROJECTS_ROOT`: Project discovery root
- `KHAOS_WEB_UI_LOG_LEVEL`: Logging verbosity
- `DEV_PORT_*`: Development server ports

### Testing Strategy
- **Unit**: Protocol, settings merge, export format generation
- **Integration**: Command handlers against mock daemon
- **E2E**: Multi-window bootstrap, import flow, analysis flow, theme sync
