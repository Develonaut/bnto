# Node System Responsibility Matrix

Three layers own different concerns of the node system. This document is the decision matrix for "where does this logic go?"

## The Three Layers

| Layer                | Role                                                                                            | Analogy                                        |
| -------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------- |
| **Engine (Rust)**    | Source of truth. Defines what nodes CAN do, executes them, reports progress                     | The factory floor                              |
| **@bnto/nodes (TS)** | Generated mirror of engine metadata. Types, schemas, recipe data in JS-land                     | The parts catalog (printed from factory specs) |
| **Editor**           | Visual state + user interaction. Renders configs, manages undo/redo, converts to/from ReactFlow | The workbench                                  |

## Decision Matrix

| Question                                                                  | Answer                                                                                     |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Does it define what a node type IS? (params, constraints, capabilities)   | **Engine** — `ParameterDef` in `metadata.rs`                                               |
| Does it execute node logic? (compress, resize, rename)                    | **Engine** — `NodeProcessor` trait impl                                                    |
| Does it validate a Definition structure? (required fields, edge validity) | **Engine** — `definition_json_schema()` generates the schema                               |
| Does it need to be available in TypeScript?                               | **@bnto/nodes** — but as a GENERATED file from engine, not hand-written                    |
| Is it a predefined recipe composition? (which nodes, how they connect)    | **Engine** — `engine/recipes/*.bnto.json` (source of truth, embedded via `include_str!()`) |
| Is it a pure type/interface shared across TS consumers?                   | **@bnto/nodes** — `definition.ts`, `recipe.ts` (hand-written)                              |
| Does it classify node types? (isContainer, isIoNode)                      | **@bnto/nodes** — reads from generated `NODE_TYPE_INFO`                                    |
| Does it manage visual state? (selection, position, undo)                  | **Editor** — store + actions                                                               |
| Does it bridge Definition ↔ ReactFlow?                                    | **Editor** — adapters                                                                      |
| Does it render UI for node configuration?                                 | **Editor** — ConfigPanel, SchemaForm                                                       |

## The Golden Rule

> **If it describes what a node CAN do → Engine.**
> **If it makes engine knowledge available in TypeScript → @bnto/nodes (generated).**
> **If it's about the visual editing experience → Editor.**
> **@bnto/nodes should be mostly generated code + types. Minimize hand-written logic. Recipes are engine-owned (`engine/recipes/`).**

**Import boundary:** `@bnto/nodes` is consumed ONLY by `@bnto/registry`. Editor and core never import from `@bnto/nodes` directly — they import from `@bnto/core` (which re-exports from `@bnto/registry`, which re-exports from `@bnto/nodes`). See [architecture.md](../../rules/architecture.md#import-boundary-rules).

## Engine Node Processor Patterns

For detailed patterns on creating and extending Rust node processors — parameter contracts, shared encoding, testing requirements — see [engine-node-patterns.md](engine-node-patterns.md).

## What Each Layer Contains

### Engine (`engine/crates/`)

- `ParameterDef` — parameter names, types, constraints, defaults, conditional visibility, surfaceability
- `NodeMetadata` — processor self-description (type, accepts, platforms, parameters, inputCardinality)
- `NodeTypeInfo` — type-level metadata (label, category, isContainer, icon)
- `InputCardinality` — per-processor declaration: `perFile` (processes one file at a time) or `batch` (needs all files)
- `NodeProcessor` trait — execution logic per node type
- `PipelineExecutor` — graph walking, topological ordering, progress events
- `definition_json_schema()` — JSON Schema for `.bnto.json` validation

### @bnto/nodes (`packages/@bnto/nodes/`)

**Generated** (from `engine/catalog.snapshot.json` via `generate-from-catalog.ts`):

- `generated/catalog.ts` — `NODE_TYPES`, `NODE_TYPE_INFO`, `PROCESSORS`, `PROCESSOR_MAP`
- `generated/schemas.ts` — Zod schemas + `NodeSchemaDefinition` per engine-backed type
- `generated/definitionSchema.ts` — JSON Schema for `.bnto.json`

**Hand-written** (pure types and compositions):

- `definition.ts` — `Definition`, `Edge`, `Port`, `Metadata` interfaces
- `recipe.ts` — `Recipe`, `AcceptSpec`
- `generated/recipes.ts` — predefined recipe metadata (generated from engine catalog snapshot)
- `isContainerNodeType.ts`, `isIoNodeType.ts` — helpers that READ from generated `NODE_TYPE_INFO`
- `validate.ts` — structural validation (future: migrate to engine-generated JSON Schema via ajv)

### Editor (`packages/editor/`)

- `store/` — Zustand store (nodes, edges, configs, undo/redo, selection)
- `actions/` — pure state mutation functions (addNode, removeNode, updateParams)
- `adapters/` — Definition ↔ RF node conversion (definitionToBento, rfNodesToDefinition)
- `hooks/` — React bindings (useEditorNode, useEditorActions, useEditorExport)
- `components/` — ConfigPanel, SchemaForm, CompartmentNode, Canvas

## Planned Migration: Validation

`validateDefinition()` currently uses hand-written structural checks + generated Zod schemas. The engine already exports `DEFINITION_JSON_SCHEMA` (JSON Schema Draft 2020-12). Future path: replace hand-written structural validation with ajv/JSON Schema validation using the engine-generated schema. This eliminates the need for manual sync between engine and `@bnto/nodes` validation logic.

## Vocabulary: Graph vs Definition

Two concrete terms for the two data representations in the editor:

| Term           | What it is                                                                           | Shape                                                           | Who owns it                                               |
| -------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------- | --------------------------------------------------------- |
| **Definition** | The `.bnto.json` nested tree. What the engine executes. What gets exported/imported. | Recursive `Definition` with `nodes[]` children                  | Engine / `@bnto/nodes` / stored in editor as `definition` |
| **Graph**      | The flat editor working state. What ReactFlow renders, what the user drags.          | `nodes: BentoNode[]` + `edges: Edge[]` + `configs: NodeConfigs` | Editor store                                              |

**There is no overlap.** Top-level node params live in the graph (`configs`). Nested container children live in the definition. On export, `rfNodesToDefinition()` merges both into a complete Definition.

Use these terms consistently in code comments, PR descriptions, and conversation. "Definition" = the nested tree. "Graph" = the flat editor state.

## Common Violations to Watch For

1. **Hardcoded type lists** — e.g., `if (type === "group" || type === "loop")`. Use `isContainerNodeType()` instead, which reads from engine-generated metadata.
2. **Hand-written param schemas** — new node parameters must be added in the engine's `metadata()` impl, then propagated via codegen. Never add params directly to `schemas.ts`.
3. **Business logic in editor** — the editor should never compute what a node CAN do. It reads from `@bnto/nodes` (which reads from engine). The editor only manages visual state and user interactions.
4. **Validation logic in @bnto/nodes that duplicates engine** — structural validation should migrate to engine-generated JSON Schema. Type-specific validation already uses generated Zod schemas.
