# SVG to Image Node

> Convert SVG files to raster images (PNG, JPEG, WebP).

**Category:** vector | **Platforms:** browser | **Container:** no

## Accepts

- `image/svg+xml`

## Parameters

| Parameter | Type | Default | Range | Description |
|-----------|------|---------|-------|-------------|
| format | enum | png | [object Object], [object Object], [object Object] | The target raster format to convert SVG to |
| quality | number | 80 | 1-100 | Output quality (1 = lowest, 100 = highest). Applies to JPEG; PNG is lossless; WebP is lossless-only. |

## Configuration Example

```json
{
  "type": "vector-rasterize",
  "parameters": {
      "format": "png",
      "quality": 80
  }
}
```

---

*Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate.*
