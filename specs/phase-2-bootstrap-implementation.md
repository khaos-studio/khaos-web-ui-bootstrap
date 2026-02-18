# Plan: khaos-web-ui Phase 2 Bootstrap

## Task Description

Build khaos-web-ui Phase 2 MVP: A Tauri v2 native desktop application with three independent Nuxt (Vue 3) window applications — **Projects**, **Settings**, and **Dashboard** — that communicate with the khaos-wfl daemon via Unix Domain Socket (UDS) IPC. The application enables storytellers to discover KSPD projects, configure LLM providers, analyze screenplays, and manage narrative feedback signals.

The Rust backend will implement a Tauri IPC bridge that routes all daemon communication through the khaos-wfl daemon (not directly to khaos-tools). The frontend will use Shadcn/ui Vue components, TailwindCSS, and Pinia for per-window state management.

## Objective

Complete Phase 2a-2d deliverables:

1. **Tauri v2 + Nuxt scaffold** with multi-window support, shared utilities, and dev/build tooling
2. **Projects Window** — KSPD project discovery, browsing, search/filter, recent projects, create flow
3. **Settings Window** — LLM provider configuration, API key management (OS keychain), connection testing
4. **Dashboard Window** — Project overview, virtualized scene list, analysis results display, export
5. **Import Flow** — File picker + metadata entry → wfl.parser.parse → project creation
6. **Tauri Rust IPC Bridge** — Connect to khaos-wfl daemon via UDS, implement all necessary Tauri commands
7. **Justfile** — Complete rewrite for Tauri/Nuxt development workflow (dev, build, test, lint)

By end of Phase 2d, the application should load in < 2 seconds, handle 100+ projects smoothly (virtualized), persist settings across restarts, and route all screenplay analysis through the khaos-wfl daemon with live progress updates.

## Problem Statement

The existing khaos-tui (TUI) is powerful but limited to terminal environments. Teams need a native desktop UI that:
- Works on macOS/Windows/Linux without terminal knowledge
- Provides rich visual feedback (scene lists, metadata, analysis results)
- Integrates with the khaos-wfl daemon for queued analysis and real-time progress
- Offers modern UX patterns (multi-window, dark mode, responsive design)

The web-ui must integrate with the same khaos-wfl daemon that powers other tools, ensuring consistency in project structure, analysis results, and feedback signals across the Khaos ecosystem.

## Solution Approach

**Three-Window Architecture:**
Each window (Projects, Settings, Dashboard) is a self-contained Nuxt application bundled by Tauri. This provides:
- **Isolation:** Window crashes don't affect others; each has independent memory/state
- **Simplicity:** Reuse Nuxt conventions; no complex cross-window IPC plumbing
- **Extensibility:** Easy to add new windows (Reports, Signals, Timeline) later

**Tauri Rust Backend:**
The Rust backend (`src-tauri/`) acts as the daemon client and command executor:
- **UDS Connection:** Connects to khaos-wfl daemon via Unix Domain Socket (native on macOS/Linux; Windows uses named pipes via Tauri abstraction)
- **NDJSON Protocol:** Implements the wfl daemon IPC spec (v1.2) — JSON messages, ULID-based request IDs, type-safe Rust structs
- **Tauri Commands:** Exposes IPC methods as Tauri commands (e.g., `invoke('discover_projects')` from frontend)
- **Event Bridging:** Subscribes to wfl daemon topics (analysis.progress, analysis.completed, parser.completed, etc.) and broadcasts to active windows via Tauri event bus

**Frontend State & Communication:**
- **Pinia Stores:** Settings, Projects, Analysis results — per-window, no cross-window sync (each window controls its own state)
- **Composables:** `useIpc()` wraps Tauri invoke calls with error handling; `useSettings()`, `useProjects()`, `useAnalysis()` manage store access
- **Shadcn/ui Vue:** Copy-paste component library; configure TailwindCSS for theming
- **Virtualization:** TanStack Vue Virtual for scene lists (100+ items @ 60fps)

**Import Flow:**
1. User clicks "Create Project" → Import window appears
2. File picker → select `.fountain`, `.fdx`, `.md`, etc.
3. Enter project title + confirm
4. Call `wfl.parser.parse` (async) → daemon queues parse operation
5. Stream progress via `wfl.analysis.progress` events
6. On completion, create KSPD project directory + manifest
7. Return to Projects, refresh list

**Daemon Integration:**
All screenplay analysis, parsing, and signal management flows through khaos-wfl daemon:
- Projects window discovers KSPD projects by scanning filesystem (local operation, no daemon needed)
- Dashboard window queries project data via `wfl.parser.query` (sync)
- Settings window tests provider connection via `wfl.ping` (sync handshake)
- Analysis requests go via `wfl.analyze` (async with progress events)
- Import operations use `wfl.parser.parse` (async)

---

## Relevant Files

### Existing Reference Implementations

**khaos-tui** (`~/Spikes/khaos-tui/`)
- `code/cmd/khaos-tui/main.go` — Go entrypoint; sets up Bubble Tea TUI
- `code/internal/domain/models.go` — Domain models (Project, Config, ImportState)
- `code/internal/config/config.go` — XDG config loading/saving
- `code/internal/projects/discovery.go` — Async project discovery logic
- `code/internal/cli/adapter.go` — khaos-tools CLI wrapper (reference for daemon client pattern)
- `code/internal/views/` — TUI views (welcome, import, project) as reference for window structure
- `docs/architecture/ARCHITECTURE.md` — Layered architecture; use as pattern

**khaos-wfl** (`~/Projects/khaos-wfl/`)
- `Daemon-IPC-spec.md` — Frozen v1.2 IPC protocol (normative); all daemon communication must follow this
- `README.md` — Daemon startup, env vars, troubleshooting
- Examples: `wfl.ping`, `wfl.getStatus`, `wfl.getCapabilities`, `wfl.analyze`, `wfl.parser.parse`, `wfl.graph.query`, `wfl.getSignals`

**khaos-tools** (`~/Spikes/khaos-tools/`)
- `code/` — Screenplay analysis library; used by wfl daemon
- Not directly called by web-ui (all calls go through wfl daemon)

### New Files to Create

