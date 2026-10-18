# Decode Files Node

> Decode each file's content from a text encoding (base64) into its bytes.

**Category:** file | **Platforms:** browser | **Container:** no

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| encoding | enum | base64 | base64 | The text encoding each file is in. base64 is the standard alphabet (+ and /), padded or not; whitespace and line breaks are ignored. URL-safe base64 (- and _) is not accepted. |
| filename | string | - | - | Name for each decoded file. {{name}} is the input's name without its last extension and {{ext}} that extension, e.g. {{name}}.json. Leave empty to keep the input's name. |

## Configuration Example

```json
{
  "type": "file-decode",
  "parameters": {
      "encoding": "base64"
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
