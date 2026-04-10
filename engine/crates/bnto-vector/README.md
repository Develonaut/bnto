# bnto-vector

SVG rasterization library. Converts SVG bytes to raster pixels.

## Overview

`bnto-vector` provides pure Rust SVG rasterization via resvg/usvg/tiny-skia. It parses SVG input, scales to a configurable DPI, and renders to a `tiny_skia::Pixmap` (RGBA premultiplied). No coupling to the image processing pipeline — this is a standalone library consumed by `bnto-image` for SVG input support.

## API

| Function         | What It Does                                          |
| ---------------- | ----------------------------------------------------- |
| `rasterize_svg`  | Parse SVG bytes and render to a pixel buffer at a DPI |

## Directory Structure

```
src/
├── lib.rs            # Public exports
└── rasterize.rs      # rasterize_svg() + VectorError + RasterizeOptions
```

## Development

```bash
cargo test -p bnto-vector           # Native unit tests
task wasm:lint                      # Clippy
```