**Tauri Backend** (`src-tauri/`)
```
src-tauri/
├── Cargo.toml                      # Tauri + wfl client deps
├── tauri.conf.json                 # Multi-window setup
├── src/
│   ├── main.rs                     # Tauri app entry, window creation
│   ├── lib.rs                      # Module exports
│   ├── wfl_client.rs               # khaos-wfl daemon NDJSON client (UDS connector)
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── projects.rs             # discover_projects, get_project, open_project
│   │   ├── settings.rs             # load_settings, save_settings, test_connection
│   │   ├── analysis.rs             # analyze_scene, get_analysis, get_scene_list
│   │   ├── import.rs               # start_parse, get_parse_progress
│   │   └── signals.rs              # get_signals, acknowledge_signal, resolve_signal
│   ├── services/
│   │   ├── mod.rs
│   │   ├── config.rs               # XDG config paths, load/save app settings
│   │   ├── project_discovery.rs    # Filesystem scanning (async)
│   │   └── keychain.rs             # OS keychain integration (optional, Tauri provides)
│   └── types.rs                    # Shared types, serialization
```

**Frontend Shared** (`shared/`)
```
shared/
├── components/                     # Shadcn/ui Vue copies
├── composables/
│   ├── useIpc.ts                   # Tauri invoke wrapper
│   ├── useSettings.ts              # Settings store access
│   ├── useProjects.ts              # Projects store access
│   ├── useAnalysis.ts              # Analysis store access
│   └── useDaemonEvents.ts          # Listen to wfl daemon events
├── stores/
│   ├── settings.ts                 # Pinia: provider config, theme
│   ├── projects.ts                 # Pinia: project list, recent, selected
│   ├── analysis.ts                 # Pinia: analysis results, loading state
│   └── import.ts                   # Pinia: import flow state
├── types/
│   └── index.ts                    # TypeScript interfaces (ProjectMetadata, Scene, etc.)
└── styles/
    ├── globals.css
    ├── tokens.css                  # Design tokens (colors, spacing, fonts)
    └── tailwind.config.ts
```

**Window Apps** (`windows/`)
```
windows/
├── projects/
│   ├── app.vue
│   ├── nuxt.config.ts
│   ├── package.json
│   ├── pages/
│   │   └── index.vue               # Main project list view
│   └── components/
│       ├── ProjectList.vue         # Virtualized list
│       ├── ProjectSearchBar.vue
│       └── ProjectCard.vue
├── settings/
│   ├── app.vue
│   ├── nuxt.config.ts
│   ├── package.json
│   ├── pages/
│   │   └── index.vue               # Settings form
│   └── components/
│       ├── ProviderSelect.vue
│       ├── ModelInput.vue
│       ├── ApiKeyInput.vue
│       └── ConnectionTest.vue
└── dashboard/
    ├── app.vue
    ├── nuxt.config.ts
    ├── package.json
    ├── pages/
    │   ├── index.vue               # Project overview + scene list
    │   └── scene-detail.vue        # Scene detail modal/panel
    └── components/
        ├── SceneList.vue           # Virtualized scene list
        ├── SceneDetail.vue
        ├── AnalysisResults.vue
        └── ExportMenu.vue
```

**Root Config & Build** (root level)
```
├── package.json                    # Root package (workspaces or lerna)
├── tsconfig.json                   # Shared TS config
├── tailwind.config.ts              # Shared Tailwind config
├── justfile                        # Complete rewrite for Tauri/Nuxt dev
├── .env.example                    # KHAOS_WFL_SOCKET, KHAOS_TOOLS_PATH, etc.
└── Makefile                        # Optional backup to justfile
```

