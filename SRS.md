# khaos-web-ui Software Requirements Specification

**Version:** 1.0  
**Date:** Feb 17, 2026  
**Status:** Phase 2 Bootstrap

---

## 1. System Overview

khaos-web-ui is a Tauri-based native desktop application that provides a GUI for khaos-tools screenplay analysis platform. It consists of three independent windows, each a Nuxt application communicating via IPC.

```
┌─────────────────────────────────────────────┐
│ khaos-web-ui (Tauri Desktop App)            │
│                                             │
│ ┌─────────────────────────────────────────┐ │
│ │ Tauri Runtime (Rust + Native Windowing) │ │
│ └─────────────────────────────────────────┘ │
│                                             │
│ ┌───────────┐  ┌──────────┐  ┌───────────┐ │
│ │ Projects  │  │ Settings │  │ Dashboard │ │
│ │ (Nuxt)    │  │ (Nuxt)   │  │ (Nuxt)    │ │
│ └───────────┘  └──────────┘  └───────────┘ │
│        ↓             ↓             ↓        │
│ ┌─────────────────────────────────────────┐ │
│ │ IPC Bridge (Tauri Commands)             │ │
│ └─────────────────────────────────────────┘ │
│        ↓                                    │
└────────┼────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────┐
│ khaos-tools Daemon                      │
│ (HTTP/Unix socket API)                  │
└─────────────────────────────────────────┘
```

---

## 2. Technical Architecture

### 2.1 Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| **Desktop Framework** | Tauri | v2.x |
| **Frontend Framework** | Vue 3 + Nuxt | Latest |
| **Component Library** | Shadcn/ui (Vue) | Latest |
| **Styling** | TailwindCSS | v3+ |
| **State Management** | Pinia | v2+ |
| **IPC Communication** | Tauri Commands | v2.x |
| **Build Tool** | Vite | Latest |
| **Package Manager** | npm | Latest |

### 2.2 Project Structure

```
khaos-web-ui-bootstrap/
├── src-tauri/                    # Tauri Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri app entry point
│   │   ├── commands/
│   │   │   ├── projects.rs      # Project discovery IPC
│   │   │   ├── settings.rs      # Settings management IPC
│   │   │   └── analysis.rs      # Analysis query IPC
│   │   └── services/
│   │       ├── khaos_client.rs  # khaos-tools daemon communication
│   │       └── config.rs        # App configuration
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── windows/                      # Nuxt window applications
│   ├── projects/                # Projects window
│   │   ├── app.vue
│   │   ├── pages/
│   │   ├── components/
│   │   └── composables/
│   ├── settings/                # Settings window
│   │   ├── app.vue
│   │   ├── pages/
│   │   └── composables/
│   └── dashboard/               # Dashboard window
│       ├── app.vue
│       ├── pages/
│       ├── components/
│       └── composables/
│
├── shared/                       # Shared Vue code
│   ├── components/              # Shadcn/ui components
│   ├── composables/
│   │   ├── useIpc.ts           # Tauri IPC wrapper
│   │   ├── useSettings.ts      # Settings store access
│   │   └── useProjects.ts      # Projects store access
│   ├── stores/
│   │   ├── settings.ts         # Pinia settings store
│   │   ├── projects.ts         # Pinia projects store
│   │   └── analysis.ts         # Analysis results store
│   ├── styles/
│   │   ├── globals.css
│   │   ├── tokens.css          # Design tokens
│   │   └── tailwind.config.ts
│   └── types/
│       └── index.ts            # Shared TypeScript types
│
├── package.json
├── tsconfig.json
├── tailwind.config.ts
└── README.md
```

---

## 3. IPC Communication Protocol

### 3.1 Tauri Commands

**Projects Window:**
```rust
// Discover projects
#[tauri::command]
async fn discover_projects() -> Result<Vec<ProjectMetadata>, String>

// Get project details
#[tauri::command]
async fn get_project(path: String) -> Result<ProjectDetail, String>

// Open project in Dashboard
#[tauri::command]
async fn open_project(path: String) -> Result<(), String>
```

