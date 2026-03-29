# Watermark Node

> Overlay a watermark image onto source images.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Parameters

| Parameter | Type   | Default      | Range                                                                                                        | Description                                                                                                                 |
| --------- | ------ | ------------ | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| watermark | file   | —            | —                                                                                                            | The watermark image to overlay (base64-encoded).                                                                            |
| position  | enum   | bottom-right | top-left, top-center, top-right, middle-left, center, middle-right, bottom-left, bottom-center, bottom-right | Where to place the watermark on the image.                                                                                  |
| size      | number | 25           | 1–100                                                                                                        | Watermark width as a percentage of the source image width.                                                                  |
| opacity   | number | 80           | 0–100                                                                                                        | Watermark transparency (0 = invisible, 100 = fully opaque).                                                                 |
| offsetX   | number | 10           | 0–500                                                                                                        | Horizontal offset from the edge in pixels.                                                                                  |
| offsetY   | number | 10           | 0–500                                                                                                        | Vertical offset from the edge in pixels.                                                                                    |
| quality   | number | 80           | 1–100                                                                                                        | Output quality (1 = lowest, 100 = highest). WebP is lossless-only; quality has no effect until lossy WebP support is added. |

## Configuration Example

```json
{
  "type": "image-watermark",
  "parameters": {
    "watermark": "<file>",
    "position": "bottom-right",
    "size": 25,
    "opacity": 80,
    "offsetX": 10,
    "offsetY": 10,
    "quality": 80
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
