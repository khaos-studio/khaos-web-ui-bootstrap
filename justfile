# KhaosFoundation Justfile
# Installer Packaging for Khaos Foundational Tools

# ============================================================================
# Build Commands
# ============================================================================

# Build macOS installer package (download, sign binaries, build .pkg)
build:
  ./build.sh

# Build Linux .deb and .rpm packages (amd64)
build-linux arch="amd64":
  ./build-linux.sh --arch {{arch}}

# Build macOS distribution pkg via Packages.app (stamps VERSION into HTML resources)
build-distro:
  #!/usr/bin/env bash
  set -euo pipefail
  VERSION=$(tr -d '[:space:]' < VERSION)
  WELCOME=packaging/macos/resources/welcome.html
  cp "$WELCOME" "$WELCOME.bak"
  trap 'mv "$WELCOME.bak" "$WELCOME"' EXIT
  sed -i '' "s/__VERSION__/$VERSION/g" "$WELCOME"
  packagesbuild KhaosFoundation-distro.pkgproj || true

# Full pipeline: build + sign + release
build-package *FLAGS:
  ./build-package.sh {{FLAGS}}

# ============================================================================
# Signing & Release Commands
# ============================================================================

# Sign and notarize macOS package
sign *FLAGS:
  ./sign.sh {{FLAGS}}

# Publish release to GitHub
release:
  ./release.sh

# ============================================================================
# Quality Commands
# ============================================================================

# Lint all shell scripts
lint:
  @command -v shellcheck >/dev/null 2>&1 && shellcheck *.sh packaging/*/scripts/* scripts/* || echo "⚠ shellcheck not installed"

# Lint markdown files (dry-run, no changes)
md-lint:
  @echo "Linting markdown files..."
  node node_modules/markdownlint-cli2/markdownlint-cli2.js 'docs/**/*.md' 'specs/**/*.md' 'packaging/**/*.md' 'README.md'
  @echo "✓ Markdown linting passed"

# Apply autofix to markdown files
md-lint-fix:
  @echo "Fixing markdown files..."
  node node_modules/markdownlint-cli2/markdownlint-cli2.js --fix 'docs/**/*.md' 'specs/**/*.md' 'packaging/**/*.md' 'README.md'
  @echo "✓ Markdown files fixed"

# Run all checks (lint + md-lint)
check: lint md-lint
  @echo "✓ All checks passed"

# ============================================================================
# Environment & Status Commands
# ============================================================================

# Verify all required tools are available
doctor:
  @echo "Checking required tools..."
  @command -v gh >/dev/null 2>&1 && echo "✓ gh $(gh --version | head -1)" || echo "✗ gh (GitHub CLI) not found"
  @command -v pkgbuild >/dev/null 2>&1 && echo "✓ pkgbuild available" || echo "✗ pkgbuild not found"
  @command -v productbuild >/dev/null 2>&1 && echo "✓ productbuild available" || echo "✗ productbuild not found"
  @command -v productsign >/dev/null 2>&1 && echo "✓ productsign available" || echo "✗ productsign not found"
  @command -v codesign >/dev/null 2>&1 && echo "✓ codesign available" || echo "✗ codesign not found"
  @command -v xcrun >/dev/null 2>&1 && echo "✓ xcrun available" || echo "✗ xcrun not found"
  @command -v nfpm >/dev/null 2>&1 && echo "✓ nfpm $(nfpm --version 2>/dev/null || echo 'available')" || echo "✗ nfpm not found (needed for Linux packages)"
  @command -v shellcheck >/dev/null 2>&1 && echo "✓ shellcheck available" || echo "⚠ shellcheck not found (optional)"
  @echo ""
  @echo "Checking auth..."
  @gh auth status 2>&1 | head -3 || echo "✗ GitHub CLI not authenticated"
  @test -f .env && echo "✓ .env file present" || echo "⚠ .env file missing (needed for notarization)"

# Show current installer version
version:
  @cat VERSION | tr -d '[:space:]'
  @echo ""

