# Chore: Replicate TUI Project Creation Flow in Projects Window

## Chore Description
Implement the full project creation (import) flow in the Projects window, replicating the multi-step wizard from `khaos-tui`. The TUI flow at `~/Spikes/khaos-tui/code/internal/views/import/` guides users through: (1) selecting a screenplay file via native file picker, (2) entering a project title with validation, (3) confirming the destination or resolving name collisions, (4) executing the `khaos-tools parser parse` command with streamed log output, and (5) displaying the result. The web UI currently has a placeholder `+ Create` button (`app.vue:139`) and stub Rust import commands (`commands/import.rs`) that need to be fully implemented.

## Relevant Files
Use these files to resolve the chore:

### Existing Files to Modify

- **`src-tauri/src/commands/import.rs`** — Stub import commands (`start_parse`, `get_parse_progress`, `cancel_parse`). Must be replaced with real implementations that validate inputs, resolve target paths, detect collisions, invoke `khaos-tools parser parse`, stream progress via Tauri events, and handle cancellation.
- **`src-tauri/src/types.rs`** — Shared Rust types. Needs new types: `ImportRequest`, `ImportProgress`, `ImportResult`, `CollisionInfo`, and title normalization/validation functions ported from TUI's `domain/models.go:244-273`.
- **`src-tauri/src/events.rs`** — Event bridge. Already has `DAEMON_PARSER_PROGRESS` and `DAEMON_PARSER_COMPLETED` events plus `ParserProgressEvent`/`ParserCompletedEvent` payloads — these will be emitted during import execution.
- **`src-tauri/src/services/discovery.rs`** — Has `is_kspd()` (line 68) and `get_projects_root()` (line 184). These are reused for collision detection and target path resolution.
- **`src-tauri/src/main.rs`** — Already registers `commands::import::*` handlers (lines 36-38). May need new commands added (e.g., `validate_import_file`, `check_collision`).
- **`src-tauri/Cargo.toml`** — Needs `tauri-plugin-dialog` dependency for native file picker.
- **`src-tauri/tauri.conf.json`** — Needs `dialog` plugin registered in Tauri config.
- **`windows/projects/app.vue`** — Root layout. The `handleCreate` placeholder (line 139) must toggle the import wizard overlay. Needs import wizard component integration.
- **`windows/projects/stores/projects.ts`** — Pinia store. Needs import state management: wizard step tracking, file/title state, progress logs, and actions that invoke new Tauri commands.
- **`windows/projects/package.json`** — Needs `@tauri-apps/plugin-dialog` dependency for frontend file dialog API.
- **`shared/types/index.ts`** — Shared TS types. Needs `ImportState`, `ImportStep`, `ImportProgress`, `CollisionInfo` interfaces to match new Rust types.

### New Files

- **`windows/projects/components/ImportWizard.vue`** — Main import wizard component. Multi-step overlay dialog with step indicator, renders the active step component, handles navigation between steps.
- **`windows/projects/components/import/StepFilePicker.vue`** — Step 1: File selection. Native file picker button (via `@tauri-apps/plugin-dialog`), manual path input, file extension validation (.fountain, .fdx, .sbx, .md), error display.
- **`windows/projects/components/import/StepTitle.vue`** — Step 2: Title input. Text input with validation (non-empty, max 255 chars), shows selected file path, error display.
- **`windows/projects/components/import/StepConfirm.vue`** — Step 3a: Confirm destination. Shows summary (title, output path), Yes/No buttons.
- **`windows/projects/components/import/StepCollision.vue`** — Step 3b: Collision resolution. Radio options: overwrite, rename suggestions (up to 5), go back. Shown only when target path already exists.
- **`windows/projects/components/import/StepExecute.vue`** — Step 4: Import execution. Spinner, streaming log viewport, cancel button. Listens to Tauri parser events.
- **`windows/projects/components/import/StepResult.vue`** — Step 5: Result display. Success/failure banner, output path, error details if failed, "Done" button to close wizard and refresh project list.
- **`windows/projects/stores/import.ts`** — Dedicated Pinia store for import wizard state. Manages: current step, file path, title, output path, collision info, progress logs, import status. Keeps `projects.ts` focused on project list concerns.
- **`src-tauri/src/services/import.rs`** — Import service layer. Contains: `validate_import_file()`, `normalize_title()`, `validate_title()`, `resolve_target_path()`, `check_collision()`, `generate_suggested_names()`, `execute_parse()` (spawns `khaos-tools` child process and streams output via Tauri events).
- **`windows/projects/__tests__/stores/import.test.ts`** — Unit tests for the import store.
- **`windows/projects/__tests__/components/ImportWizard.test.ts`** — Unit tests for the import wizard component.

