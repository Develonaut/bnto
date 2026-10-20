# Copy Files Node

> Copy output files into a destination folder, with conflict handling.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| destination | string | - | - | Folder to copy files into. |
| create_dirs | boolean | true | - | Automatically create the destination folder if it doesn't exist. |
| conflict | enum | skip | skip, overwrite, rename, newer | What to do when a file with the same name already exists. "newer" copies only when the destination is missing or older than the source (path input only), keeps the source's modified time, and never deletes. |
| tolerance_ms | number | 2000 | ≥ 0 | Modified-time gap treated as equal in "newer" mode; FAT and Google Drive round times. |

## Configuration Example

```json
{
  "type": "file-copy",
  "parameters": {
      "create_dirs": true,
      "conflict": "skip",
      "tolerance_ms": 2000
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