**Settings Window:**
```rust
// Load settings
#[tauri::command]
async fn load_settings() -> Result<AppSettings, String>

// Save settings
#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), String>

// Test provider connection
#[tauri::command]
async fn test_connection(provider: String, config: ProviderConfig) -> Result<bool, String>
```

**Dashboard Window:**
```rust
// Get project data
#[tauri::command]
async fn get_scene_list(project_path: String) -> Result<Vec<Scene>, String>

// Get analysis results
#[tauri::command]
async fn get_analysis(project_path: String, scene_id: String) -> Result<Analysis, String>

// Trigger analysis
#[tauri::command]
async fn analyze_scene(project_path: String, scene_id: String) -> Result<Analysis, String>
```

### 3.2 khaos-tools Daemon Communication

Each IPC command internally calls khaos-tools via HTTP or Unix socket:

```
Tauri App → Tauri IPC Command → Rust Service → HTTP/Unix Socket → khaos-tools Daemon
```

**Service Discovery:**
- Environment variable: `KHAOS_TOOLS_URL` (default: `http://localhost:9000`)
- Fallback: Auto-detect Unix socket at `~/.khaos/daemon.sock`

---

## 4. Component Specifications

### 4.1 Projects Window

**Responsibilities:**
- Discover KSPD projects in user's filesystem
- Display project list with metadata
- Handle project selection
- Spawn Dashboard window

**Data Model:**
```typescript
interface ProjectMetadata {
  id: string
  path: string
  title: string
  author?: string
  sceneCount: number
  characterCount: number
  lastModified: Date
}
```

**Interactions:**
- List virtualization: TanStack Virtual (if 50+ projects)
- Search/filter by title or author
- Sort by date, scene count, title
- Double-click → opens Dashboard

---

### 4.2 Settings Window

**Responsibilities:**
- Display current LLM provider configuration
- Allow provider/model selection
- Manage API keys securely
- Test connections
- Persist settings

**Data Model:**
```typescript
interface AppSettings {
  provider: 'ollama' | 'openai' | 'anthropic'
  model: string
  apiKey?: string
  endpoint?: string
  temperature?: number
  maxTokens?: number
}
```

**Security:**
- API keys stored in OS keychain (if available)
- Fallback: Encrypted local storage
- Never log API keys
- Clear on app uninstall

---

### 4.3 Dashboard Window

**Responsibilities:**
- Display project overview
- Show scene list
- Display analysis results
- Trigger analysis workflows
- Export results

**Data Model:**
```typescript
interface ProjectDetail {
  metadata: ProjectMetadata
  scenes: Scene[]
  characters: Character[]
  analysis?: AnalysisResults
}

interface Scene {
  id: string
  number: number
  heading: string
  description: string
  characters: string[]
  analysis?: SceneAnalysis
}

interface AnalysisResults {
  scenes: Map<string, SceneAnalysis>
  characters: Map<string, CharacterAnalysis>
  locations?: Map<string, LocationAnalysis>
}
```

**Interactions:**
- Scene list: virtualized, sortable
- Select scene → detail panel (side modal or expand)
- Trigger analysis: "Analyze All" or per-scene
- Export: PDF, JSON, markdown

---

## 5. State Management (Pinia)

### 5.1 Stores

**Settings Store:**
- Manages AppSettings
- Syncs with local storage
- Notifies on changes

**Projects Store:**
- Caches project list
- Handles filtering/sorting
- Tracks recent projects

**Analysis Store:**
- Caches analysis results
- Invalidates on project change
- Handles loading states

```typescript
// Example: useSettingsStore()
const settings = useSettingsStore()
settings.setProvider('openai')
settings.setModel('gpt-4o-mini')
await settings.saveSettings()
const isValid = await settings.testConnection()
```

---

## 6. Responsive Design

### 6.1 Breakpoints (TailwindCSS)

| Breakpoint | Width | Use Case |
|-----------|-------|----------|
| sm | 640px | Mobile (reference only) |
| md | 768px | Tablet |
| lg | 1024px | Laptop (default) |
| xl | 1280px | Desktop (wide) |
| 2xl | 1536px | Ultra-wide |

### 6.2 Layout Strategy

**Projects Window:**
- Desktop (lg+): Full list view, metadata visible
- Tablet (md): Condensed list, metadata on click
- Mobile (sm): Not supported in MVP