**Documentation**
```
├── docs/
│   ├── ARCHITECTURE.md             # System overview, layer descriptions
│   ├── WINDOWS.md                  # Per-window feature spec
│   ├── IPC.md                      # Tauri commands reference
│   ├── STORES.md                   # Pinia store API
│   ├── COMPONENTS.md               # Shadcn/ui setup guide
│   └── DEVELOPMENT.md              # Dev setup, local daemon, testing
└── README.md                       # Quick start, status
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1-2)

**Goals:**
- Tauri v2 project template scaffold
- Multi-window setup (Projects, Settings, Dashboard)
- Rust backend structure + wfl_client NDJSON implementation
- Basic project discovery (filesystem scan, no daemon yet)
- Local dev setup (justfile, package.json workspaces)

**Deliverables:**
- `src-tauri/` with `wfl_client.rs` + command stubs
- `windows/{projects,settings,dashboard}/` with Nuxt scaffolds
- Root `package.json`, `justfile`, `.env.example`
- Initial `docs/ARCHITECTURE.md`

### Phase 2: Core IPC & Projects Window (Week 2)

**Goals:**
- Tauri commands fully wired (invoke/listen pattern)
- wfl_client connects to daemon via UDS (Unix socket path from env)
- `discover_projects` command returns project list
- Projects window displays virtualized project list, search/filter
- Settings window provider select + API key input (no save yet)
- Dashboard window skeleton (can open, displays project title)

**Deliverables:**
- `discover_projects`, `get_project`, `load_settings`, `save_settings` commands working
- Projects window MVP (list, search, click to open)
- Settings window MVP (provider form, no persistence)
- Dashboard window skeleton
- Pinia stores for projects, settings, analysis

### Phase 3: Settings & Import (Week 3)

**Goals:**
- Settings persistence (XDG config on macOS/Linux)
- OS keychain for API key storage (via Tauri)
- `test_connection` command (wfl.ping handshake)
- Import flow (file picker → parse via wfl.parser.parse)
- Parsing progress events (wfl.parser.progress → Tauri event bus → dashboard)

**Deliverables:**
- Settings window fully functional (load/save, test connection)
- Import flow working end-to-end
- Event streaming from wfl daemon to frontend
- Dashboard window shows parse progress

### Phase 4: Dashboard & Analysis (Week 4)

**Goals:**
- Dashboard displays project data (scenes, characters, metadata)
- Scene list virtualization (100+ items, 60fps)
- Analysis results display (per-scene, per-character)
- Export functionality (JSON, markdown, PDF)
- Dark mode toggle (TailwindCSS + design tokens)
- Error handling and edge cases

**Deliverables:**
- Dashboard window fully featured
- Scene analysis working (trigger via `wfl.analyze`, stream results)
- Export working
- Shadcn/ui components integrated
- Dark mode toggle in Settings

### Phase 5: Polish & Testing (Week 5+)

**Goals:**
- Performance tuning (startup < 2s, list scroll 60fps, memory < 150MB)
- Cross-platform testing (macOS, Linux, Windows if possible)
- Error handling for daemon disconnect, missing khaos-tools, etc.
- Documentation complete
- Release build (code signing prep, distribution format)

**Deliverables:**
- All acceptance criteria met
- E2E tests (Playwright or native Tauri testing)
- Documentation complete
- Release-ready build

---

## Team Orchestration

### Team Members

- **Tauri Backend Engineer**
  - Name: `backend-rust`
  - Role: Build Rust backend, implement wfl_client NDJSON protocol, Tauri commands, async runtime
  - Agent Type: `general-purpose` (Haiku for scaffolding, Sonnet for wfl_client + command impl)
  - Resume: `true` (long-running effort with context accumulation)

- **Frontend Engineer (Projects & Settings Windows)**
  - Name: `frontend-projects`
  - Role: Build Projects window (discovery, list, search), Settings window (provider config), shared Pinia stores
  - Agent Type: `general-purpose` (Sonnet for Vue/Nuxt complexity)
  - Resume: `true`

- **Frontend Engineer (Dashboard & Analysis)**
  - Name: `frontend-dashboard`
  - Role: Build Dashboard window (scene list, analysis results), import flow, virtualization, export
  - Agent Type: `general-purpose` (Sonnet)
  - Resume: `true`

- **Integration & Testing Engineer**
  - Name: `integration-qa`
  - Role: Test IPC communication, daemon events, cross-window functionality, E2E testing
  - Agent Type: `general-purpose` (Sonnet for integration complexity)
  - Resume: `true`

---

## Step by Step Tasks

### Phase 1: Foundation

#### 1. Tauri v2 Project Scaffold
- **Task ID**: `scaffold-tauri-project`
- **Depends On**: `none`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Create Tauri v2 project with multi-window config (`tauri.conf.json`)
- Set up `src-tauri/Cargo.toml` with dependencies: `serde`, `tokio`, `uuid`, `anyhow`, `serde_json`
- Add Tauri 2.x API imports for commands, events, window management
- Create `src-tauri/src/main.rs` with `tauri::Builder` setup, window creation for projects/settings/dashboard
- Verify Tauri dev server starts without errors

#### 2. Rust Backend Structure & wfl_client
- **Task ID**: `impl-wfl-client`
- **Depends On**: `scaffold-tauri-project`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Implement `src-tauri/src/wfl_client.rs`:
  - UDS socket connector (read env var `KHAOS_WFL_SOCKET`, fallback to `~/.khaos/wfl.sock`)
  - NDJSON encoder/decoder for IPC messages (envelopes with `v`, `id`, `type`, `method`, `params`)
  - ULID-based request ID generation
  - Connection pooling or single-connection model for Tauri app lifecycle
  - Error types wrapping wfl daemon errors (PROJECT_MISMATCH, BUSY, INVALID_PARAMS, etc.)
- Implement `src-tauri/src/commands/mod.rs` with stubs for all command modules
- Add Tauri event emitter for pushing daemon events (wfl.analysis.progress, etc.) to frontend
- Test wfl_client locally with a running daemon (if available) or mock

#### 3. Frontend Project Structure & Nuxt Scaffolds
- **Task ID**: `scaffold-frontend`
- **Depends On**: `scaffold-tauri-project`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true` (can run alongside backend scaffold)
- Create `windows/{projects,settings,dashboard}/` directory structure
- Set up per-window `package.json` and `nuxt.config.ts` with Tauri plugin
- Create `shared/` directory with base composables, stores, types
- Set up root `package.json` with workspaces or lerna (monorepo management)
- Set up `tsconfig.json`, `tailwind.config.ts` (shared across windows)
- Verify each window can be built independently

#### 4. Shared Infrastructure & Tooling
- **Task ID**: `setup-tooling`
- **Depends On**: `scaffold-tauri-project`, `scaffold-frontend`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Create root `justfile` with recipes:
  - `dev` — Start Tauri dev server (hot reload)
  - `dev:backend` — Cargo watch on src-tauri
  - `dev:frontend` — Nuxt dev servers on different ports
  - `build` — Tauri build (compiles Rust + bundles windows)
  - `build:windows` — Windows-specific build
  - `lint` — Cargo clippy + ESLint
  - `test` — Cargo test + Vitest
  - `format` — Cargo fmt + Prettier
- Create `.env.example` with variables:
  - `KHAOS_WFL_SOCKET` (default: `~/.khaos/wfl.sock`)
  - `KHAOS_TOOLS_PATH` (optional, for direct tool access if needed)
  - `DEV_PORT_PROJECTS`, `DEV_PORT_SETTINGS`, `DEV_PORT_DASHBOARD` (dev server ports)
- Create `docs/DEVELOPMENT.md` with local setup instructions

#### 5. Initial Documentation & Architecture
- **Task ID**: `docs-architecture`
- **Depends On**: `scaffold-tauri-project`, `scaffold-frontend`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Write `docs/ARCHITECTURE.md`:
  - System overview diagram (Tauri Runtime → 3 Nuxt Windows → Rust Backend → wfl daemon)
  - Layer descriptions (Presentation, Application, Infrastructure, Domain)
  - IPC communication flow (invoke → Tauri command → wfl_client → daemon → event emit → window listen)
  - Pinia store structure (Settings, Projects, Analysis, Import)
  - Data models (ProjectMetadata, Scene, AnalysisResults, ImportState)
- Write `docs/IPC.md` with Tauri command reference (stubs for now)
- Write `README.md` with quick start, current status, phase progress

### Phase 2: Core IPC & Projects Window

