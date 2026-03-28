# Resize Images Node

> Change image dimensions while maintaining quality.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| width | number | — | ≥ 1 | Target width in pixels |
| height | number | — | ≥ 1 | Target height in pixels |
| maintainAspect | boolean | true | — | Keep the original width-to-height ratio when resizing |
| quality | number | 80 | 1–100 | Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added. |

## Configuration Example

```json
{
  "type": "image-resize",
  "parameters": {
      "maintainAspect": true,
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
