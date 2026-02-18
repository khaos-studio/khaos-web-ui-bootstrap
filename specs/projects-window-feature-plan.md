# Feature Plan: Projects Window

**Date**: 2026-02-18
**Status**: Planning
**Phase**: 2b

---

## 1) Context & Goals

### Feature Goal
Enable users to discover, browse, and select KSPD screenwriting projects from configured filesystem roots. Provide search/filter capabilities and quick access to recently opened projects.

### User Outcome
Users can:
- ✅ See all KSPD projects on their system
- ✅ Search/filter projects by title, author, path
- ✅ Quickly access recently opened projects
- ✅ Open a project and navigate to Dashboard for analysis

### Non-Goals (Out of Scope - Phase 2b)
- Project creation/deletion (Phase 2c)
- Project metadata editing (Phase 3)
- Folder watching/auto-refresh (Phase 3)
- Cloud project sync (Phase 4)

### Target Users
- Screenwriters and script analysts using khaos-wfl workflow
- Users with local KSPD project collections (5-500+ projects typical)

### Constraints
- **Performance**: Discover 100+ projects in < 500ms (SSD, no cache variance)
- **UI**: Scene list scrolling smooth for 100+ rows (virtualization required)
- **Compatibility**: macOS/Linux (Windows deferred to Phase 2d)
- **No external dependencies**: Filesystem only (no cloud API calls in v0.1)

### Success Criteria
- [ ] Projects discovered from configured root
- [ ] Metadata displayed: title, author, path, scene count, modified time
- [ ] Search/filter works on title, author, path
- [ ] Recent projects (last 5) displayed prominently
- [ ] Click project → Dashboard opens with project context
- [ ] Performance: Discovery < 500ms for typical (100) projects
- [ ] No crashes on missing/invalid projects

---

## 2) Codebase Recon

### Existing Related Code
- **Frontend**: `windows/projects/app.vue` (placeholder)
- **Backend Commands**: `src-tauri/src/commands/projects.rs` (stubbed: discover_projects, search_projects, get_project)
- **Backend Services**: `src-tauri/src/services/discovery.rs` (stubbed project discovery)
- **Shared Types**: `src-tauri/src/types.rs` (Project DTO defined)
- **Shared Stores**: `shared/stores/` (empty, ready for Pinia implementation)

### Extension Points
- **Command boundary**: All commands already registered in `main.rs`
- **Service layer**: discovery.rs is the right place for filesystem logic
- **Frontend state**: Pinia stores in `windows/projects/stores/`
- **IPC**: Tauri invoke() already wired

### Likely Touched Areas
```
src-tauri/
  src/
    services/
      discovery.rs          ← Project discovery (filesystem scan)
    commands/
      projects.rs           ← Wire to services
windows/projects/
  stores/
    projects.ts            ← Pinia state management
  components/
    ProjectList.vue        ← Main list component
    ProjectSearch.vue      ← Search bar
    ProjectCard.vue        ← Individual project display
  app.vue                  ← Layout & orchestration
shared/
  types/
    index.ts               ← Project type (already defined)
```

### Dependencies & Integrations
- **Config service** (TBD Phase 2b): `KHAOS_PROJECTS_ROOT` env var or config file
- **Tauri fs APIs**: Read directory, get file metadata
- **Recent projects cache**: Persist to OS config directory
- **Event bridge**: Listen for dashboard open event (for recent tracking)

### Unknowns to Verify
- [ ] KSPD project structure/detection logic (filename patterns?)
- [ ] How to read project metadata (.kspdrc? JSON manifest?)
- [ ] Exact path structure for config storage (macOS: `~/Library/Application Support/khaos-ui/`)
- [ ] How to detect "recently opened" (timestamp tracking?)

---

## 3) Risks & Trade-offs

### Breaking Changes
- None (greenfield feature, no legacy code)

### Backwards Compatibility
- Not applicable (v0.1 bootstrap)

### Migration Needs
- None for Phase 2b

### Performance Risks
- **Filesystem scanning**: Large collections (500+ projects) could be slow
  - **Mitigation**: Implement caching with invalidation strategy
  - **Mitigation**: Add progress indicator for initial scan
- **UI rendering**: 100+ projects in dropdown = poor UX
  - **Mitigation**: Virtualization (vue-virtual-scroller or native)
  - **Mitigation**: Search/filter on frontend (already narrowing)

### Testing Complexity
- **File I/O mocking**: Need mock filesystem for unit tests
- **Cross-platform**: Paths differ (macOS/Linux vs Windows)
- **Edge cases**: Symlinks, missing files, permission errors

### Security Implications
- **Path traversal**: Validate project paths within configured root only
- **File permissions**: Handle gracefully (read-only UI, no write)
- **Privacy**: Project paths stored locally; not sent to server

---

## 4) Proposed Design

