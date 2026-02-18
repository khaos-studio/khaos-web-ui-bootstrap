# Feature: Dashboard Window

## Feature Description
Implement the Dashboard window for khaos-web-ui-bootstrap, porting the khaos-tui dashboard experience to a web UI. The Dashboard is the primary workspace where users interact with an opened KSPD project — browsing scenes, characters, and locations, viewing detailed analysis results, and triggering AI-powered analysis. It features a three-section navigation (Scenes/Characters/Locations), a scrollable content list with per-item state indicators, a detail panel for viewing rich analysis markdown, and real-time progress tracking for analysis operations.

## User Story
As a screenwriter or script analyst
I want to open a project and browse its scenes, characters, and locations with AI analysis
So that I can understand narrative structure, character arcs, and location significance

## Problem Statement
The Dashboard window is currently a placeholder. After selecting a project in the Projects window, users have no way to:
- View parsed scenes, characters, and locations from their KSPD project
- Trigger AI analysis on individual items or entire sections
- View rich analysis results with narrative insights
- Track analysis progress in real time
- Navigate between scenes, characters, and locations

The khaos-tui provides a proven, feature-rich dashboard that serves as the behavioral reference.

## Solution Statement
Port the khaos-tui dashboard architecture to Vue 3 + Tauri, following the patterns established in the Projects and Settings windows:
- **Rust services layer** for CLI adapter integration (khaos-tools queries and analysis)
- **Tauri commands** bridging frontend to backend with async operations and event streaming
- **Pinia store** managing dashboard state (sections, items, analysis state maps, detail view)
- **Vue components** rendering the three-section layout with state badges, detail panel, and progress indicators

## Relevant Files
Use these files to implement the feature:

### Existing Files to Modify
- `src-tauri/src/commands/dashboard.rs` — Currently stubbed, needs full implementation
- `src-tauri/src/types.rs` — Add dashboard domain types (SceneSummary, SceneAnalysis, etc.)
- `src-tauri/src/events.rs` — Add analysis progress/completion events
- `src-tauri/src/main.rs` — Register new dashboard commands
- `src-tauri/src/commands/mod.rs` — Ensure dashboard module is registered
- `src-tauri/src/services/mod.rs` — Register dashboard service module
- `shared/types/index.ts` — Add dashboard TypeScript types
- `windows/dashboard/app.vue` — Replace placeholder with full dashboard
- `windows/dashboard/nuxt.config.ts` — Verify Pinia and shared styles configured
- `windows/dashboard/package.json` — Add @tauri-apps/api dependency
- `src-tauri/src/commands/windows.rs` — Dashboard window already handled here

### New Files
- `src-tauri/src/services/dashboard.rs` — Dashboard service layer (CLI adapter calls)
- `windows/dashboard/stores/dashboard.ts` — Pinia store for dashboard state
- `windows/dashboard/components/DashboardHeader.vue` — Project header with KPIs
- `windows/dashboard/components/SectionNav.vue` — Left sidebar navigation (Scenes/Characters/Locations)
- `windows/dashboard/components/ItemList.vue` — Scrollable list with state badges
- `windows/dashboard/components/DetailPanel.vue` — Rich analysis detail view
- `windows/dashboard/components/StateBadge.vue` — Per-item analysis state indicator
- `windows/dashboard/components/AnalysisContent.vue` — Rendered markdown analysis

### Reference Files (khaos-tui)
- `~/Spikes/khaos-tui/code/internal/domain/models.go` — Domain structs (SceneSummary, SceneAnalysis, etc.)
- `~/Spikes/khaos-tui/code/internal/cli/adapter.go` — CLI adapter method signatures
- `~/Spikes/khaos-tui/code/internal/dashboard/model.go` — Dashboard state machine
- `~/Spikes/khaos-tui/code/internal/dashboard/view.go` — Dashboard rendering

## Implementation Plan

### Phase 1: Foundation (Rust Types + Service Layer)

Add all dashboard domain types to `types.rs` and create the `services/dashboard.rs` module that wraps khaos-tools CLI calls. The service must handle:
- Querying project summary, scenes, characters, locations via `khaos-tools parser query`
- Querying analysis results via `khaos-tools bot query`
- Triggering analysis via `khaos-tools bot analyze`
- Scanning analysis index directory for existing results
- Streaming progress events during analysis

### Phase 2: Tauri Commands

Replace the 4 stubbed dashboard commands with real implementations and add new commands for the full workflow:
- `get_project_summary` — Load project KPIs
- `get_scenes` — Load scene list
- `get_characters` — Load character list
- `get_locations` — Load location list
- `get_scene_detail` — Load scene + analysis for detail view
- `get_character_detail` — Load character + analysis
- `get_location_detail` — Load location + analysis
- `analyze_scene` — Trigger single scene analysis
- `analyze_all_scenes` — Trigger all-scenes analysis
- `analyze_character` — Trigger single character analysis
- `analyze_all_characters` — Trigger all-characters analysis
- `analyze_location` — Trigger single location analysis
- `analyze_all_locations` — Trigger all-locations analysis
- `scan_analysis_index` — Scan filesystem for existing analysis results