## Step by Step Tasks
IMPORTANT: Execute every step in order, top to bottom.

### Step 1: Add Tauri Dialog Plugin Dependencies

- Add `tauri-plugin-dialog = "2.0"` to `src-tauri/Cargo.toml` under `[dependencies]`
- Add `@tauri-apps/plugin-dialog` to `windows/projects/package.json` dependencies
- Register the dialog plugin in `src-tauri/src/main.rs` by adding `.plugin(tauri_plugin_dialog::init())` to the builder chain before `.build()`
- Add `"dialog:default"` to Tauri capabilities/permissions if needed (check `src-tauri/capabilities/` or `tauri.conf.json`)
- Run `pnpm install` from root and `cargo check` from `src-tauri/` to verify dependencies resolve

### Step 2: Add Shared Types for Import Flow

- In `shared/types/index.ts`, add these interfaces:
  ```typescript
  export type ImportStep = 'file' | 'title' | 'confirm' | 'collision' | 'execute' | 'result'
  export type ImportStatus = 'idle' | 'in_progress' | 'success' | 'failed'

  export interface CollisionInfo {
    existing_path: string
    suggested_names: string[]
  }

  export interface ImportProgress {
    phase: string
    line: string
  }

  export interface ImportResult {
    success: boolean
    project_id?: string
    output_path?: string
    error?: string
  }
  ```
- In `src-tauri/src/types.rs`, add matching Rust structs:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct CollisionInfo {
      pub existing_path: String,
      pub suggested_names: Vec<String>,
  }

  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct ImportResult {
      pub success: bool,
      pub project_id: Option<String>,
      pub output_path: Option<String>,
      pub error: Option<String>,
  }
  ```

### Step 3: Implement Rust Import Service Layer

- Create `src-tauri/src/services/import.rs` with the following functions ported from TUI logic:
  - `validate_title(title: &str) -> Result<(), String>` — Non-empty, max 255 chars (from TUI `domain/models.go:244-253`)
  - `normalize_project_filename(title: &str) -> String` — Replace `<>:"/\|?*` and whitespace with `_`, trim, fallback to "project" (from TUI `domain/models.go:255-273`)
  - `resolve_target_path(projects_dir: &str, title: &str) -> Result<String, String>` — Normalizes title, appends `.kspd`, joins with projects_dir (from TUI `cli/adapter.go:ResolveTargetPath`)
  - `validate_import_file(path: &str) -> Result<(), String>` — Check file exists, not a directory, extension is .fountain/.fdx/.sbx/.md (from TUI `import/model.go:261-290`)
  - `check_collision(target_path: &str) -> bool` — Reuse `discovery::is_kspd()` on target path
  - `generate_suggested_names(projects_dir: &str, title: &str, max: usize) -> Vec<String>` — Append `_1`, `_2`, etc. up to `max`, skip names that already exist (from TUI `import/model.go:611-623`)
  - `execute_parse(app_handle: AppHandle, input_path: &str, output_path: &str) -> Result<ImportResult, String>` — Locate `khaos-tools` binary (via PATH or config), spawn `khaos-tools parser parse --output {output_path} {input_path}`, stream stdout/stderr lines as `daemon:parser-progress` Tauri events, emit `daemon:parser-completed` on finish, return `ImportResult`
- Register the module in `src-tauri/src/services/mod.rs`: add `pub mod import;`

### Step 4: Implement Rust Import Commands

- Replace the stub implementations in `src-tauri/src/commands/import.rs` with real logic:
  - `validate_import_file(file_path: String) -> Result<(), String>` — NEW command. Calls `services::import::validate_import_file()`
  - `check_import_collision(title: String) -> Result<Option<CollisionInfo>, String>` — NEW command. Resolves target path via `services::import::resolve_target_path()`, checks collision, returns `CollisionInfo` with suggested names if collision exists, `None` otherwise
  - `start_parse(app_handle: AppHandle, file_path: String, title: String, output_path: String, overwrite: bool) -> Result<String, String>` — REPLACE existing stub. Validates inputs, spawns async task that calls `services::import::execute_parse()`, returns a request ID (UUID). Emits progress events.
  - `get_parse_progress(request_id: String) -> Result<serde_json::Value, String>` — Keep or remove; progress is now event-driven
  - `cancel_parse(request_id: String) -> Result<(), String>` — Implement cancellation by killing the spawned child process
