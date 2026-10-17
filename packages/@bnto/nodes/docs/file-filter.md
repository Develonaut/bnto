# Filter Files Node

> Keep or drop files by extension, name or path pattern, and size.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| extensions | string | - | - | Comma-separated list of file extensions to keep (e.g., "jpg,png,svg"). Leave empty to allow all extensions. |
| name_pattern | string | - | - | Glob pattern to match against filenames (e.g., "photo*", "*.backup.*"). Uses glob syntax with * and ? wildcards. |
| pattern_mode | enum | glob | glob, regex | How to interpret the name pattern. |
| match | enum | name | name, path | What the name pattern is tested against: the file's name, or its whole relative path (folders separated by /). |
| keep | boolean | true | - | When on, files matching every criterion are kept and the rest dropped. When off, matching files are dropped and the rest kept. |
| min_size | number | 0 | ≥ 0 | Minimum file size in bytes. Files smaller than this are dropped. |
| max_size | number | 0 | ≥ 0 | Maximum file size in bytes. Files larger than this are dropped. 0 means no limit. |

## Configuration Example

```json
{
  "type": "file-filter",
  "parameters": {
      "pattern_mode": "glob",
      "match": "name",
      "keep": true,
      "min_size": 0,
      "max_size": 0
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
