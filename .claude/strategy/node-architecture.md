# Node Architecture: "A Node Is a Node Is a Node"

**Last Updated:** March 2026

---

## Core Insight

Every node — primitive, control flow, or compound — has the same contract: typed inputs in, typed outputs out. From the outside, you can't tell whether a node runs WASM, iterates its children, or delegates to a subgraph. This is what makes the system composable.

A recipe IS a compound node. A group IS a compound node. The system supports arbitrary nesting, even if the UI only exposes flat composition today.

---

## Three-Layer Model

### Layer 1: Engine Primitives

What WASM actually executes. Single file in, single result out. The atoms.

| Node Type     | Operations                      | WASM Function                                                                       |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------- |
| `image`       | `compress`, `resize`, `convert` | `compress_image_combined`, `resize_image_combined`, `convert_image_format_combined` |
| `spreadsheet` | `clean`, `rename`               | `clean_csv_combined`, `rename_csv_columns_combined`                                 |
| `file-system` | `rename`                        | `rename_file_combined`                                                              |

**Dispatch model:** The WASM registry maps `nodeType:operation` compound keys to WASM functions. The `operation` comes from the node's `parameters.operation` field. Example: node type `image` with `parameters.operation = "compress"` resolves to `image:compress` → `compress_image_combined`.

Legacy slug-based keys (`compress-images` → `compress_image_combined`) are preserved for backward compatibility but are not the primary dispatch path.

### Layer 2: Control Flow

Executor-level concerns. WASM doesn't know about these.

| Node Type  | Behavior                                        |
| ---------- | ----------------------------------------------- |
| `loop`     | Iterate children once per input file            |
| `group`    | Execute children sequentially on the full batch |
| `parallel` | Execute children concurrently (future)          |

Control flow nodes have `children: PipelineNode[]` in the execution model. The executor handles them recursively — a `loop` calls `executePipeline` on its children once per file, a `group` calls it once on the full batch.

### Layer 3: Compound Nodes / Recipes

Named subgraphs with defined I/O interfaces.

- A recipe is a root `group` node containing children
- The recipe's `Definition` (from `@bnto/nodes`) is the authoring format
- `flattenDefinition()` converts a `Definition` tree → `PipelineDefinition` for execution
- Casual users see one "Compress Images" node; power users can drill into the subgraph (future UI)

---

## The Node Contract

```
┌─────────────────────────────────┐
│         Every Node              │
│                                 │
│  Input: FileInput[]             │
│  Output: FileResult[]           │
│                                 │
│  Primitive → runs WASM          │
│  Control  → interprets body     │
│  Compound → runs subgraph       │
│                                 │
│  From outside: data in, data out│
└─────────────────────────────────┘
```

Every node type satisfies the same interface. The executor doesn't need special cases for each type — it dispatches based on whether the node has `children` (container) or not (primitive).

---

## WASM Dispatch Model

The WASM registry maps compound keys to functions:

```
Registry Key          → WASM Function
─────────────────────────────────────
"image:compress"      → compress_image_combined
"image:resize"        → resize_image_combined
"image:convert"       → convert_image_format_combined
"spreadsheet:clean"   → clean_csv_combined
"spreadsheet:rename"  → rename_csv_columns_combined
"file-system:rename"  → rename_file_combined
```

Resolution order:

1. Try `nodeType:operation` (e.g., `image:compress`)
2. Fall back to direct key (e.g., `compress-images` — legacy slug)

---

## Recipe Definitions (Simplified)

Browser recipes use a flat 3-node structure: `input → processing → output`. No explicit `loop` nodes — the executor iterates files through each processing node inherently.

```
input → image (operation: "compress") → output
```

The processing node carries its operation in `parameters.operation`. Go template expressions (`{{...}}`) are removed — they were Go engine artifacts that have no meaning in the browser executor.

---

## Recursive Composability

A `Definition` is recursive — `nodes?: Definition[]` allows arbitrary nesting:

```
Recipe (root group)
  ├── input
  ├── group (sub-pipeline)
  │   ├── image (compress)
  │   └── image (resize)
  └── output
```

The editor renders top-level children of the recipe's root group. Future UI can drill into nested groups to reveal internals.

---

## Execution Semantics

### Primitive Nodes

- WASM resolves via `nodeType:operation` → runs function → returns result
- Executor iterates all files through the node

### Loop Nodes (future power-user definitions)

- Has `children: PipelineNode[]`
- Executor runs children sub-pipeline once PER file
- Each iteration gets a single-file batch

### Group Nodes (future power-user definitions)

- Has `children: PipelineNode[]`
- Executor runs children sub-pipeline once on the FULL batch
- Equivalent to a flat sequential pipeline

### Parallel Nodes (future)

- Has `children: PipelineNode[]`
- Executor runs children concurrently (worker pool)

---

## Graph Analysis (Future)

From Atomiton: when definitions support arbitrary edges (not just linear chains), the executor will need:

- **Topological sort** — determine execution order from the edge graph
- **Cycle detection** — reject invalid definitions
- **Fan-in/fan-out** — a node's output feeds multiple consumers, or a node consumes from multiple sources

Currently, recipes use linear chains (edges define order but the executor walks `nodes[]` sequentially). Graph analysis will be needed when the visual editor supports freeform wiring.

---

## Key Files

| File                                                   | Role                                             |
| ------------------------------------------------------ | ------------------------------------------------ |
| `packages/@bnto/nodes/src/recipes/*.ts`                | Recipe definitions (simplified, no loops)        |
| `packages/core/src/adapters/browser/wasmLoader.ts`     | WASM registry with `nodeType:operation` keys     |
| `packages/core/src/adapters/browser/slugCapability.ts` | Slug → `{nodeType, operation}` mapping           |
| `packages/core/src/engine/executePipeline.ts`          | Runtime-agnostic executor with container support |
| `packages/core/src/engine/flattenDefinition.ts`        | `Definition` tree → `PipelineDefinition`         |
| `packages/core/src/engine/types.ts`                    | `PipelineNode` with optional `children`          |