#### 6. Implement wfl_client Daemon Communication
- **Task ID**: `impl-daemon-client`
- **Depends On**: `impl-wfl-client`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Complete `wfl_client.rs`:
  - `connect()` — Establish UDS connection with retries
  - `ping()` — wfl.ping handshake (test connection)
  - `get_status()` — wfl.getStatus (check daemon health)
  - `get_capabilities()` — wfl.getCapabilities (supported methods/topics)
  - `subscribe(topics)` → listen for daemon events (wfl.analysis.progress, etc.)
  - Request/response serialization with error handling
- Test with daemon running or provide mock responses

#### 7. Implement Tauri Commands: Projects
- **Task ID**: `impl-commands-projects`
- **Depends On**: `impl-daemon-client`, `scaffold-tauri-project`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Implement `src-tauri/src/commands/projects.rs`:
  - `#[tauri::command] async fn discover_projects() -> Result<Vec<ProjectMetadata>, String>`
    - Scan filesystem: `~/khaos/projects/` (configurable), find `.kspd` directories
    - Extract metadata: title (from manifest or dirname), scene count, character count, lastModified
    - Return sorted by modification date
  - `#[tauri::command] async fn get_project(path: String) -> Result<ProjectDetail, String>`
    - Load project manifest, extract scenes/characters list
    - Call `wfl.parser.query` to get narrative data (if cached, or defer to dashboard)
  - `#[tauri::command] async fn open_project(path: String) -> Result<(), String>`
    - Create Dashboard window with project context
- Test locally with dummy project directory

#### 8. Implement Tauri Commands: Settings & Analysis Stubs
- **Task ID**: `impl-commands-settings-stubs`
- **Depends On**: `impl-daemon-client`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Implement `src-tauri/src/commands/settings.rs` stubs:
  - `#[tauri::command] async fn load_settings() -> Result<AppSettings, String>` — Load from XDG config (stub)
  - `#[tauri::command] async fn save_settings(settings: AppSettings) -> Result<(), String>` — Save to XDG config (stub)
  - `#[tauri::command] async fn test_connection(provider: String, config: ProviderConfig) -> Result<bool, String>` — Call wfl.ping (stub)
- Implement `src-tauri/src/commands/analysis.rs` stubs:
  - `#[tauri::command] async fn get_scene_list(project_path: String) -> Result<Vec<Scene>, String>`
  - `#[tauri::command] async fn analyze_scene(project_path: String, scene_id: String) -> Result<Analysis, String>`
  - `#[tauri::command] async fn get_analysis(project_path: String, scene_id: String) -> Result<Analysis, String>`

#### 9. Pinia Stores: Projects & Settings
- **Task ID**: `impl-pinia-stores`
- **Depends On**: `scaffold-frontend`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Create `shared/stores/projects.ts`:
  - State: `projects: ProjectMetadata[]`, `selectedProject: ProjectMetadata | null`, `recentProjects: string[]`, `isLoading: boolean`
  - Actions: `discoverProjects()`, `selectProject()`, `addToRecent()`, `clearCache()`
  - Getters: `sortedProjects()`, `filteredProjects(query)`, `projectCount`
- Create `shared/stores/settings.ts`:
  - State: `provider: string`, `model: string`, `apiKey: string`, `theme: 'light' | 'dark'`, `testingConnection: boolean`
  - Actions: `setProvider()`, `setModel()`, `loadSettings()`, `saveSettings()`, `testConnection()`
  - Getters: `isConfigured()`, `hasApiKey()`
- Create `shared/types/index.ts` with interfaces:
  - `ProjectMetadata`, `ProjectDetail`, `Scene`, `Character`, `Location`, `AppSettings`, `Analysis`, `AnalysisResults`

#### 10. Projects Window: List & Search UI
- **Task ID**: `impl-projects-window`
- **Depends On**: `impl-commands-projects`, `impl-pinia-stores`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Create `windows/projects/pages/index.vue`:
  - Load projects on mount via `invoke('discover_projects')`
  - Display virtualized list (TanStack Vue Virtual) with TailwindCSS cards
  - Show project metadata: title, author, scene count, modified date
  - Implement search/filter by title or author
  - Double-click to open project → `invoke('open_project', { path })` → create Dashboard window
  - Show loading state, error handling
- Create `windows/projects/components/`:
  - `ProjectList.vue` — Virtualized list component
  - `ProjectSearchBar.vue` — Search input
  - `ProjectCard.vue` — Single project card
- Wire to `projects` store, test with mock data

#### 11. Settings Window: Provider Config UI
- **Task ID**: `impl-settings-window`
- **Depends On**: `impl-commands-settings-stubs`, `impl-pinia-stores`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Create `windows/settings/pages/index.vue`:
  - Provider dropdown (ollama, openai, anthropic, mock)
  - Model input field
  - API key input (password field, show/hide toggle)
  - Test Connection button → `invoke('test_connection')` → show result
  - Save button (not yet persisted, just store update)
  - Theme toggle (light/dark) — update CSS, store theme preference
  - About section (version info, daemon status)
- Create `windows/settings/components/`:
  - `ProviderSelect.vue`
  - `ModelInput.vue`
  - `ApiKeyInput.vue`
  - `ConnectionTest.vue`
  - `ThemeToggle.vue`
- Wire to `settings` store

#### 12. Dashboard Window: Skeleton & Project Display
- **Task ID**: `impl-dashboard-window`
- **Depends On**: `impl-commands-projects`
- **Assigned To**: `frontend-dashboard`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Create `windows/dashboard/pages/index.vue`:
  - Receive project path from Tauri window label or query param
  - Load project metadata via `invoke('get_project', { path })`
  - Display project title, metadata (author, scene count, character count, last modified)
  - Show loading state while fetching project data
  - Placeholder for scene list (will be implemented in Phase 3)
- Create `windows/dashboard/components/ProjectHeader.vue`
- Wire to `projects` store

#### 13. Integration: Tauri IPC & Event Bus
- **Task ID**: `integrate-ipc-events`
- **Depends On**: `impl-commands-projects`, `impl-pinia-stores`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Set up Tauri event listener in Rust that bridges daemon events to frontend:
  - Listen on wfl daemon socket for events (wfl.analysis.progress, wfl.parser.completed, etc.)
  - Emit Tauri events for each daemon event (use `window.emit()` or broadcast pattern)
