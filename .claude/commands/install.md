---
description: Verify packaging environment and build tools are available
argument-hint: [hil]
---

# Purpose

Verify that the packaging environment has all required tools and dependencies, then report readiness to the user.

## Variables

MODE: $1 (optional - if "hil", run interactive mode)

## Workflow

1. **First**, execute `Skill(/prime)` to understand the codebase
2. Read `README.md` for requirements
3. Verify key tools are available:
   - `gh` (GitHub CLI, authenticated)
   - `pkgbuild` (macOS package builder)
   - `productsign` (macOS package signing)
   - `codesign` (binary signing)
   - `xcrun notarytool` (notarization)
   - `xcrun stapler` (stapling)
   - `nfpm` (Linux .deb/.rpm packaging)
4. Check for `.env` file with Apple signing credentials
5. Verify GitHub authentication: `gh auth status`
6. Report to user

## Report

**Status**: SUCCESS or FAILED

**Environment**:

- gh: [version or missing]
- pkgbuild: [available or missing]
- productsign: [available or missing]
- codesign: [available or missing]
- nfpm: [version or missing]

**Auth**:

- GitHub CLI: [authenticated or not]
- Apple signing: [.env present or missing]

**What worked**:

- [completed checks]

**What failed** (if any):

- [errors with context]

**Next steps**:

- [what to do now]
