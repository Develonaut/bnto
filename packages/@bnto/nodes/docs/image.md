# Image Node

> Image processing: compress, resize, and convert formats.

**Category:** image | **Platforms:** browser | **Container:** no

## Accepts

- `image/jpeg`
- `image/png`
- `image/webp`

## Operations

### compress

Reduce image file size while maintaining quality

Accepts: `image/jpeg`, `image/png`, `image/webp`

### convert

Convert images between JPEG, PNG, and WebP formats

Accepts: `image/jpeg`, `image/png`, `image/webp`

### resize

Change image dimensions while maintaining quality

Accepts: `image/jpeg`, `image/png`, `image/webp`

## Parameters

| Parameter      | Type    | Default | Range                     | Visible When              | Description                                           |
| -------------- | ------- | ------- | ------------------------- | ------------------------- | ----------------------------------------------------- |
| operation      | enum    | —       | compress, convert, resize | always                    | Processing operation                                  |
| quality        | number  | 80      | 1–100                     | compress, resize, convert | Output quality (1 = lowest, 100 = highest)            |
| format         | enum    | —       | jpeg, png, webp           | convert                   | The target image format to convert to                 |
| width          | number  | —       | ≥ 1                       | resize                    | Target width in pixels                                |
| height         | number  | —       | ≥ 1                       | resize                    | Target height in pixels                               |
| maintainAspect | boolean | true    | —                         | resize                    | Keep the original width-to-height ratio when resizing |

## Configuration Example

```json
{
  "type": "image",
  "parameters": {
    "operation": "compress",
    "quality": 80
  }
}
```

---

_Auto-generated from engine catalog v1.0.0. Run `task nodes:generate` to regenerate._