### High-Level Approach

```
┌─────────────────────────────────────────────────────────┐
│ Projects Window (app.vue)                               │
│  ├─ ProjectSearch.vue (input, filters)                  │
│  ├─ ProjectList.vue (virtualized list)                  │
│  │   ├─ ProjectCard.vue (each item)                     │
│  │   └─ RecentProjects.vue (pinned section)             │
│  └─ Pinia Store (projects, search, selected)            │
└─────────────────────────────────────────────────────────┘
         ↓ invoke('discover_projects')
┌─────────────────────────────────────────────────────────┐
│ Tauri Backend (src-tauri/src/commands/projects.rs)      │
│  ├─ discover_projects() → service                       │
│  ├─ search_projects() → service                         │
│  └─ get_project() → service                             │
└─────────────────────────────────────────────────────────┘
         ↓ calls
┌─────────────────────────────────────────────────────────┐
│ Discovery Service (services/discovery.rs)               │
│  ├─ scan_root() [filesystem]                            │
│  ├─ load_project_metadata() [JSON]                      │
│  ├─ get_recent_projects() [cache file]                  │
│  └─ save_recent_projects() [cache file]                 │
└─────────────────────────────────────────────────────────┘
```

### New Components

**Frontend**:
- `windows/projects/stores/projects.ts` — Pinia store (projects[], search, selected)
- `windows/projects/components/ProjectList.vue` — Virtualized list
- `windows/projects/components/ProjectCard.vue` — Single project display
- `windows/projects/components/ProjectSearch.vue` — Search/filter input

**Backend**:
- `src/services/discovery.rs` — Fully implemented (filesystem scan + metadata)
- Update `src/commands/projects.rs` — Wire commands to service

### Data Flow

```
1. App startup
   → invoke('discover_projects')
   → Backend scans root directory
   → Returns Vec<Project> with metadata
   → Store.projects = results
   → Renders ProjectList with virtualization

2. User types in search
   → Filter store.projects locally
   → Renders matching projects

3. User clicks project
   → store.selected = project
   → invoke('set_active_project', {project_id})
   → Backend updates recent projects cache
   → Emit event to Dashboard window (or open it)
```

### Error Handling
- **Missing root**: Show friendly message, guide to config
- **Permission denied**: Skip project, log warning
- **Invalid JSON**: Skip project, log warning
- **Network error**: Not applicable (filesystem only)

### Observability
- Log project discovery start/end with count
- Log search queries (anonymized)
- Trace slow discovery operations

---

## 5) Incremental Implementation Plan

### Phase 2b-1: Backend Discovery Service (Prep)
**Goal**: Implement filesystem scanning without UI

Tasks:
1. Define KSPD project detection logic (file patterns)
2. Implement `discovery.rs::scan_root()` (recursive scan)
3. Implement `discovery.rs::load_project_metadata()` (read .kspdrc or infer)
4. Implement `discovery.rs::get_recent_projects()` (load from cache)
5. Add unit tests for discovery (mock filesystem)
6. Verify discovery < 500ms for 100 projects

### Phase 2b-2: Backend Commands Wiring
**Goal**: Expose discovery via Tauri commands

Tasks:
1. Update `commands/projects.rs::discover_projects()` to call service
2. Update `commands/projects.rs::search_projects()` (on backend or frontend?)
3. Add config service for `KHAOS_PROJECTS_ROOT`
4. Add error translation (DiscoveryError → user-facing strings)
5. Test each command with mock service

### Phase 2b-3: Frontend State & Store
**Goal**: Pinia store for projects state

Tasks:
1. Create `windows/projects/stores/projects.ts`
   - State: projects[], search, selected, loading
   - Actions: loadProjects(), setSearch(), selectProject()
   - Getters: filteredProjects, recentProjects
2. Connect store to commands (invoke on mount)
3. Add loading/error states
4. Test store logic (unit tests)

### Phase 2b-4: Frontend Components & Layout
**Goal**: Build UI with search, list, recent section

Tasks:
1. Create `ProjectSearch.vue` (input, debounce)
2. Create `ProjectCard.vue` (display title, author, path, modified)
3. Create `ProjectList.vue` (virtualized with vue-virtual-scroller or native)
4. Create `RecentProjects.vue` (pinned section, last 5)
5. Update `app.vue` (layout orchestration)
6. Add Tailwind CSS styling
7. Test components with mock store

### Phase 2b-5: Cross-Window Navigation
**Goal**: Open Dashboard when project selected

Tasks:
1. Implement project selection (click card → store.selected)
2. Emit event or call Tauri command to open Dashboard
3. Pass project context to Dashboard (env var or config)
4. Update recent projects cache on selection
5. Test navigation flow end-to-end

### Phase 2b-6: Testing & Polish
**Goal**: Integration tests, performance, edge cases

