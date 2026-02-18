# khaos-web-ui Product Requirements Document

**Version:** 1.0  
**Date:** Feb 17, 2026  
**Status:** Phase 2 Bootstrap

---

## 1. Vision & Mission

**Vision:** Native desktop UI for Khaos storytelling platform that provides intuitive, focused windows for project discovery, configuration, and screenplay analysis.

**Mission:** Enable storytellers to interact with khaos-tools through a native desktop app that feels responsive, modern, and purpose-built for creative workflows.

---

## 2. Product Overview

khaos-web-ui is a Tauri-based desktop application that spawns independent Vue windows for specific tasks:

1. **Projects Window** — Discover, browse, and select KSPD projects
2. **Settings Window** — Configure LLM providers, models, API keys
3. **Dashboard Window** — View project details, scene lists, analysis results

Each window is a self-contained Nuxt app that communicates with khaos-tools daemon via IPC.

---

## 3. User Personas

### Screenwriter (Primary)
- 35-50 years old, uses screenwriting software daily
- Wants to analyze screenplay structure without leaving desktop
- Expects native app UX, responsive interaction
- Needs fast project switching and quick access to analysis

### Indie Filmmaker (Secondary)
- 25-40 years old, wears multiple hats (writer, director, producer)
- Manages multiple projects, collaborating with teams
- Needs to share analysis reports with production team
- Values speed and clean UI

### Storyteller/Author (Tertiary)
- Any age, writes novels or interactive fiction
- Curious about narrative structure tools
- Wants lightweight, non-intrusive UI

---

## 4. Feature Specifications

### Window 1: Projects

**Purpose:** Discover, browse, and manage KSPD projects

**Key Features:**
- Display list of available KSPD projects (local + git repos)
- Show project metadata (title, author, scene count, characters)
- Filter/search projects
- Open project → spawns Dashboard window
- Create new project (link to KSPD workflow)
- Recent projects quick-access

**User Flow:**
1. Open khaos-web-ui → Projects window displays
2. Browse project list (virtualized if 50+ projects)
3. Click project → opens in Dashboard window
4. Can open multiple projects in separate Dashboard windows

**Success Criteria:**
- Load 100+ projects without lag (TanStack Virtual)
- Project discovery < 500ms
- Responsive on desktop/tablet

---

### Window 2: Settings

**Purpose:** Configure LLM providers and authentication

**Key Features:**
- Provider selection (Ollama, OpenAI, Anthropic, local)
- Model configuration per provider
- API key management (secure storage)
- Test connection button
- Ollama status check (if applicable)
- Dark/light mode toggle
- About / version info

**User Flow:**
1. Open khaos-web-ui Settings
2. Select provider from dropdown
3. Enter model name or API key
4. Click "Test Connection" to validate
5. Changes saved automatically
6. Dashboard uses these settings for analysis

**Success Criteria:**
- Settings persist across app restarts
- Connection test completes < 2 seconds
- Clear error messages if connection fails
- API keys stored securely (OS keychain if available)

---

### Window 3: Dashboard

**Purpose:** Display project details and analysis results

**Key Features:**
- Project title and metadata (scene count, character count)
- Scene list view (virtualized, sortable)
- Click scene → detail panel (side or modal)
- Analysis results view (by scene, character, location)
- Export/share options (report, JSON)
- Quick actions (analyze all, detect characters, etc.)
- Back to Projects button

**User Flow:**
1. Click project in Projects window
2. Dashboard opens showing project overview
3. Browse scene list
4. Select scene → view details + analysis
5. Trigger analysis → results populate
6. Export report if needed

**Success Criteria:**
- Scene list renders 50+ scenes smoothly (virtualization)
- Analysis results display within 100ms
- Export completes < 5 seconds

---

## 5. User Workflows

### First-Time User
1. Open khaos-web-ui
2. See Settings prompt (no provider configured)
3. Configure provider + test connection
4. Return to Projects
5. Create new project or browse existing

### Returning User
1. Open khaos-web-ui
2. Projects window shows recent projects
3. Click project → Dashboard
4. Analyze screenplay

### Power User (Multiple Projects)
1. Open Projects
2. Open Project A → Dashboard A
3. Open Project B → Dashboard B (new window)
4. Flip between windows via alt-tab
5. Compare analysis side-by-side

---

## 6. Design System

- **Components:** Shadcn/ui (Vue port)
- **Styling:** TailwindCSS + custom design tokens
- **Color Palette:** TBD (design phase)
- **Icons:** Lucide-vue or similar
- **Typography:** System font stack

---

## 7. Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| **Startup Time** | < 2 seconds |
| **Scene List (100 items)** | 60fps scroll, no jank |
| **Project Load** | < 500ms |
| **Analysis Display** | < 100ms render |
| **Memory per Window** | < 150MB |
| **CPU Idle** | < 2% |

---

## 8. Out of Scope (Phase 2)

- Mobile support (desktop only for MVP)
- Collaborative editing (Phase 3+)
- Advanced visualization (graphs, charts) — Phase 3+
- Browser-based version — Phase 3+
- Custom keyboard shortcuts — Phase 3+
- Plugin system — Phase 4+

---

## 9. Success Metrics

- App startup time < 2 seconds
- Scene list (100+ items) renders smoothly
- 0 crashes in first week of use
- Settings persist correctly across sessions
- Analysis results display within expected timeframes
- User can open/switch projects without friction

---

## 10. Timeline

- **Phase 2a (1-2 weeks):** Tauri template + Projects window
- **Phase 2b (1 week):** Settings window
- **Phase 2c (1 week):** Dashboard window
- **Phase 3 (2-3 weeks):** MVP polish + internal testing
- **Phase 4:** Documentation + team training

---

## 11. Related Documents

- `SRS.md` — Technical requirements and architecture
- `PHASE1_DECISIONS_LOCKED.md` — Framework/component decisions
- `khaos-tui` — Reference implementation (TUI pattern)
