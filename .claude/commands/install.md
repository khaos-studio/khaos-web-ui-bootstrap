---
description: Verify development environment and build tools are available
argument-hint: [hil]
---

# Purpose

Verify that the development environment has all required tools and dependencies for the Tauri v2 + Nuxt 3 project, then report readiness.

## Variables

MODE: $1 (optional - if "hil", run interactive mode)

## Workflow

1. **First**, execute `Skill(/prime)` to understand the codebase
2. Read `README.md` for requirements
3. Verify key tools are available:
   - `node` (Node.js runtime)
   - `pnpm` (package manager)
   - `rustc` / `cargo` (Rust toolchain)
   - `cargo tauri` (Tauri CLI)
   - `gh` (GitHub CLI, authenticated)
4. Verify optional tools:
   - `cargo clippy` (Rust linter)
   - `cargo fmt` (Rust formatter)
5. Check dependencies: `pnpm install --frozen-lockfile`
6. Verify Rust compiles: `cd src-tauri && cargo check`
7. Report to user

## Report

**Status**: SUCCESS or FAILED

**Environment**:

- Node.js: [version or missing]
- pnpm: [version or missing]
- Rust: [version or missing]
- Cargo: [version or missing]
- Tauri CLI: [version or missing]
- clippy: [available or missing]
- rustfmt: [available or missing]

**Auth**:

- GitHub CLI: [authenticated or not]

**Verification**:

- pnpm install: [success or failed]
- cargo check: [success or failed]

**What worked**:

- [completed checks]

**What failed** (if any):

- [errors with context]

**Next steps**:

- [what to do now]
