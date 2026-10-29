# Rename Files Node

> Rename files using patterns, find/replace, case rules, counters, and sanitization.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| sanitize | enum | - | slugify, strip, normalize | Clean the file name (not its folders) before other transforms. To fold folders into the name, replace "/" with "-" using Find in an earlier rename step |
| separator | string | - | - | Character to replace spaces and special characters in the file name (used with Sanitize) |
| maxLength | number | 0 | ≥ 0 | Maximum file name length after sanitize, folders not counted (0 = no limit) |
| find | string | - | - | Text or regex pattern to search for in the whole relative path, folders included |
| replace | string | - | - | Replacement text (used with Find) |
| case | enum | - | lower, upper, title | Transform the file name (not its folders) to a specific case |
| prefix | string | - | - | Text to prepend to the file name (not its folders) |
| suffix | string | - | - | Text to append to the file name, before its extension |
| pattern | string | - | - | Template for the whole relative path, folders included (supports {{name}}, {{ext}}, {{index}}, {{date}}, {{counter}}). Replaces the other name transforms; without {{name}} it drops the folders |
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
