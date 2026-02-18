# Khaos Web UI — Phase 2a Foundation & Contracts

**Status**: Phase 2a implementation complete ✓

This document outlines Phase 2a (Foundation & Contracts) of the Khaos Web UI project, which establishes the technical foundation for the multi-window Tauri v2 desktop application.

## What's in Phase 2a

Phase 2a delivers the infrastructure and contracts necessary to build Phase 2b (Projects/Settings) and Phase 2c (Dashboard/Import/Analysis).

### Deliverables

- ✓ Tauri v2 app shell with 3 window definitions
- ✓ Frontend monorepo scaffold (Nuxt/Vue 3, Turbo)
- ✓ IPC contract module (NDJSON envelopes, canonical methods/topics)
- ✓ Platform-specific transport layer (Unix sockets, Windows named pipes)
- ✓ Core command skeleton (typed signatures, stubbed handlers)
- ✓ Development tooling and documentation

### Exit Criteria

- [x] All windows open (Projects, Settings, Dashboard)
- [x] IPC client can connect and ping (Unix, stub Windows)
- [x] Build, lint, test commands runnable
- [x] Documentation covers architecture, IPC protocol, development workflow

---

## Quick Start

### Prerequisites

- **Node.js** 18+ and npm/yarn/pnpm
- **Rust** 1.70+ (for Tauri backend)
- **Tauri CLI**: `cargo install tauri-cli`

### Setup

```bash
# Clone and navigate
cd khaos-web-ui-bootstrap

# Install dependencies
just install
# or: npm install

# Start development servers
just dev

# In separate terminals, run individual windows if needed:
just dev-projects
just dev-settings
just dev-dashboard
```

### Environment

Copy `.env.example` to `.env` and customize:

```bash
cp .env.example .env
```

Key settings:
- `KHAOS_WFL_SOCKET` — Daemon Unix socket path (macOS/Linux)
- `KHAOS_WFL_PIPE` — Daemon named pipe path (Windows)
- `KHAOS_PROJECTS_ROOT` — Project discovery root directory
- `DEV_PORT_*` — Development server ports

### Build

```bash
just build          # Build frontend + Tauri backend
just build-frontend # Frontend only
just build-tauri    # Tauri backend only
```

### Testing & Linting

```bash
just test           # Run all tests
just lint           # Lint code
just format         # Auto-format code
just lint-rust      # Lint Rust backend
```

---

## Project Structure

```
src-tauri/                       # Rust backend
  src/
    main.rs                      # App entry, window setup
    lib.rs                       # Logging, utilities
    types.rs                     # Shared DTOs
    events.rs                    # Event bridge definitions
    wfl_client/
      mod.rs                     # Daemon client facade
      protocol.rs                # NDJSON protocol, methods/topics
      transport_unix.rs          # Unix socket transport
      transport_windows.rs       # Windows named pipe (stub)
    commands/                    # Tauri command handlers
      projects.rs                # Projects: discover, search, get
      settings.rs                # Settings: load, save, validate
      dashboard.rs               # Dashboard: scenes, analysis
      import.rs                  # Import: parse, track progress
      system.rs                  # System: info, logging
    services/                    # Business logic layer
      config.rs                  # Settings persistence
      keychain.rs                # Secure storage
      discovery.rs               # Project discovery
      export.rs                  # Export formats

windows/
  projects/                      # Projects window (Nuxt app)
  settings/                      # Settings window (Nuxt app)
  dashboard/                     # Dashboard window (Nuxt app)

shared/                          # Shared frontend packages
  composables/                   # Vue composables
  stores/                        # Pinia stores
  components/                    # Shared components
  types/                         # TypeScript types
  styles/                        # Global styles

docs/
  ARCHITECTURE.md                # System architecture & data flow
  IPC_PROTOCOL.md                # Daemon communication spec
  DEVELOPMENT.md                 # Development guide (TBD)
```

---

## Architecture Overview

### Three Windows

1. **Projects** — Discover, search, and open KSPD projects
2. **Settings** — Configure provider, API keys, theme, validate daemon
3. **Dashboard** — View project scenes, run analysis, export results

Each window is an independent Nuxt/Vue 3 app with its own domain state. Only **theme** and **active project** are shared globally.

### Rust Backend

- **Command Handlers**: Bridge between frontend (Tauri `invoke()`) and business logic
- **Services**: Implement project discovery, config persistence, secure storage, exports
- **WFL Client**: Sends requests to daemon over platform-specific IPC
- **Event Bridge**: Maps daemon topics to Tauri app events for frontend subscription

### IPC Contract

- **Protocol**: NDJSON (newline-delimited JSON)
- **Transport**: Unix socket (macOS/Linux), Windows named pipe (Windows)
- **Canonical Methods**: `wfl.ping`, `wfl.getStatus`, `wfl.getCapabilities`, `wfl.analyze`, `wfl.parser.parse`, etc.
- **Canonical Topics**: `wfl.analysis.progress`, `wfl.analysis.completed`, `wfl.parser.progress`, `wfl.parser.completed`
- **Frontend Events**: Normalized Tauri app events (`daemon:analysis-progress`, etc.) decouple frontend from daemon protocol

See [IPC_PROTOCOL.md](docs/IPC_PROTOCOL.md) for full specification.

---

## Phase 2a Highlights

