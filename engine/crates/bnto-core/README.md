# bnto-core

Foundation crate for the bnto engine. Types, traits, pipeline executor, and node registry.

## Overview

`bnto-core` defines the contracts that all node crates implement and the executor that runs pipelines. It has **no WASM dependencies**. It compiles to native Rust for tests and CLI, or links into the `bnto-wasm` cdylib for browser execution.

Every other engine crate depends on `bnto-core`.

## Key Concepts

### NodeProcessor Trait

Every node implements this trait:

```rust
pub trait NodeProcessor: Send + Sync {
    fn metadata(&self) -> NodeMetadata;
    fn process(&self, input: NodeInput) -> Result<NodeOutput, BntoError>;
}
```

- `metadata()` - self-describes the processor: type, operation, accepted formats, parameters with constraints
- `process()` - transforms input bytes into output bytes. Stateless; all config comes via the params map in `NodeInput`

### NodeRegistry

Maps compound keys (`"nodeType:operation"`) to `Box<dyn NodeProcessor>`. The executor looks up processors by key at runtime.

```rust
registry.register("image", "compress", Box::new(CompressImages));
let processor = registry.get("image", "compress")?;
```

### Pipeline Executor

`execute_pipeline()` is the main entry point. It:

1. Parses the `PipelineDefinition` (deserialized from `.bnto.json`)
2. Walks nodes in dependency order
3. Chains output from one node as input to the next
4. Handles container semantics (loop iterates per-file, group sequences children, parallel fans out)
5. Emits `PipelineEvent`s for progress reporting
6. After a node fails, skips the rest of its chain except closing steps (nodes marked `"always": true`), which run on the files the failed step handed over (`BntoError::with_output`); the run still fails with the first error, and `PipelineFailed` comes once, last. See `.claude/strategy/engine-execution.md#closing-steps-always`

### Recipe Check

