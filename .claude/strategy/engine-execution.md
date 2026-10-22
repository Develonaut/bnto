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

1. **Resolve each level's flow** -- a level (the root, or a container's children) with edges is ordered topologically; one without edges runs in `nodes` order. Bad edges (a cycle, an unknown or other-level node, into `input`, out of `output`, a repeat, an unconnected node) fail the run before `PipelineStarted`. See [Edges and Data Flow](#edges-and-data-flow).
2. **Identify I/O nodes** -- `input` and `output` nodes are structural markers. The executor reads their config (accepted types, output format) but does not "process" them.
3. **Walk nodes in order** -- for each processing node, determine its container semantics and dispatch to the registered `NodeProcessor`, on the files its incoming edges carry.
4. **Emit progress events** -- structured events at each stage (see taxonomy below).
5. **Collect results** -- the files on the edges into `output` (in edge order) are returned to the caller; without edges, the last processing node's files.

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
- **A failed step's output:** a processor fails with files by returning `BntoError::with_output(error, NodeOutput)` (variant `FailedWithOutput`, which displays as the inner error). A per-file node's handed-over files are every file it made before failing plus what the failing call handed over. A step that fails without output (a bad param, a tool missing from `searchPath`) hands over no files, and the closing steps start from none. `shell-run` uses this: a command that exits non-zero hands over its captured stdout in stdout mode (named and tagged as on success, within `maxOutputSize`), and the files it wrote in file mode. The native context reports a non-zero exit as `BntoError::CommandFailed { message, stdout }`, which keeps the stdout and displays exactly as the old `ProcessingFailed` message.
- **Verdict:** the run's error is the first failure, unchanged, so the CLI still ends `Pipeline failed: <first error>` and exits 1. A closing step that also fails is joined after it (`BntoError::ClosingStepFailed`): `<first error>; closing step '<id>' also failed: <its error>`. Later closing steps still run, starting from the files the failed closing step handed over.

**Scope:**

- **Containers:** a group's or loop iteration's children follow the same rule among themselves. A failed container hands the enclosing chain the files its own chain ended with (the failed step's, or its last closing step's), so the enclosing chain's closing steps receive them. A container marked `always` runs as a closing step, its children normally. Loop `onError: "continue"` is unchanged: an iteration's closing steps run, then the iteration counts as failed.
- **Auto-iteration:** a failure inside a run closes that run's own chain first (a per-file sequence closes the failing file's pass; other files' passes don't run, as before). The `always` nodes of every later run then run as one explicit chain on what it handed over, so a per-file closing step there gets all those files in one node call.
- **I/O markers:** `always` on an `input` / `output` node has no effect; the output step writes nothing on a failed run. `bnto check` reports it.

- **Levels with edges:** after the first failure at a level, every step that hasn't run is skipped, branches that don't depend on the failure included, and passes its input through; `always` steps run in execution order on what their incoming edges carry. A closing step after a fan-in gets the failed branch's handed-over files and each skipped branch's input, in edge order. The level then hands over its output sources' files, so a failed group passes its sinks' files to the parent's closing steps (`executor/flow_closing.rs`).

**Events:** closing steps emit `NodeStarted` / `NodeCompleted` / `NodeFailed` as usual, numbered by their position (so the CLI's step counter skips the skipped nodes). Skipped nodes emit nothing. `NodeFailed` is emitted for every failing node as before. `PipelineFailed` is emitted once per failed run, last, after every closing step: its `node_id` is the top-level node whose failure is the verdict (the container's id when the failure was inside one) and its `error` is the verdict message. Before closing steps, `PipelineFailed` was emitted at every node failure, including inside containers and in loop iterations that `onError: "continue"` then skipped.

### Conditional Steps (`when`)

**Decision (October 2026):** a node can carry `"when": { "value", "op", "expected" }` beside `always`, like GitHub Actions `if:` or Ansible `when:`. The step runs only when the comparison holds. See [conditional-steps.md](../decisions/conditional-steps.md) for the options weighed.

- **Comparison:** `op` is `equals`, `not-equals`, `after` or `before`, the same words and code as `assert` (`bnto-core/src/compare.rs`). `value` and `expected` take templates: the fields in scope (below), `{{ctx.platform}}`, `{{env.*}}`, `{{node.<id>.*}}`, `{{item.*}}`. A number from a sole placeholder is compared as its text.
- **When:** decided in `execute_node` just before the node would start (`executor/guard.rs`), so per file under auto-iteration and per iteration in a loop.
- **Skip:** the node emits `NodeSkipped { node_id, node_index, total_nodes, node_type, parent_node_id, reason }` instead of `NodeStarted`, hands its input files on unchanged, and records no `{{node.<id>.*}}` metadata. With edges, its consumers get those files. A skipped container never starts its children. A skip is not a failure.
- **With `always`:** a false `when` skips a closing step too.
- **Errors:** an unknown op fails the run at that node (`NodeFailed`, then the usual verdict). `bnto check` reports it earlier, along with malformed placeholders in the guard and `when` on an I/O node.
- **Pre-flight ignores guards:** consent lists every shell command (SEC-16) and the raw param checks walk every branch, so a branch for another OS can't hide a refused step.
- **Run summary:** `NodeTally` keeps a skipped top-level node's reason as `skip_reason`, so `bnto run --json` carries it.
- **Shared decision:** `When::decide_in(&TemplateContext)` resolves and compares; the executor and the CLI's `bnto dry-run` both call it, so a preview can't disagree with a run.

