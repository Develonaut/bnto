# Delete Files Node

> Delete the files flowing in, each under a base folder; missing files are skipped and counted.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| base | string | - | - | Folder every deleted file must be under. A file outside it (including through ".." or a link) refuses the run, so nothing is deleted. |

## Configuration Example

```json
{
  "type": "file-delete",
  "parameters": {
      "base": "<string>"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