- Create `shared/composables/useDaemonEvents.ts`:
  - `useDaemonEvents()` hook to listen for Tauri events
  - Example: `useDaemonEvents().on('wfl.analysis.progress', (progress) => { ... })`
- Test event flow from daemon → Rust → frontend

#### 14. Phase 1 Validation & Testing
- **Task ID**: `validate-phase1`
- **Depends On**: `impl-projects-window`, `impl-settings-window`, `impl-dashboard-window`, `integrate-ipc-events`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Test local dev setup:
  - `just dev` starts all windows
  - Each window loads without errors
  - Tauri dev tools console shows no errors
- Test Projects window:
  - Discover projects returns list (mock data or real directory)
  - Search/filter works
  - Click project → Dashboard window opens
- Test Settings window:
  - Load current settings (defaults if none saved)
  - Theme toggle works
- Test IPC communication:
  - Tauri commands execute without errors
  - Results serialize/deserialize correctly
- Document results, note any blockers

### Phase 2: Settings Persistence & Import

#### 15. Settings Persistence: XDG Config & Keychain
- **Task ID**: `impl-settings-persistence`
- **Depends On**: `validate-phase1`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Implement `src-tauri/src/services/config.rs`:
  - XDG config directory: `~/.config/khaos-web-ui/` (macOS: `~/Library/Application Support/khaos-web-ui/`)
  - Config file: `settings.json` with schema (provider, model, theme, etc.)
  - `load_config()` — Read from disk, return defaults if missing
  - `save_config()` — Write atomically (write to temp, then rename)
  - Use Tauri's `tauri::api::app_dir` for path resolution
- Implement API key storage:
  - Use Tauri's native keychain bindings (if available) or platform-specific crates
  - Option A: Store encrypted API key in config (use `ring` or `sodiumoxide` crate for encryption)
  - Option B: Use OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
- Complete `#[tauri::command] async fn load_settings()` and `async fn save_settings()`
- Test load/save cycle, verify settings persist across app restart

#### 16. Settings: Test Connection (wfl.ping)
- **Task ID**: `impl-test-connection`
- **Depends On**: `impl-daemon-client`, `impl-settings-persistence`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Complete `#[tauri::command] async fn test_connection(provider: String, config: ProviderConfig) -> Result<bool, String>`:
  - Call wfl daemon `wfl.ping` to verify daemon is running
  - Optional: Make HTTP request to LLM provider endpoint to validate API key + model
  - Return success/failure with error message (connection refused, invalid API key, etc.)
  - Handle timeouts (< 2 seconds per spec)
- Update Settings window to show connection test result (success checkmark or error message)

#### 17. Import Flow: File Picker & Metadata
- **Task ID**: `impl-import-flow-ui`
- **Depends On**: `impl-dashboard-window`
- **Assigned To**: `frontend-dashboard`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Create Import modal/wizard:
  - Step 1: File picker (select `.fountain`, `.fdx`, `.md`, `.sbx`)
  - Step 2: Enter project title, confirm output path
  - Step 3: Confirm project creation
  - Step 4: Show parse progress (stream updates)
  - Step 5: Show result (success/failure)
- Create `windows/projects/components/ImportModal.vue` or dedicated import page
- Create `shared/stores/import.ts`:
  - State: `currentStep`, `inputPath`, `title`, `outputPath`, `parseProgress`, `parseStatus`
  - Actions: `startImport()`, `nextStep()`, `prevStep()`, `updateProgress()`
- Wire to Tauri commands (implemented in next task)

#### 18. Import Flow: Backend (wfl.parser.parse)
- **Task ID**: `impl-import-backend`
- **Depends On**: `impl-daemon-client`, `integrate-ipc-events`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Implement `src-tauri/src/commands/import.rs`:
  - `#[tauri::command] async fn start_parse(project_path: String, input_file: String, format: String) -> Result<String, String>`
    - Call `wfl.parser.parse` on daemon (async)
    - Return operation ID
  - `#[tauri::command] async fn get_parse_progress(operation_id: String) -> Result<ParseProgress, String>`
    - Query daemon for progress on parse operation
    - Or listen to `wfl.parser.progress` events
- Bridge `wfl.parser.progress` and `wfl.parser.completed` events to frontend
- On completion, create KSPD project directory + manifest (via daemon or locally)
- Test parse flow end-to-end

#### 19. Import Flow: Project Creation
- **Task ID**: `impl-project-creation`
- **Depends On**: `impl-import-backend`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- On `wfl.parser.completed` event:
  - Create KSPD project directory (if not exists): `<output_path>/<project_title>.kspd/`
  - Create `manifest.json` with project metadata (title, author, createdAt)
  - Create `story/` directory + placeholder files
  - Create `metadata/` directory for signals, events, snapshots
  - Return success to frontend
- Update Projects window to refresh list after import

#### 20. Event Streaming: Analysis Progress
- **Task ID**: `impl-event-streaming`
- **Depends On**: `integrate-ipc-events`, `impl-import-backend`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Complete daemon event subscription:
  - Subscribe to `wfl.analysis.progress` topic on daemon connection
  - Subscribe to `wfl.parser.progress` topic
  - Subscribe to `wfl.analysis.completed` topic
  - Emit Tauri event for each received daemon event
- Update Dashboard window to listen for progress events and update UI in real-time
- Test progress updates during import or analysis

#### 21. Phase 2 Validation
- **Task ID**: `validate-phase2`
- **Depends On**: `impl-test-connection`, `impl-import-flow-ui`, `impl-project-creation`, `impl-event-streaming`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Test Settings window:
  - Provider dropdown, model input, API key input
  - Save settings → verify persisted in config file
  - Load app → settings loaded from disk
  - Test Connection button → wfl.ping called, result shown
- Test Import flow end-to-end:
  - Click "Create Project" → Import modal opens
  - Select file → enter title → confirm
  - Watch progress updates in real-time
  - Project created on disk
  - Return to Projects, refresh → new project appears
- Test event streaming:
  - Start analysis → progress events arrive at frontend
  - Dashboard updates in real-time
- Document results, address any failures

### Phase 3: Dashboard & Analysis

