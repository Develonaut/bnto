# Overlay Image Node

> Overlay an image onto source images at a configurable position, size, and opacity.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| overlay | file | - | - | The image to overlay (base64-encoded). |
| position | enum | bottom-right | [object Object], [object Object], [object Object], [object Object], [object Object], [object Object], [object Object], [object Object], [object Object] | Where to place the overlay on the image. |
| size | number | 25 | 1-500 | Overlay width as a percentage of the source image width. |
| opacity | number | 80 | 0-100 | Overlay transparency (0 = invisible, 100 = fully opaque). |
| offsetX | number | 0 | -500-500 | Horizontal pixel offset from the position. Positive = right, negative = left. |
| offsetY | number | 0 | -500-500 | Vertical pixel offset from the position. Positive = down, negative = up. |
| quality | number | 80 | 1-100 | Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added. |

## Configuration Example

```json
{
  "type": "image-overlay",
  "parameters": {
      "overlay": "<file>",
      "position": "bottom-right",
      "size": 25,
      "opacity": 80,
      "offsetX": 0,
      "offsetY": 0,
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
