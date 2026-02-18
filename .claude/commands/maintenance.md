---
description: Run repository maintenance checks and report health status
---

# Purpose

Execute repository maintenance tasks: verify build assets, check release state, and report health status.

## Workflow

1. Run `Skill(/prime)` to understand the codebase
2. Check git repository health: `git fsck --no-full`
3. Verify build artifacts are clean: check `build/` directory state
4. Check latest GitHub release: `gh release view --json tagName,publishedAt,assets`
5. Verify upstream releases are current:
   - `gh release view -R khaos-studio/khaos-tools --json tagName`
   - `gh release view -R khaos-studio/khaos-tui --json tagName`
   - `gh release view -R khaos-studio/khaos-wfl --json tagName`
6. Check release notes coverage: compare `docs/releases/` against published releases
7. Lint shell scripts: `shellcheck *.sh` (if available)
8. Report to user

## Report

**Status**: SUCCESS or FAILED

**Health Checks**:

- Git repository: [ok/issues]
- Build artifacts: [clean/stale]
- Shell script lint: [pass/issues]

**Release State**:

- Current installer version: [version]
- Latest upstream khaos-tools: [version]
- Latest upstream khaos-tui: [version]
- Latest upstream khaos-wfl: [version]
- Release notes coverage: [complete/missing for versions]

**What worked**:

- [completed actions]

**What failed** (if any):

- [errors with context]

**Next steps**:

- [what to do now]
