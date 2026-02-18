# Development Guide

## Overview

This guide covers the development workflow for Khaos Web UI, including setup, building, testing, and debugging.

## Environment Setup

### Requirements

| Component | Version | Link |
|-----------|---------|------|
| Node.js | 18+ | https://nodejs.org |
| npm | 8+ | Included with Node.js |
| Rust | 1.70+ | https://rustup.rs |
| Tauri CLI | 2.0+ | `cargo install tauri-cli` |

### Installation

```bash
# 1. Install Rust and Tauri CLI
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli

# 2. Verify installations
node --version       # v18.x.x or higher
npm --version        # 8.x.x or higher
rustc --version      # 1.70+
cargo --version      # 1.70+
```

### Clone & Setup

```bash
# Clone repository
git clone <url> khaos-web-ui-bootstrap
cd khaos-web-ui-bootstrap

# Install dependencies
npm install

# Copy environment template
cp .env.example .env
# Edit .env with your daemon socket path and project roots
```

## Development Workflow

### Starting Development Servers

```bash
# Terminal 1: Start daemon (if not already running)
khaos-wfl daemon

# Terminal 2: Start all frontend dev servers + watch Rust
just dev

# Or start individual windows:
just dev-projects
just dev-settings
just dev-dashboard
```

This will:
- Start Tauri in dev mode with hot-reload
- Serve all three Nuxt windows on ports 5173-5175
- Watch Rust backend for changes

### Building

```bash
# Production build
just build

# Frontend only
just build-frontend

# Tauri backend only
just build-tauri

# Verify build succeeded
ls -la src-tauri/target/release/khaos-web-ui*
```

### Testing

```bash
# Run all tests
just test

# Run tests in watch mode
just test-watch

# Coverage report
just test-coverage

# Specific test
npm --workspace @khaos/projects-window test -- --run

# Rust tests
cd src-tauri && cargo test
```

### Code Quality

```bash
# Lint TypeScript/Vue
just lint

# Fix linting issues
just lint-fix

# Format code
just format

# Lint Rust
just lint-rust

# Check environment
just doctor
```

## Architecture & Patterns

### Command Handlers

Commands are Tauri's RPC mechanism for frontend-to-backend communication.

**Structure:**
```rust
#[command]
pub async fn discover_projects() -> Result<Vec<Project>, String> {
    // 1. Validate inputs (already typed by Tauri)
    // 2. Call service layer
    // 3. Map result to frontend type
    // 4. Return Result<T, String> where Err(msg) becomes frontend error
}
```

**Error handling:**
- Keep command signatures simple: `Result<T, String>`
- Services handle detailed error types
- Convert to user-facing messages at command boundary

### Services Layer

Services contain business logic, isolated from Tauri concerns.

**Structure:**
```rust
// src/services/discovery.rs
pub fn discover_projects() -> Result<Vec<Project>, DiscoveryError> {
    // Implement filesystem scanning, caching, etc.
    // May call wfl_client for daemon operations
}
```

**Principles:**
- Services are testable in isolation
- Error types are detailed (enum for different cases)
- No Tauri or HTTP dependencies

### State Management (Frontend)

Each window maintains its own Pinia store for domain data.

**Example (Projects window):**
```typescript
// stores/projects.ts
export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])
  const filter = ref('')
  const selected = ref<Project | null>(null)

  async function discover() {
    projects.value = await invoke('discover_projects')
  }

  return { projects, filter, selected, discover }
})
```

**Cross-window state (Settings):**
```typescript
// Shared theme store
export const useThemeStore = defineStore('theme', () => {
  const theme = ref<'light' | 'dark' | 'system'>('system')

  // On change: emit app:settings-changed, persist to config
  // On load: hydrate from persisted config
})
```

### IPC Communication

Frontend invokes Rust commands; Rust communicates with daemon over IPC.

**Frontend:**
```typescript
// Invoke command
const result = await invoke('analyze_scene', {
  projectId: 'proj-123',
  sceneId: 'scene-456'
})

// Listen for daemon events
onTauriEvent('daemon:analysis-progress', (event) => {
  console.log(event.payload.progress)
})
```

**Rust (Command Handler):**
```rust
#[command]
pub async fn analyze_scene(project_id: String, scene_id: String) -> Result<String, String> {
    let client = DaemonClient::connect().await?;
    let response = client.request("wfl.analyze", json!({
        "project_id": project_id,
        "scene_id": scene_id
    })).await?;
    Ok(response["request_id"].as_str().unwrap_or("").to_string())
}
```

**Rust (Event Bridge):**
```rust
// Backend listens for daemon progress and re-emits as Tauri event
async fn bridge_analysis_progress(app_handle: AppHandle) {
    // Subscribe to daemon topic, listen for events
    // For each event: emit to frontend
    app_handle.emit_all("daemon:analysis-progress", event)?;
}
```

## Module Organization

### Rust Backend

```
src-tauri/src/
├── main.rs              # App entry, window setup
├── lib.rs               # Logging, utilities
├── types.rs             # Shared DTOs (Project, SystemInfo, etc.)
├── events.rs            # Event payloads (AnalysisProgressEvent, etc.)
├── wfl_client/          # Daemon communication
│   ├── mod.rs           # Client facade
│   ├── protocol.rs      # NDJSON, methods, topics
│   ├── transport_unix.rs
│   └── transport_windows.rs
├── commands/            # Tauri command handlers
│   ├── mod.rs
│   ├── projects.rs      # discover, search, get
│   ├── settings.rs      # load, save, validate
│   ├── dashboard.rs     # scenes, analysis
│   ├── import.rs        # parse, progress
│   └── system.rs        # info, logging
└── services/            # Business logic
    ├── config.rs        # Settings persistence
    ├── keychain.rs      # Secure storage
    ├── discovery.rs     # Project discovery
    └── export.rs        # Export formats
```

