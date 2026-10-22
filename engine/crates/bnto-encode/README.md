# bnto-encode

Shared image format detection and encoding for the Bnto engine.

## Overview

`bnto-encode` provides `ImageFormat` (magic byte detection + extension fallback) and `encode_image()` (JPEG/PNG/WebP encoding with quality control; JPEG and WebP are lossy). Extracted from `bnto-image` so that both `bnto-image` and `bnto-vector` can encode raster output without circular dependencies.

## API

| Export         | What It Does                                                                |
| -------------- | --------------------------------------------------------------------------- |
| `ImageFormat`  | Enum (Jpeg, Png, WebP) with `detect()`, `from_magic_bytes()`, `mime_type()` |
| `encode_image` | Encode a `DynamicImage` to bytes in the target format with quality control  |

## Key Dependencies

- `image` (0.25) - JPEG and PNG encoding
- `oxideav-webp` (0.3) - pure Rust lossy WebP (VP8) encoder, builds for `wasm32`. The `image` crate only encodes lossless WebP

## Development

```bash
cargo test -p bnto-encode         # Unit tests (format detection + encoding)
```
