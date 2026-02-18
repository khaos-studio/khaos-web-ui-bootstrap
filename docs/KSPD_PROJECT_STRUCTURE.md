# KSPD Project Structure Reference

Based on khaos-tui implementation patterns.

## Project Detection (IsKSPD)

A directory is a valid KSPD project if it meets **either** of these conditions:

1. **Directory name ends with `.kspd` extension**
   ```
   /projects/my-screenplay.kspd/
   ```

2. **Contains a `manifest.json` file in the root**
   ```
   /projects/my-project/
   ├── manifest.json
   ├── metadata/
   │   ├── scenes.json
   │   ├── characters.json
   │   └── locations.json
   └── ...
   ```

## Project Metadata

### From Directory
- **Path**: The project directory path
- **LastModified**: Directory's modification time (mtime)

### From manifest.json
```json
{
  "title": "My Screenplay",
  "author": "John Doe",
  ...other fields...
}
```

**Rules:**
- If `manifest.json` exists and has `"title"` field → use it
- Otherwise → use directory name as title
- Remove `.kspd` extension from display title

### Entity Counts

Located in `metadata/` subdirectory:

```
/project-root/
├── metadata/
│   ├── scenes.json       ← Array or {scenes: [...]}
│   ├── characters.json   ← Array or {characters: [...]}
│   ├── locations.json    ← Array or {locations: [...]}
│   └── artifacts/
│       └── story-storage.db (indicates project is "ingested")
```

**Counting logic:**
```rust
// scenes.json can be:
// [scene1, scene2, ...] → count array length
// {scenes: [scene1, scene2, ...]} → count nested array length
// If file missing → count = 0
```

## Phase 2 Simplified Metadata (MVP)

For Phase 2b, we focus on minimum viable project info:

```rust
pub struct Project {
    pub id: String,              // Generated UUID from path
    pub title: String,           // From manifest.json or dir name
    pub path: String,            // Absolute path to project root
    pub scene_count: usize,      // From metadata/scenes.json
    pub modified: i64,           // Directory mtime as Unix timestamp
    pub author: Option<String>,  // From manifest.json (optional)
}
```

**Future (Phase 2c+):**
- `character_count` (from metadata/characters.json)
- `location_count` (from metadata/locations.json)
- `ingested` (if story-storage.db exists)
- `analyzed` (if analysis results exist)
- Git info (repo, remote, status)

## Configuration

### Config File Location
```
~/.config/khaos-ui/config.json  (XDG standard)
```

### Config Schema
```json
{
  "projects_root": "/Users/username/Projects",
  "recent_projects": ["proj-id-1", "proj-id-2", ...],
  "schema_version": 1
}
```

### Fallback Resolution Order
1. Check `KHAOS_PROJECTS_ROOT` environment variable
2. Check `~/.config/khaos-ui/config.json` → `projects_root` field
3. Default to `$HOME/Projects`

## Implementation Patterns (from khaos-tui)

### Discovery Algorithm
```
1. Scan projects_root directory
2. For each subdirectory:
   a. Skip symlinks (use os.Lstat not os.Stat)
   b. Check if IsKSPD(path) → if not, skip
   c. Load ReadProjectMetadata(path)
   d. On error: log warning but continue (non-fatal)
3. Sort results by LastModified (newest first)
4. Return Vec<Project>
```

### Performance Considerations
- **Parallelization**: Use 4 workers for concurrent discovery
- **Caching**: Could cache results with invalidation on file change (Phase 2c)
- **Target**: < 500ms for 100 typical projects on SSD

### Error Handling
- **Missing root**: Return error to user (guide to configure)
- **Permission denied**: Skip project, log warning, continue
- **Invalid manifest.json**: Treat as missing, use directory name
- **Missing metadata files**: Return 0 for missing counts

## Example Project Structure

```
/Users/username/Projects/
├── screenplay-1.kspd/
│   ├── manifest.json              # title: "My Screenplay"
│   ├── metadata/
│   │   ├── scenes.json            # 45 scenes
│   │   ├── characters.json        # 12 characters
│   │   ├── locations.json         # 8 locations
│   │   └── artifacts/
│   │       └── story-storage.db   # Ingested
│   └── [screenplay files]
│
├── draft-project/                 # No .kspd suffix, has manifest.json
│   ├── manifest.json              # title: "Draft Project"
│   ├── metadata/
│   │   ├── scenes.json            # 20 scenes
│   │   ├── characters.json        # 8 characters
│   │   └── locations.json         # 5 locations
│   └── [screenplay files]
│
└── another-project.kspd/
    ├── manifest.json
    ├── metadata/
    │   ├── scenes.json
    │   ├── characters.json
    │   └── locations.json
    └── [screenplay files]
```

## Manifest.json Example

```json
{
  "title": "The Great Heist",
  "author": "Jane Smith",
  "description": "A thrilling heist film",
  "version": "1.0.0",
  "created": "2025-01-15T10:30:00Z",
  "modified": "2025-02-18T14:45:00Z"
}
```

**Minimum required for Phase 2b**: Only `title` is needed
**Author field**: Optional but recommended

## Metadata JSON Formats

### scenes.json - Array Format
```json
[
  {
    "id": "scene_001",
    "number": "1",
    "heading": "INT. COFFEE SHOP - MORNING",
    "duration": "2:30"
  },
  ...
]
```

### scenes.json - Object Format
```json
{
  "scenes": [
    { "id": "scene_001", ... },
    ...
  ]
}
```

### characters.json
```json
[
  {
    "id": "char_001",
    "name": "John Doe",
    "role": "Protagonist"
  },
  ...
]
```

### locations.json
```json
[
  {
    "id": "loc_001",
    "name": "Coffee Shop",
    "description": "Downtown cafe"
  },
  ...
]
```

## Implementation Checklist

- [ ] **Detection**: Implement `is_kspd(path)` check
- [ ] **Metadata Loading**: Read manifest.json and count entities
- [ ] **Config Service**: Load projects_root from env/config/default
- [ ] **Discovery**: Parallel scan with error handling
- [ ] **Sorting**: Sort by modified time (newest first)
- [ ] **Caching**: Optional - cache results locally
- [ ] **Recent Projects**: Track last 5 opened projects
- [ ] **Testing**: Mock filesystem for unit tests

## References

- **khaos-tui discovery.go**: Parallel discovery with 4 workers
- **khaos-tui metadata.go**: Metadata reading patterns
- **khaos-tui config**: XDG-standard config paths
