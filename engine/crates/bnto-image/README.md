# bnto-image

Image processing nodes. Compress, resize, and convert images in the browser via WASM.

## Overview

`bnto-image` provides three `NodeProcessor` implementations for image transformation. All codecs are pure Rust (no system dependencies), WASM-compatible, and run entirely client-side. Supports JPEG, PNG, and WebP formats with automatic format detection via magic bytes. The convert processor also accepts SVG input, rasterizing via `bnto-vector` before encoding.

## Processors

| Processor            | Node Type        | What It Does                                                                                                        |
| -------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------- |
| `CompressImages`     | `image-compress` | Re-encodes at a target compression level (1-100). PNG uses quantization (median cut + dithering) for ~57% reduction |
| `ResizeImages`       | `image-resize`   | Scales to target dimensions with aspect ratio control. Lanczos3 filter                                              |
| `ConvertImageFormat` | `image-convert`  | Converts between JPEG, PNG, and WebP with quality control. Also accepts SVG input (rasterized via `bnto-vector`)    |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── common.rs         # Shared helpers (accepts list, quality param extraction)
├── compress.rs       # CompressImages processor + metadata
├── resize.rs         # ResizeImages processor + metadata
├── convert.rs        # ConvertImageFormat processor + metadata
├── svg.rs            # SVG detection and rasterization (convert-only)
├── format.rs         # ImageFormat enum (JPEG/PNG/WebP detection via magic bytes)
├── orientation.rs    # EXIF orientation handling
├── quantize.rs       # PNG quantization wrapper (quantizr)
├── wasm_bridge.rs    # #[wasm_bindgen] bridge functions
└── test_utils.rs     # Test fixture helpers
tests/
├── wasm.rs           # Basic WASM integration tests
├── wasm_codec.rs     # Codec-specific tests
├── wasm_convert.rs   # Format conversion tests
├── wasm_resize.rs    # Resize tests
├── wasm_progress.rs  # Progress event tests
└── wasm_stress.rs    # Stress tests
```

## Key Dependencies

- `image` (0.25) - pure Rust codecs for JPEG, PNG, WebP (no rayon, WASM incompatible)
- `bnto-vector` (0.1) - SVG rasterization via resvg/usvg/tiny-skia (used by convert processor)
- `quantizr` (1.4) - PNG quantization via median cut algorithm
- `png` (0.18) - low-level PNG encoder for indexed color output

## Development

```bash
cargo test -p bnto-image          # Native unit tests
task wasm:test                    # Full WASM integration tests
```
