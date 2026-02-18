---
description: Quick-start agent understanding of the codebase structure
---

# Purpose

Quickly orient an agent to understand the KhaosFoundation installer packaging workspace by exploring its structure and configuration.

## Workflow

1. Run `git ls-files` to get the complete list of tracked files
2. Read `README.md` for project overview
3. Read `justfile` for available commands
4. Read `.claude/settings.local.json` for hook configuration
5. Explore key project files:
   - `build.sh` - Main macOS build (download, sign, package)
   - `build-linux.sh` - Linux packaging (.deb/.rpm via nfpm)
   - `build-package.sh` - Full build+sign+release orchestrator
   - `sign.sh` - Code signing and notarization
   - `release.sh` - GitHub release publishing
   - `common.sh` - Shared build utilities
6. Explore packaging assets:
   - `packaging/macos/` - macOS installer scripts and LaunchAgent plist
   - `packaging/linux/` - Linux systemd unit and postinstall scripts
   - `scripts/` - WFL daemon helper scripts
   - `KhaosFoundation.pkgproj` - macOS installer project config
   - `nfpm.yaml` - Linux package spec (nfpm)
7. Check `docs/releases/` for versioned release notes
8. Follow the `Report` section

## Report

Provide a summary covering:

- **Workspace Purpose**: Installer packaging for Khaos foundational tools (khaos-tools, khaos-tui, khaos-wfl)
- **What It Packages**:
  - Downloads latest pre-built binaries from upstream GitHub releases
  - Bundles into macOS .pkg (signed/notarized) and Linux .deb/.rpm
  - Includes WFL daemon service integration (LaunchAgent / systemd)
- **Build Pipeline**:
  - `./build.sh` - Download upstream binaries, sign, build .pkg
  - `./build-linux.sh` - Build Linux packages via nfpm
  - `./sign.sh` - Productsign + notarize macOS package
  - `./release.sh` - Publish to GitHub releases
  - `./build-package.sh` - Full pipeline orchestrator
- **Key Configuration**:
  - `KhaosFoundation.pkgproj` - macOS package project
  - `nfpm.yaml` - Linux package spec
  - `.env` - Apple signing/notarization credentials
- **Installation Targets**:
  - Binaries: `/usr/local/bin/`
  - Prompts: `/usr/local/share/khaos-tools/prompts/` + `~/.khaos/prompts`
  - Daemon: platform-specific (LaunchAgent or systemd user unit)
- **Requirements**: `gh`, `pkgbuild`, `nfpm`, `codesign`, `productsign`, `xcrun`
- **Commands**: See `.claude/commands/` directory
- **Recommended Next Steps**: Based on user's task