### Frontend

```
windows/
├── projects/            # Nuxt app
│   ├── app.vue
│   ├── pages/
│   ├── components/
│   ├── composables/
│   ├── stores/
│   └── nuxt.config.ts
├── settings/
│   └── (similar structure)
└── dashboard/
    └── (similar structure)

shared/
├── composables/         # Shared Vue composables
│   └── useDaemonEvents.ts (TBD)
├── stores/              # Global stores
│   └── theme.ts         # Theme + active project (cross-window sync)
├── components/          # Shared Vue components
├── types/               # Shared TypeScript types
│   └── index.ts
└── styles/
    └── index.css        # Global styles (Tailwind)
```

## Testing Strategy

### Unit Tests

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_projects_scans_filesystem() {
        // Mock filesystem, assert results
    }
}
```

**Frontend:**
```typescript
describe('useProjectsStore', () => {
  it('should filter projects by query', () => {
    const store = useProjectsStore()
    store.projects = [...]
    store.filter = 'screenplay'
    expect(store.filteredProjects).toHaveLength(1)
  })
})
```

### Integration Tests

Mock daemon transport and test command handlers:

```rust
#[tokio::test]
async fn test_analyze_command_returns_request_id() {
    let mock_daemon = MockDaemonClient::new();
    mock_daemon.on_request("wfl.analyze", json!({"request_id": "123"}));

    let result = analyze_scene("proj".into(), "scene".into()).await?;
    assert_eq!(result, "123");
}
```

### E2E Tests

Run against real daemon (or test fixture) with Playwright:

```typescript
test('end-to-end: import → analyze → export', async () => {
  // 1. Open Projects window
  // 2. Import screenplay file
  // 3. Trigger analysis
  // 4. Wait for completion
  // 5. Export result
  // 6. Verify file was created
})
```

## Debugging

### Enable Debug Logging

```bash
export KHAOS_WEB_UI_LOG_LEVEL=debug
just dev
```

View logs in:
- Rust backend: stderr (terminal)
- Frontend: Browser DevTools console

### Browser DevTools

- Open DevTools: F12 or right-click → Inspect
- Frontend console: `window.__TAURI__.invoke('...')` to test commands
- Network tab: View IPC transport (Unix socket requests not visible; would need proxy)

### Rust Debugger

```bash
# With rust-gdb (macOS requires additional setup)
rust-gdb --args target/debug/khaos-web-ui
# Or use VS Code with CodeLLDB extension
```

### Tauri Devtools

Built-in DevTools available in dev mode:
- Right-click any window → Inspect Element
- Access Rust backend logs via console

## Performance Optimization

### Profiling

**Frontend:**
```typescript
// Chrome DevTools → Performance tab
// Record interactions, analyze flame graph
```

**Rust:**
```bash
# Benchmark with criterion
cargo bench --release
```

### Common Optimizations

- **Scene list**: Virtual scrolling (only render visible rows)
- **Project discovery**: Cache results, debounce searches
- **Store selectors**: Memoize filtered/sorted results
- **Daemon calls**: Batch requests where possible

## Environment Variables

See `.env.example` for all available variables:

```bash
# Daemon IPC paths
KHAOS_WFL_SOCKET=/tmp/khaos-wfl.sock      # Unix socket (macOS/Linux)
KHAOS_WFL_PIPE=\\.\pipe\khaos-wfl          # Named pipe (Windows)

# Project discovery
KHAOS_PROJECTS_ROOT=$HOME/Projects

# Logging
KHAOS_WEB_UI_LOG_LEVEL=info

# Dev server ports
DEV_PORT_PROJECTS=5173
DEV_PORT_SETTINGS=5174
DEV_PORT_DASHBOARD=5175
```

## Troubleshooting

### "Daemon unreachable"

```bash
# 1. Verify daemon is running
pgrep khaos-wfl || echo "Daemon not running"

# 2. Check socket path
ls -la /tmp/khaos-wfl.sock  # macOS/Linux
# or
net use | grep khaos-wfl    # Windows

# 3. Verify KHAOS_WFL_SOCKET environment variable
echo $KHAOS_WFL_SOCKET
```

### Build errors

```bash
# Clear build cache
just clean

# Update toolchain
rustup update
cargo install tauri-cli --force

# Check dependencies
npm ls
cargo tree
```

### Hot reload not working

- Ensure dev server is running on correct port
- Check for conflicting processes: `lsof -i :5173`
- Restart Tauri dev mode

## Release Checklist

Before releasing a new version:

- [ ] All tests passing: `just test`
- [ ] No linting issues: `just lint && just lint-rust`
- [ ] Build succeeds: `just build`
- [ ] Manual smoke test:
  - [ ] All windows open
  - [ ] Projects discovery works
  - [ ] Settings form fills in
  - [ ] Daemon check succeeds
  - [ ] Theme toggle works across windows
- [ ] Version bumped in `Cargo.toml` and `package.json`
- [ ] Changelog updated
- [ ] Git tag created: `git tag v0.x.y`

See [create-release skill](/.claude/skills/create-release/) for automated process.

## Further Reading

- [Architecture Overview](ARCHITECTURE.md)
- [IPC Protocol Specification](IPC_PROTOCOL.md)
- [Phase 2 Bootstrap Plan](../specs/phase-2-bootstrap-implementation-v2.md)
- [Tauri Docs](https://tauri.app/docs)
- [Vue 3 Docs](https://vuejs.org)
- [Pinia Docs](https://pinia.vuejs.org)