- Register new commands in `src-tauri/src/main.rs` `generate_handler![]`: add `commands::import::validate_import_file` and `commands::import::check_import_collision`

### Step 5: Create Import Pinia Store

- Create `windows/projects/stores/import.ts` with:
  - **State**: `step: ImportStep` (default `'file'`), `filePath: string`, `title: string`, `outputPath: string`, `collision: CollisionInfo | null`, `logs: string[]`, `status: ImportStatus`, `error: string | null`, `requestId: string | null`, `isOpen: boolean`
  - **Actions**:
    - `openWizard()` — Reset all state, set `isOpen = true`, set `step = 'file'`
    - `closeWizard()` — Set `isOpen = false`, cancel any in-progress parse
    - `setFile(path: string)` — Validate via `invoke('validate_import_file')`, set `filePath`, advance to `'title'` step
    - `setTitle(title: string)` — Client-side validation (non-empty, max 255), call `invoke('check_import_collision')`, if collision set `collision` and advance to `'collision'` step, else set `outputPath` and advance to `'confirm'` step
    - `confirmImport(outputPath?: string, overwrite?: boolean)` — Call `invoke('start_parse')`, set `status = 'in_progress'`, advance to `'execute'` step, listen for Tauri events (`daemon:parser-progress`, `daemon:parser-completed`)
    - `cancelImport()` — Call `invoke('cancel_parse')` if `requestId` exists
    - `goBack()` — Navigate to previous step based on current step
    - `reset()` — Reset all state to defaults
  - **Getters**: `canGoBack`, `isImporting`, `isComplete`
  - **Event Listeners**: On mount, listen to `daemon:parser-progress` and `daemon:parser-completed` Tauri events. On progress, append to `logs`. On completed, set `status` and advance to `'result'` step.

### Step 6: Build Import Wizard Component Shell

- Create `windows/projects/components/ImportWizard.vue`:
  - Full-screen overlay (same pattern as delete confirmation dialog in `app.vue:54-86`)
  - Step indicator bar showing steps: File → Title → Confirm → Import → Done
  - Active step highlighting
  - Dynamic `<component :is="currentStepComponent" />` rendering
  - Close button (X) in top-right corner that calls `importStore.closeWizard()`
  - Wire to import store for step tracking

### Step 7: Build Step Components

- **`StepFilePicker.vue`**:
  - "Browse" button that calls `open()` from `@tauri-apps/plugin-dialog` with filters for `.fountain`, `.fdx`, `.sbx`, `.md`
  - Text input for manual path entry (optional, secondary)
  - Display selected file path
  - Validation error display
  - "Next" button that calls `importStore.setFile(path)`
  - "Cancel" link that calls `importStore.closeWizard()`

- **`StepTitle.vue`**:
  - Shows selected file path (read-only, muted text)
  - Text input for project title, placeholder "My Screenplay"
  - Character count indicator (max 255)
  - Validation error display
  - "Back" button, "Next" button that calls `importStore.setTitle(value)`

- **`StepConfirm.vue`**:
  - Summary panel: Title, Source File, Destination Path
  - "Import" primary button that calls `importStore.confirmImport()`
  - "Back" secondary button
  - "Cancel" link

- **`StepCollision.vue`**:
  - Warning banner: "A project already exists at this path"
  - Radio group with options:
    - "Overwrite existing project" (value: current outputPath, overwrite=true)
    - "Rename to: {suggested_name_N}" for each suggestion (value: new outputPath)
    - "Go back and change title" (navigates back to title step)
  - "Continue" button that calls `importStore.confirmImport(selectedPath, overwrite)`

- **`StepExecute.vue`**:
  - Spinner animation
  - "Importing..." status text
  - Scrollable log viewport showing streamed lines from `importStore.logs`
  - Auto-scroll to bottom on new lines
  - "Cancel" button that calls `importStore.cancelImport()`

- **`StepResult.vue`**:
  - Conditional success/failure display:
    - Success: green checkmark, "Project imported successfully!", output path
    - Failure: red X, error message, expandable log viewer
  - "Done" button that closes wizard and calls `projectsStore.loadProjects()` to refresh the list

### Step 8: Integrate Wizard into Projects App

