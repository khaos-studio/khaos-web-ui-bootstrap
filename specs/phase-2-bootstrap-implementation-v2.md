# Plan: khaos-ui Phase 2 Bootstrap (Revised v2)

## Status

Draft: Proposed replacement for `specs/phase-2-bootstrap-implementation.md`.

## Purpose

Deliver Phase 2 MVP of `khaos-ui` as a Tauri v2 desktop app with three Nuxt (Vue 3) windows:
- Projects
- Settings
- Dashboard

This revision resolves prior spec issues:
- Removes undefined task dependencies
- Defines a concrete cross-window theme synchronization model
- Normalizes daemon method/topic naming and event routing
- Specifies Windows named-pipe transport implementation (not implied Tauri abstraction)
- Unifies export formats
- Separates daemon reachability from provider credential validation

---

## Scope

### In Scope (Phase 2)

- Tauri v2 app shell with 3 windows
- Rust backend command layer and daemon client
- Daemon integration over local IPC:
  - Unix Domain Socket on macOS/Linux
  - Named Pipe on Windows
- Projects discovery and navigation
- Settings persistence + keychain storage
- Import flow via parser API + progress
- Dashboard scene list + analysis rendering
- Export: JSON, Markdown, CSV
- Theme system with cross-window sync
- Baseline tests and documentation

### Out of Scope (Phase 2)

- Collaborative editing
- Cloud sync
- Advanced graph visualizations
- Mobile
- PDF export (deferred to Phase 3)

---

## Architecture

### Runtime Components

1. Tauri host process (`src-tauri`)
2. Three Nuxt windows (`windows/projects`, `windows/settings`, `windows/dashboard`)
3. Shared frontend package (`shared/*`)
4. Local `khaos-wfl` daemon

### Data and Command Flow

1. Frontend invokes Tauri command (`invoke('...')`)
2. Rust command handler validates input, routes to service/client
3. `wfl_client` sends NDJSON request to daemon over local IPC transport
4. Response mapped to typed DTOs and returned to caller
5. Long-running daemon events are bridged to Tauri event bus
6. Windows subscribe through `useDaemonEvents()`

### State Model

- Domain state remains per-window (Pinia stores are local to each window)
- Global UX state uses coordinated sync for only two shared concerns:
  - Theme (`light|dark|system`)
  - Selected active project ID (optional convenience sync)

Cross-window sync mechanism:
- Source of truth persisted in config service
- Settings updates emit Tauri app event (`app:settings-changed`)
- Each window listens and updates local store
- On window startup, state hydrates from persisted config

This keeps domain isolation while enabling required UX consistency.

---

## IPC Contract (Normative for Phase 2)

### Transport

- macOS/Linux: UDS path from `KHAOS_WFL_SOCKET` or default platform path
- Windows: Named Pipe path from `KHAOS_WFL_PIPE` or default (`\\.\pipe\khaos-wfl`)

`wfl_client` must implement platform-specific connectors using Rust `cfg` gates:
- `tokio::net::UnixStream` for Unix
- `tokio` named-pipe client for Windows

### Envelope

All daemon traffic uses NDJSON envelopes per Daemon IPC spec v1.2.

### Canonical Method Names

- `wfl.ping`
- `wfl.getStatus`
- `wfl.getCapabilities`
- `wfl.analyze`
- `wfl.parser.parse`
- `wfl.parser.query`
- `wfl.getSignals`

### Canonical Topic Names

Use daemon topic names exactly as defined by daemon spec/capabilities. In app code and docs, do not mix prefixed and unprefixed variants. Default Phase 2 expectations:
- `wfl.analysis.progress`
- `wfl.analysis.completed`
- `wfl.parser.progress`
- `wfl.parser.completed`

If daemon capabilities return different names, capabilities-driven mapping is required at runtime.

### Tauri Event Names (Frontend-facing)

To prevent direct daemon-topic coupling in frontend components, Rust backend re-emits normalized app events:
- `daemon:analysis-progress`
- `daemon:analysis-completed`
- `daemon:parser-progress`
- `daemon:parser-completed`
- `daemon:status`
- `app:settings-changed`

---

## Functional Requirements

### Projects Window

- Discover KSPD projects from configured roots
- Show list metadata: title, author, scene count, modified time
- Search/filter by title/author/path
- Recent projects section
- Open selected project in Dashboard window

### Settings Window

- Provider selection (`mock`, `ollama`, `openai`, `anthropic`, others from capabilities)
- Model configuration
- API key secure storage in OS keychain
- Theme preference
- Two explicit validation actions:
  1. `Check Daemon` (reachability via `wfl.ping`)
  2. `Validate Provider` (credential/config preflight; see below)

Provider validation behavior:
- Step A: call `wfl.getCapabilities`
- Step B: if daemon advertises provider validation/preflight, invoke it
- Step C: otherwise run lowest-cost supported check and report limitation clearly
- UI must display which checks ran and their outcomes

### Dashboard Window