# Set installer version (updates VERSION file and syncs pkgproj)
set-version NEW_VERSION:
  @echo "{{NEW_VERSION}}" > VERSION
  @python3 -c "import re; f='KhaosFoundation.pkgproj'; t=open(f).read(); t=re.sub(r'(<key>VERSION</key>\s*<string>)[^<]*(</string>)', r'\g<1>{{NEW_VERSION}}\g<2>', t); open(f,'w').write(t)"
  @echo "Version set to {{NEW_VERSION}}"

# Show latest upstream release versions
upstream:
  @echo "Latest upstream releases:"
  @echo -n "  khaos-tools: " && gh release view -R khaos-studio/khaos-tools --json tagName --jq '.tagName' 2>/dev/null || echo "unknown"
  @echo -n "  khaos-tui:   " && gh release view -R khaos-studio/khaos-tui --json tagName --jq '.tagName' 2>/dev/null || echo "unknown"
  @echo -n "  khaos-wfl:   " && gh release view -R khaos-studio/khaos-wfl --json tagName --jq '.tagName' 2>/dev/null || echo "unknown"

# Show build metadata from last build (if available)
build-info:
  @test -f build/upstream-release-metadata.env && cat build/upstream-release-metadata.env || echo "No build metadata found. Run 'just build' first."

# ============================================================================
# Claude Code Commands
# ============================================================================

cld-super:
  claude --model opus --dangerously-skip-permissions --init

# Deterministic codebase setup (hook only, fast, CI-friendly)
cld-init:
  claude --model haiku --dangerously-skip-permissions --init

cld-esc:
  claude --model haiku --dangerously-skip-permissions --init --plugin-dir /Users/k/Spikes/escapement

# Deterministic codebase maintenance
cld-maintain:
  claude --model haiku --dangerously-skip-permissions --maintenance

# Agentic codebase setup (hook + agent analysis)
cld-init-agent:
  claude --model haiku --dangerously-skip-permissions --init "/install"

# Agentic codebase setup interactive (hook + clarifying questions)
cld-init-interactive:
  claude --model haiku --dangerously-skip-permissions --init "/install true"

# Agentic codebase maintenance
cld-maintain-agent:
  claude --model haiku --dangerously-skip-permissions --maintenance "/maintenance"

# ============================================================================
# Utility Commands
# ============================================================================

# Clean build artifacts
clean:
  rm -rf build/
  @echo "✓ Cleaned build directory"

# List all justfile recipes
recipes:
  @just --list

# Show full help
full-help:
  @echo "KhaosFoundation - Installer Packaging"
  @echo ""
  @echo "Usage: just <recipe> [arguments]"
  @echo ""
  @echo "Build & Release:"
  @echo "  just build                Build macOS .pkg installer"
  @echo "  just build-linux [arch]   Build Linux .deb/.rpm (default: amd64)"
  @echo "  just build-package        Full pipeline: build + sign + release"
  @echo "  just sign                 Sign and notarize macOS package"
  @echo "  just release              Publish release to GitHub"
  @echo "  just clean                Clean build artifacts"
  @echo ""
  @echo "Quality:"
  @echo "  just lint                 Lint shell scripts (shellcheck)"
  @echo "  just md-lint              Lint markdown files"
  @echo "  just md-lint-fix          Fix markdown lint issues"
  @echo "  just check                Run all checks (lint + md-lint)"
  @echo ""
  @echo "Status:"
  @echo "  just doctor               Verify required tools are available"
  @echo "  just version              Show installer version"
  @echo "  just set-version X.Y.Z   Set installer version (VERSION + pkgproj)"
  @echo "  just upstream             Show latest upstream release versions"
  @echo "  just build-info           Show build metadata from last build"
  @echo ""
  @echo "Claude Code:"
  @echo "  just cld-super            Claude with opus, skip permissions"
  @echo "  just cld-init             Deterministic setup"
  @echo "  just cld-maintain         Deterministic maintenance"
  @echo "  just cld-init-agent       Agentic setup"
  @echo "  just cld-maintain-agent   Agentic maintenance"
  @echo ""
  @echo "Utility:"
  @echo "  just recipes              List all recipes"
  @echo "  just full-help            Show this help message"
  @echo ""
  @echo "For more information, see README.md"