#### 22. Dashboard: Scene List (Virtualized)
- **Task ID**: `impl-scene-list`
- **Depends On**: `validate-phase2`
- **Assigned To**: `frontend-dashboard`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Implement `get_scene_list` Tauri command (call `wfl.parser.query` with `queryType: 'scenes'`)
- Create `windows/dashboard/components/SceneList.vue`:
  - Use TanStack Vue Virtual for 100+ items
  - Display scene number, heading, character list, analysis status
  - Sortable by: scene number, character count, last modified
  - Click scene → detail panel/modal
- Create `shared/stores/analysis.ts`:
  - State: `currentScene: Scene | null`, `sceneList: Scene[]`, `analysisResults: Map<sceneId, Analysis>`
  - Actions: `selectScene()`, `updateSceneList()`, `cacheAnalysis()`
- Test with 100+ item list, verify 60fps scroll

#### 23. Dashboard: Scene Detail & Analysis Results
- **Task ID**: `impl-scene-detail`
- **Depends On**: `impl-scene-list`
- **Assigned To**: `frontend-dashboard`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Create `windows/dashboard/components/SceneDetail.vue`:
  - Show selected scene heading, characters, description
  - Display analysis results (dialogue, action, character consistency signals)
  - Show signals (high-level feedback from wfl daemon if available)
  - Quick action buttons: "Analyze This Scene", "View Signals"
- Create `windows/dashboard/components/AnalysisResults.vue`:
  - Display analysis output (key insights, signals)
  - Format: prose + structured data (tables, lists)
- Implement `analyze_scene` command:
  - Call `wfl.analyze` with scope: `{ kind: 'scene', targets: [sceneId] }`
  - Stream progress and results to frontend
- Wire to analysis store

#### 24. Dashboard: Export Functionality
- **Task ID**: `impl-export`
- **Depends On**: `impl-scene-detail`
- **Assigned To**: `frontend-dashboard`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Create `windows/dashboard/components/ExportMenu.vue`:
  - Export options: JSON, Markdown, CSV
  - Scope: current scene, all scenes, all results
- Implement export logic:
  - JSON: Dump analysis results as JSON
  - Markdown: Format as markdown report
  - CSV: Tabular format for spreadsheet import
- Use browser API to download files (`fetch` → blob → save)
- Test export formats

#### 25. Shadcn/ui Integration & Theming
- **Task ID**: `impl-shadcn-ui`
- **Depends On**: `validate-phase2`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Set up Shadcn/ui Vue:
  - Install `@radix-vue`, `tailwindcss`, `class-variance-authority`
  - Copy base components: Button, Card, Input, Select, Modal, Dropdown, Tabs, etc.
  - Place in `shared/components/`
- Configure TailwindCSS:
  - Create `shared/styles/tokens.css` with design tokens (colors, spacing, fonts)
  - Theme colors (primary, secondary, background, foreground)
  - Dark mode CSS variables
- Create theme switcher:
  - `useDarkMode()` composable
  - Toggle in Settings window
  - Persist theme preference
- Replace all placeholder UI with Shadcn/ui components

#### 26. Dark Mode Support
- **Task ID**: `impl-dark-mode`
- **Depends On**: `impl-shadcn-ui`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `true`
- Implement dark mode:
  - TailwindCSS `dark:` variants
  - CSS variables in `shared/styles/tokens.css` for theme colors
  - `useDarkMode()` composable reads from settings store
  - Apply `dark` class to document root when active
- Test dark mode across all windows
- Ensure contrast and readability

#### 27. Phase 3 Validation & Performance
- **Task ID**: `validate-phase3`
- **Depends On**: `impl-scene-list`, `impl-scene-detail`, `impl-export`, `impl-shadcn-ui`, `impl-dark-mode`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Test Dashboard window:
  - Open project → scene list loads and displays 100+ scenes
  - Scroll list at 60fps (use DevTools Performance profiler)
  - Click scene → detail panel shows content, analysis results
  - Click "Analyze Scene" → progress updates, results show
  - Export works for all formats
- Test Shadcn/ui components:
  - All components render correctly
  - Buttons, dropdowns, modals work
- Test dark mode:
  - Toggle in Settings → all windows update theme
  - Contrast meets accessibility standards
- Performance profiling:
  - Startup time < 2 seconds
  - Memory per window < 150MB (use Chrome DevTools Memory profiler)
  - Idle CPU < 2% (use Activity Monitor or `top`)

### Phase 4-5: Polish & Release

#### 28. Error Handling & Edge Cases
- **Task ID**: `impl-error-handling`
- **Depends On**: `validate-phase3`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Handle errors:
  - Daemon not running → show helpful message + link to troubleshooting
  - Project directory missing → graceful error
  - Invalid project structure → show validation errors
  - Network errors (if using HTTP provider) → retry logic + user-facing message
  - File I/O errors → try/catch + user notification
- Implement error modal/toast notifications
- Log errors for debugging (use `log` crate, write to `~/.khaos-web-ui/logs/`)
- Test error paths (simulate daemon crash, missing files, etc.)

#### 29. Cross-Platform Compatibility
- **Task ID**: `test-cross-platform`
- **Depends On**: `validate-phase3`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Test on macOS (native target):
  - Verify Tauri build succeeds
  - Test UDS socket path resolution
  - Test Keychain integration (API key storage)
- Test on Linux (if CI available):
  - Verify UDS socket path resolution
  - Test Secret Service keychain (if implemented)
- Test on Windows (if resources available):
  - Verify named pipe equivalent of UDS
  - Test Credential Manager keychain
- Document platform-specific setup in `DEVELOPMENT.md`

#### 30. E2E Testing
- **Task ID**: `impl-e2e-tests`
- **Depends On**: `test-cross-platform`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Set up Playwright or Tauri native testing:
  - Test user flow: Open app → Configure Settings → Discover Projects → Open Project → Analyze Scene
  - Test multi-window: Open Projects + Dashboard side-by-side, verify independent state
  - Test import: Full import flow from file picker to project creation
- Write test suite (smoke tests, happy path, error scenarios)
- Run tests on CI/CD pipeline (GitHub Actions)