- Project overview summary
- Virtualized scene list (100+ scenes target)
- Scene detail and analysis panels
- Trigger scene/all analysis using `wfl.analyze`
- Live progress via bridged daemon events
- Export current scope/all scopes: JSON, Markdown, CSV

### Import Flow

- File picker for `.fountain`, `.fdx`, `.md`, `.sbx`
- Metadata entry (title, slug/path confirmation)
- Invoke `wfl.parser.parse`
- Show parser progress and completion
- Create/register project entry and refresh Projects list

---

## Non-Functional Requirements

- Startup target: < 2s on reference dev machine
- Scene list scrolling: smooth for 100+ rows
- Project discovery: < 500ms for 100 local projects on SSD (excluding cold filesystem cache variance)
- Analysis panel render after data arrival: < 100ms for typical payload
- Per-window memory budget target: < 150MB (guideline, not hard fail in debug)
- Graceful daemon disconnect and reconnect handling

---

## Repository Layout

```text
src-tauri/
  src/
    main.rs
    lib.rs
    wfl_client/
      mod.rs
      protocol.rs
      transport_unix.rs
      transport_windows.rs
    commands/
      mod.rs
      projects.rs
      settings.rs
      dashboard.rs
      import.rs
      system.rs
    services/
      config.rs
      keychain.rs
      discovery.rs
      export.rs
    events.rs
    types.rs
windows/
  projects/
  settings/
  dashboard/
shared/
  composables/
  stores/
  components/
  types/
  styles/
```

---

## Implementation Plan

## Phase 2a: Foundation and Contracts (Week 1)

### Task P2A-01: Scaffold Tauri + Multi-window
- Depends On: none
- Outputs:
  - Tauri v2 app builds
  - 3 window definitions
  - window entry routing wired

### Task P2A-02: Frontend Monorepo Scaffolding
- Depends On: P2A-01
- Outputs:
  - `windows/*` Nuxt apps booting
  - shared TS config and packages
  - workspace scripts

### Task P2A-03: IPC Contract Module
- Depends On: P2A-01
- Outputs:
  - NDJSON envelope types
  - request id generation
  - error translation model
  - documented canonical method/topic constants

### Task P2A-04: Platform Transport Layer
- Depends On: P2A-03
- Outputs:
  - Unix transport implementation
  - Windows named-pipe transport implementation
  - env var and default resolution logic

### Task P2A-05: Core Commands Skeleton
- Depends On: P2A-03, P2A-04
- Outputs:
  - command registry
  - stubbed handlers with typed signatures

### Task P2A-06: Tooling and Docs Bootstrap
- Depends On: P2A-02
- Outputs:
  - `justfile` for dev/build/test/lint
  - `.env.example`
  - architecture and IPC docs baseline

Milestone Exit Criteria (Phase 2a):
- All windows open
- IPC client can connect and ping on at least one platform
- Build/lint/test commands are runnable

## Phase 2b: Projects and Settings Core (Week 2)

### Task P2B-01: Project Discovery Service
- Depends On: P2A-05
- Outputs:
  - filesystem scan service
  - project DTO mapping
  - recent-project persistence

### Task P2B-02: Projects Commands + UI
- Depends On: P2B-01, P2A-02
- Outputs:
  - `discover_projects`, `get_project`, `open_project`
  - projects list/search/filter UI
  - dashboard handoff behavior

### Task P2B-03: Settings Persistence + Keychain
- Depends On: P2A-05
- Outputs:
  - load/save settings
  - secure API key storage via OS keychain
  - explicit non-secret config storage

### Task P2B-04: Daemon and Provider Validation Commands
- Depends On: P2A-05, P2B-03
- Outputs:
  - `check_daemon_connection` (`wfl.ping`)
  - `validate_provider_config` (capability-driven preflight)
  - structured check report DTO

### Task P2B-05: Settings UI + Theme Sync
- Depends On: P2B-03, P2B-04
- Outputs:
  - settings form
  - check-daemon + validate-provider actions
  - theme toggle persistence
  - cross-window sync using `app:settings-changed`

Milestone Exit Criteria (Phase 2b):
- Projects and Settings are feature-complete for MVP baseline
- Theme toggle in Settings updates all windows without restart
- Daemon check and provider validation are clearly differentiated in UI

## Phase 2c: Import and Dashboard Analysis (Week 3)

### Task P2C-01: Event Subscription Bridge
- Depends On: P2A-05
- Outputs:
  - runtime subscription manager
  - daemon topic to app event mapping table
  - reconnect and re-subscribe behavior

### Task P2C-02: Import Backend Commands
- Depends On: P2C-01
- Outputs:
  - `start_parse`
  - parse progress state tracking
  - parse completion handling + project registration

### Task P2C-03: Import UI Flow
- Depends On: P2C-02, P2A-02
- Outputs:
  - file picker + metadata form
  - progress and result states
  - projects list refresh integration

### Task P2C-04: Dashboard Scene and Analysis Commands
- Depends On: P2C-01
- Outputs:
  - scene list retrieval/query
  - analyze commands (scene/all)
  - analysis result retrieval and mapping

