# @bnto/editor

Visual recipe editor — the editing experience for `.bnto.json` definitions.

## Overview

The editor manages the visual state of a recipe being edited. It bridges the nested `Definition` tree (what the engine executes) and the flat `Graph` (what ReactFlow renders). Built on Zustand for state, pure action functions for mutations, and ReactFlow for the canvas.

Consumed by `apps/web/editor/`. Depends on `@bnto/nodes` for type metadata and `@bnto/core` for execution.

## Vocabulary

Two data representations flow through the editor:

| Term           | What It Is                                                                           | Shape                                                           |
| -------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------- |
| **Definition** | The `.bnto.json` nested tree. What the engine executes. What gets exported/imported. | Recursive `Definition` with `nodes[]` children                  |
| **Graph**      | The flat editor working state. What ReactFlow renders. What the user drags.          | `nodes: BentoNode[]` + `edges: Edge[]` + `configs: NodeConfigs` |

On load, `definitionToGraph()` flattens a Definition into the Graph. On export, `rfNodesToDefinition()` merges the Graph back into a complete Definition.

## Directory Structure

```
src/
├── store/                    # Zustand store
│   ├── createEditorStore.ts  # Store factory
│   ├── types.ts              # EditorState, EditorSnapshot, PanelState
│   ├── captureSnapshot.ts    # Undo snapshot capture
│   └── withUndo.ts           # Undo/redo wrapper
├── actions/                  # Pure state mutation functions
│   ├── addNode.ts            # Router → addTopLevel / addChildIntoContainer / addSiblingChild
│   ├── removeNode.ts         # Remove node + cleanup edges/configs
│   ├── updateParams.ts       # Update node config parameters
│   ├── expandContainer.ts    # Materialize container children into graph
│   ├── executionState.ts     # Per-node execution status
│   └── runExecution.ts       # Start execution flow
├── adapters/                 # Definition ↔ ReactFlow bridge
│   ├── definitionToGraph.ts  # Definition → flat BentoNode[] + edges + configs
│   ├── rfNodesToDefinition.ts # Graph → Definition (export)
│   ├── layoutNodes.ts        # Position calculation (centered rows)
│   └── createCompartmentNode.ts # BentoNode factory
├── hooks/                    # React binding layer
│   ├── factories/            # Hook factories (closures capturing storeApi)
│   ├── useNodes.ts           # Node state subscription
│   ├── useDefinition.ts      # Definition state
│   ├── useExecution.ts       # Execution state
│   └── useHistory.ts         # Undo/redo
├── clients/                  # Public client API (5 domain clients)
│   ├── nodeClient.ts         # Node CRUD operations
│   ├── definitionClient.ts   # Definition import/export
│   ├── executionClient.ts    # Run recipes
│   ├── historyClient.ts      # Undo/redo
│   └── panelClient.ts        # Panel visibility
├── services/                 # Internal service layer
├── components/               # React UI
│   ├── EditorCanvas/         # ReactFlow canvas wrapper
│   ├── nodes/                # Custom ReactFlow node types
│   ├── ConfigPanel/          # Right sidebar config panel
│   ├── controls/             # Form controls (schema-driven)
│   ├── SchemaForm.tsx        # Schema-driven form renderer
│   └── SchemaField.tsx       # Schema-driven field renderer
└── draft/                    # Draft persistence (localStorage)
    ├── draftStorage.ts       # CRUD operations
    ├── serializeDraft.ts     # State → JSON
    └── deserializeDraft.ts   # JSON → State
```

## Key Concepts

- **Actions are pure functions** — take `EditorState`, return `Partial<EditorState>`. No React, no store. Testable with plain objects
- **Hooks are thin wrappers** — get state from store, call action, apply result. ~5 lines each
- **Configs live separate from nodes** — `node.data` holds visual-only fields (label, icon). Domain fields (operation, params) live in `configs[nodeId]`
- **Containers expand into the graph** — container children live in `definition` (the nested tree). On expand, they're materialized into `store.nodes` and `store.configs`
- **Synthetic nodes** — `placeholder`, `containerGroup`, `addDivider` nodes are injected by the rendering pipeline and filtered from ReactFlow change handlers

## Entry Point

```tsx
import { createReactEditor } from "@bnto/editor";

const { instance, storeApi } = createReactEditor(definition);

// In components:
const { nodes } = instance.nodes.useNodes();
instance.nodes.addNode("image");
instance.history.undo();
```

`createReactEditor()` wraps the imperative `createEditor()` with React hook factories merged onto each domain client.

## Rendering Pipeline

The canvas goes through a memoized transformation pipeline:

```
store.nodes → useLayoutNodes → useExecutionNodes → usePlaceholderNodes → useAddDividerNodes → ReactFlow
```

Each step is a hook that transforms the node array. Synthetic nodes (placeholders, group overlays, add-divider buttons) are injected at the end and filtered from RF change handlers.

## Development

```bash
task ui:build       # TypeScript compilation
task ui:test        # Run tests (Vitest)
task e2e:editor     # Editor E2E tests (serial)
```

Tests are co-located with source files in `store/`, `actions/`, `adapters/`, and `hooks/`.
