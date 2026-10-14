# Export Items as CSV Node

> Write the files passing through, with their metadata, as one CSV row each.

**Category:** spreadsheet | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| path | string | - | - | File to write, one row per file passing through; its folder must exist |
| name_column | string | file | - | Header of the first column, which holds each file's name |
| overwrite | boolean | false | - | Replace a CSV already at the path; off keeps it (it may be an earlier run's record) and fails the run |

## Configuration Example

```json
{
  "type": "spreadsheet-export",
  "parameters": {
      "path": "<string>",
      "name_column": "file",
      "overwrite": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
