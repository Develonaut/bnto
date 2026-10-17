# List Files Node

> Turn lists of paths into files under a base folder; every path is listed, found or missing.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| paths | object | - | - | Paths to list, relative to the base folder, added to any read from input files (one per line, or NUL-separated). |
| base | string | - | - | Folder the listed paths are relative to. Defaults to the working directory; absolute paths and ".." are refused. |

## Configuration Example

```json
{
  "type": "file-list",
  "parameters": {}
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
