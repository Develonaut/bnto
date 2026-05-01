# Copy Files Node

> Place output files in a destination directory with conflict handling.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| destination | string | - | - | Directory path to copy files into. |
| create_dirs | boolean | true | - | Automatically create the destination directory if it doesn't exist. |
| conflict | enum | skip | skip, overwrite, rename | What to do when a file with the same name already exists. |

## Configuration Example

```json
{
  "type": "file-copy",
  "parameters": {
      "create_dirs": true,
      "conflict": "skip"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
