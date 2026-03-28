# Convert Image Format Node

> Convert images between JPEG, PNG, and WebP formats.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| format | enum | jpeg | jpeg, png, webp | The target image format to convert to |
| quality | number | 80 | 1–100 | Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added. |

## Configuration Example

```json
{
  "type": "image-convert",
  "parameters": {
      "format": "jpeg",
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
