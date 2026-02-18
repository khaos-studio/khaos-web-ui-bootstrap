# Plan: Settings Window Implementation

## Task Description

Implement a fully functional Settings window for khaos-web-ui-bootstrap, replicating the TUI's settings behavior. The Settings window allows users to configure AI provider selection (ollama, openai, mistralai, anthropic, groq, mock), override default models, and validate provider credentials and daemon connectivity. The window displays real-time status information for selected providers, including API key configuration status and special checks for Ollama (installation and model availability).

## Objective

By the end of this plan execution:
1. **Settings Window UI** - Fully functional Vue 3 Settings window with provider selection, model override, and real-time status
2. **Rust Backend** - Complete settings service implementation with config loading/saving, provider validation, daemon checks, and Ollama status detection
3. **Frontend Store** - Pinia store managing settings state machine, provider selection, validation status, and model configuration
4. **Integration** - Settings window integrated into the main dashboard with proper window lifecycle management
5. **Tests** - Full test coverage for store, components, and backend services
6. **Acceptance Criteria Met** - All settings functionality working end-to-end with zero regressions

## Problem Statement

The Settings window is currently just a placeholder. Users cannot:
- Configure AI provider selection
- Override default models for their selected provider
- Validate provider credentials and configurations
- Check daemon connectivity
- Detect Ollama installation and model availability
- Persist settings changes to disk

The TUI provides a proven behavioral reference for all these features. We need to port this functionality to the web UI while maintaining Vue 3 patterns and Tauri architecture established during the import wizard implementation.

## Solution Approach

