# bnto-image

Image processing nodes. Compress, resize, convert, overlay, and strip metadata from images in the browser via WASM.

## Overview

`bnto-image` provides `NodeProcessor` implementations for image transformation. All codecs are pure Rust (no system dependencies), WASM-compatible, and run entirely client-side. Supports JPEG, PNG, and WebP formats with automatic format detection via magic bytes. The convert processor also accepts SVG input, rasterizing via `bnto-vector` before encoding.

Format detection and raster encoding live in [`bnto-encode`](../bnto-encode/) — a shared crate that both `bnto-image` and `bnto-vector` depend on.

## Processors

| Processor            | Node Type          | What It Does                                                                                                        |
| -------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------- |
| `CompressImages`     | `image-compress`   | Re-encodes at a target compression level (1-100). PNG uses quantization (median cut + dithering) for ~57% reduction |
| `ResizeImages`       | `image-resize`     | Scales to target dimensions with aspect ratio control. Lanczos3 filter                                              |
| `ConvertImageFormat` | `image-convert`    | Converts between JPEG, PNG, and WebP with quality control. Also accepts SVG input (rasterized via `bnto-vector`)    |
| `OverlayImage`       | `image-overlay`    | Composites a watermark/overlay onto images with configurable position, size, and opacity                            |
| `StripExif`          | `image-strip-exif` | Removes all EXIF metadata (GPS, camera info, timestamps) via decode/re-encode                                       |

## Directory Structure

```
src/
├── lib.rs            # Public exports + re-exports from bnto-encode
├── common.rs         # Shared helpers (accepts list, quality param extraction)
├── compress.rs       # CompressImages processor
├── resize.rs         # ResizeImages processor
├── convert.rs        # ConvertImageFormat processor (incl. SVG input)
├── overlay.rs        # OverlayImage processor
├── strip_exif.rs     # StripExif processor
├── svg.rs            # SVG detection and rasterization (convert-only)
├── orientation.rs    # EXIF orientation handling
├── quantize.rs       # PNG quantization wrapper (quantizr)
├── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
└── test_utils.rs     # Test fixture helpers
```

## Key Dependencies

- `bnto-encode` - shared image format detection (`ImageFormat`) and encoding (`encode_image()`)
- `image` (0.25) - pure Rust codecs for JPEG, PNG, WebP (no rayon, WASM incompatible)
- `bnto-vector` (0.1) - SVG rasterization via resvg/usvg/tiny-skia (used by convert processor)
- `quantizr` (1.4) - PNG quantization via median cut algorithm
- `png` (0.18) - low-level PNG encoder for indexed color output

## Development

```bash
cargo test -p bnto-image          # Native unit tests
task wasm:test                    # Full WASM integration tests
```
