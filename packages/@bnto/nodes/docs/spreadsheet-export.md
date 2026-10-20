# Export Items as CSV Node

> Export the files passing through, with their metadata, as one CSV row each.

**Category:** spreadsheet | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| path | string | - | - | File to write, one row per file passing through; its folder must exist |
| nameColumn | string | file | - | Header of the first column, which holds each file's name |
| overwrite | boolean | false | - | Replace a CSV already at the path; off keeps it (it may be an earlier run's record) and fails the run |

## Configuration Example

```json
{
  "type": "spreadsheet-export",
  "parameters": {
      "path": "<string>",
      "nameColumn": "file",
      "overwrite": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
