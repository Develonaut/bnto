# Engine Execution Architecture

**Created:** March 2026
**Status:** Canonical reference -- informs all execution work
**Related:** [architecture.md](../rules/architecture.md), [core-api.md](../rules/core-api.md), [editor-architecture.md](editor-architecture.md), [io-nodes.md](io-nodes.md)

---

## Core Principle: Engine Owns Execution

The Rust engine (`bnto-core`) owns the full pipeline executor -- graph walking, file iteration, container node semantics, control flow, progress reporting. JS / `@bnto/core` is a **thin adapter** that converts browser types (File to bytes, Definition to WASM-compatible structs) and relays progress events to the UI.

```
Engine (Rust)                        JS / @bnto/core
--------------------------------------------  ----------------------------------------
Graph walking                                 File -> ArrayBuffer -> Vec<u8>
Node ordering (topological)                   Definition -> WASM struct
Container node semantics (loop/group)         Progress events -> Zustand/DOM
Per-file iteration                            Download result bytes
NodeProcessor dispatch                        Error surface to UI
Progress event emission
```

**Why engine-side:** The executor must work identically across all consumers -- browser (WASM), CLI (native binary), desktop (Tauri), server (future). If JS orchestrates execution, every new consumer needs its own orchestrator. If the engine owns it, consumers are thin I/O adapters.

---

## Multi-Consumer Design

The same `bnto-core` executor runs in every target. Only the I/O bridge differs.

| Target          | Crate                   | I/O Bridge                | File Source                          |
| --------------- | ----------------------- | ------------------------- | ------------------------------------ |
| Browser (WASM)  | `bnto-wasm`             | `wasm-bindgen` JS interop | `File` -> `ArrayBuffer` -> `Vec<u8>` |
| CLI (native)    | `bnto-cli` (planned)    | `std::fs`                 | Local filesystem paths               |
| Desktop (Tauri) | `bnto-tauri` (planned)  | Tauri commands            | Local filesystem via Tauri API       |
| Server          | `bnto-server` (planned) | HTTP/gRPC                 | R2 presigned URLs -> bytes           |

**`bnto-core` has zero WASM dependencies.** It is a pure Rust `rlib` crate. The `bnto-wasm` crate is the only `cdylib` -- it bridges `bnto-core` types and the executor to JavaScript via `wasm-bindgen`. This separation means `bnto-core` compiles natively for CLI/desktop/server with no conditional compilation for WASM.

---

## Crate Responsibilities

```
bnto-core (rlib)          bnto-wasm (cdylib)           Node crates (rlib)
------------------        -------------------          -------------------
PipelineExecutor          JS <-> Rust bridge           NodeProcessor impls
NodeProcessor trait       wasm-bindgen exports         Per-type file transforms
ProgressEvent types       File/bytes conversion        bnto-image, bnto-csv,
Graph resolution          Progress callback bridge       bnto-file, etc.
Container semantics       Error serialization
Topological ordering
```

---

## Pipeline Executor

The executor receives a fully-resolved node graph (a `Definition` with all nodes, edges, and configs). It does not fetch, parse, or look up recipes -- that is the caller's job.

### Execution Steps

