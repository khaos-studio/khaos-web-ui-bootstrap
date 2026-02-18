---
description: Quick-start agent understanding of the codebase structure
---

# Purpose

Quickly orient an agent to understand the Khaos Web UI desktop application — a Tauri v2 + Nuxt 3 multi-window app built with pnpm workspaces.

## Workflow

1. Run `git ls-files` to get the complete list of tracked files
2. Read `README.md` for project overview
3. Read `justfile` for available commands
4. Explore project configuration:
   - `package.json` — Root config, scripts, devDependencies
   - `pnpm-workspace.yaml` — Workspace structure
   - `turbo.json` — Turborepo task config
   - `src-tauri/tauri.conf.json` — Tauri window/app config
   - `src-tauri/Cargo.toml` — Rust dependencies
5. Explore Rust backend:
   - `src-tauri/src/lib.rs` — Tauri app setup and command registration
   - `src-tauri/src/main.rs` — Rust entrypoint
   - `src-tauri/src/commands/` — IPC command handlers (projects, settings, dashboard, import)
   - `src-tauri/src/services/` — Business logic (discovery, config, import, export)
   - `src-tauri/src/types.rs` — Shared Rust types/DTOs
   - `src-tauri/src/events.rs` — Cross-window event definitions
   - `src-tauri/capabilities/default.json` — Permission/capability config
6. Explore frontend windows:
   - `windows/projects/` — Projects window (discovery, search, import wizard)
   - `windows/settings/` — Settings window (provider config)
   - `windows/dashboard/` — Dashboard window (project analysis)
   - Each window has: `app.vue`, `nuxt.config.ts`, `package.json`, `components/`, `stores/`
7. Explore shared code:
   - `shared/types/index.ts` — Shared TypeScript types
   - `shared/stores/` — Shared Pinia stores
   - `shared/styles/` — Shared CSS
8. Check `docs/` for architecture documentation
9. Check `specs/` for implementation plans
10. Follow the `Report` section

## Report

Provide a summary covering:

- **Project**: Khaos Web UI — Tauri v2 desktop app for screenwriting project management and analysis
- **Stack**: Tauri v2 (Rust backend) + Nuxt 3 (Vue 3 frontend) + pnpm workspaces + Turborepo
- **Architecture**: Multi-window (projects, settings, dashboard), each a separate Nuxt app
- **Backend**: Rust services for project discovery, config management, IPC with WFL daemon
- **Frontend**: Vue 3 + Pinia stores + Tailwind CSS per window
- **IPC**: Tauri invoke() commands bridging frontend to Rust backend
- **Build**: `pnpm run build` (frontend via Turbo), `cargo build` (Rust), `pnpm tauri build` (full app)
- **Test**: `pnpm run test` (vitest), `cargo test` (Rust)
- **Key Config Files**: `tauri.conf.json`, `turbo.json`, `pnpm-workspace.yaml`
- **Commands**: See `justfile` and `.claude/commands/`
- **Recommended Next Steps**: Based on user's task
