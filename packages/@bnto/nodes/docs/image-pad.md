# Pad Images Node

> Pad images onto a larger, centered canvas, filling the added space with transparency or a color.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| width | number | - | 1-16384 | Canvas width in pixels. Leave empty to keep the image's width. |
| height | number | - | 1-16384 | Canvas height in pixels. Leave empty to keep the image's height. |
| background | string | transparent | - | Fill for the added space: "transparent" or a hex color (#fff, #ffffff, #ffffff80). JPEG has no transparency, so a transparent fill comes out white. |
| quality | number | 80 | 1-100 | Output quality (1 = lowest, 100 = highest). JPEG and WebP are encoded lossily at this quality. |

## Configuration Example

```json
{
  "type": "image-pad",
  "parameters": {
      "background": "transparent",
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
