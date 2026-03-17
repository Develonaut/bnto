# bnto-core

Foundation crate for the bnto engine — types, traits, pipeline executor, and node registry.

## Overview

`bnto-core` defines the contracts that all node crates implement and the executor that runs pipelines. It has **no WASM dependencies** — it compiles to native Rust for tests and CLI, or links into the `bnto-wasm` cdylib for browser execution.

Every other engine crate depends on `bnto-core`.

## Directory Structure

```
src/
├── lib.rs                # Public API — re-exports all modules
├── errors.rs             # BntoError enum (thiserror)
├── events.rs             # PipelineEvent tagged union for progress reporting
├── metadata.rs           # NodeMetadata, ParameterDef, Constraints, NodeCategory
├── processor.rs          # NodeProcessor trait, NodeInput, NodeOutput
├── progress.rs           # ProgressReporter (target-agnostic closures)
├── registry.rs           # NodeRegistry — compound key dispatch
├── pipeline.rs           # PipelineDefinition, PipelineNode, PipelineFile
├── definition_schema.rs  # JSON Schema generator for .bnto.json
└── executor/
    ├── mod.rs            # Public API + node dispatch logic
    ├── primitive.rs      # Leaf node execution
    ├── container.rs      # Container execution (loop, group, parallel)
    └── tests/            # 13 test modules, 45+ tests
```

## Key Concepts

### NodeProcessor Trait

Every node implements this trait:

```rust
pub trait NodeProcessor: Send + Sync {
    fn metadata(&self) -> NodeMetadata;
    fn process(&self, input: NodeInput) -> Result<NodeOutput, BntoError>;
}
```

- `metadata()` — self-describes the processor: type, operation, accepted formats, parameters with constraints
- `process()` — transforms input bytes into output bytes. Stateless — all config comes via the params map in `NodeInput`

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

### PipelineEvent

Structured progress events consumed by the JS layer:

- `PipelineStarted { total_nodes, total_files }`
- `NodeStarted { node_id, node_type, operation }`
- `FileProgress { node_id, file_index, total_files, file_name }`
- `NodeCompleted { node_id, duration_ms }`
- `PipelineCompleted { total_duration_ms }`
- `PipelineError { node_id, message }`

### ParameterDef

Self-describing parameter metadata used by the TypeScript codegen to generate UI controls:

- Name, type, default value, constraints (min/max/step/options)
- Conditional visibility (`visible_when` — show param only when another param has a specific value)
- Surfaceability flag (whether the param appears in the config panel)

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

- `basic.rs` — simple pipeline execution
- `containers.rs` — loop/group/parallel node semantics
- `errors.rs` — error propagation and recovery
- `metadata.rs` — processor self-description
- `progress.rs` — event emission ordering
- `recipes.rs` — full recipe integration tests
- `sub_pipelines.rs` — nested pipeline execution