### Phase 3: Frontend Store + Types

Create the Pinia store managing:
- **Section state**: current section (scenes/characters/locations), selected item
- **Data**: scenes[], characters[], locations[], projectSummary
- **Analysis state maps**: per-item state (pending/analyzing/analyzed/failed), errors, progress
- **Detail view**: open/closed, loaded metadata, loaded analysis, related items
- **Loading states**: per-section loading flags

### Phase 4: UI Components

Build the dashboard layout matching the TUI structure:
```
┌──────────────────────────────────────────────────┐
│ DashboardHeader: project title, KPIs, status     │
├────────────┬─────────────────────────────────────┤
│ SectionNav │ ItemList OR DetailPanel              │
│ · Scenes   │                                      │
│ · Chars    │ [StateBadge] Scene 1 - INT. CAFE    │
│ · Locs     │ [StateBadge] Scene 2 - EXT. PARK    │
│            │ ...                                  │
├────────────┴─────────────────────────────────────┤
│ Footer: actions (Analyze, Analyze All, Back)     │
└──────────────────────────────────────────────────┘
```

## Step by Step Tasks
IMPORTANT: Execute every step in order, top to bottom.

### 1. Add Dashboard Domain Types to Rust
- Add to `src-tauri/src/types.rs`:
  - `SceneSummary { id, index, slugline, duration, word_count, line_count, characters: Vec<String> }`
  - `CharacterSummary { id, name, dialogue_lines, words, scene_count, percentage: f64 }`
  - `LocationSummary { id, name, scene_count, page_count }`
  - `ProjectSummary { scenes, characters, locations, compositions }`
  - `GraphSummary { total_entities, total_relationships }`
  - `SceneAnalysis` — full 30+ field struct matching khaos-tui domain
  - `CharacterAnalysis` — full struct matching khaos-tui domain
  - `LocationAnalysis` — full struct matching khaos-tui domain
  - `AnalysisState` enum: `Pending`, `Analyzing`, `Analyzed`, `Failed`
  - `ItemWithState<T>` — wraps any summary with its analysis state
  - `DashboardData` — combined response with summary + all lists
  - `AnalysisResult` — wraps analysis with success/error info
- All types must derive `Debug, Clone, Serialize, Deserialize` with `#[serde(crate = "serde")]`
- Run `cargo check`

### 2. Create Dashboard Service Layer
- Create `src-tauri/src/services/dashboard.rs`
- Implement CLI adapter wrapper functions using `tokio::process::Command` to call `khaos-tools`:
  - `query_project_summary(kspd_path) -> ProjectSummary` via `khaos-tools parser query --format json summary`
  - `query_scenes(kspd_path) -> Vec<SceneSummary>` via `khaos-tools parser query --format json scenes`
  - `query_characters(kspd_path) -> Vec<CharacterSummary>` via `khaos-tools parser query --format json characters`
  - `query_locations(kspd_path) -> Vec<LocationSummary>` via `khaos-tools parser query --format json locations`
  - `query_scene_analysis(kspd_path, scene_id) -> Option<SceneAnalysis>` via `khaos-tools bot query --format json scene <id>`
  - `query_character_analysis(kspd_path, character_id) -> Option<CharacterAnalysis>`
  - `query_location_analysis(kspd_path, location_id) -> Option<LocationAnalysis>`
  - `analyze_scene(app_handle, kspd_path, scene_id)` — run analysis, emit progress events
  - `analyze_all_scenes(app_handle, kspd_path)` — batch analyze, emit progress events
  - `analyze_character(app_handle, kspd_path, character_id)`
  - `analyze_all_characters(app_handle, kspd_path)`
  - `analyze_location(app_handle, kspd_path, location_id)`
  - `analyze_all_locations(app_handle, kspd_path)`
  - `scan_analysis_index(kspd_path) -> AnalysisIndex` — scan `metadata/analysis/` dir
- Use the same `find_khaos_tools()` pattern from `services/import.rs`
- Register `pub mod dashboard;` in `services/mod.rs`
- Run `cargo check`

### 3. Add Analysis Events
- Add to `src-tauri/src/events.rs`:
  - `ANALYSIS_SCENE_PROGRESS` event constant
  - `ANALYSIS_SCENE_COMPLETED` event constant
  - `ANALYSIS_ALL_COMPLETED` event constant
  - `AnalysisProgressEvent { item_type, item_id, progress, status }` payload struct
  - `AnalysisCompletedEvent { item_type, item_id, success, error }` payload struct