#### 31. Documentation & User Guides
- **Task ID**: `finalize-documentation`
- **Depends On**: `finalize-all`
- **Assigned To**: `frontend-projects`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Complete documentation:
  - `README.md` — Quick start, features, status
  - `docs/ARCHITECTURE.md` — System design (finalize)
  - `docs/WINDOWS.md` — Per-window feature reference
  - `docs/IPC.md` — Tauri commands API (final)
  - `docs/STORES.md` — Pinia stores API
  - `docs/COMPONENTS.md` — Shadcn/ui setup guide
  - `docs/DEVELOPMENT.md` — Dev setup, local testing, contributing
  - `CHANGELOG.md` — Phase 2 release notes
- Create user guide (screenshots, workflow examples)

#### 32. Build & Release Preparation
- **Task ID**: `prep-release`
- **Depends On**: `finalize-documentation`, `impl-e2e-tests`
- **Assigned To**: `backend-rust`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Prepare Tauri release build:
  - Update version in `package.json` and `Cargo.toml` to `0.2.0` (Phase 2)
  - Configure code signing (macOS: signing identity, Windows: certificate)
  - Build release bundles: `.app` (macOS), `.msi` (Windows), `.AppImage` (Linux)
- Create installer/distribution package
- Set up auto-update mechanism (Tauri updater)
- Document release process in `justfile` and `docs/RELEASES.md`

#### 33. Final Validation & Sign-Off
- **Task ID**: `final-validation`
- **Depends On**: `prep-release`
- **Assigned To**: `integration-qa`
- **Agent Type**: `general-purpose`
- **Parallel**: `false`
- Run full test suite:
  - All acceptance criteria met
  - All features working as documented
  - No known bugs or regressions
- Performance validation:
  - Startup time < 2 seconds ✓
  - Scene list (100 items) smooth scroll @ 60fps ✓
  - Memory per window < 150MB ✓
  - CPU idle < 2% ✓
- Cross-platform validation (macOS, Linux, Windows if available)
- User acceptance testing (demo to stakeholders)
- Document sign-off, note any Phase 3+ items

---

## Acceptance Criteria

Phase 2 is **complete** when:

1. ✅ **Architecture**
   - Tauri v2 multi-window app with three independent Nuxt windows
   - Rust backend connects to khaos-wfl daemon via UDS
   - Pinia stores for Projects, Settings, Analysis, Import (per-window state)

2. ✅ **Projects Window**
   - Discovers KSPD projects from filesystem (async)
   - Displays project list with metadata (title, author, scenes, modified)
   - Search/filter by title or author
   - Click to open in Dashboard
   - Show recent projects
   - Performance: discovery < 500ms, list handles 100+ projects smoothly

3. ✅ **Settings Window**
   - Provider selection (ollama, openai, anthropic, mock, etc.)
   - Model name input
   - API key management (secure storage via keychain or encryption)
   - Test Connection button (calls wfl.ping)
   - Settings persist across app restarts (XDG config)
   - Theme toggle (light/dark mode)
   - About section with version, daemon status

4. ✅ **Dashboard Window**
   - Project overview: title, metadata, scene count, character count
   - Virtualized scene list (100+ items @ 60fps, TanStack Vue Virtual)
   - Scene detail panel: heading, characters, description
   - Analysis results display (per-scene and aggregated)
   - Analyze scene / analyze all (triggers wfl.analyze, streams progress)
   - Export functionality (JSON, Markdown, CSV)
   - Performance: project load < 500ms, analysis display < 100ms

5. ✅ **Import Flow**
   - File picker (supports .fountain, .fdx, .md, .sbx)
   - Title input and confirmation
   - Call wfl.parser.parse, stream progress
   - Create KSPD project on disk
   - Refresh Projects list after import

6. ✅ **Tauri IPC Bridge**
   - All commands working: discover_projects, get_project, load_settings, save_settings, test_connection, analyze_scene, get_scene_list, start_parse, get_parse_progress, etc.
   - Error handling with user-friendly messages
   - Event streaming from daemon to frontend (progress updates)

7. ✅ **UI & Design**
   - Shadcn/ui components integrated
   - TailwindCSS styling with design tokens
   - Dark mode support
   - Responsive layout (desktop, tablet)
   - No console errors, clean DevTools

8. ✅ **Tooling & Documentation**
   - Justfile with dev, build, test, lint recipes
   - `docs/ARCHITECTURE.md`, `docs/IPC.md`, `docs/DEVELOPMENT.md`
   - README with quick start and feature overview
   - `.env.example` with all required env vars

9. ✅ **Performance**
   - Startup time < 2 seconds
   - Scene list (100+ items) scrolls at 60fps, no jank
   - Project discovery < 500ms
   - Analysis display < 100ms
   - Memory per window < 150MB
   - CPU idle < 2%

10. ✅ **Testing**
    - All features manually tested end-to-end
    - Cross-platform tested (macOS, Linux)
    - Error scenarios covered (daemon down, missing files, etc.)
    - E2E test suite (Playwright or Tauri tests) in place

11. ✅ **Daemon Integration**
    - All wfl daemon calls return correctly formatted responses
    - Event topics subscribed and working (wfl.analysis.progress, etc.)
    - PROJECT_MISMATCH and other daemon errors handled gracefully
    - Daemon connection retries and timeouts work

---

## Validation Commands

### Local Development

```bash
# Install dependencies
npm install
cd src-tauri && cargo build

# Start dev environment
just dev
# Or separately:
just dev:backend
just dev:frontend

# Run Tauri dev server (hot reload)
# Windows auto-open; manually visit http://localhost:5173 for Projects

# Lint code
just lint
just format

# Test (if tests written)
just test
```

### Build & Release

```bash
# Build release app
just build

# For specific platforms:
just build:macos
just build:windows
just build:linux

# Verify bundle
ls target/release/bundle/

# Code sign (macOS)
codesign -v dist/khaos-web-ui.app
```

### Manual Testing Checklist

