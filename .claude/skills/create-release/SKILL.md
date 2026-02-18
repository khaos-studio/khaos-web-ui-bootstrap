---
name: create-release
description: Create a clean, repeatable release using git tags and ldflags-based version injection. Defaults to a PATCH release unless explicitly specified.
---


## Name
create-release

## Description
Create a clean, repeatable release using git tags and ldflags-based version injection. Defaults to a PATCH release unless explicitly specified.

---

## Scope & Intent

This skill standardizes release creation by ensuring:
- Version is automatically extracted from git tags at build time (via ldflags)
- CHANGELOG is updated
- Release notes are added
- README is updated only when necessary
- A git tag is created for the release
- Semantic versioning rules are followed
- Pre-release validation checks are performed
- Build verification confirms version injection works correctly

This skill does NOT:
- Build artifacts for deployment
- Publish packages
- Assume languages, frameworks, or tooling
- Modify source code version constants (version is git-tag driven)
- Modify unrelated files
- Create releases that skip intermediate versions

---

## Semantic Versioning Rules

Format: MAJOR.MINOR.PATCH

Default behavior:
- If no version type is specified → increment PATCH
- Only one version increment per release
- Never skip intermediate versions (v1.1.0 must exist before v1.3.0)

Version meanings:
- MAJOR — breaking changes
- MINOR — new features, backward compatible
- PATCH — fixes, small improvements, documentation-only changes

---

## Required Files

Check for existing files only:
- CHANGELOG.md (must exist)
- README.md (optional update)
- Release notes location (one of):
  - RELEASE_NOTES.md
  - docs/releases/<version>.md
  - CHANGELOG section (if notes are embedded)

Do not create new structures or assume formats.

---

## Release Workflow (Strict Order)

### 0. Pre-Release Validation

Before performing any release operations, execute these validation checks:

**0a. Retrieve Latest Git Tag**
- Execute: `git describe --tags --abbrev=0` (or `git tag -l` if no tags exist)
- Store as `lastGitTag`
- If no tags exist, assume v0.0.0
- Log: "Latest git tag: vA.B.C" or "No previous tags found; starting at v0.0.0"

**0b. Verify Repository State**
- Check current branch is main/master (optional; log if different)
- Check working tree is clean: `git status --porcelain`
  - If dirty: ERROR "Working tree has uncommitted changes. Commit or stash before releasing."
- Verify CHANGELOG.md exists
- Log: "Pre-release validation complete"

---

### 1. Determine Target Version

- Retrieve latest git tag (from step 0a)
- Parse semantic version: extract MAJOR.MINOR.PATCH
- Compute next version based on:
  - Explicit instruction: `major | minor | patch`
  - Default to `patch` if unspecified
- Validation:
  - Ensure target version ≥ latest git tag
  - Ensure target version does not already exist as a tag
  - Ensure intermediate versions are not skipped (e.g., v1.1.0 before v1.3.0)

**Example**: If latest tag is v1.3.0 and type is patch → target is v1.3.1

**Log**: "Target version: vX.Y.Z (computed from vA.B.C + [MAJOR|MINOR|PATCH])"

---

### 1.5 Review Commit History Since Previous Release

Before updating documentation, review what has changed since the last release.

**Retrieve Commits**:
- Execute: `git log [lastGitTag]..HEAD --oneline --reverse`
- This shows all commits since the previous release
- Parse commit messages to understand changes:
  - `feat:` → New feature (suggests MINOR bump)
  - `fix:` → Bug fix (suggests PATCH bump)
  - `BREAKING CHANGE:` → Breaking change (suggests MAJOR bump)
  - `docs:` → Documentation only (no version bump alone)
  - `refactor:` → Code refactoring (no version bump alone)
  - `chore:` → Maintenance (no version bump alone)

**Categorize Changes**:
- Group commits by type (Features, Fixes, Breaking Changes, etc.)
- Note if multiple categories suggest a higher version bump
- Log categories found to help populate CHANGELOG

**Validation Guidance**:
- If no commits found since last tag, confirm this is intentional (rare)
- If only documentation/chore commits, PATCH is appropriate
- If features mixed with fixes, recommend MINOR instead of PATCH
- Document any unusual commit patterns

**Log example**:
```
Commits since v1.3.0:
  - feat: Add scene caching layer
  - fix: Resolve version consistency issue
  - fix: Handle edge case in parser
  - docs: Update README with new API
  - refactor: Simplify parser state management

Summary: 1 breaking change, 1 new feature, 2 fixes, 1 docs update
Current release type: MINOR (has 1 new feature + 2 fixes)
```

