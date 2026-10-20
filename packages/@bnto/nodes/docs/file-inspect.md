# Inspect Files Node

> Inspect files and attach their metadata (folder, size, extension, MIME type, modified time, hash) to the output.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| includeHash | boolean | false | - | Compute and include a SHA-256 hash of the file content. |

## Configuration Example

```json
{
  "type": "file-inspect",
  "parameters": {
      "includeHash": false
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
