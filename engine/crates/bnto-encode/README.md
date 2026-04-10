# bnto-encode

Shared image format detection and encoding for the Bnto engine.

## Overview

`bnto-encode` provides `ImageFormat` (magic byte detection + extension fallback) and `encode_image()` (JPEG/PNG/WebP encoding with quality control). Extracted from `bnto-image` so that both `bnto-image` and `bnto-vector` can encode raster output without circular dependencies.

## API

| Export         | What It Does                                                                |
| -------------- | --------------------------------------------------------------------------- |
| `ImageFormat`  | Enum (Jpeg, Png, WebP) with `detect()`, `from_magic_bytes()`, `mime_type()` |
| `encode_image` | Encode a `DynamicImage` to bytes in the target format with quality control  |

## Development

```bash
cargo test -p bnto-encode         # Unit tests (format detection + encoding)
```