Tasks:
1. Write integration tests (backend + mock frontend)
2. Performance profile discovery (measure < 500ms target)
3. Test edge cases: missing root, permission errors, symlinks
4. Error state UI (empty, error, loading)
5. Accessibility audit (keyboard nav, ARIA labels)

---

## 6) Testing Plan

### Unit Tests (Backend)
- `services/discovery.rs`:
  - `test_detect_kspd_project()` — Verify file pattern detection
  - `test_load_metadata()` — JSON parsing, missing fields
  - `test_recent_projects_cache()` — Write/read cache file
  - `test_search_filter()` — Title/author/path matching

### Unit Tests (Frontend)
- `stores/projects.ts`:
  - `test_load_projects_action()` — Mock invoke, populate state
  - `test_search_filter_getter()` — Filter logic
  - `test_select_project_action()` — Update selected
- `components/ProjectSearch.vue`:
  - `test_debounce()` — Search input debouncing
  - `test_filter_emit()` — Emit filter events

### Integration Tests
- `commands/projects.rs`:
  - `test_discover_projects_e2e()` — Invoke → backend → response
  - `test_search_projects_e2e()` — Search invocation

### E2E / Manual Tests
- Discover 100+ projects, verify < 500ms
- Search filters correctly (title, author, path)
- Click project, navigate to Dashboard
- Recent projects section updates
- Handle missing root gracefully
- Handle permission errors gracefully

### Edge Cases
- Empty project collection (show "no projects found")
- Very large collection (500+) — performance profile
- Symlinks to projects — should follow or skip?
- Nested projects — should find all or stop at first level?
- Invalid/corrupted metadata — skip with warning
- No read permissions — skip with warning

---

## 7) Assumptions & Unknowns

### Assumptions
- KSPD projects have standard file structure/markers (to be verified)
- Project metadata is readable from filesystem (not encrypted)
- `KHAOS_PROJECTS_ROOT` can be configured via env var or config file
- Recent projects stored locally (no cloud sync in v0.1)
- Single-selection UX (not bulk operations)

### Unknowns (RESOLVED via khaos-tui reference)

✅ **KSPD project detection**:
- Directory name ends with `.kspd` extension, OR
- Directory contains `manifest.json` file

✅ **Project metadata**:
- Title: From `manifest.json["title"]`, fallback to directory name
- Author: Optional field in `manifest.json["author"]`
- Scene count: From `metadata/scenes.json` (array or {scenes: [...]})
- Modified: Directory mtime

✅ **Config storage**:
- File: `~/.config/khaos-ui/config.json` (XDG standard)
- Field: `projects_root: "/path/to/projects"`
- Fallback: `KHAOS_PROJECTS_ROOT` env var, then `$HOME/Projects`

✅ **Cross-window communication**:
- Use Tauri app event: `app:project-selected`
- Pass project ID in event payload
- Dashboard listens and updates its store

---

## 8) Next Up (Top Priority)

1. **Verify KSPD project structure** — Ask user or check khaos-wfl docs
2. **Implement discovery.rs** — Filesystem scanning + metadata loading
3. **Create Pinia store** — State management for projects
4. **Build ProjectList & ProjectCard** — UI components
5. **Wire commands** — Connect frontend to backend
6. **Test end-to-end** — Verify flow and performance

---

## Blockers & Questions

**Q1: What is the KSPD project structure?**
- How do we detect a KSPD project? Filename? Directory pattern? Marker file?
- Where is metadata stored (title, author, scene count)?

**Q2: How should we read project metadata efficiently?**
- Is it in a JSON file? Filename? Database?
- Should we cache it or scan fresh each time?

**Q3: How to pass project context to Dashboard?**
- Use Tauri app event?
- Write to shared config file?
- Use window URL params?

**Q4: Should search happen on frontend or backend?**
- Frontend: Faster, keep it local, all projects in memory
- Backend: Can index/optimize, but network overhead

---

## Files to Create/Modify

### Create (New)
- `windows/projects/stores/projects.ts`
- `windows/projects/components/ProjectList.vue`
- `windows/projects/components/ProjectCard.vue`
- `windows/projects/components/ProjectSearch.vue`
- `windows/projects/components/RecentProjects.vue`

### Modify (Existing)
- `windows/projects/app.vue`
- `src-tauri/src/services/discovery.rs`
- `src-tauri/src/commands/projects.rs`
- `src-tauri/src/services/mod.rs` (if needed)

### Documentation
- `docs/PROJECTS_WINDOW.md` — Feature documentation

---

## Success Metrics

- [ ] All 100+ projects discovered in < 500ms
- [ ] Search returns results in < 100ms
- [ ] UI renders 100 projects smoothly (no jank)
- [ ] All tests pass (unit + integration)
- [ ] No crashes on edge cases
- [ ] Users can navigate to Dashboard from project selection