- In `windows/projects/app.vue`:
  - Import `ImportWizard` component and `useImportStore`
  - Replace the `handleCreate` placeholder (line 139) with `importStore.openWizard()`
  - Add `<ImportWizard v-if="importStore.isOpen" />` to the template (after the delete confirmation dialog)
  - When import completes successfully, auto-refresh project list via `store.loadProjects()`

### Step 9: Write Unit Tests

- **`windows/projects/__tests__/stores/import.test.ts`**:
  - Test initial state values
  - Test `openWizard()` resets state and sets `isOpen`
  - Test `closeWizard()` clears state
  - Test `setFile()` validation (mocked Tauri invoke)
  - Test `setTitle()` with collision detection (mocked invoke returning `CollisionInfo`)
  - Test `setTitle()` without collision (mocked invoke returning `null`)
  - Test step navigation (`goBack()`)
  - Test `confirmImport()` sets status and step
  - Test `reset()` clears everything

- **`windows/projects/__tests__/components/ImportWizard.test.ts`**:
  - Test wizard renders when `isOpen` is true
  - Test step indicator highlights current step
  - Test close button calls `closeWizard()`

- **Rust tests in `src-tauri/src/services/import.rs`**:
  - `test_validate_title_empty()` — returns error for empty/whitespace
  - `test_validate_title_too_long()` — returns error for >255 chars
  - `test_validate_title_valid()` — returns Ok for valid titles
  - `test_normalize_project_filename()` — special chars replaced, spaces to underscore, trimmed
  - `test_normalize_project_filename_empty()` — falls back to "project"
  - `test_resolve_target_path()` — joins dir + normalized title + .kspd
  - `test_validate_import_file_nonexistent()` — returns error
  - `test_validate_import_file_wrong_extension()` — returns error
  - `test_generate_suggested_names()` — returns non-colliding alternatives

### Step 10: Validate

- Run all validation commands listed below to confirm zero regressions

## Validation Commands
Execute every command to validate the chore is complete with zero regressions.

- `cd /Users/k/Spikes/khaos-web-ui-bootstrap/src-tauri && cargo check` — Verify Rust code compiles with new import service, commands, types, and dialog plugin
- `cd /Users/k/Spikes/khaos-web-ui-bootstrap/src-tauri && cargo test` — Run all Rust unit tests including new import service tests
- `cd /Users/k/Spikes/khaos-web-ui-bootstrap && pnpm install` — Ensure new frontend dependencies resolve
- `cd /Users/k/Spikes/khaos-web-ui-bootstrap/windows/projects && npx vitest run` — Run all frontend unit tests (existing + new import tests)
- `cd /Users/k/Spikes/khaos-web-ui-bootstrap/windows/projects && npx nuxi typecheck` — TypeScript type checking for new interfaces and component types

## Notes
- The TUI uses `osascript` for native file picker on macOS (`import/model.go:303-331`). The web UI should use `@tauri-apps/plugin-dialog` instead, which provides cross-platform native file dialogs through Tauri's plugin system.
- The TUI streams `khaos-tools` CLI output via Go channels (`import/model.go:546-581`). The web UI equivalent is spawning the child process in Rust with `tokio::process::Command`, reading stdout/stderr line-by-line, and emitting each line as a `daemon:parser-progress` Tauri event. The frontend listens to these events and appends to the log viewport.
- The TUI's `cli.Adapter.ParseScreenplay()` invokes `khaos-tools parser parse --output {outputPath} {inputPath}` (`cli/adapter.go:127-156`). The Rust service should locate `khaos-tools` via PATH lookup (using `which` crate or `std::process::Command`) or a config override.
- The existing `events.rs` already defines `ParserProgressEvent` and `ParserCompletedEvent` payloads with `DAEMON_PARSER_PROGRESS` and `DAEMON_PARSER_COMPLETED` event names — reuse these directly.
- The TUI also initializes a git repo after successful parse (`import/model.go:568-572`). Consider adding this as a follow-up, or include `git init` in the parse service if `git` is available on PATH.
- Title normalization must match TUI behavior exactly: replace `<>:"/\|?*` with `_`, collapse whitespace to `_`, trim leading/trailing `_`, fallback to `"project"` if empty.
- Collision detection reuses `discovery::is_kspd()` which checks for `.kspd` extension OR `manifest.json` presence — same as TUI's `projects.IsKSPD()`.
- The import wizard should be a modal overlay, not a separate window, to stay within the Projects window context and allow easy return to the project list.
