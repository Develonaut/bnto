# Engine Execution Architecture

**Created:** March 2026
**Status:** Canonical reference -- informs all execution work
**Related:** [architecture.md](../rules/architecture.md), [core-api.md](../scopes/backend/core-api.md), [editor-architecture.md](editor-architecture.md), [io-nodes.md](io-nodes.md)

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
| CLI (native)    | `bnto` (planned)        | `std::fs`                 | Local filesystem paths               |
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

### Node Output Metadata

After a leaf node runs, the executor records its node-level `NodeOutput.metadata` under its node id, and every later node in the run reads it through `{{node.<id>.<key>}}`. Records flow the way files do: along a chain, out of a group to the nodes after it, and out of a loop as every iteration's records merged in order (iterations don't see each other's). A per-file node's record merges its files' metadata in file order. Keys a processor declares as counts (`NodeProcessor::count_keys`, e.g. file-copy's `copied`) add up instead of taking the later value, so the record holds totals for the whole node. The run's result carries every node's final record (`PipelineResult::node_metadata`), which `bnto run --json` reports per node. See [recipe-fields.md](recipe-fields.md#node-output-references) for the template rules.

### Closing Steps (`always`)

**Decision (October 2026):** a node can be marked `"always": true` on the node object, beside `id` / `type` / `parameters` (never inside `parameters`, which belong to the processor). It defaults to false. Like GitHub Actions `if: always()` or a `finally` block, it marks a closing step that runs even after an earlier step failed: keep a tool's log when the tool fails, clean up a temp file whatever the result.

The rule, applied within each chain (`executor/chain.rs`, `executor/closing.rs`):

- **Passing run:** an `always` node runs in order like any node, once.
- **After a failure:** the first failing node's error is recorded. Every later node in the chain that isn't `always` is skipped and emits no events. Each later `always` node runs, in order. The first one receives the files the failed step handed over; each next one receives the previous one's output, as any chain does. Closing steps record their `{{node.<id>.*}}` metadata like any node.
- **A failed step's output:** a processor fails with files by returning `BntoError::with_output(error, NodeOutput)` (variant `FailedWithOutput`, which displays as the inner error). A per-file node's handed-over files are every file it made before failing plus what the failing call handed over. A step that fails without output (a bad param, a tool missing from `search_path`) hands over no files, and the closing steps start from none. `shell-command` uses this: a command that exits non-zero hands over its captured stdout in stdout mode (named and tagged as on success, within `maxOutputSize`), and the files it wrote in file mode. The native context reports a non-zero exit as `BntoError::CommandFailed { message, stdout }`, which keeps the stdout and displays exactly as the old `ProcessingFailed` message.
- **Verdict:** the run's error is the first failure, unchanged, so the CLI still ends `Pipeline failed: <first error>` and exits 1. A closing step that also fails is joined after it (`BntoError::ClosingStepFailed`): `<first error>; closing step '<id>' also failed: <its error>`. Later closing steps still run, starting from the files the failed closing step handed over.

**Scope:**

- **Containers:** a group's or loop iteration's children follow the same rule among themselves. A failed container hands the enclosing chain the files its own chain ended with (the failed step's, or its last closing step's), so the enclosing chain's closing steps receive them. A container marked `always` runs as a closing step, its children normally. Loop `onError: "continue"` is unchanged: an iteration's closing steps run, then the iteration counts as failed.
- **Auto-iteration:** a failure inside a run closes that run's own chain first (a per-file sequence closes the failing file's pass; other files' passes don't run, as before). The `always` nodes of every later run then run as one explicit chain on what it handed over, so a per-file closing step there gets all those files in one node call.
- **I/O markers:** `always` on an `input` / `output` node has no effect; the output step writes nothing on a failed run. `bnto check` reports it.

**Events:** closing steps emit `NodeStarted` / `NodeCompleted` / `NodeFailed` as usual, numbered by their position (so the CLI's step counter skips the skipped nodes). Skipped nodes emit nothing. `NodeFailed` is emitted for every failing node as before. `PipelineFailed` is emitted once per failed run, last, after every closing step: its `node_id` is the top-level node whose failure is the verdict (the container's id when the failure was inside one) and its `error` is the verdict message. Before closing steps, `PipelineFailed` was emitted at every node failure, including inside containers and in loop iterations that `onError: "continue"` then skipped.

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
| `PipelineFailed`    | Once, last, when a run fails (after its closing steps) | `node_id` (top-level node whose failure is the verdict), `error` |

In the browser, `bnto-wasm` bridges these events to a JS callback. `@bnto/core` maps them to Zustand store updates or DOM-direct attribute changes (for high-frequency `FileProgress` events).

### CLI progress display

**Decision (October 2026):** the CLI reporter (`bnto/src/progress.rs`, layout in `progress_indent.rs`) numbers top-level steps against the pipeline and indents container children under their container, keeping the container-relative `node_index`/`total_nodes` the engine reports for events with `parent_node_id` set. Each nesting level adds one indent step (depth is tracked from parent ids); `done` and `FAILED` lines follow the same indent.

```
  [2/3] loop
      [1/1] echo
            done 1 file in 3ms
        done 1 file in 4ms
  [3/3] next
```

The TUI already nests child nodes under their container (`ExecutionMessage::ChildNodeStarted` in `tui/screens/execution.rs`), one level deep.

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

## Smart Iteration

The executor supports two iteration modes, controlled by `settings.iteration` on the `PipelineDefinition`:

- **`explicit` (default):** Execute exactly what's defined. Containers control iteration. This is the existing behavior — zero change.
- **`auto`:** Wrap contiguous per-file processor sequences in implicit per-file loops. Flat recipes (`input → compress → output`) produce identical output to explicit-loop recipes (`input → loop → [compress] → output`).

Auto mode partitions the flat node sequence using `inputCardinality` metadata on each processor. Contiguous `perFile` nodes become an implicit per-file loop; container nodes and future `batch` nodes act as partition barriers.

See [smart-iteration.md](smart-iteration.md) for the full design, golden test equivalence proof, and editor UX.

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
