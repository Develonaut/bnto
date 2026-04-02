# Strip EXIF Node

> Remove all EXIF metadata from images (GPS, camera info, timestamps).

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| quality | number | 80 | 1-100 | Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added. |

## Configuration Example

```json
{
  "type": "image-strip-exif",
  "parameters": {
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
