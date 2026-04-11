# bnto-vector

SVG processing library — rasterization and optimization.

## Overview

`bnto-vector` provides two capabilities:

1. **Rasterization** — Convert SVG to raster images (PNG, JPEG, WebP) via resvg/usvg/tiny-skia
2. **Optimization** — Strip editor cruft from SVG files (metadata, comments, unused namespaces, empty groups) via roxmltree/xmlwriter

Both use libraries already in the WASM binary via the resvg transitive dependency chain — zero additional binary size.

## Processors

| Processor         | Node Type          | What It Does                                         |
| ----------------- | ------------------ | ---------------------------------------------------- |
| `VectorRasterize` | `vector-rasterize` | Convert SVG files to raster images (PNG, JPEG, WebP) |
| `OptimizeSvg`     | `vector-optimize`  | Remove editor metadata, comments, and bloat from SVG |

## API

| Export            | What It Does                                                  |
| ----------------- | ------------------------------------------------------------- |
| `VectorRasterize` | NodeProcessor for SVG to raster conversion                    |
| `OptimizeSvg`     | NodeProcessor for SVG optimization (9 XML-level passes)       |
| `rasterize_svg`   | Parse SVG bytes and render to a pixel buffer at a DPI         |
| `optimize_svg`    | Parse SVG string and return optimized SVG with passes applied |

## Directory Structure

```
src/
├── lib.rs                # Public exports
├── common.rs             # Shared helpers (SVG accepts list, param definitions)
├── processor.rs          # VectorRasterize processor
├── rasterize.rs          # rasterize_svg() + VectorError + RasterizeOptions
└── optimize/
    ├── mod.rs            # optimize_svg() entry point, config, constants
    ├── processor.rs      # OptimizeSvg NodeProcessor
    ├── write.rs          # Tree-walking writer (roxmltree → xmlwriter)
    └── xml_passes.rs     # Pure pass decision functions
```

## Development

```bash
cargo test -p bnto-vector           # Native unit tests
task wasm:lint                      # Clippy
```