**Include in CHANGELOG**:
- Use this summary to populate release notes
- Reference key commits in the change summary
- Provide context for why this version was released

---

### 2. Update CHANGELOG.md

- Add new section at top for target version
- Include:
  - Version number in header: `## [X.Y.Z] - YYYY-MM-DD`
  - Release date (today's date)
  - Existing category structure (if present in file)
  - Brief summary of changes
- Do not modify historical entries
- Log: "Updated CHANGELOG.md with version vX.Y.Z"

---

### 3. Add Release Notes

- Create or update release notes for the version
- Content guidelines:
  - High-level summary
  - Notable behavior or user-visible changes
  - No implementation details

If release notes already exist:
- Update only the matching version section

Location detection (checked in order):
- `RELEASE_NOTES.md`
- `docs/releases/<version>.md`
- CHANGELOG section (if all notes are embedded there)

Log: "Release notes created at [file path]"

---

### 4. Update README.md (Conditional)

Update README only if:
- Version is explicitly referenced
- Usage or behavior changed
- Documented features were added, changed, or removed

Do NOT update README for:
- Internal refactors
- Invisible fixes
- Pure documentation releases unless versioned

Log: "README update: [SKIPPED|UPDATED]"

---

### 5. Git Commit

Create a single release commit including:
- CHANGELOG updates
- Release notes
- README changes (if any)

**Commit message format**:
```
chore(release): vX.Y.Z
```

**Commit process**:
1. Stage all modified files: `git add [file1] [file2] ...`
2. Create commit with message above
3. Capture commit hash for logging
4. Verify commit was created: `git log -1 --oneline`

**Log example**:
```
Staged files:
  - CHANGELOG.md
  - docs/releases/v1.3.0.md

Created commit: a1b2c3d [chore(release): v1.3.0]
```

---

### 6. Create Git Tag

- Create an annotated tag (not lightweight)
- Tag name: `vX.Y.Z`
- Tag message: short release summary (2-3 lines)
- Point tag to current commit (the one created in Step 5)

**Tag creation command**:
```bash
git tag -a vX.Y.Z -m "Release version vX.Y.Z: [brief description]"
```

**Verification**:
- Verify tag points to correct commit: `git show vX.Y.Z`
- Verify tag appears in list: `git tag | grep vX.Y.Z`

**Log**: "Created tag: vX.Y.Z on commit [hash]"

---

### 7. Verify Build Version Injection

Since version is now injected via ldflags at build time, verify the system works correctly.

**7a. Verify Git Tag Exists and is Reachable**:
- Execute: `git tag | grep vX.Y.Z`
- Execute: `git describe --tags --match "v*"` to confirm tag is reachable
- Ensure tag vX.Y.Z appears in the list
- Log: "Git tag verification: PASS - vX.Y.Z exists and is reachable"

**7b. Build and Verify Version Injection**:
- Execute: `make build` (Makefile extracts version from git tag via ldflags)
- Verify build succeeds
- Execute binary with version flag: `./khaos-parser --version`
- Verify output contains correct version (e.g., "khaos-parser vX.Y.Z")
- Log: "Build verification: PASS - binary reports version vX.Y.Z"

**7c. Verify Tag Points to Release Commit**:
- Execute: `git rev-list -n 1 vX.Y.Z` to get tag commit hash
- Verify this matches the release commit created in Step 5
- Log: "Tag commit verification: PASS - tag vX.Y.Z points to release commit [hash]"

---

### 8. Do NOT Push

Unless explicitly instructed by user, do NOT push:
- Do not push commits to remote
- Do not push tags to remote
- Leave for user to review and push manually

Log: "Release prepared. Ready to review. Push when ready: git push origin main && git push origin vX.Y.Z"

---

## Completion Report

At completion, report:

```
Release completed successfully

Previous version: vA.B.C
New version: vX.Y.Z

Files modified:
- [file path 1]
- [file path 2]
- ...

Release commit: [hash] - chore(release): vX.Y.Z
Release tag: vX.Y.Z (points to [hash])

Verification Results:
✓ Git tag exists and is reachable
✓ Binary built successfully with correct version
✓ Tag points to release commit

Next steps:
  → Review changes: git log -1 vX.Y.Z
  → Test binary: ./khaos-parser --version
  → Push when ready: git push origin main && git push origin vX.Y.Z
```

---

## Multi-Release Workflows

For projects with phased work, the skill supports creating multiple sequential releases.

### Use Case: Three-Phase Remediation

**Scenario**:
```
Phase 1 (v1.1.0): Schema extension at commit c13d723
Phase 2 (v1.2.0): Functional refactoring at commit 331d9f6  
Phase 3 (v1.3.0): Display alignment at current HEAD
```

**User instruction format**:
```
Create the following sequential releases:
- v1.1.0 at commit c13d723 (Phase 1: schema extension complete)
- v1.2.0 at commit 331d9f6 (Phase 2: functional refactoring complete)
- v1.3.0 at HEAD (Phase 3: display alignment complete)
```

**Implementation**:
For each release in sequence:
1. Checkout specified commit (or stay at current if HEAD)
2. Update CHANGELOG with release entry
3. Create commit: `chore(release): vX.Y.Z`
4. Create tag: `vX.Y.Z` on that commit
5. Verify version injection works (Step 7)
6. Move to next release

**Result Structure**:
```
v1.0.15 (previous)
  ↓
v1.1.0 @ c13d723     ← Phase 1 boundary marker
  ↓
v1.2.0 @ 331d9f6     ← Phase 2 boundary marker
  ↓
v1.3.0 @ HEAD        ← Phase 3 boundary marker
```

**Benefits**:
- Each phase is independently checkable: `git checkout v1.1.0`
- Version history reflects actual development phases
- Bisecting between phases works correctly
- Tags serve as reliable phase boundaries
- Binaries built from any tag auto-report correct version via ldflags

---

## Error Messages and Recovery

### Common Errors and Resolutions

**Error: "Working tree has uncommitted changes"**
- Cause: Local modifications exist
- Resolution: Commit or stash changes, then retry

**Error: "Tag vX.Y.Z already exists"**
- Cause: Attempting to create a release that already exists
- Resolution:
  1. Check if this version was already released: `git show vX.Y.Z`
  2. If legitimate, use a newer version
  3. If error, investigate why tag exists

**Error: "No CHANGELOG.md found"**
- Cause: project doesn't have a changelog
- Resolution: Create CHANGELOG.md in repo root, then retry

**Error: "Binary does not report correct version"**
- Cause: ldflags not properly injected during build, or git tag missing
- Resolution:
  1. Verify git tag exists: `git tag | grep vX.Y.Z`
  2. Verify Makefile has correct ldflags: `grep -A2 "ldflags" Makefile`
  3. Run `git describe --tags --abbrev=0` to confirm tag is reachable
  4. Run `make build` with verbose output or inspection
  5. Verify build command passes version flag correctly to linker

**Error: "Target version is before or equal to latest tag"**
- Cause: Attempting to release a version that doesn't advance semantic versioning
- Resolution:
  1. Check latest tag: `git tag -l | sort -V | tail -1`
  2. Compute correct next version based on commit history
  3. Retry with valid target version

---

## Logging and Diagnostics

The skill should log the following at each step:

**Step 0 Logs**:
- "Latest git tag: vA.B.C" or "No previous tags found; starting at v0.0.0"
- "Repository state: CLEAN" or "DIRTY: [files]"

**Step 1 Logs**:
- "Target version: vX.Y.Z (computed from vA.B.C + [type])"
- "Validation: target version [OK|SKIP-ERROR|AHEAD-WARNING]"

**Step 1.5 Logs**:
- "Commits since [lastGitTag]: [count]"
- "Commit categories: [list of types found]"
- "Recommended release type: [MAJOR|MINOR|PATCH]"

**Step 2-4 Logs**:
- "Updated CHANGELOG.md with version vX.Y.Z"
- "Release notes created at [path]" or "Release notes skipped"
- "README update: [SKIPPED|UPDATED]"

**Step 5-6 Logs**:
- "Staged files for commit: [file list]"
- "Created commit: [hash] - chore(release): vX.Y.Z"
- "Created tag: vX.Y.Z on commit [hash]"

**Step 7 Logs**:
- "Git tag verification: PASS - vX.Y.Z exists and is reachable"
- "Build verification: PASS - binary reports version vX.Y.Z"
- "Tag commit verification: PASS - tag points to release commit [hash]"

**Step 8 Logs**:
- "Release completed successfully"
- "Ready to review and push: git push origin main && git push origin vX.Y.Z"

---

## Activation Guidance

Use when user says or implies:
- "create a release"
- "cut a new version"
- "prepare a patch/minor/major release"
- "update changelog and tag"
- "create releases v1.1.0, v1.2.0, v1.3.0"
