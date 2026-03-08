# Node Architecture

**Created:** March 2026
**Status:** Canonical reference
**Related:** [engine-execution.md](engine-execution.md), [io-nodes.md](io-nodes.md), [editor-architecture.md](editor-architecture.md)

---

## Overview

Nodes are the building blocks of recipes. Each node has a type, configuration, and connections to other nodes. The `@bnto/nodes` package defines node types, schemas, and validation as pure TypeScript -- engine-agnostic, framework-agnostic.

The Rust engine (`bnto-core`) implements the `NodeProcessor` trait for each node type and owns the pipeline executor that walks the node graph. See [engine-execution.md](engine-execution.md) for the full execution architecture.

---

## Node Types

10 node types defined in `@bnto/nodes`, grouped by category:

| Category      | Type        | Description                                               | Browser Capable |
| ------------- | ----------- | --------------------------------------------------------- | --------------- |
| **io**        | `input`     | Declares what the recipe accepts (file types, text, etc.) | Yes             |
| **io**        | `output`    | Declares what the recipe produces                         | Yes             |
| **image**     | `image`     | Compress, resize, convert images                          | Yes             |
| **file**      | `file`      | Rename, move, organize files                              | Yes             |
| **data**      | `csv`       | Clean, filter, transform CSV data                         | Yes             |
| **data**      | `json`      | Parse, transform, query JSON                              | Yes             |
| **network**   | `http`      | HTTP requests (GET, POST, etc.)                           | Server only     |
| **transform** | `transform` | Generic data transformation                               | Yes             |
| **ai**        | `ai`        | AI/LLM API calls                                          | Server only     |
| **control**   | `group`     | Container -- groups child nodes                           | Yes             |

---

## Node Definition Schema

Each node type has a parameter schema defined in `@bnto/nodes`. Schemas drive:

- **Editor UI** -- auto-generated config panels (controls, labels, constraints, help text)
- **Validation** -- parameter validation at edit time and execution time
- **Defaults** -- sensible defaults for every parameter
- **Conditional visibility** -- `visibleWhen` / `requiredWhen` rules for dynamic forms

```typescript
interface ParameterSchema {
  type: "string" | "number" | "boolean" | "enum" | "file";
  label: string;
  description?: string;
  default?: unknown;
  min?: number;
  max?: number;
  options?: { label: string; value: string }[];
  visibleWhen?: ConditionalRule;
  requiredWhen?: ConditionalRule;
}
```

---

## Container Nodes

Container nodes hold child nodes and control iteration semantics. Three container types exist:

| Type         | Semantics                                | Use Case                                     |
| ------------ | ---------------------------------------- | -------------------------------------------- |
| **loop**     | Process children once per input file     | Default -- each file processed independently |
| **group**    | Pass full file batch to children         | Merge, aggregate, multi-file operations      |
| **parallel** | Same as group (reserved for concurrency) | Future optimization                          |

Container nodes do not have a `NodeProcessor` implementation. The engine executor handles their iteration semantics directly. See [engine-execution.md](engine-execution.md) for details.

---

## I/O Nodes

`input` and `output` are structural markers, not processing nodes. They declare the contract between a recipe and its execution environment.

- **`input`** -- what the recipe accepts (MIME types, extensions, max count, max size)
- **`output`** -- what the recipe produces (format, naming convention)

The executor reads I/O node config but does not "run" them. The execution environment (browser UI, CLI, desktop) uses I/O node declarations to render appropriate widgets (dropzone, download button, etc.).

See [io-nodes.md](io-nodes.md) for the full I/O node design.

---

## Execution Semantics

**The Rust engine owns all execution logic.** The `bnto-core` crate contains the `PipelineExecutor` which handles:

- Topological sorting of the node graph
- Container node iteration (loop per-file, group full-batch)
- Dispatching to `NodeProcessor` implementations per-file, per-node
- Structured progress event emission
- Error handling and propagation

JS / `@bnto/core` is a thin adapter. It converts browser types to engine-compatible formats, makes a single WASM call (`run_pipeline`), and relays progress events to the UI. JS does not walk the graph, iterate files, or make per-node execution decisions.

See [engine-execution.md](engine-execution.md) for the full executor design, progress event taxonomy, and multi-consumer architecture.

---

## Node Registration

Node types are registered in two places:

1. **`@bnto/nodes` (TypeScript)** -- type metadata, parameter schemas, validation rules. Used by the editor, config panels, and recipe validation.
2. **`bnto-core` (Rust)** -- `NodeProcessor` trait implementations. Used by the engine executor at runtime.

Both registries must stay in sync. The `@bnto/nodes` package is the source of truth for node type metadata. The Rust crates implement the processing logic.

---

## Adding a New Node Type

1. Add the type to `@bnto/nodes`: type info, parameter schema, category, browser-capability flag
2. Create a Rust crate (or add to existing crate) implementing `NodeProcessor`
3. Register the processor in `bnto-core`'s executor dispatch
4. If browser-capable, wire through `bnto-wasm` bridge
5. Add unit tests (Rust native) and WASM integration tests
6. Update the editor's `NodePalette` to show the new type