### Task P2C-05: Dashboard UI
- Depends On: P2C-04
- Outputs:
  - virtualized scene list
  - scene detail panel
  - live progress indicators
  - aggregated results panel

### Task P2C-06: Export Service and UI
- Depends On: P2C-05
- Outputs:
  - JSON/Markdown/CSV exporters
  - export menu and scope controls
  - file-save flow via Tauri dialog/fs APIs

Milestone Exit Criteria (Phase 2c):
- End-to-end: import -> project appears -> analyze -> progress -> results -> export

## Phase 2d: Hardening and Release Readiness (Week 4)

### Task P2D-01: Error Handling Framework
- Depends On: P2C-06
- Outputs:
  - normalized user-facing error categories
  - retry policy for transient daemon errors
  - logs with correlation IDs

### Task P2D-02: Integration and E2E Tests
- Depends On: P2C-06
- Outputs:
  - integration tests for command layer
  - E2E smoke tests across 3 windows
  - daemon-down and reconnect scenarios

### Task P2D-03: Performance Validation
- Depends On: P2D-02
- Outputs:
  - startup, memory, list scroll benchmarks
  - optimization fixes for failed thresholds

### Task P2D-04: Documentation Finalization
- Depends On: P2D-02
- Outputs:
  - final README + architecture + IPC + development docs
  - troubleshooting and platform notes

### Task P2D-05: Release Prep
- Depends On: P2D-03, P2D-04
- Outputs:
  - version bump
  - signed build workflow docs
  - release checklist and rollback notes

Milestone Exit Criteria (Phase 2d):
- Acceptance criteria pass
- Release candidate built on target platforms

---

## Dependency Graph Integrity

All task dependencies in this spec reference existing task IDs only.
No placeholder IDs (e.g., `finalize-all`) are permitted.

Validation rule to enforce during execution:
- Every `Depends On` token must match an existing task ID or `none`.

---

## Acceptance Criteria

Phase 2 is complete when all are true:

1. Architecture
- Tauri v2 app with Projects/Settings/Dashboard windows
- Rust daemon client works on Unix and Windows transports
- Frontend stores use per-window domain state
- Theme sync works cross-window via app event + persisted settings

2. Projects
- Discover/search/open works reliably
- 100+ projects handled without UI blocking

3. Settings
- Save/load config across restarts
- API keys stored in OS keychain
- `Check Daemon` and `Validate Provider` both implemented and distinguishable

4. Import
- Parse flow completes and progress is visible
- Newly imported projects become discoverable immediately

5. Dashboard
- Scene list virtualization functional
- Analysis requests and live updates functional
- Results render correctly for scene and aggregate scopes

6. Export
- JSON/Markdown/CSV export available and verified

7. Reliability
- Daemon disconnect handled with user-visible state and recovery path
- Error messages are actionable

8. Tooling & Docs
- `just dev|build|test|lint|format` available
- README and docs are current

9. Testing
- Integration tests for command layer
- E2E smoke path: settings -> projects -> import -> dashboard analyze -> export

---

## Risk Register and Mitigations

1. Protocol drift between app assumptions and daemon capabilities
- Mitigation: runtime capabilities handshake, constants module, compatibility tests

2. Windows transport regressions
- Mitigation: dedicated `transport_windows.rs` tests and CI lane

3. Theme sync race conditions across windows
- Mitigation: event includes full settings payload + startup hydration + idempotent reducer

4. Performance degradation with large projects
- Mitigation: virtualization, memoized selectors, bounded event buffers

5. Misleading connection status
- Mitigation: split daemon reachability and provider validation into separate checks and UI states

---

## Testing Strategy

### Unit
- Protocol parsing/serialization
- Settings merge logic
- Export format generation

### Integration
- Command handlers against mock daemon transport
- Event bridge mapping and re-subscription logic

### E2E
- Multi-window bootstrap and navigation
- Import progress flow
- Analysis progress flow
- Theme change propagation to all open windows

### Manual Verification Checklist

- Start daemon and launch app
- Check daemon status in Settings
- Validate provider config (positive and negative cases)
- Discover/open project
- Import screenplay and observe parser progress
- Run analysis and observe live progress
- Export in all 3 formats
- Toggle theme and verify all open windows update
- Restart app and verify persistence

---

## Environment Variables

- `KHAOS_WFL_SOCKET` (Unix)
- `KHAOS_WFL_PIPE` (Windows)
- `KHAOS_PROJECTS_ROOT` (optional override)
- `KHAOS_WEB_UI_LOG_LEVEL`
- `DEV_PORT_PROJECTS`
- `DEV_PORT_SETTINGS`
- `DEV_PORT_DASHBOARD`

---

## Commands (Target)

```bash
just dev
just build
just test
just lint
just format
```

---

## Change Log From Previous Spec

- Fixed invalid dependency: removed `finalize-all`
- Defined explicit cross-window sync for theme
- Normalized daemon method/topic naming policy
- Added explicit Windows named-pipe implementation requirement
- Unified exports to JSON/Markdown/CSV
- Split connection checks into daemon reachability vs provider validation