- Run `cargo check`

### 4. Implement Dashboard Commands
- Replace stubs in `src-tauri/src/commands/dashboard.rs`:
  - `get_scenes(project_path) -> Vec<SceneSummary>`
  - `get_characters(project_path) -> Vec<CharacterSummary>`
  - `get_locations(project_path) -> Vec<LocationSummary>`
  - `get_project_summary(project_path) -> ProjectSummary`
  - `get_scene_detail(project_path, scene_id) -> SceneDetail` (summary + analysis)
  - `get_character_detail(project_path, character_id) -> CharacterDetail`
  - `get_location_detail(project_path, location_id) -> LocationDetail`
  - `analyze_scene(app, project_path, scene_id) -> AnalysisResult`
  - `analyze_all(app, project_path, section) -> AnalysisResult` (replace existing stub)
  - `analyze_character(app, project_path, character_id) -> AnalysisResult`
  - `analyze_location(app, project_path, location_id) -> AnalysisResult`
  - `scan_analysis_index(project_path) -> AnalysisIndex`
- Register new commands in `main.rs` invoke_handler
- Run `cargo check` and `cargo test`

### 5. Add Shared TypeScript Types
- Add to `shared/types/index.ts`:
  - `SceneSummary`, `CharacterSummary`, `LocationSummary`
  - `ProjectSummary`, `GraphSummary`
  - `SceneAnalysis`, `CharacterAnalysis`, `LocationAnalysis`
  - `AnalysisState` type: `'pending' | 'analyzing' | 'analyzed' | 'failed'`
  - `DashboardSection` type: `'scenes' | 'characters' | 'locations'`
  - `AnalysisResult` interface
- Ensure types match Rust backend exactly (snake_case field names)

### 6. Create Dashboard Pinia Store
- Create `windows/dashboard/stores/dashboard.ts`
- State:
  - `projectPath: string`
  - `summary: ProjectSummary | null`
  - `currentSection: DashboardSection` (default 'scenes')
  - `scenes: SceneSummary[]`, `characters: CharacterSummary[]`, `locations: LocationSummary[]`
  - `analysisStates: Record<string, AnalysisState>` (keyed by item ID)
  - `analysisErrors: Record<string, string>`
  - `selectedId: string | null`
  - `detailOpen: boolean`
  - `detailData: { summary, analysis, relatedScenes } | null`
  - `loading: boolean`, `error: string | null`
  - `analyzing: boolean` (global "analyze all" in progress)
- Actions:
  - `loadProject(path)` — load summary + all section lists + analysis index
  - `switchSection(section)` — change current section
  - `selectItem(id)` — open detail panel, load detail + analysis
  - `closeDetail()` — close detail panel
  - `analyzeItem(id)` — trigger analysis for selected item
  - `analyzeAll()` — trigger analyze-all for current section
  - `refreshAnalysisStates()` — re-scan analysis index
- Getters:
  - `currentItems` — scenes/characters/locations based on currentSection
  - `currentItemsWithState` — items enriched with analysis state
  - `selectedItem` — currently selected item

### 7. Create SectionNav Component
- Create `windows/dashboard/components/SectionNav.vue`
- Three navigation items: Scenes, Characters, Locations
- Show item counts from store
- Highlight active section with blue background
- Emit `@select` event on click

### 8. Create StateBadge Component
- Create `windows/dashboard/components/StateBadge.vue`
- Render colored dot + label based on analysis state:
  - pending: gray dot
  - analyzing: amber dot + spinner
  - analyzed: green dot
  - failed: red dot
- Accept `state: AnalysisState` prop

### 9. Create ItemList Component
- Create `windows/dashboard/components/ItemList.vue`
- Render scrollable list of items for current section
- Each row: StateBadge + item title (slugline for scenes, name for chars/locs)
- Show secondary info (word count, scene count, etc.)
- Highlight selected item
- Emit `@select` on click, `@analyze` on analyze button

### 10. Create DetailPanel Component
- Create `windows/dashboard/components/DetailPanel.vue`
- Show when `detailOpen` is true
- Header with item title and close button
- Metadata section (scene number, duration, word count, etc.)
- Analysis content section with rendered markdown
- "Analyze" button if no analysis exists
- Loading spinner while analysis loads
- Back button to return to list

### 11. Create AnalysisContent Component
- Create `windows/dashboard/components/AnalysisContent.vue`
- Renders analysis data as structured HTML sections
- For scenes: Summary, Narrative, Structure, Characters, Themes, Evaluation
- For characters: Arc, Profile, Dialogue, Relationships, Themes
- For locations: Atmosphere, Significance, Symbols, Themes
- Use markdown rendering for text fields

