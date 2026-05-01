# Filter Files Node

> Drop files that don't match extension, pattern, or size criteria.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| extensions | string | - | - | Comma-separated list of file extensions to keep (e.g., "jpg,png,svg"). Leave empty to allow all extensions. |
| name_pattern | string | - | - | Glob pattern to match against filenames (e.g., "photo*", "*.backup.*"). Uses glob syntax with * and ? wildcards. |
| pattern_mode | enum | glob | glob, regex | How to interpret the name pattern. |
| min_size | number | 0 | ≥ 0 | Minimum file size in bytes. Files smaller than this are dropped. |
| max_size | number | 0 | ≥ 0 | Maximum file size in bytes. Files larger than this are dropped. 0 means no limit. |

## Configuration Example

```json
{
  "type": "file-filter",
  "parameters": {
      "pattern_mode": "glob",
      "min_size": 0,
      "max_size": 0
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
