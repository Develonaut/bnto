# bnto-vector

SVG rasterization library and processor. Converts SVG files to raster images.

## Overview

`bnto-vector` provides pure Rust SVG rasterization via resvg/usvg/tiny-skia, and a `NodeProcessor` implementation (`vector-rasterize`) that converts SVG input to PNG, JPEG, or WebP output. Encoding uses the shared `bnto-encode` crate.

## Processors

| Processor         | Node Type          | What It Does                                         |
| ----------------- | ------------------ | ---------------------------------------------------- |
| `VectorRasterize` | `vector-rasterize` | Convert SVG files to raster images (PNG, JPEG, WebP) |

## API

| Export            | What It Does                                          |
| ----------------- | ----------------------------------------------------- |
| `VectorRasterize` | NodeProcessor for SVG to raster conversion            |
| `rasterize_svg`   | Parse SVG bytes and render to a pixel buffer at a DPI |

## Directory Structure

```
src/
├── lib.rs            # Public exports
├── common.rs         # Shared helpers (SVG accepts list, param definitions)
├── processor.rs      # VectorRasterize processor
└── rasterize.rs      # rasterize_svg() + VectorError + RasterizeOptions
```

## Development

```bash
cargo test -p bnto-vector           # Native unit tests
task wasm:lint                      # Clippy
```