`check_pipeline()` checks a `PipelineDefinition` against a registry without running it, walking the nodes as the executor does (I/O nodes skipped, container children entered). Each `CheckFinding` names the node, its type and, where one is at fault, the param. It reports an unknown node type, each message the node's `validate()` returns, and a malformed placeholder: an unclosed `{{`, or a `{{namespace.` outside `fields`, `env`, `ctx`, `node`, `item` and `vars`. It also reports `always` on an I/O node, where it has no effect, and every edge problem at every level (the root and each container's children): a cycle (named as `a → b → a`), an edge to an unknown id or one at another level, a duplicate edge, an edge into `input` or out of `output`, and a node no edge touches. These come from `graph::level_errors()`, which lists every error `level_flow()` would refuse with (`level_flow` returns the first), so check and run never disagree. Each is reported on the node at fault and names the edge. Placeholders are checked for form only, so no vars or inputs are needed. It returns a `CheckReport`: the `findings`, plus `run_time`, a `RunTimeParam` per param left for `bnto run -p` to give. A param the recipe leaves out (no key, or a required one given blank text or null) is needed when the run refuses without it and stops refusing once it has a value. The run refuses what `validate()` refuses, and a required param with no value even when `validate()` lets it through (`required_params::missing_required`, which a run's pre-flight refuses before any step starts); it is left for run time when `-p` can give it, the way the CLI applies one: the node's id is non-empty without `:` or `=`, no earlier node (searching each container's children before the next node) has that id, the node declares no field of that name, and the param is not an object (`-p` gives text, numbers or true/false). A needed param that fails any of these is a finding saying why no run can give it. It also flags each placeholder in the output node's `directory` that no run can fill before it starts (see below).

### Output Folder

`fill_output_directory()` fills the output node's `directory` before a run, since the CLI and TUI make that folder before any step: the node's fields in scope, `{{env.*}}` for a key the recipe declares in `secrets`, every `{{ctx.*}}` but `started`, and the input node's file metadata. `{{vars.*}}` is the consumer's to fill first (the CLI does). It returns `Ok(None)` when no output node names a folder, and an `Err` with one line per placeholder left over or filled with empty text, so the caller stops before making the folder. A filled value is never scanned again.

### PipelineEvent

Structured progress events consumed by the JS layer:

- `PipelineStarted { total_nodes, total_files }`
- `NodeStarted { node_id, node_type, operation }`
- `FileProgress { node_id, file_index, total_files, file_name }`
- `NodeProgress { node_id, done, total, message }` — a node's own live report, throttled; `total` is `None` when the node can't know it (nodes call `ProgressReporter::report` or `report_count`)
- `NodeCompleted { node_id, duration_ms }`
- `PipelineCompleted { total_duration_ms }`
- `PipelineError { node_id, message }`

### ParameterDef

Self-describing parameter metadata used by the TypeScript codegen to generate UI controls:

- Name, type, default value, constraints (min/max/step/options)
- Conditional visibility (`visible_when` - show param only when another param has a specific value)
- Surfaceability flag (whether the param appears in the config panel)

## Directory Structure

```
src/
├── lib.rs                # Public API: re-exports all modules
├── errors.rs             # BntoError enum (thiserror)
├── events.rs             # PipelineEvent tagged union for progress reporting
├── metadata.rs           # NodeMetadata, ParameterDef, Constraints, NodeCategory
├── processor.rs          # NodeProcessor trait, NodeInput, NodeOutput
├── progress.rs           # ProgressReporter (target-agnostic closures): percent, count, command output
├── progress_throttle.rs  # Clock-injected throttle for live progress
├── registry.rs           # NodeRegistry: compound key dispatch
├── pipeline.rs           # PipelineDefinition, PipelineNode, PipelineFile
├── pipeline_edge.rs      # PipelineEdge: a connection between two sibling nodes
├── pipeline_settings.rs  # PipelineSettings: iteration mode and the run's time limit (`timeout`)
├── graph/                # level_flow(): validate one level's edges, Kahn order, file routing (not yet used by the executor)
├── run_summary.rs        # RunSummary (serializable run outcome) + NodeTally (per-node counts from events)
├── definition.rs         # Document-shape types for .bnto.json (authoring view)
├── definition_schema.rs  # JSON Schema generator for .bnto.json
├── slash_path.rs         # slash_path(): relative path -> `/`-joined output name on every OS
├── required_params.rs    # missing_required(): required params left out (no default) or blank; pre-flight refuses them
├── time.rs               # rfc3339_utc() / parse_rfc3339_utc(): epoch ms <-> RFC 3339 UTC
├── control/              # Control processors: assert (fail the run when a comparison does not hold), report (print a templated line)
├── data/                 # Data processors: edit-fields (set item fields from templates over the others), summarize (one item per group, with its count and field totals)
├── check/                # check_pipeline(): unknown types, validate() refusals, placeholder form, edges, params left for run time
├── output_directory/     # fill_output_directory(): the output folder filled before a run, or refused
└── executor/
    ├── mod.rs            # Public API + node dispatch logic
    ├── chain.rs          # Node chaining: each output feeds the next
    ├── closing.rs        # After a failure: skip nodes, run `always` closing steps
    ├── primitive.rs      # Leaf node execution
    ├── container.rs      # Container execution (loop, group, parallel)
    ├── time_limit.rs     # The run's time limit, checked before each node, file and iteration
    └── tests/            # 13 test modules, 45+ tests
```

## Development

```bash
# Run unit tests for this crate
cargo test -p bnto-core

# Run all workspace tests
task wasm:test:unit

# Lint
task wasm:lint
```

## Testing

Extensive executor tests in `src/executor/tests/` (13 modules, 45+ test functions):

- `basic.rs` - simple pipeline execution
- `containers.rs` - loop/group/parallel node semantics
- `errors.rs` - error propagation and recovery
- `metadata.rs` - processor self-description
- `progress.rs` - event emission ordering
- `recipes.rs` - full recipe integration tests
- `sub_pipelines.rs` - nested pipeline execution
