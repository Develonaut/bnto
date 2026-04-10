# Rust Engine

Execution engine for bnto recipes. Compiles to WebAssembly for the browser, native for desktop and CLI.

## Overview

The engine transforms files through pipelines of composable nodes: compress images, clean CSVs, rename files, and more. The same Rust codebase targets multiple runtimes. WASM for browser execution (M1, delivered), native binary for desktop via Tauri (M3, planned), and server-side for premium recipes (M4, planned).

For browser execution, all node crates compile into a single `.wasm` binary (`bnto-wasm`), loaded by a Web Worker and orchestrated by `@bnto/core`. For desktop, the same crates compile as native Rust with no WASM overhead.

## Directory Structure

```
engine/
├── Cargo.toml                # Workspace root: shared deps, WASM-optimized profiles
├── rust-toolchain.toml       # Pinned stable toolchain + wasm32-unknown-unknown target
├── catalog.snapshot.json     # Generated node catalog (consumed by @bnto/nodes codegen)
├── .cargo/config.toml        # Cargo aliases (fmt-check, lint)
└── crates/
    ├── bnto-core/            # Foundation: types, traits, pipeline executor, registry
    ├── bnto-image/           # Image nodes: compress, resize, convert
    ├── bnto-spreadsheet/    # Spreadsheet nodes: clean, convert, merge, rename
    ├── bnto-file/            # File nodes: rename
    └── bnto-wasm/            # Unified cdylib entry point (produces the .wasm binary)
```

## Crate Dependency Graph

```
bnto-wasm (cdylib, the only crate that produces .wasm)
    ├── bnto-image (rlib)
    ├── bnto-spreadsheet (rlib)
    ├── bnto-file  (rlib)
    └── bnto-core  (rlib, foundation for all above)
```

Node crates depend on `bnto-core` for the `NodeProcessor` trait, error types, and pipeline infrastructure. `bnto-wasm` links them all into a single shared-heap binary.

## Key Concepts

- **`NodeProcessor` trait** - contract every node implements: `metadata()` for self-description, `process()` for execution
- **`NodeRegistry`** - maps compound keys (`nodeType:operation`, e.g. `"image:compress"`) to processor instances
- **`PipelineExecutor`** - walks the node graph, chains I/O between nodes, handles containers (loop/group/parallel), emits structured progress events
- **`PipelineEvent`** - tagged union for progress reporting: `PipelineStarted`, `NodeStarted`, `FileProgress`, `NodeCompleted`, `PipelineCompleted`, etc.
- **`catalog.snapshot.json`** - engine self-description file generated from the registry; consumed by `@bnto/nodes` codegen to produce TypeScript types and schemas

## WASM Compilation

Only `bnto-wasm` is a `cdylib`. It's the single entry point compiled by `wasm-pack`. Node crates are `rlib` (Rust libraries) that link into the unified binary.

**Build output** lands in `crates/bnto-wasm/pkg/`:

- `bnto_wasm_bg.wasm` - the binary
- `bnto_wasm.js` - JS glue code
- `bnto_wasm.d.ts` - TypeScript declarations
- `package.json` - npm package metadata

The web app loads this from `apps/web/public/wasm/`.

## Development

```bash
# Build
task wasm:build           # Release build (web target, size-optimized)
task wasm:build:dev       # Dev build (faster, better error messages)

# Test
task wasm:test            # Rust unit tests + WASM integration tests
task wasm:test:unit       # Native unit tests only (fast, no JS runtime)

# Quality
task wasm:lint            # clippy - must pass clean
task wasm:fmt             # Auto-format
task wasm:fmt:check       # Check formatting (CI)

# Benchmark
task wasm:bench           # Criterion benchmarks (native only)

# Cleanup
task wasm:clean           # Remove build artifacts
```

## Testing Strategy

Tests run at two layers:

1. **Native unit tests** (`cargo test`) - pure Rust logic, no WASM runtime. Fast. Covers processor logic, pipeline execution, metadata validation.
2. **WASM integration tests** (`wasm-pack test --node`) - tests the Rust-JS boundary via `wasm-bindgen-test`. Runs in Node.js.

Each node crate has `tests/` with WASM integration tests. `bnto-core` has extensive executor tests in `src/executor/tests/` (~45 test functions).

## Adding a New Node Type

1. Create a new crate in `crates/` (or add an operation to an existing crate)
2. Implement `NodeProcessor` - define `metadata()` and `process()`
3. Register the processor in the crate's `register()` function
4. Re-export from `bnto-wasm/src/lib.rs`
5. Run `task wasm:test` to verify
6. Regenerate the catalog: the snapshot updates automatically on build
7. Run `@bnto/nodes` codegen to produce TypeScript types
