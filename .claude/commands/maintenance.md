---
description: Run repository maintenance checks and report health status
---

# Purpose

Execute repository maintenance tasks: verify builds, check test health, and report overall project status.

## Workflow

1. Run `Skill(/prime)` to understand the codebase
2. Check git repository health: `git fsck --no-full`
3. Verify dependencies are in sync: `pnpm install --frozen-lockfile`
4. Check Rust compilation: `cd src-tauri && cargo check`
5. Run Rust tests: `cd src-tauri && cargo test`
6. Run frontend tests: `pnpm run test`
7. Check for lint issues: `cd src-tauri && cargo clippy -- -D warnings`
8. Review git status for uncommitted changes
9. Report to user

## Report

**Status**: SUCCESS or FAILED

**Health Checks**:

- Git repository: [ok/issues]
- Dependencies: [in sync/outdated]
- Rust compilation: [pass/fail]
- Rust tests: [pass/fail with count]
- Frontend tests: [pass/fail with count]
- Rust lint (clippy): [pass/warnings]

**Uncommitted Changes**:

- [list or "working tree clean"]

**What worked**:

- [completed checks]

**What failed** (if any):

- [errors with context]

**Next steps**:

- [recommended actions]
