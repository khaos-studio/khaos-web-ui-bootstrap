---
name: docusaurus-docs-management
description: Create, update, and maintain project documentation using Docusaurus, ensuring accuracy by deriving content from actual source code, inline comments, and real implementation details.
---

# Docusaurus Documentation Management Skill

This skill governs how to create and maintain high-quality project documentation using **Docusaurus**, with documentation grounded in **real implementation details**, not assumptions.

Use this skill whenever the user asks to:
- Create or update Docusaurus documentation
- Document features, APIs, architecture, or workflows
- Sync documentation with code changes
- Convert code comments or implementation details into docs

---

## Core Principles

1. **Implementation-first documentation**
   - Never invent APIs, behaviors, or flows.
   - All documentation must be derived from:
     - Source code
     - Inline comments
     - Config files
     - Actual runtime behavior described by the user

2. **Docs reflect reality**
   - If implementation details are missing or unclear, ask the user to provide:
     - Relevant files
     - Code snippets
     - Configuration
   - Do not guess.

3. **Docs stay maintainable**
   - Prefer concise explanations tied directly to code.
   - Avoid marketing language unless explicitly requested.

---

## Docusaurus Project Structure Rules

Assume a standard Docusaurus layout unless the user specifies otherwise:

```

docs/
intro.md
architecture/
api/
guides/
reference/
sidebars.js
docusaurus.config.js

```

When documenting:
- Place conceptual docs in `architecture/`
- Place developer workflows in `guides/`
- Place API and config details in `api/` or `reference/`

---

## Workflow When Creating or Updating Docs

When a documentation task is requested:

1. **Identify scope**
   - New page, update existing page, or restructure docs
   - Ask which section if not specified

2. **Inspect implementation**
   - Read relevant source files
   - Extract:
     - Function/class responsibilities
     - Inputs/outputs
     - Constraints
     - Side effects
   - Use inline comments as primary explanation sources

3. **Translate code → documentation**
   - Explain *what the code does* and *why it exists*
   - Include code snippets only when they clarify behavior
   - Keep snippets minimal and accurate

4. **Validate consistency**
   - Ensure docs match current implementation
   - If code and docs conflict, flag it explicitly

---

## Writing Style Guidelines

- Use **clear, technical language**
- Prefer:
  - “This function validates…”
  - “This module is responsible for…”
- Avoid:
  - Vague statements
  - Assumptions about future behavior
  - Restating obvious code without explanation

---

## API & Code Documentation Rules

When documenting APIs, modules, or functions:

- Always include:
  - Purpose
  - Inputs (types if visible)
  - Outputs / side effects
  - Error cases (if observable in code)
- Base everything on the actual implementation
- If comments are sparse, summarize behavior conservatively

Example structure:

```

## Function: createUser()

**Source:** `src/users/createUser.ts`

### Responsibilities

* Validates input payload
* Persists user record
* Emits creation event

### Notes

* Throws if validation fails
* Does not handle authentication

```

---

## Keeping Docs in Sync With Code

When code changes are mentioned:

1. Identify affected documentation pages
2. Update only what is impacted
3. Call out breaking changes explicitly
4. Do not silently change documented behavior

If unsure whether docs need updating, ask before proceeding.

---

## What NOT to Do

- Do not fabricate undocumented features
- Do not infer intent beyond what code shows
- Do not rewrite large doc sections without confirming scope
- Do not mix unrelated concepts on a single page

---

## Optional Enhancements (Only If Requested)

- Diagrams (based strictly on real architecture)
- Migration notes
- Versioned docs strategies
- Doc sidebar reorganization

---

## Activation Reminder

This skill activates when the user asks about:
- Docusaurus documentation
- Project docs maintenance
- Documenting code, APIs, or architecture
- Keeping docs aligned with implementation