### 12. Create DashboardHeader Component
- Create `windows/dashboard/components/DashboardHeader.vue`
- Project title and path
- KPI badges: scene count, character count, location count
- Analysis progress summary (X analyzed / Y total)
- "Analyze All" button for current section

### 13. Assemble Dashboard app.vue
- Replace placeholder in `windows/dashboard/app.vue`
- Layout: header + (sidebar nav | content area) + footer
- Content area: ItemList (default) or DetailPanel (when item selected)
- Footer: keyboard shortcut hints, action buttons
- On mount: listen for `app:project-selected` event to load project
- Keyboard shortcuts: arrow keys for navigation, Enter for detail, Esc to close, a to analyze

### 14. Wire Up Event Listeners
- Listen for `analysis:progress` events to update progress in store
- Listen for `analysis:completed` events to refresh analysis states
- Listen for `app:project-selected` events to load new project data
- Clean up listeners on unmount

### 15. Validate Everything
- Run `cd src-tauri && cargo check` — verify Rust compilation
- Run `cd src-tauri && cargo test` — run all Rust tests
- Run `pnpm run build` — verify all three windows build
- Verify dashboard window opens from projects window

## Testing Strategy

### Unit Tests
- Dashboard service: mock khaos-tools CLI output, verify parsing
- Pinia store: test all actions, getters, state transitions
- Components: test rendering with various states (loading, empty, populated, error)

### Integration Tests
- Command flow: invoke commands with test fixtures, verify responses
- Event flow: trigger analysis, verify progress events received
- Cross-window: select project in Projects window, verify Dashboard loads

### Edge Cases
- Project with no scenes (empty state)
- Analysis fails mid-batch (partial results)
- khaos-tools binary not found
- Corrupted analysis JSON files
- Very large projects (100+ scenes) — virtual scrolling
- Switching projects while analysis is running

## Acceptance Criteria
1. Dashboard opens when project is selected from Projects window
2. Three sections (Scenes, Characters, Locations) are navigable
3. Items show colored state badges (pending/analyzing/analyzed/failed)
4. Clicking an item opens a detail panel with metadata and analysis
5. "Analyze" button triggers AI analysis for individual items
6. "Analyze All" triggers batch analysis for current section
7. Analysis progress is shown in real-time via events
8. Analysis results render as structured content with proper formatting
9. Project KPIs (counts, analysis progress) display in header
10. Keyboard navigation works (arrows, Enter, Esc)
11. All existing tests continue to pass (zero regressions)
12. Frontend and backend compile without errors

## Validation Commands
Execute every command to validate the feature works correctly with zero regressions.

- `cd src-tauri && cargo check` - Verify Rust compilation
- `cd src-tauri && cargo test` - Run Rust tests
- `cd src-tauri && cargo clippy -- -D warnings` - Lint Rust code
- `pnpm run build` - Verify all three windows build
- `pnpm run test` - Run frontend tests (if configured)

## Notes

**khaos-tools CLI Commands Reference:**
```bash
# Query commands (read parsed data)
khaos-tools parser query --format json summary <kspd-path>
khaos-tools parser query --format json scenes <kspd-path>
khaos-tools parser query --format json characters <kspd-path>
khaos-tools parser query --format json locations <kspd-path>

# Bot query commands (read analysis results)
khaos-tools bot query --format json scene <kspd-path> <scene-id>
khaos-tools bot query --format json character <kspd-path> <character-id>
khaos-tools bot query --format json location <kspd-path> <location-id>

# Analysis commands (trigger AI analysis)
khaos-tools bot analyze scene <kspd-path> <scene-id>
khaos-tools bot analyze scenes <kspd-path>
khaos-tools bot analyze character <kspd-path> <character-id>
khaos-tools bot analyze characters <kspd-path>
khaos-tools bot analyze location <kspd-path> <location-id>
khaos-tools bot analyze locations <kspd-path>

# Graph ingestion
khaos-tools graph ingest <kspd-path>
```

**Analysis State Machine:**
- `pending` → item exists but no analysis found on disk
- `analyzing` → analysis command currently running
- `analyzed` → analysis JSON exists in `metadata/analysis/` directory
- `failed` → analysis attempt returned error

**Provider/Model Configuration:**
The dashboard should respect the settings window configuration. khaos-tools reads `--provider` and `--model` flags, or falls back to config. The Rust service should pass the configured provider/model from settings when invoking analysis commands.

**Performance Considerations:**
- Load analysis index first (filesystem scan is fast) to show states immediately
- Lazy-load analysis details only when detail panel opens
- Use virtual scrolling for projects with 100+ scenes
- Batch analysis state refreshes (12 items per tick, matching TUI pattern)

**Cross-Window Communication:**
- Projects window emits `app:project-selected` event with `{ project_id, project_title }`
- Dashboard listens for this event and loads the corresponding project
- The project path can be resolved via `get_project` command using the project_id