### Fields in Scope

**Decision (October 2026):** a container's `fields` are visible to every node inside it, so an if/else pair of guarded steps can share one field. The nearest declaration wins (a step's own field shadows its group's), and a group's fields never reach nodes after it. `PipelineContext.fields` carries the values the enclosing containers declare; `fields_for(node)` adds the node's own (`executor/field_scope.rs`). Leaf params, `when` guards, `bnto check` and the CLI dry run all read fields this way. `bnto run -p key=value` sets the node that declares `key`, and asks for `nodeId:key` when two do. See [conditional-steps.md](../decisions/conditional-steps.md#fields-in-scope-pr-5).

### Recipe OS Support

**Decision (October 2026):** a recipe's operating systems (`os`) are worked out from its platform guards (`when {{ctx.platform}} equals|not-equals <literal>`), never hand-written: a step runs on an OS when every guard on its path allows it, and a recipe runs where some root step does (`bnto_core::os_support::recipe_os`). `bnto run` refuses an unsupported host before consent and before any step; `bnto info`, `bnto list`, the TUI and the catalog (`Recipe.os`) show it. `os` is a separate axis from node-type `platforms` (browser / cli / server). The same rule scopes dependencies: a node's `requires` (and its processor's) are checked, by pre-flight, `bnto doctor` and `bnto install`, only on the OSes it runs on (`bnto_engine::scoped_deps`). See [conditional-steps.md](../decisions/conditional-steps.md#os-support-pr-6) and [Dependencies in scope](../decisions/conditional-steps.md#dependencies-in-scope-pr-8).

### Edges and Data Flow

**Decision (October 2026):** the executor honors a recipe's `edges`. Before, it fed every node the previous node's output whatever the edges said, so a step couldn't start from no files (a `file-list` after another node read that node's files as path lists) and couldn't take files from two earlier steps. This works the way Node-RED, NiFi and n8n's Merge do. Built in #611 (graph model), #614 (executor), #616 (closing steps), #613 (`bnto check`) and #612 (TS validation).

