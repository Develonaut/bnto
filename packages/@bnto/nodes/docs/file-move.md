# Move Files Node

> Move files into folders under a destination, by the path in each file's name.

**Category:** file | **Platforms:** cli | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| destination | string | - | - | Folder to move files under. Each file's name is its path inside it (e.g. Pistol/Slide/shot.wav); absolute paths and ".." are refused. |
| createDirs | boolean | true | - | Create missing folders under the destination. Off refuses the run when a folder is missing. |

## Configuration Example

```json
{
  "type": "file-move",
  "parameters": {
      "destination": "<string>",
      "createDirs": true
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