**Settings Window:**
- Desktop/Tablet: Full form
- Mobile: Not supported in MVP

**Dashboard Window:**
- Desktop (lg+): Sidebar list + main content
- Tablet (md): Toggle between list and content
- Mobile: Not supported in MVP

---

## 7. Performance Requirements

| Metric | Target | Implementation |
|--------|--------|-----------------|
| Startup time | < 2 sec | Lazy-load windows on demand |
| Project discovery | < 500 ms | Parallel directory scanning |
| Scene list (100 items) | 60 fps | TanStack Virtual |
| Analysis display | < 100 ms | Memoization + computed |
| Memory per window | < 150 MB | Lazy components, code splitting |
| CPU idle | < 2% | Event-driven, no polling |

---

## 8. Security Requirements

| Requirement | Implementation |
|-------------|-----------------|
| API key storage | OS keychain or encrypted local storage |
| IPC validation | Type-safe Rust commands, no raw strings |
| Network | TLS for HTTP APIs, Unix sockets for local |
| Secrets | Never log API keys or sensitive data |
| Updates | Signed Tauri updates with version check |

---

## 9. Error Handling

### 9.1 IPC Errors

All Tauri commands return `Result<T, String>`:

```typescript
try {
  const projects = await invoke('discover_projects')
} catch (err: any) {
  // Display user-friendly error
  console.error('Failed to discover projects:', err)
}
```

### 9.2 User-Facing Errors

- Connection failures: "Unable to connect to khaos-tools. Is it running?"
- Invalid settings: "Invalid API key for selected provider"
- Missing data: "No scenes found in project"

---

## 10. Testing Strategy

### 10.1 Unit Tests

- Composables: useIpc, useSettings, useProjects
- Stores: Settings, Projects, Analysis
- Utils: formatters, validators

### 10.2 Integration Tests

- IPC command → khaos-tools response
- Settings persist → reload → verify
- Project discovery → Dashboard open

### 10.3 E2E Tests

- User flow: Open app → Settings → Projects → Dashboard
- Multi-window: Open multiple projects side-by-side
- Error handling: Test with missing khaos-tools daemon

---

## 11. Documentation

- `README.md` — Getting started, architecture overview
- `WINDOWS.md` — Window-specific documentation
- `IPC.md` — Tauri commands reference
- `STORES.md` — Pinia store documentation
- `COMPONENTS.md` — Shadcn/ui components usage
- Developer guides in `docs/` folder

---

## 12. Deployment

- **Platform:** macOS (arm64, x86_64), Windows, Linux
- **Distribution:** DMG (macOS), MSI (Windows), AppImage (Linux)
- **Auto-update:** Tauri updater with version check
- **Signing:** Code-signed on macOS and Windows
- **Bundle Size:** Target < 200 MB (including runtime)

---

## 13. Phase 2 Deliverables

### 2a (Weeks 1-2)
- [ ] Tauri + Nuxt project template
- [ ] Tauri IPC bridge skeleton
- [ ] Projects window (list view)
- [ ] Basic project discovery

### 2b (Week 3)
- [ ] Settings window (provider config)
- [ ] Settings persistence
- [ ] Connection testing

### 2c (Week 4)
- [ ] Dashboard window skeleton
- [ ] Scene list view
- [ ] Analysis display

### 2d (Week 5+)
- [ ] Shadcn/ui integration
- [ ] Design tokens implementation
- [ ] Dark mode support
- [ ] Error handling polish

---

## 14. Glossary

- **KSPD** — Khaos Story Project Document (screenplay format)
- **IPC** — Inter-Process Communication (Tauri commands)
- **TUI** — Terminal User Interface (khaos-tui reference)
- **Daemon** — khaos-tools background service
- **Window** — Independent Tauri window (Projects, Settings, Dashboard)

---

## 15. References

- Tauri Docs: https://tauri.app
- Nuxt Docs: https://nuxt.com
- Shadcn/ui Vue: https://ui.shadcn.com
- TailwindCSS: https://tailwindcss.com
- Pinia: https://pinia.vuejs.org
- khaos-tui: ~/Projects/khaos/khaos-tui (reference implementation)