1. **Resolve graph** -- topological sort of nodes based on edges. Detect cycles (error).
2. **Identify I/O nodes** -- `input` and `output` nodes are structural markers. The executor reads their config (accepted types, output format) but does not "process" them.
3. **Walk nodes in order** -- for each processing node, determine its container semantics and dispatch to the registered `NodeProcessor`.
4. **Emit progress events** -- structured events at each stage (see taxonomy below).
5. **Collect results** -- output bytes from the final processing node (or output node's predecessor) are returned to the caller.

### How NodeProcessor Fits

`NodeProcessor` is the trait each node crate implements. The executor calls it per-file, per-node:

```rust
trait NodeProcessor {
    fn process(&self, input: &[u8], config: &NodeConfig) -> Result<Vec<u8>, ProcessError>;
}
```

The executor handles iteration (looping over files), ordering (which node runs first), and container semantics (how files flow through groups). `NodeProcessor` only knows about transforming a single input into a single output.

---

## Container Node Semantics

Container nodes control how files flow through their children.

| Container Type | Behavior                                                                                       | Example                                                           |
| -------------- | ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| **loop**       | Iterates children once per input file. Each file passes through all child nodes independently. | Default for most recipes -- compress each image separately.       |
| **group**      | Passes the full batch of files to children. Children see all files at once.                    | CSV merge -- combine multiple CSVs into one.                      |
| **parallel**   | Same as group for now. Reserved for future concurrent execution.                               | Future optimization -- process files on separate threads/workers. |

Non-container nodes are leaves -- they receive files and transform them via their `NodeProcessor`.

---

## Progress Event Taxonomy

The executor emits structured progress events. Each event has a `pipeline_id`, `timestamp`, and event-specific data.

| Event               | When                                    | Data                                                                   |
| ------------------- | --------------------------------------- | ---------------------------------------------------------------------- |
| `PipelineStarted`   | Execution begins                        | `pipeline_id`, `total_nodes`, `total_files`                            |
| `NodeStarted`       | A node begins processing                | `node_id`, `node_type`                                                 |
| `FileProgress`      | Progress on a single file within a node | `node_id`, `file_index`, `file_name`, `bytes_processed`, `bytes_total` |
| `NodeCompleted`     | A node finishes successfully            | `node_id`, `files_processed`, `duration_ms`                            |
| `NodeFailed`        | A node encounters an error              | `node_id`, `error`, `file_index` (if file-specific)                    |
| `PipelineCompleted` | All nodes finished successfully         | `total_duration_ms`, `files_processed`                                 |
| `PipelineFailed`    | Pipeline aborted due to error           | `failed_node_id`, `error`, `nodes_completed`                           |

In the browser, `bnto-wasm` bridges these events to a JS callback. `@bnto/core` maps them to Zustand store updates or DOM-direct attribute changes (for high-frequency `FileProgress` events).

---

## Relationship to Recipe Definitions

The executor receives a `Definition` -- a self-describing node graph with metadata, nodes, edges, and per-node configs. Key points:

- **I/O nodes are structural markers.** The executor reads `input` node config to know accepted file types and `output` node config to know the output format. It does not "run" them -- they declare the contract.
- **The executor does not look up recipes.** It receives a fully-resolved graph. Whether the definition came from a predefined recipe, a user-created recipe, or a community marketplace is irrelevant.
- **Edges define data flow.** The executor uses edges to build the topological order. Nodes without incoming edges (other than input) are roots. The executor walks the graph from input to output.

---

## Backward Compatibility

Existing per-file WASM functions (`compress_image`, `clean_csv`, `rename_file`) remain as the public `wasm-bindgen` exports. They are not removed or deprecated -- they are convenience wrappers.

Internally, the executor calls the same `NodeProcessor` implementations that back these functions. The migration path:

1. **Today:** JS calls per-file WASM functions directly (one call per file, JS handles iteration).
2. **Target:** JS calls `run_pipeline()` once with the full definition + all file bytes. The engine handles iteration, ordering, and progress internally.
3. **Both paths coexist.** Per-file functions remain for simple single-node use cases and testing. The pipeline executor is the primary path for recipe execution.

---

## JS Adapter Role (`@bnto/core`)

The JS side is deliberately thin:

| Responsibility                                         | Where                     |
| ------------------------------------------------------ | ------------------------- |
| Convert `File` objects to `ArrayBuffer` / `Uint8Array` | Browser adapter           |
| Serialize `Definition` to WASM-compatible format       | Browser adapter           |
| Call `run_pipeline()` (single WASM invocation)         | `core.executions` service |
| Receive progress callback, route to store/DOM          | Execution hooks           |
| Convert output bytes to downloadable `Blob`            | Download service          |
| Surface errors to UI                                   | Error boundary            |

`executePipeline.ts` (the JS-side orchestrator that loops over files and calls per-file WASM functions) is **deprecated** in favor of the Rust executor. It remains only as a fallback during migration.

---

## Testing Strategy

The engine executor is tested at three layers:

| Layer                         | What                                                                 | How                                                                                                                                          |
| ----------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **Unit (Rust native)**        | NodeProcessor per-type, graph resolution, container semantics        | `#[cfg(test)]` blocks in `bnto-core`. Fast, no JS.                                                                                           |
| **Integration (Rust native)** | Full pipeline execution with real recipe definitions                 | Recipe test suite in `bnto-core/tests/`. Loads `.bnto.json` fixtures, feeds test files, asserts outputs. Proves the engine works standalone. |
| **WASM integration**          | JS <-> Rust bridge, progress callback delivery                       | `wasm-bindgen-test` in `bnto-wasm/tests/`. Runs in Node.js.                                                                                  |
| **E2E (Playwright)**          | Full browser pipeline: drop file -> WASM executes -> download result | `apps/web/e2e/journeys/browser/`. Programmatic assertions on output (magic bytes, file sizes).                                               |

**The integration test suite is the proof.** If all recipe fixtures pass through the Rust executor natively with correct outputs, the engine works. The WASM and E2E layers only verify the bridge and UI integration.