- **When:** each level (the root, or a container's children) follows its own `edges` when the list is non-empty. A missing or empty list runs in `nodes` order, as before. A level whose edges are the chain its nodes already form runs through exactly the old code path, so the change moved no bundled recipe, fixture or golden.
- **Order:** topological, ties broken by `nodes` order.
- **A step's input:** the outputs of every incoming edge, joined in the order the edges are listed in `edges` (not `nodes` order). List `notes → join` before `listing → join` to put the notes first.
- **Fresh starts:** a step with no incoming edge starts with no files. At a level with an `input` node, edges out of `input` carry the run's files. At a level without one (container children), the first node with no incoming edge, in `nodes` order, gets the level's files (the loop iteration's file, or the group's batch).
- **Fan-out:** one output feeding several steps is cloned for all but the last consumer in execution order, which takes it. A `FileData::Path` clone copies only the path, so the branches share the file on disk.
- **A level's output:** at the root, the files on edges into `output`, in edge order; a step wired to nothing after it contributes no files (its `{{node.*}}` record is still kept). In a container, the outputs of its sinks (nodes with no outgoing edge), in `nodes` order.
- **Refused before `PipelineStarted`**, and reported by `bnto check` from the same rules (`graph::level_errors` / `level_flow`): a cycle, an edge to an unknown node or one at another level, into `input`, out of `output`, a repeated edge, and a node no edge touches.
- **Records:** a node reads `{{node.<id>.*}}` of any node that ran before it in execution order, at its level and enclosing ones.
- **Auto-iteration:** a branching level runs as linear segments, each through the existing chain or auto-iteration code. Fan-in and fan-out points are barriers like batch nodes, so adding a branch to an auto recipe changes where its per-file passes split.
- **Events:** `PipelineStarted.nodes` lists nodes in execution order, and `node_index` is the execution position.

**Watch for:** a step that moves or deletes a shared file (`file-move`, `file-delete`, an overwriting output) takes it from branches that run after it. They then fail naming the missing file. A recipe saved by the frozen editor, which never writes edges, keeps its old edges when a node is added, so the run is refused, naming the unconnected node, until the editor wires edges.

### Run Time Limit (`settings.timeout`)

**Decision (October 2026):** a recipe limits its whole run with `"settings": { "timeout": <seconds> }` (a whole number above 0; `PipelineSettings::timeout`, refused at parse otherwise, so `bnto check` and `bnto run` report it). No limit when unset, and nothing changes for such a run. The limit is enforced in layers:

- **Engine (every consumer):** the executor reads the limit from the definition and the run's `now_ms`, and checks it before every node, every file of a per-file node, and every loop iteration starts, and once more when the run ends (`executor/time_limit.rs`). Past it, it fails with `BntoError::TimedOut { secs }` ("Run timed out after <secs>s"). A timeout ends the run: no `always` closing step starts after it, and a loop's `onError: "continue"` never skips it like a failed item (`BntoError::is_timeout` sees through `FailedWithOutput` / `ClosingStepFailed`). Browser WASM gets this as is: it uses the same executor with `Date.now()`.
- **CLI (a step stuck mid-run):** the engine can't stop a step that is already running. `bnto run`, the TUI and `bnto dry-run`'s pipeline preview run the pipeline through `run_watchdog::watch` (`bnto/src/run_watchdog.rs`), which arms a watchdog thread for the run's limit. When it passes, the watchdog kills every registered child process tree (`ProcessRegistry::terminate_all`: process groups on unix, Job Objects on Windows), which unblocks a step waiting on a command. The run then reports the timeout itself, whatever the step returned. A run that doesn't come back within a 2s grace (stuck on something that isn't a child process) is reported for by the watchdog, after killing again: `bnto run` prints the same verdict line and exits 1, the TUI shows the timeout as the run's error (its worker thread is left to finish on its own), and the dry-run preview prints its usual `pipeline preview failed:` warning and exits 1, since it can't finish printing.
- **Browser (a WASM step stuck mid-run):** `definitionToPipeline` carries `settings.timeout` into the pipeline it sends to WASM, and `@bnto/core`'s browser execution service races the run against it (`lib/withTimeLimit.ts`). When it passes, the service terminates the Web Worker (the only way to stop running WASM) and the run fails with the engine's message, "Run timed out after <secs>s". The next run lazily starts a fresh worker. Runs sharing that worker fail too.

`bnto run --timeout <seconds>` overrides the recipe's limit by writing it into the definition (`bnto/src/run_limit.rs`), so both layers use one value. The limit covers the whole run, including pre-flight dependency checks. A shell-run's own `timeout` still stops just that command.

**Not yet:** per-node timeouts beyond shell-run's.

---

## Progress Event Taxonomy

The executor emits structured progress events. Each event has a `pipeline_id`, `timestamp`, and event-specific data.

| Event               | When                                                   | Data                                                                   |
| ------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------- |
| `PipelineStarted`   | Execution begins                                       | `pipeline_id`, `total_nodes`, `total_files`                            |
| `NodeStarted`       | A node begins processing                               | `node_id`, `node_type`                                                 |
| `FileProgress`      | Progress on a single file within a node                | `node_id`, `file_index`, `file_name`, `bytes_processed`, `bytes_total` |
| `NodeProgress`      | A running node's own live report (throttled, 100ms)    | `node_id`, `done`, `total` (`null` when unknown), `message`            |
| `NodeCompleted`     | A node finishes successfully                           | `node_id`, `files_processed`, `duration_ms`                            |
| `NodeFailed`        | A node encounters an error                             | `node_id`, `error`, `file_index` (if file-specific)                    |
| `PipelineCompleted` | All nodes finished successfully                        | `total_duration_ms`, `files_processed`                                 |
| `PipelineFailed`    | Once, last, when a run fails (after its closing steps) | `node_id` (top-level node whose failure is the verdict), `error`       |

In the browser, `bnto-wasm` bridges these events to a JS callback. `@bnto/core` maps them to Zustand store updates or DOM-direct attribute changes (for high-frequency `FileProgress` events).

### Live node progress

**Decision (October 2026):** every node reports live progress through its `ProgressReporter`, the same way for every node type: `report(percent, msg)` when it knows how far through it is, `report_count(count, msg)` when it can't know the total (files found during a folder walk, rows read from a stream). The executor relays both as `NodeProgress` (a percent is `done` of `total: 100`; a count has `total: null`) and throttles them to one per 100ms per node run, using the run's injected clock (so it works in WASM). Nodes report as often as they like and never throttle themselves. `CommandOutput` lines are not throttled.

The CLI draws the step's live line from `FileProgress` and `NodeProgress` (`bnto/src/progress_line.rs` decides, `live_bar.rs` draws): a bar of files finished when the step spans several files, otherwise a spinner with the node's running message, or a bar when the node reports a percent. The line ticks on its own (spinner and elapsed time), so a step stuck on one long item still visibly moves. indicatif draws nothing when stderr isn't a terminal.

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
- **Edges define data flow.** At a level with edges, the executor orders nodes topologically and routes files along the edges: a step gets its incoming edges' files in edge order, and a step without one starts empty. A level without edges runs in `nodes` order. See [Edges and Data Flow](#edges-and-data-flow).

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

Serializing (`transforms/definitionToPipeline.ts`) drops only what is editor-only (positions, ports, edge handles). It keeps every level's `edges` and the recipe's `settings` (iteration mode and time limit), so a browser run iterates and routes files as `bnto run` does. Dropping `settings.iteration` left output files unchanged for every bundled recipe but changed its progress events: an `auto` run sends `NodeStarted` / `NodeCompleted` per file per node, an `explicit` one once per node.

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
