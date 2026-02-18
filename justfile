# Khaos UI - Development Justfile
# Tauri v2 + Nuxt 3 + pnpm workspace

# ============================================================================
# Development Commands
# ============================================================================

# Start Tauri dev mode (frontend + backend)
dev:
  cd src-tauri && cargo tauri dev

# Start only frontend dev servers (all windows via Turborepo)
dev-frontend:
  pnpm run dev

# Start specific window dev server
dev-projects:
  pnpm --filter @khaos/projects-window run dev

dev-settings:
  pnpm --filter @khaos/settings-window run dev

dev-dashboard:
  pnpm --filter @khaos/dashboard-window run dev

# ============================================================================
# Build Commands
# ============================================================================

# Build all frontend packages (via Turborepo)
build-frontend:
  pnpm run build

# Check Rust compilation (fast, no binary output)
check-rust:
  cd src-tauri && cargo check

# Build Rust backend (debug)
build-rust:
  cd src-tauri && cargo build

# Build Rust backend (release)
build-rust-release:
  cd src-tauri && cargo build --release

# Build complete Tauri application
build:
  cd src-tauri && cargo tauri build --bundles app

# ============================================================================
# Test Commands
# ============================================================================

# Run all frontend tests (via Turborepo)
test:
  pnpm run test

# Run projects window tests
test-projects:
  pnpm --filter @khaos/projects-window run test

# Run Rust backend tests
test-rust:
  cd src-tauri && cargo test

# Run all tests (frontend + backend)
test-all: test test-rust

# ============================================================================
# Lint & Format Commands
# ============================================================================

# Lint all frontend code
lint:
  pnpm run lint

# Lint Rust code with clippy
lint-rust:
  cd src-tauri && cargo clippy -- -D warnings

# Lint everything
lint-all: lint lint-rust

# Format frontend code
format:
  pnpm run format

# Format Rust code
format-rust:
  cd src-tauri && cargo fmt

# Format everything
format-all: format format-rust

# ============================================================================
# Environment Setup
# ============================================================================

# Install all dependencies
install:
  pnpm install

# Verify environment and required tools
doctor:
  @echo "Checking required tools..."
  @echo ""
  @echo "=== Node.js ==="
  @node --version 2>/dev/null && echo "✓ Node.js installed" || echo "✗ Node.js not found"
  @pnpm --version 2>/dev/null && echo "✓ pnpm $(pnpm --version)" || echo "✗ pnpm not found"
  @echo ""
  @echo "=== Rust ==="
  @rustc --version 2>/dev/null && echo "✓ Rust installed" || echo "✗ Rust not found"
  @cargo --version 2>/dev/null && echo "✓ Cargo installed" || echo "✗ Cargo not found"
  @echo ""
  @echo "=== Tauri ==="
  @cargo tauri --version 2>/dev/null && echo "✓ Tauri CLI installed" || echo "✗ Tauri CLI not found (install: cargo install tauri-cli)"
  @echo ""
  @echo "=== Optional ==="
  @cargo clippy --version 2>/dev/null && echo "✓ clippy installed" || echo "⚠ clippy not found"
  @cargo fmt --version 2>/dev/null && echo "✓ rustfmt installed" || echo "⚠ rustfmt not found"
  @echo ""
  @echo "=== GitHub ==="
  @gh auth status 2>&1 | head -3 || echo "⚠ GitHub CLI not authenticated"

# ============================================================================
# Claude Code Commands
# ============================================================================

cld-super:
  claude --model opus --dangerously-skip-permissions --init

cld-init:
  claude --model haiku --dangerously-skip-permissions --init

cld-esc:
  claude --model haiku --dangerously-skip-permissions --init --plugin-dir /Users/k/Spikes/escapement

cld-maintain:
  claude --model haiku --dangerously-skip-permissions --maintenance

cld-init-agent:
  claude --model haiku --dangerously-skip-permissions --init "/install"

cld-init-interactive:
  claude --model haiku --dangerously-skip-permissions --init "/install true"

cld-maintain-agent:
  claude --model haiku --dangerously-skip-permissions --maintenance "/maintenance"

# ============================================================================
# Utility Commands
# ============================================================================

# Clean all build artifacts
clean:
  rm -rf windows/*/dist windows/*/.nuxt windows/*/.output
  cd src-tauri && cargo clean
  @echo "✓ Cleaned build artifacts"

# Clean and reinstall dependencies
reset: clean
  rm -rf node_modules windows/*/node_modules shared/node_modules
  pnpm install
  @echo "✓ Reset complete"

# List all recipes
recipes:
  @just --list