```bash
# 1. Projects Window
# - Open app, see project list
# - Search for project, verify filter works
# - Double-click project, Dashboard opens

# 2. Settings Window
# - Open Settings
# - Change provider (dropdown)
# - Enter API key
# - Click "Test Connection", watch result
# - Close and reopen app, verify settings loaded

# 3. Dashboard Window
# - Project loaded with scene list
# - Scroll 100+ scenes, verify smooth @ 60fps
# - Click scene, detail panel appears
# - Click "Analyze", watch progress update
# - Click "Export", download works

# 4. Import Flow
# - Click "Create Project"
# - Select .fountain file
# - Enter project title
# - Watch parse progress
# - Verify project created on disk
# - Return to Projects, refresh, new project appears

# 5. Dark Mode
# - Open Settings
# - Toggle theme
# - All windows update to dark mode

# 6. Performance (Chrome DevTools)
# Performance → Start Recording
# - Open app (should complete in < 2s)
# - Scroll scene list (should maintain 60fps)
# Memory → Take Snapshot
# - Per-window memory < 150MB
# Task Manager (Windows) or Activity Monitor (macOS)
# - Idle CPU < 2%
```

### Performance Validation

```bash
# macOS: Use Instruments
open -a Instruments

# Linux: Use perf
perf stat -p <pid>

# Check memory
ps aux | grep khaos-web-ui

# Check CPU
top -pid <pid>
```

### Daemon Communication Test (if daemon running)

```bash
# Test daemon connection
nc -U ~/.khaos/wfl.sock
# Send: {"id":"test-1","method":"wfl.ping","params":{}}
# Expect: {"v":1,"id":"test-1","type":"res","ok":true,"result":{"ts":"2026-02-17T..."}}

# Watch daemon events
tail -f ~/.khaos/daemon.log
```

---

## Notes

### Key Technical Decisions

1. **Three Separate Nuxt Apps:** Isolation, simplicity, extensibility. Trade-off: 3x bundle size, no shared runtime state (Pinia per-window).

2. **wfl Daemon Integration:** All analysis, parsing, and signals flow through khaos-wfl daemon (not direct khaos-tools calls). This ensures consistency with other tools in the Khaos ecosystem.

3. **UDS/NDJSON:** Unix Domain Socket for efficiency and security; NDJSON protocol per daemon IPC spec.

4. **Rust Backend:** Handles all daemon communication, project discovery, config management. Frontend stays thin (view layer only).

5. **Pinia per-Window:** No cross-window shared state initially (keep it simple). Phase 3+ can add cross-window IPC if needed.

### Dependencies & Prerequisites

- **Local:** Node.js 18+, Rust 1.70+, Tauri CLI
- **Runtime:** khaos-wfl daemon running or available at `$KHAOS_WFL_SOCKET` (env var)
- **Optional:** khaos-tools in PATH (for parsing formats wfl doesn't support)
- **Development:** Playwright (for E2E tests), Chrome/Firefox (for DevTools profiling)

### Known Limitations & Phase 3+ Roadmap

- **MVP:** No mobile support, no collaborative editing, no advanced graphs/charts
- **Phase 3:** Browser-based version, advanced visualization, plugin system
- **Phase 4+:** Custom keyboard shortcuts, git integration, project templates

### File Structure Reference

```
khaos-web-ui-bootstrap/
├── src-tauri/                  # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── wfl_client.rs       # wfl daemon IPC client
│   │   ├── commands/
│   │   ├── services/
│   │   └── types.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── windows/
│   ├── projects/               # Projects window (Nuxt)
│   ├── settings/               # Settings window (Nuxt)
│   └── dashboard/              # Dashboard window (Nuxt)
├── shared/                      # Shared Vue/TS code
│   ├── components/             # Shadcn/ui copies
│   ├── composables/            # useIpc, useSettings, etc.
│   ├── stores/                 # Pinia stores
│   ├── types/                  # TypeScript interfaces
│   └── styles/                 # Globals, tokens, Tailwind
├── docs/
├── specs/                       # Plans and specs (including this file)
├── package.json                # Root package, workspaces
├── tsconfig.json
├── tailwind.config.ts
├── justfile                    # New: dev, build, lint, test, format
└── .env.example
```

### Support & Troubleshooting

**Daemon won't connect:**
- Verify daemon running: `ps aux | grep khaos-wfl`
- Check socket path: `ls -la ~/.khaos/wfl.sock`
- Set `KHAOS_WFL_SOCKET` env var if custom path

**Projects not discovering:**
- Check projects directory: `ls ~/khaos/projects/`
- Verify KSPD structure (should have `.kspd` extension or `manifest.json`)
- Check logs: `tail ~/.khaos-web-ui/logs/`

**Analysis not starting:**
- Ensure daemon is healthy: `wfl.ping` should respond
- Check khaos-tools installed: `which khaos-tools`
- Review daemon logs for errors

**Import failing:**
- Verify input file format matches (Fountain, FDX, Markdown, SBX)
- Check output directory writable: `touch ~/khaos/projects/.test && rm ~/khaos/projects/.test`
- Review parse logs in dashboard

---

## Team Coordination & Timeline

**Total Effort:** ~5-6 weeks (Phase 1-5)
- **Week 1-2:** Foundation (scaffold, backend structure, frontend scaffolds)
- **Week 2-3:** Core IPC & Projects Window (daemon client, list UI, settings config)
- **Week 3-4:** Import & Analysis (parse flow, scene list, analysis results)
- **Week 4-5:** Polish & Release (dark mode, error handling, testing, docs)

**Team Structure:**
- 2 backend engineers (Rust/Tauri, one starts wfl_client, other does commands + services)
- 2 frontend engineers (Projects/Settings, Dashboard/Analysis)
- 1 QA/Integration engineer (validates phases, tests E2E)

**Critical Path:**
1. Tauri scaffold → foundation
2. wfl_client → unblocks all daemon communication
3. Tauri commands → frontend can invoke
4. Pinia stores → UI state management
5. Windows implemented in parallel (Projects, Settings, Dashboard)
6. Integration & event streaming → live updates
7. Polish & E2E testing → ship-ready

**Parallel Opportunities:**
- Frontend window development (Projects/Settings/Dashboard can start after scaffold)
- Shadcn/ui setup (while backend works on daemon client)
- Documentation (started in Phase 1, finalized in Phase 5)

**Sync Points (Team Meetings):**
- Monday: Phase kickoff, review blockers
- Wednesday: Mid-week sync, integration issues
- Friday: Phase validation, next week planning