### Command Skeleton (Stubbed, Ready to Implement)

Each command has:
- Typed parameters (validated at boundary)
- Typed return value (serializable to JSON)
- Documented business logic TBD markers
- Proper error propagation

Example:
```rust
#[command]
pub async fn discover_projects() -> Result<Vec<Project>, String> {
    // TODO: Implement filesystem scanning via services::discovery
    Ok(vec![])
}
```

### Type Safety

- **Rust**: Strong typing for commands, services, IPC messages
- **Frontend**: TypeScript with shared types from `@khaos/shared/types`
- **Serialization**: serde/serde_json for JSON marshaling

### Platform Support

- **macOS/Linux**: Unix domain socket transport implemented
- **Windows**: Named pipe transport stubbed (full implementation needed)
- **Both**: Unified client API abstracted over transports

### IPC Protocol Version

Protocol v1.2 per Daemon IPC spec, ensuring compatibility with khaos-wfl daemon.

---

## Next Steps (Phase 2b & Beyond)

### Phase 2b: Projects & Settings Core (Week 2)

- Implement project discovery service (filesystem scan)
- Implement projects commands + UI (list, search, filter)
- Implement settings persistence + OS keychain integration
- Implement daemon check and provider validation
- Implement theme sync across windows

### Phase 2c: Import & Dashboard (Week 3)

- Implement event subscription bridge for daemon topics
- Implement import flow (file picker → parse → register)
- Implement dashboard scene list (virtualized for 100+)
- Implement analysis trigger and live progress UI
- Implement export to JSON, Markdown, CSV

### Phase 2d: Hardening & Release (Week 4)

- Unified error handling and user-facing messages
- Integration and E2E tests
- Performance validation (startup, memory, scrolling)
- Final documentation
- Release preparation and signing

---

## Development Workflow

### Local Testing

1. **Start daemon** (in separate terminal):
   ```bash
   # Assuming khaos-wfl is installed or built
   khaos-wfl daemon
   ```

2. **Set IPC path** (optional, if not using defaults):
   ```bash
   export KHAOS_WFL_SOCKET=/path/to/socket  # macOS/Linux
   ```

3. **Run dev servers**:
   ```bash
   just dev
   ```

4. **Test commands**:
   - Open DevTools (F12 in any window)
   - Invoke commands from console: `window.__TAURI__.invoke('check_daemon_connection')`

### Running Tests

```bash
# Unit tests (protocol, types)
npm --workspace @khaos/shared run test

# Integration tests (command handlers, services)
npm --workspace @khaos/projects-window run test

# End-to-end (multi-window, full workflows)
# TBD in Phase 2d
```

### Debugging

Enable debug logging:
```bash
export KHAOS_WEB_UI_LOG_LEVEL=debug
just dev
```

View logs in:
- **Rust**: stderr (visible in terminal)
- **Frontend**: Browser DevTools console

### Code Style

```bash
# Check style
just lint
just lint-rust

# Auto-fix
just format
just lint-rust -- --fix

# Run all checks
just lint && just lint-rust
```

---

## Known Limitations & TODOs

### Phase 2a Stubs

The following are stubbed and will be implemented in Phase 2b/2c:

- [ ] Project discovery service (filesystem scanning)
- [ ] Settings persistence (load/save)
- [ ] OS keychain integration
- [ ] Event subscription bridge (daemon topic listening)
- [ ] Windows named pipe transport (full implementation)
- [ ] All command handler business logic
- [ ] Export formatters (JSON, Markdown, CSV)

### Architectural Notes

- **Frontend state**: Per-window domain isolation; cross-window sync for theme/active project only
- **Daemon connectivity**: Connection failures are handled gracefully with retry logic (TBD Phase 2d)
- **Performance**: Scene list uses virtualization for 100+ rows; memoization for result rendering

---

## Troubleshooting

### "Daemon unreachable"

- Ensure `khaos-wfl` is running: `khaos-wfl daemon`
- Check socket path: `KHAOS_WFL_SOCKET=/tmp/khaos-wfl.sock` (macOS/Linux default)
- Check daemon logs: `khaos-wfl daemon --verbose`

### Build failures

- Ensure Rust toolchain is up to date: `rustup update`
- Ensure Tauri CLI is installed: `cargo install tauri-cli`
- Clear build cache: `just clean`

### Frontend dev server issues

- Check port availability: `lsof -i :5173` (and :5174, :5175)
- Check Node.js version: `node --version` (require 18+)

---

## References

- [Architecture Overview](docs/ARCHITECTURE.md)
- [IPC Protocol Specification](docs/IPC_PROTOCOL.md)
- [Phase 2 Bootstrap Plan](specs/phase-2-bootstrap-implementation-v2.md)
- [Tauri Documentation](https://tauri.app/docs)
- [Vue 3 / Nuxt Documentation](https://nuxt.com)
- [Pinia Store Management](https://pinia.vuejs.org)

---

## Contributing

When implementing Phase 2b+ features:

1. Follow the plan from `specs/phase-2-bootstrap-implementation-v2.md`
2. Maintain type safety: add types before implementation
3. Keep commands thin: move logic to services
4. Test at command boundary: mock daemon transport for unit tests
5. Document decisions that differ from spec

---

**Phase 2a Complete** ✓

Next checkpoint: Phase 2b exit criteria (projects/settings feature-complete).