**Architecture Pattern:** Follow the same patterns established in the import wizard implementation:
- **Rust Backend:** New `services/settings.rs` module with functions for config loading, saving, provider validation, and status checks
- **Tauri Commands:** Implement settings commands in `commands/settings.rs` with proper async/event-driven architecture
- **Frontend Store:** Pinia store (`stores/settings.ts`) managing settings state with async actions
- **Components:**
  - `SettingsWindow.vue` - Main window shell with layout
  - `ProviderSelector.vue` - Provider list component (similar to TUI's provider pane)
  - `ModelConfiguration.vue` - Model override and provider-specific settings
  - `StatusIndicator.vue` - Real-time status display for selected provider
  - `SettingsHeader.vue` - Window header with title
- **Types:** Extend shared types with `SettingsConfig`, `ProviderInfo`, `ValidationStatus`
- **Events:** Use Tauri events for provider validation status updates and daemon connection checks

**Key Features to Implement:**
1. Provider list with 6 options (ollama, openai, mistralai, anthropic, groq, mock)
2. Dynamic model input with provider-specific defaults shown
3. Effective model calculation (user override > provider default)
4. API key environment variable checks for cloud providers (OPENAI_API_KEY, MISTRALAI_API_KEY, ANTHROPIC_API_KEY, GROQ_API_KEY)
5. Ollama-specific checks: installation detection, `ollama list` command to verify qwen3 model
6. Daemon connectivity check via wfl_client
7. Real-time validation feedback with success/error states
8. Save/cancel workflow with proper state management

**Cross-Platform Considerations:**
- Use `std::process::Command` for checking `ollama` installation (cross-platform via exec.LookPath equivalent)
- Use `which` command or Rust's `which` crate for binary detection
- Environment variable access via `std::env::var()` works on macOS/Linux
- Config file paths handled consistently with import service

## Relevant Files

### Backend Files (Rust)

Key files to create/modify:
- `src-tauri/src/services/settings.rs` (NEW) - Settings service layer with validation logic
- `src-tauri/src/commands/settings.rs` (MODIFY) - Implement all 4 command stubs
- `src-tauri/src/types.rs` (MODIFY) - Add SettingsConfig, ProviderValidationResult types
- `src-tauri/src/services/config.rs` (MODIFY) - Implement config loading/saving if not done
- `src-tauri/src/events.rs` (MODIFY) - Add settings-related events
- `src-tauri/src/services/mod.rs` (MODIFY) - Register settings service module
- `src-tauri/Cargo.toml` (MODIFY if needed) - Ensure necessary dependencies present

### Frontend Files (Vue 3)

Key files to create/modify:
- `windows/settings/stores/settings.ts` (NEW) - Pinia store for settings state
- `windows/settings/components/SettingsWindow.vue` (MODIFY) - Replace placeholder
- `windows/settings/components/ProviderSelector.vue` (NEW) - Provider list UI
- `windows/settings/components/ModelConfiguration.vue` (NEW) - Model input and config UI
- `windows/settings/components/StatusIndicator.vue` (NEW) - Provider status display
- `windows/settings/components/SettingsHeader.vue` (NEW) - Window header
- `shared/types/index.ts` (MODIFY) - Add SettingsConfig, ValidationStatus types
- `windows/settings/__tests__/stores/settings.test.ts` (NEW) - Store unit tests
- `windows/settings/__tests__/components/SettingsWindow.test.ts` (NEW) - Component tests

### Configuration Files
- `src-tauri/capabilities/default.json` (VERIFY) - Ensure command capability for settings commands

## Implementation Phases

### Phase 1: Foundation (Backend Service Layer)

1. Create `src-tauri/src/services/settings.rs` with core functions:
   - `load_settings()` - Load Config from ~/.khaos-ui/config.json
   - `save_settings()` - Save Config to ~/.khaos-ui/config.json
   - `validate_provider()` - Check provider validity
   - `get_default_model()` - Return provider's default model
   - `check_api_key()` - Verify environment variable is set for cloud providers
   - `check_ollama_installation()` - Use `which ollama` to detect Ollama
   - `check_ollama_model()` - Run `ollama list` and check for qwen3 model
   - `check_daemon_connection()` - Ping wfl daemon via socket/TCP

2. Implement `services/config.rs` stub:
   - Load/parse ~/.khaos-ui/config.json with fallback to defaults
   - Validate config structure
   - Create directory if doesn't exist

3. Update types in `src-tauri/src/types.rs`:
   - `SettingsConfig` struct with provider and model fields
   - `ProviderInfo` struct with id, title, description, requires_key
   - `ValidationStatus` enum (Unchecked, Valid, Invalid)
   - Update `ProviderValidationResult` if needed

4. Implement settings commands in `src-tauri/src/commands/settings.rs`:
   - `load_settings()` - Call service layer
   - `save_settings()` - Call service layer + emit event to all windows
   - `check_daemon_connection()` - Call service layer
   - `validate_provider_config()` - Call service layer with capability-driven validation

### Phase 2: Frontend Store & Types

1. Create shared types in `shared/types/index.ts`:
   - Export SettingsConfig, ProviderInfo, ValidationStatus

2. Create Pinia store `windows/settings/stores/settings.ts`:
   - State: settings, currentProvider, currentModel, validationStatus, loading, error, daemonStatus
   - Actions:
     - `loadSettings()` - Invoke load_settings command
     - `saveSettings()` - Invoke save_settings command
     - `selectProvider()` - Update provider with validation
     - `setModel()` - Update model field
     - `validateProvider()` - Invoke validate_provider_config command
     - `checkDaemonConnection()` - Invoke check_daemon_connection command
   - Getters:
     - `effectiveModel()` - Return user override or provider default
     - `isValid()` - Check if current config is valid
     - `canSave()` - Check if save is allowed

### Phase 3: Frontend UI Components

1. Create `ProviderSelector.vue`:
   - Render 6 provider options (ollama, openai, mistralai, anthropic, groq, mock)
   - Highlight selected provider with blue background
   - Handle provider selection and switch to ModelConfiguration
   - Keyboard support: arrow keys to navigate, enter to select

2. Create `ModelConfiguration.vue`:
   - Text input for model override (optional)
   - Display effective model (user override or provider default)
   - Display provider-specific key validation status
   - For cloud providers: show "API key status: SET" or "missing PROVIDER_API_KEY"
   - For Ollama: show installation status and model availability
   - For mock: show "API key status: not required"

3. Create `StatusIndicator.vue`:
   - Render validation status with icons (✓ for valid, ✗ for invalid, ⏳ for checking)
   - Show error messages
   - Update in real-time as validation runs

4. Create `SettingsHeader.vue`:
   - Title "Settings"
   - Subtitle "Provider/model configuration for analysis commands"
   - Close button

5. Modify `SettingsWindow.vue`:
   - Import and use all child components
   - Arrange in two-pane layout (provider list on left, config on right)
   - Add save/cancel buttons
   - Handle keyboard shortcuts (Tab, Shift+Tab, Escape)
   - Lifecycle: load settings on mount, save/discard on close

### Phase 4: Integration & Polish

1. Register settings store in main app
2. Add window lifecycle hooks for load/save
3. Add keyboard navigation support
4. Add proper error handling and user feedback
5. Implement settings-changed event handling across windows
6. Add smooth transitions and loading states

### Phase 5: Testing

1. Write Pinia store unit tests (24+ tests covering all actions and getters)
2. Write component snapshot and behavior tests
3. Write integration tests for command flow
4. Verify all Rust tests pass
5. Manual testing of end-to-end flow

## Team Orchestration

As the team lead, I will orchestrate this implementation using specialized agents. The plan will be executed using task-based coordination with clear dependencies and parallel execution where possible.

### Team Members

- **Backend Service Builder**
  - Name: `backend-settings-builder`
  - Role: Implement Rust services, types, and commands for settings functionality
  - Agent Type: `general-purpose`
  - Resume: true (continue with same context across tasks)

- **Frontend Store Developer**
  - Name: `frontend-store-developer`
  - Role: Implement Pinia store and TypeScript types for settings state management
  - Agent Type: `general-purpose`
  - Resume: true

- **Frontend UI Component Developer**
  - Name: `frontend-ui-developer`
  - Role: Build Vue 3 components for Settings window UI
  - Agent Type: `general-purpose`
  - Resume: true

- **Test Engineer**
  - Name: `test-engineer`
  - Role: Write comprehensive unit and integration tests
  - Agent Type: `general-purpose`
  - Resume: true

- **Integration & Validation Lead**
  - Name: `integration-lead`
  - Role: Coordinate integration, handle window lifecycle, validate all acceptance criteria
  - Agent Type: `general-purpose`
  - Resume: true

## Step by Step Tasks

### 1. Foundation: Rust Types and Service Stubs
- **Task ID**: `settings-types-and-stubs`
- **Depends On**: none
- **Assigned To**: `backend-settings-builder`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `src-tauri/src/services/settings.rs` with function signatures and documentation
- Create/update `src-tauri/src/types.rs` with SettingsConfig, ProviderInfo, ValidationStatus types
- Update `src-tauri/src/services/mod.rs` to register settings module
- Verify code compiles with `cargo check`

### 2. Implement Config Service
- **Task ID**: `settings-config-service`
- **Depends On**: `settings-types-and-stubs`
- **Assigned To**: `backend-settings-builder`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Implement `services/config.rs` with full config loading/saving
- Handle ~/.khaos-ui/config.json file operations
- Add error handling for missing files and invalid JSON
- Add test coverage for config operations

### 3. Implement Provider Validation Logic
- **Task ID**: `settings-provider-validation`
- **Depends On**: `settings-config-service`
- **Assigned To**: `backend-settings-builder`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Implement API key checking functions for cloud providers
- Implement Ollama detection and model checking
- Implement daemon connection checking
- Add comprehensive error handling

### 4. Implement Settings Commands
- **Task ID**: `settings-commands-impl`
- **Depends On**: `settings-provider-validation`
- **Assigned To**: `backend-settings-builder`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Implement all 4 command stubs in `commands/settings.rs`
- Add proper error propagation and logging
- Add events emission for validation status updates
- Run `cargo test` to verify all tests pass

### 5. Create Shared TypeScript Types
- **Task ID**: `settings-shared-types`
- **Depends On**: none
- **Assigned To**: `frontend-store-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Extend `shared/types/index.ts` with Settings-related types
- Ensure types align with Rust backend types
- Add JSDoc comments for each type

### 6. Implement Settings Pinia Store
- **Task ID**: `settings-pinia-store`
- **Depends On**: `settings-shared-types`, `settings-commands-impl`
- **Assigned To**: `frontend-store-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Create `windows/settings/stores/settings.ts` with full state management
- Implement all actions for loading, saving, validation
- Implement computed getters for effective model and validation status
- Add proper error handling and loading states

### 7. Create UI Components - Provider Selector
- **Task ID**: `settings-provider-selector-component`
- **Depends On**: `settings-pinia-store`
- **Assigned To**: `frontend-ui-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `windows/settings/components/ProviderSelector.vue`
- Render 6 provider options with proper styling
- Implement keyboard navigation (arrow keys, enter)
- Connect to Pinia store for state management
- Add hover and focus states

### 8. Create UI Components - Model Configuration
- **Task ID**: `settings-model-config-component`
- **Depends On**: `settings-pinia-store`
- **Assigned To**: `frontend-ui-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `windows/settings/components/ModelConfiguration.vue`
- Render model input with provider-specific UI
- Show effective model calculation
- Display provider status information
- Connect to Pinia store

### 9. Create UI Components - Status and Header
- **Task ID**: `settings-status-header-components`
- **Depends On**: `settings-pinia-store`
- **Assigned To**: `frontend-ui-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `windows/settings/components/StatusIndicator.vue`
- Create `windows/settings/components/SettingsHeader.vue`
- Add proper styling and animations
- Connect to Pinia store for status updates

### 10. Update Main Settings Window Component
- **Task ID**: `settings-window-main-component`
- **Depends On**: `settings-provider-selector-component`, `settings-model-config-component`, `settings-status-header-components`
- **Assigned To**: `frontend-ui-developer`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Modify `windows/settings/app.vue` (rename to SettingsWindow.vue if needed)
- Integrate all child components
- Implement two-pane layout
- Add save/cancel buttons and keyboard shortcuts
- Add lifecycle hooks for loading/saving settings

### 11. Write Pinia Store Tests
- **Task ID**: `settings-store-tests`
- **Depends On**: `settings-pinia-store`
- **Assigned To**: `test-engineer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `windows/settings/__tests__/stores/settings.test.ts`
- Write 30+ unit tests covering:
  - Initial state
  - All actions (loadSettings, saveSettings, selectProvider, etc.)
  - All getters (effectiveModel, isValid, canSave)
  - Error handling
  - Edge cases
- Ensure all tests pass

### 12. Write Component Tests
- **Task ID**: `settings-component-tests`
- **Depends On**: `settings-window-main-component`
- **Assigned To**: `test-engineer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Create `windows/settings/__tests__/components/SettingsWindow.test.ts`
- Write component snapshot and behavior tests
- Test keyboard navigation
- Test provider selection
- Test save/cancel flow
- Ensure all tests pass

### 13. Run Backend Tests
- **Task ID**: `settings-backend-tests`
- **Depends On**: `settings-commands-impl`
- **Assigned To**: `test-engineer`
- **Agent Type**: `general-purpose`
- **Parallel**: true
- Run `cargo test` in src-tauri directory
- Verify all Rust tests pass (existing + new)
- Check test coverage for settings module

### 14. Integration & Window Lifecycle
- **Task ID**: `settings-integration`
- **Depends On**: `settings-window-main-component`, `settings-backend-tests`
- **Assigned To**: `integration-lead`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Register settings window in main app
- Implement proper window lifecycle hooks
- Add settings-changed event handling
- Test cross-window communication when settings change

### 15. Manual Testing & Validation
- **Task ID**: `settings-manual-testing`
- **Depends On**: `settings-integration`, `settings-store-tests`, `settings-component-tests`
- **Assigned To**: `integration-lead`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Test provider selection and switching
- Test model input and effective model calculation
- Verify API key detection for each cloud provider
- Test Ollama installation detection
- Test daemon connection check
- Test settings save/load persistence
- Test keyboard navigation
- Verify no regressions in existing functionality
- Confirm all acceptance criteria met

### 16. Final Validation and Commit
- **Task ID**: `settings-final-validation`
- **Depends On**: `settings-manual-testing`
- **Assigned To**: `integration-lead`
- **Agent Type**: `general-purpose`
- **Parallel**: false
- Run full test suite: `cargo test` + `vitest run`
- Verify git status shows proper file changes
- Create comprehensive commit with all settings changes
- Document any deviations from plan

## Acceptance Criteria

1. **Settings Window Displays** - Settings window opens and shows provider list, model input, and status information
2. **Provider Selection Works** - Users can select from 6 providers (ollama, openai, mistralai, anthropic, groq, mock)
3. **Model Override** - Users can enter custom model names; effective model shows override or provider default
4. **API Key Detection** - For cloud providers, system displays API key status (SET or missing with env var name)
5. **Ollama Detection** - For Ollama provider, system checks installation and qwen3 model availability
6. **Daemon Check Works** - System can ping WFL daemon and report connectivity status
7. **Settings Persist** - Settings saved to ~/.khaos-ui/config.json and reloaded on app start
8. **Keyboard Navigation** - All UI elements navigable via Tab, Shift+Tab, arrow keys, Enter
9. **Keyboard Shortcuts** - Tab switches focus, Escape/B returns to dashboard
10. **All Tests Pass** - Zero test failures in both Rust backend and Vue frontend
11. **No Regressions** - Existing functionality (projects, import) continues to work perfectly
12. **Code Quality** - No cargo warnings, proper error handling, comprehensive logging

## Validation Commands

Execute these commands to validate the implementation is complete and working:

```bash
# Backend validation
cd /Users/k/Spikes/khaos-web-ui-bootstrap/src-tauri
cargo test                           # All Rust tests pass
cargo check                          # No compiler warnings
cargo build --release               # Release build succeeds

# Frontend validation
cd /Users/k/Spikes/khaos-web-ui-bootstrap
pnpm install                         # Dependencies installed
pnpm run -C windows/settings vitest  # All settings tests pass
pnpm run lint                        # No linting errors

# Manual integration testing (if UI available)
# - Open Settings window
# - Select each provider and verify status display
# - Test Ollama detection (if Ollama available)
# - Test API key detection
# - Test save/load persistence
# - Verify no crashes or console errors
```

## Notes

**Key Implementation Insights from TUI:**
- The TUI uses Bubbletea's list component with 6 static provider items
- Provider selection is done via arrow keys and enter on a list
- Model input is an optional text field that can be focused via Tab
- The right pane shows dynamic information based on selected provider
- Ollama checking happens asynchronously when Ollama is selected via `ollamaStatusMsg`
- The TUI persists settings by calling `appModel.SaveConfig()`

**Web UI Differences:**
- Use Vue 3 components instead of Bubbletea
- Use Pinia instead of bubbletea state management
- Use Tauri commands for backend communication instead of Go methods
- Use Tauri events for real-time status updates
- Use native HTML form inputs with proper accessibility

**Cross-Platform Considerations:**
- Ollama binary detection works on macOS/Linux via `which ollama`
- `ollama list` command works the same on both platforms
- Environment variable checking via `std::env::var()` is cross-platform
- Config file location (~/.khaos-ui/config.json) works on both platforms

**Dependencies:**
- No new crate dependencies needed (use existing Tauri, serde, tokio)
- May use `which` crate if system `which` command not reliable

**Timeline Estimate:**
- Foundation & Backend: ~4-5 hours
- Frontend Store: ~2-3 hours
- UI Components: ~4-5 hours
- Testing: ~3-4 hours
- Integration & Validation: ~2-3 hours
- **Total**: ~15-20 hours of work across 5 team members (parallelizable)
