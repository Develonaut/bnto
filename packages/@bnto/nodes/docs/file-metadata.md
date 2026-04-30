# File Metadata Node

> Extract file metadata (size, extension, MIME type, hash) and attach to output.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter    | Type    | Default | Range | Description                                             |
| ------------ | ------- | ------- | ----- | ------------------------------------------------------- |
| include_hash | boolean | false   | -     | Compute and include a SHA-256 hash of the file content. |

## Configuration Example

```json
{
  "type": "file-metadata",
  "parameters": {
    "include_hash": false
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
