# Feature Planning

Create a new plan in specs/*.md to implement the `Feature` using the exact specified markdown `Plan Format`. Follow the `Instructions` to create the plan use the `Relevant Files` to focus on the right files.

## Instructions

- You're writing a plan to implement a net new feature that will add value to the installer packaging system.
- Create the plan in the `specs/*.md` file. Name it appropriately based on the `Feature`.
- Use the `Plan Format` below to create the plan.
- Research the codebase to understand existing patterns, architecture, and conventions before planning the feature.
- IMPORTANT: Replace every <placeholder> in the `Plan Format` with the requested value. Add as much detail as needed to implement the feature successfully.
- Use your reasoning model: THINK HARD about the feature requirements, design, and implementation approach.
- Follow existing patterns and conventions in the codebase. Don't reinvent the wheel.
- Design for extensibility and maintainability.
- Respect requested files in the `Relevant Files` section.
- Start your research by reading the `README.md` file.

## Relevant Files

Focus on the following files:
- `README.md` - Project overview and install instructions.
- `build.sh` - Main macOS build script (downloads, signs, packages binaries).
- `build-linux.sh` - Linux packaging script (.deb/.rpm via nfpm).
- `build-package.sh` - Full build+sign+release orchestrator.
- `sign.sh` - Code signing and notarization.
- `release.sh` - GitHub release publishing.
- `common.sh` - Shared build utilities and helpers.
- `KhaosFoundation.pkgproj` - macOS installer project config.
- `nfpm.yaml` - Linux package specification.
- `packaging/macos/` - macOS installer scripts and LaunchAgent plist.
- `packaging/linux/` - Linux systemd unit and postinstall scripts.
- `scripts/` - WFL daemon helper scripts.
- `docs/releases/` - Versioned release notes.

Ignore all other files in the codebase.

## Plan Format

```md
# Feature: <feature name>

## Feature Description
<describe the feature in detail, including its purpose and value to the packaging system>

## User Story
As a <type of user>
I want to <action/goal>
So that <benefit/value>

## Problem Statement
<clearly define the specific problem or opportunity this feature addresses>

## Solution Statement
<describe the proposed solution approach and how it solves the problem>

## Relevant Files
Use these files to implement the feature:

<find and list the files that are relevant to the feature describe why they are relevant in bullet points. If there are new files that need to be created to implement the feature, list them in an h3 'New Files' section.>

## Implementation Plan
### Phase 1: Foundation
<describe the foundational work needed before implementing the main feature>

### Phase 2: Core Implementation
<describe the main implementation work for the feature>

### Phase 3: Integration
<describe how the feature will integrate with existing build/packaging pipeline>

## Step by Step Tasks
IMPORTANT: Execute every step in order, top to bottom.

<list step by step tasks as h3 headers plus bullet points. use as many h3 headers as needed to implement the feature. Order matters, start with the foundational shared changes required then move on to the specific implementation. Include verification steps throughout. Your last step should be running the `Validation Commands` to validate the feature works correctly with zero regressions.>

## Testing Strategy
### Manual Verification
<describe manual verification steps for the feature>

### Build Pipeline Verification
<describe how to verify the feature works across the build pipeline>

### Edge Cases
<list edge cases that need to be tested>

## Acceptance Criteria
<list specific, measurable criteria that must be met for the feature to be considered complete>

## Validation Commands
Execute every command to validate the feature works correctly with zero regressions.

<list commands you'll use to validate with 100% confidence the feature is implemented correctly with zero regressions. every command must execute without errors so be specific about what you want to run to validate the feature works as expected.>
- `shellcheck *.sh` - Lint all shell scripts
- `./build.sh` - Verify macOS build completes successfully
- `./build-linux.sh --arch amd64` - Verify Linux build completes successfully

## Notes
<optionally list any additional notes, future considerations, or context that are relevant to the feature that will be helpful to the developer>
```

## Feature
$ARGUMENTS
