# Rename Files Node

> Transform filenames using patterns, find/replace, and case rules.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| find | string | - | - | Text or regex pattern to search for in the filename |
| replace | string | - | - | Replacement text (used with Find) |
| case | enum | - | lower, upper, title | Transform the filename to a specific case |
| prefix | string | - | - | Text to prepend to the filename |
| suffix | string | - | - | Text to append before the file extension |
| pattern | string | - | - | Template for the output filename (supports {{name}}, {{ext}}, {{index}}, {{date}}) |

## Configuration Example

```json
{
  "type": "file-rename",
  "parameters": {}
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
