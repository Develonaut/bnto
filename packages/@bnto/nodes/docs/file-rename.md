# Rename Files Node

> Rename files using patterns, find/replace, case rules, counters, and sanitization.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| sanitize | enum | - | slugify, strip, normalize | Clean the filename before other transforms |
| separator | string | - | - | Character to replace spaces and special characters (used with Sanitize) |
| maxLength | number | 0 | ≥ 0 | Maximum filename length after sanitize (0 = no limit) |
| find | string | - | - | Text or regex pattern to search for in the filename |
| replace | string | - | - | Replacement text (used with Find) |
| case | enum | - | lower, upper, title | Transform the filename to a specific case |
| prefix | string | - | - | Text to prepend to the filename |
| suffix | string | - | - | Text to append before the file extension |
| pattern | string | - | - | Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}) |
| counterStart | number | 1 | ≥ 0 | Starting number for the {{counter}} variable |
| counterPad | number | 0 | 0-10 | Zero-pad width for the counter (e.g., 3 → 001, 002) |
| extension | string | - | - | Replace the file extension (without dot) |

## Configuration Example

```json
{
  "type": "file-rename",
  "parameters": {
      "separator": "-",
      "maxLength": 0,
      "counterStart": 1,
      "counterPad": 0
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
