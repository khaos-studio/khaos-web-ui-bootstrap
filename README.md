# khaos-web-ui Bootstrap

Tauri + Vue 3 + Nuxt multi-window architecture.

## Windows (Apps)

1. **Projects** - Display and manage KSPD projects (like khaos-tui welcome)
2. **Settings** - Configure providers, models, API keys (like khaos-tui settings)
3. **Dashboard** - Open project, display scene list, analyze (like khaos-tui project detail)

## Tech Stack

- Framework: Vue 3 + Nuxt
- Windowing: Tauri (native desktop)
- Components: Shadcn/ui (copy-paste)
- Styling: TailwindCSS + design tokens
- Communication: Tauri IPC → khaos-tools daemon

## Status

Bootstrapping Phase 2 prototype.

