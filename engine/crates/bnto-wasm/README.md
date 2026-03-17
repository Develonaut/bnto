# bnto-wasm

Unified WebAssembly entry point — the single binary loaded by the browser.

## Overview

`bnto-wasm` is the only `cdylib` in the workspace. It links all node crates (`bnto-image`, `bnto-csv`, `bnto-file`) and `bnto-core` into one `.wasm` binary with a shared heap. The browser loads this binary in a Web Worker via `wasm-bindgen`.

## Directory Structure

```
src/
├── lib.rs          # Public API — setup(), version(), re-exports all node crates
├── execute.rs      # execute_pipeline() — WASM bridge for full pipeline execution
└── catalog.rs      # node_catalog() — serialized registry metadata for codegen
benches/
├── pipeline_bench.rs   # Criterion pipeline benchmarks (native only)
└── recipe_bench.rs     # Criterion recipe benchmarks (native only)
tests/
├── wasm.rs                        # Basic WASM integration tests
├── recipe_integration.rs          # Full recipe execution tests
├── recipe_integration_data.rs     # Output data validation
└── recipe_integration_events.rs   # Progress event validation
```

## WASM Exports

```rust
#[wasm_bindgen] pub fn setup()              // One-time initialization (panic hook)
#[wasm_bindgen] pub fn version() -> String  // Engine version string
#[wasm_bindgen] pub fn greet(name: &str)    // Health check
#[wasm_bindgen] pub fn execute_pipeline()   // Run a full pipeline definition
#[wasm_bindgen] pub fn node_catalog()       // Get registry metadata as JSON
```

The JS layer (`@bnto/core` browser adapter) calls `execute_pipeline()` with a serialized definition and file bytes. Progress events stream back via a callback.

## Why One Binary

- **Single shared heap** — no cross-crate WASM boundary overhead
- **Smaller total size** — shared dependencies (serde, image codecs) deduplicate
- **Simpler loading** — one fetch, one instantiation, one Web Worker
- **Atomic versioning** — the binary version matches all node capabilities

## Build Output

`wasm-pack build` produces `pkg/`:

```
pkg/
├── bnto_wasm_bg.wasm    # The binary
├── bnto_wasm.js          # JS glue code
├── bnto_wasm.d.ts        # TypeScript declarations
└── package.json          # npm package metadata
```

Copied to `apps/web/public/wasm/` for browser loading.

## Development

```bash
task wasm:build         # Release build (size-optimized)
task wasm:build:dev     # Dev build (faster compilation, better errors)
task wasm:test          # All tests (native + WASM integration)
task wasm:bench         # Criterion benchmarks (native only)
```

## Testing

Integration tests verify the full pipeline path through the WASM boundary:

- **Recipe tests** — execute predefined recipes end-to-end, verify output bytes
- **Event tests** — verify progress event ordering and completeness
- **Data tests** — validate output file formats (magic bytes, dimensions, metadata)
