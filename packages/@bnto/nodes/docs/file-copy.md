# Copy Files Node

> Copy output files into a destination folder, with conflict handling.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| destination | string | - | - | Folder to copy files into. |
| createDirs | boolean | true | - | Automatically create the destination folder if it doesn't exist. |
| conflict | enum | skip | skip, overwrite, rename, newer | What to do when a file with the same name already exists. "newer" copies only when the destination is missing or older than the source (path input only), keeps the source's modified time, and never deletes. A copy that would replace a file another incoming file is read from is refused before anything is copied (inside a loop, each iteration's files are checked on their own). |
| toleranceMs | number | 2000 | ≥ 0 | Modified-time gap treated as equal in "newer" mode; FAT and Google Drive round times. |

## Configuration Example

```json
{
  "type": "file-copy",
  "parameters": {
      "createDirs": true,
      "conflict": "skip",
      "toleranceMs": 2000
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
