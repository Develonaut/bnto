# Editor Architecture

**Last Updated:** March 2026
**Status:** Architecture defined — informing Sprint 4 + 4B implementation
**Execution model:** See [node-architecture.md](node-architecture.md) for the three-layer node model and WASM dispatch

---

## What This Is

The recipe editor lets users create, customize, and export `.bnto.json` recipes. There are **two editors** — a visual bento box grid and a code editor — but they share the same foundation. Users can switch between them on the fly. Both are free (see [pricing-model.md](pricing-model.md)).

This document defines the **shared editor layer** — the architecture that makes both editors work from the same state, the package boundaries, and the layering rules.

---

## Two Editors, One Foundation

| Editor                 | What It Is                                                           | Who It's For                  | Technology                   |
| ---------------------- | -------------------------------------------------------------------- | ----------------------------- | ---------------------------- |
| **Bento Box** (visual) | Spatial grid of compartments — add, remove, arrange, configure nodes | Visual thinkers, casual users | React Flow (`@xyflow/react`) |
| **Code Editor** (JSON) | Schema-aware `.bnto.json` text editor with slash commands            | Power users, developers       | CodeMirror 6                 |

Both editors are **dumb views of the same store.** Switching between them is instant — render a different component, same state. No data migration, no serialization delay, no loss.

The visual editor shows a bento box where each compartment is a node. The code editor shows the same recipe as formatted JSON. Edit in either, see the change in both.

---

## Layered Architecture

The editor follows the same layered pattern as `@bnto/core`. The Zustand store owns all state (nodes, edges, configs, metadata, undo/redo) and passes nodes/edges to ReactFlow as props (**controlled mode**). This eliminates sync bugs between stores and enables atomic state updates (e.g., add a node + push undo + revalidate in one `setState` call).

```
@bnto/nodes (pure types + functions)
         ↓
Pure actions (editor/actions/ — EditorState → Partial<EditorState>)
         ↓
Zustand store (nodes, edges, configs, metadata, undo/redo)
         ↓
Thin wrapper hooks (useAddNode, useRemoveNode, useUpdateParams)
         ↓
Consumer hooks (useEditorActions, useEditorUndoRedo, useEditorExport)
         ↓
Dumb components (render only — zero business logic)
    ↓                    ↓
BentoCanvas          CodeEditor
(visual grid)        (JSON text)
```

### Layer 1: Pure Functions (`@bnto/nodes`)

Framework-agnostic. No React, no store, no DOM. Fully testable in isolation.

**Already built:**

- `Definition`, `Recipe`, `Port`, `Edge` types
- `NODE_TYPE_INFO` registry (10 node types with metadata)
- `NODE_SCHEMAS` (parameter schemas for all 10 types with `visibleWhen`, `requiredWhen`, enums, min/max, defaults)
- `validateDefinition()`, `validateWorkflow()`, `validateEdges()`
- 6 predefined recipes (`compressImages()`, `cleanCsv()`, etc.)

**Sprint 4 Wave 1 (to build):**

- `createBlankDefinition()` — minimal valid Definition
- `addNode(definition, nodeType, position?)` — insert with defaults
- `removeNode(definition, nodeId)` — remove + cascade cleanup
- `updateNodeParams(definition, nodeId, params)` — validate + merge
- `moveNode(definition, nodeId, position)` — update position
- `definitionToRecipe(definition, metadata?)` — wrap for export

These are pure `Definition → Definition` transforms. Immutable — never mutate, always return new.

### Layer 2: State Management (Zustand controlled mode)

The Zustand store owns **all** editor state — nodes, edges, configs, metadata, undo/redo. ReactFlow receives nodes/edges as controlled props and handles rendering + interaction. The store's `onNodesChange` / `onEdgesChange` handlers apply RF's change events back to the store (controlled mode).

**Thin node.data:** RF nodes carry visual-only data (`label`, `variant`, `width`, `height`, `status`). Domain data (`nodeType`, `name`, `parameters`) lives in a separate `configs` map — parameter changes don't trigger RF re-renders.

```typescript
// Store owns everything:
interface EditorState {
  nodes: BentoNode[]; // RF nodes (visual data in node.data)
  edges: Edge[]; // RF edges
  configs: NodeConfigs; // Domain data keyed by node ID (no RF re-render)
  recipeMetadata: RecipeMetadata; // Root-level recipe fields
  isDirty: boolean;
  validationErrors: ValidationError[];
  executionState: ExecutionState; // Per-node execution status
  undoStack: EditorSnapshot[]; // Snapshots of nodes + configs
  redoStack: EditorSnapshot[];
}
```

**Key rules:**

- **No `definition` in the store during editing.** Definition exists only at load/export boundaries
- **Selection via RF's `node.selected` flag.** The store's `selectNode` action maps over nodes
- **Undo/redo snapshots capture both nodes AND configs** atomically
- Factory pattern: `createEditorStore()` — supports initial state injection for testing
- Business mutations (`addNode`, `removeNode`, `updateConfigParams`) live in **action hooks** that use `storeApi.setState()` for atomic updates — not in the store itself
- For the code editor (future), the store mediates between CM6 document state and the shared `Definition` format. The code editor's relationship to the store differs from the visual editor's — see [Switchable Editors](#switchable-editors)

### Layer 3: React Hooks (Three-Layer Pattern)

Hooks follow a three-layer pattern: pure actions → thin wrapper hooks → consumer hooks.

**Pure actions** (`editor/actions/`) — pure functions that take `EditorState` + args and return `Partial<EditorState>` (or null if blocked). No React, no store access — independently testable with plain state objects. This is where ALL state mutation logic lives: undo capture, dirty flag, validation, auto-select, I/O guards.

**Thin wrapper hooks** (`useAddNode`, `useRemoveNode`, `useUpdateParams`) — ~5-line hooks that call `storeApi.getState()`, pass it to the pure action, and apply the result via `storeApi.setState()`. No business logic in the callback — just bridge the action to the store.

**Consumer hooks** — the public API components import:

```typescript
useEditorActions(); // → composed API: addNode, removeNode, updateParams + store actions
useEditorNode(nodeId); // → node data + config + schema + visible params
useEditorUndoRedo(); // → undo, redo, canUndo, canRedo
useEditorExport(); // → exportAsRecipe, download, canExport
useEditorSelection(); // → selectedNodeId (from RF selection change events)
useAutoSelect(options); // → handleSelectNode + center-on-select effect
useNodePalette(); // → available node types grouped by category
```

**Rules:**

- Hooks select specific slices, never the whole store
- Action hooks use `storeApi.setState()` for atomic multi-field updates
- Components only import consumer hooks (Layer 3), never action hooks directly
- Hooks are the only React layer. Everything below is framework-agnostic

### Layer 4: Dumb Components

Components receive data via hooks and render. That's it. Zero business logic, zero data fetching, zero direct store access.

**Bento Box (visual):**

- `BentoCanvas` — React Flow grid, renders compartments
- `CompartmentNode` — surface Card for a single node
- `NodePalette` — sidebar listing available node types
- `NodeConfigPanel` — schema-driven form for selected node
- `EditorToolbar` — action bar (add, remove, run, export, undo/redo)

**Code Editor (JSON):**

- `CodeEditor` — CodeMirror 6 wrapper, renders JSON text
- Slash command extensions — inline node insertion menu
- Breadcrumb panel — JSON path navigation

**Shared (editor chrome):**

- `RecipeEditor` — top-level shell that composes toolbar + active editor + config panel
- Editor mode toggle — switch between visual / code / split view

---

## ReactFlow Performance Patterns

**Status:** Target architecture — being adopted incrementally via editor refactor PRs.

The bento box visual editor uses ReactFlow (`@xyflow/react`) as the canvas runtime. This section documents the performance patterns and state ownership model that make the editor fast and sync-bug-free. These patterns are proven in production at [atomiton](https://github.com/atomiton) and align with ReactFlow's recommended architecture.

### Controlled Mode: Zustand Owns, ReactFlow Renders

**The core insight: Zustand owns state, ReactFlow renders it.**

The store owns all node/edge state. ReactFlow receives `nodes`, `edges`, `onNodesChange`, and `onEdgesChange` as controlled props. RF handles rendering, interactions (drag, select, zoom), and emits change events — the store applies those changes and re-renders RF with updated props. This is RF's **controlled mode** (similar to controlled `<input>` in React).

```
Load:   Definition → definitionToBento() → store.setState()   [one-time conversion]
Edit:   Store owns state. RF renders as controlled props.      [during editing]
Export: store.nodes → rfNodesToDefinition() → Definition       [on-demand conversion]
```

**Why controlled mode:** It enables atomic state updates. When adding a node, the store sets nodes + configs + undoStack + isDirty + validationErrors in a single `setState()` call — one render, one consistent state. With RF as the store, mutations require reading from RF, computing, writing back, then separately updating Zustand — two stores, race conditions, sync effects.

**Thin node.data:** RF nodes carry visual-only fields (`label`, `variant`, `width`, `height`, `status`). Domain data (`nodeType`, `name`, `parameters`) lives in the store's `configs` map. This means parameter edits update `configs` only — RF doesn't re-render.

**Six principles:**

1. **Zustand owns, RF renders.** Controlled props: `nodes={store.nodes}`, `onNodesChange={store.onNodesChange}`.
2. **Definition only at boundaries.** `definitionToBento()` on load. `rfNodesToDefinition()` on export. During editing, `Definition` doesn't exist.
3. **Pure builder functions.** Node creation is a pure function (`createCompartmentNode`). Hooks orchestrate: pure function → `storeApi.setState()`.
4. **Thin node.data.** Visual-only data in RF nodes, domain data in separate configs map. No RF re-render on parameter changes.
5. **DOM-direct execution state.** Progress via data attributes and CSS variables during execution. Zero re-renders.
6. **No sync effects.** Zero `useEffect` watching one store and writing to another. Single source of truth eliminates sync bugs entirely.

### Three-Layer Pattern: Actions → Hooks → Components

All editor mutations follow the same layering: pure actions → thin wrapper hooks → consumer hooks. Each layer has a single responsibility.

```
Layer 1: Pure Helpers + Actions (store/ + actions/)
         No React. No store. Fully testable in isolation.
         captureSnapshot(), pushToStack(), revalidateState()   (helpers)
         addNode(), removeNode(), updateParams()               (actions)

              ↓

Layer 2: Thin Wrapper Hooks (editor/hooks/)
         ~5 lines each. Call action, apply result to store.
         useAddNode(), useRemoveNode(), useUpdateParams()

              ↓

Layer 3: Consumer Hooks (editor/hooks/)
         Compose wrapper hooks + store selectors into the public API.
         useEditorActions(), useEditorExport(), useEditorUndoRedo()
```

**Layer 1 — Pure helpers and actions** are the foundation. Helpers are small utilities (snapshot, stack, validation). Actions are the state mutation functions — they take `EditorState` + args and return `Partial<EditorState>` or null. All business logic lives here: I/O guards, undo capture, auto-select, dirty flag, validation. Tested with plain state objects — no React, no store.

```typescript
// Pure action — takes state, returns next state. No React, no store.
import { addNode } from "../actions/addNode";
const result = addNode(currentState, "image");
// result = { nextState: Partial<EditorState>, nodeId: string } | null
```

**Layer 2 — Thin wrapper hooks** bridge pure actions to the store. Each hook is ~5 lines: get state, call action, apply result. **No business logic in the callback.**

```typescript
// Thin wrapper hook — bridges action to store
function useAddNode() {
  const storeApi = useEditorStoreApi();
  return useCallback(
    (type: NodeTypeName, position?) => {
      const result = addNode(storeApi.getState(), type, position);
      if (!result) return null;
      storeApi.setState(result.nextState);
      return result.nodeId;
    },
    [storeApi],
  );
}
```

**Layer 3 — Consumer hooks** compose wrapper hooks with store selectors. These are the hooks that components import.

```typescript
// Consumer hook — composes wrapper hooks + store selectors
function useEditorActions() {
  const addNode = useAddNode();
  const removeNode = useRemoveNode();
  const updateParams = useUpdateParams();
  const storeActions = useEditorStore(useShallow(s => ({
    selectNode: s.selectNode, undo: s.undo, redo: s.redo, ...
  })));
  return { ...storeActions, addNode, removeNode, updateParams };
}
```

**The rule:** Components only import Layer 3. Layer 3 composes Layer 2. Layer 2 calls Layer 1. Never skip layers. Business logic NEVER lives inside hook callbacks — it belongs in pure action functions.

### State Ownership Table

All state lives in the Zustand store. ReactFlow renders it as controlled props.

| State                     | Owner            | Access Pattern                                  | Why                                                              |
| ------------------------- | ---------------- | ----------------------------------------------- | ---------------------------------------------------------------- |
| **nodes** (visual data)   | Zustand store    | `useEditorStore(s => s.nodes)`                  | Controlled mode — store owns, RF renders                         |
| **edges**                 | Zustand store    | `useEditorStore(s => s.edges)`                  | Store applies RF edge change events                              |
| **configs** (domain data) | Zustand store    | `useEditorStore(s => s.configs[id])`            | Separate from RF nodes — no RF re-render on param changes        |
| **Node selection**        | Zustand (via RF) | `useEditorSelection()` → RF `onSelectionChange` | RF emits selection events, store tracks via `node.selected` flag |
| **Viewport** (pan, zoom)  | ReactFlow        | `useReactFlow().fitView()`                      | RF handles gestures, zoom — not in Zustand                       |
| **isDirty**               | Zustand store    | `useEditorStore(s => s.isDirty)`                | Editor metadata                                                  |
| **validationErrors**      | Zustand store    | `useEditorStore(s => s.validationErrors)`       | Derived from nodes + configs at mutation time                    |
| **recipeMetadata**        | Zustand store    | `useEditorStore(s => s.recipeMetadata)`         | Recipe-level metadata (id, name, type, version)                  |
| **undoStack / redoStack** | Zustand store    | `useEditorUndoRedo()`                           | Atomic snapshots of nodes + configs                              |
| **executionState**        | Zustand store    | `useEditorStore(s => s.executionState)`         | Per-node execution status tracking                               |

**The store owns everything.** ReactFlow is a pure renderer — it receives `nodes` and `edges` as props and emits change events that the store applies. Domain data lives in `configs` to avoid RF re-renders on parameter changes.

### Entry/Exit Boundary Pattern

`Definition` (the persistent format from `@bnto/nodes`) is converted to/from ReactFlow nodes at two boundaries only. During editing, `Definition` doesn't exist — RF is truth.

```
                    ┌─────────────────────────────────┐
                    │       EDITING (RF is truth)       │
                    │                                   │
  ┌─────────┐      │  ┌───────────┐                   │      ┌─────────┐
  │Definition│─────▶│  │ RF Nodes  │  setNodes()       │─────▶│Definition│
  │ (load)   │ ENTRY│  │ (graph)   │  getNodes()       │ EXIT │ (export) │
  └─────────┘      │  └───────────┘                   │      └─────────┘
                    │                                   │
                    └─────────────────────────────────┘

ENTRY: definitionToBento(definition) → { nodes: BentoNode[], configs: NodeConfigs }
  - Called once on recipe load or createBlank()
  - Splits Definition into visual nodes (RF) + domain configs (separate map)
  - Store receives both via setState()
  - After this call, Definition is discarded

EXIT: rfNodesToDefinition(nodes, metadata, configs) → Definition
  - Called on export (download .bnto.json) or save
  - Reads store's nodes + configs + metadata
  - Builds a valid Definition from store state
  - Pure function — does not modify store state
```

**Entry boundary:** When a recipe loads, `definitionToBento()` splits the `Definition` into RF nodes (visual) and configs (domain). The store receives both via `setState()`. The original `Definition` is discarded.

**Exit boundary:** When the user exports, `rfNodesToDefinition()` reads the store's nodes, configs, and metadata to reconstruct a `Definition`. Pure function — read-only.

**No mid-editing Definition.** During editing, there is no `Definition` object being maintained. Mutations go through `storeApi.setState()`. This eliminates the entire category of sync bugs where Definition and store state drift apart.

### O(1) Node Access via `nodeLookup`

ReactFlow (v12.10+) maintains an internal `nodeLookup` Map alongside the `nodes` array. Use it for direct access instead of `nodes.find()`.

```typescript
// O(1) — use for targeted lookups
const node = useStore((s) => s.nodeLookup.get(nodeId));

// O(n) — avoid for single lookups, fine for rendering all nodes
const allNodes = useStore((s) => s.nodes);
```

For bnto's typical scale (3-10 nodes), the performance difference is negligible. The pattern matters more for correctness — `nodeLookup` returns the internal node with selection state, measured dimensions, and computed positions that the `nodes` array doesn't always reflect immediately.

### DOM-Direct Execution Feedback

During recipe execution, node progress updates at high frequency (potentially hundreds of events per second). Rendering these through React would cause thrashing. Instead, update DOM attributes directly.

```typescript
// During execution — bypass React, update DOM directly
function useNodeExecutionState(nodeId: string) {
  const nodeRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const unsubscribe = executionEmitter.on("progress", (event) => {
      const el = nodeRef.current;
      if (!el) return;
      el.setAttribute("data-execution-state", event.state);
      el.style.setProperty("--progress", String(event.progress));
    });
    return unsubscribe;
  }, [nodeId]);

  return nodeRef;
}
```

CSS handles the visual feedback:

```css
[data-execution-state="running"] {
  /* pulse animation */
}
[data-execution-state="completed"] {
  /* success styling */
}
[data-execution-state="error"] {
  /* error styling */
}
```

This pattern keeps execution visualization at 0 React re-renders regardless of update frequency.

---

## No Edges in the Visual Editor

The bento box visual editor does **not** show or create edges (connections between nodes). Execution order is implied by **compartment position** — spatial arrangement on the grid determines the pipeline flow.

- **Visual editor:** No edge UI. No drag-to-connect. Order = position (top-left → bottom-right scan, or explicit array order).
- **Code editor:** Can show/edit the `edges` array directly in JSON if needed for advanced use.
- **Definition model:** The `Definition` type retains `edges` for compatibility with the engine. The editor store can auto-derive edges from node order when exporting.

This is a deliberate simplification. The bento box metaphor is about **spatial arrangement** — where you place things in the box — not about drawing wires between them.

---

## Switchable Editors

Users can switch between visual and code editors on the fly. This is a core requirement.

### How It Works

```
User clicks "Code" toggle
  → RecipeEditor swaps BentoCanvas for CodeEditor
  → CodeEditor reads Definition from store, serializes to JSON, renders
  → User edits JSON
  → CodeEditor parses JSON, validates, writes back to store

User clicks "Visual" toggle
  → RecipeEditor swaps CodeEditor for BentoCanvas
  → BentoCanvas reads Definition from store, renders compartments
  → User drags compartments, adds nodes
  → BentoCanvas writes changes back to store

User clicks "Split" toggle
  → Both render side by side, both reading from the same store
  → Changes in either sync through the store instantly
```

### Sync Rules

- **Store is truth.** Both editors read from `useEditorDefinition()`. Neither caches its own copy.
- **Visual → Store:** Node add/remove/move dispatches store actions directly. Instant.
- **Code → Store:** JSON text changes are debounced (200ms), parsed, validated, then written to store. Only valid JSON updates the store — invalid JSON shows inline diagnostics but doesn't corrupt state.
- **Store → Code:** When the store changes externally (from visual editor), the code editor receives the new Definition, serializes to JSON, and updates the CM6 document via a transaction marked with an `externalUpdate` annotation. This prevents sync loops.
- **No dual-write.** A change originates in ONE editor, flows through the store, and the other editor picks it up. Never both writing simultaneously.

---

## Package Strategy

### Today: Co-located in `apps/web`

Following the co-location rule, all editor code lives in `apps/web` until a second consumer exists. The directory structure is designed so the editor state layer can be lifted into a package later with zero API changes.

```
apps/web/
├── editor/                        # Editor feature module (future @bnto/editor)
│   ├── store/                     # Thin Zustand store (metadata only)
│   │   ├── createEditorStore.ts   # Factory — isDirty, validation, undo/redo, metadata
│   │   └── types.ts               # EditorState, EditorActions
│   ├── hooks/                     # React hooks (RF-first + business)
│   │   ├── useAddNode.ts          # RF-first: pure fn → setNodes()
│   │   ├── useRemoveNode.ts       # RF-first: setNodes(filter)
│   │   ├── useUpdateNodeParams.ts # RF-first: setNodes(map)
│   │   ├── useEditorActions.ts    # Business: composes RF hooks + undo
│   │   ├── useEditorExport.ts     # Business: getNodes() → Definition
│   │   ├── useEditorUndoRedo.ts   # Business: RF snapshot stack
│   │   ├── useEditorSelection.ts  # RF store selector
│   │   └── useNodePalette.ts      # Available node types
│   ├── adapters/                  # Editor ↔ renderer adapters (pure functions)
│   │   ├── definitionToBento.ts   # Definition → CompartmentNodeType[] (load boundary)
│   │   ├── rfNodesToDefinition.ts # CompartmentNodeType[] → Definition (export boundary)
│   │   └── definitionToJson.ts    # Definition → formatted JSON string
│   └── index.ts                   # Barrel export
├── components/
│   ├── editor/                    # Editor UI components (dumb)
│   │   ├── RecipeEditor.tsx       # Top-level shell
│   │   ├── EditorToolbar.tsx      # Action bar
│   │   ├── NodePalette.tsx        # Node type browser
│   │   ├── NodeConfigPanel.tsx    # Schema-driven config form
│   │   └── EditorModeToggle.tsx   # Visual / Code / Split switch
│   ├── bento/                     # Bento box visual editor
│   │   ├── BentoCanvas.tsx        # React Flow canvas
│   │   └── CompartmentNode.tsx    # Surface Card node
│   └── code-editor/              # CodeMirror 6 editor
│       ├── CodeEditor.tsx         # CM6 wrapper component
│       ├── useCodeEditor.ts       # CM6 imperative hook
│       ├── bntoTheme.ts           # OKLCH theme extension
│       ├── bntoSlashCommands.ts   # Slash command extension
│       └── storeSync.ts           # CM6 ↔ store sync extension
```

### Future: `@bnto/editor` Package

When the desktop app (Tauri) needs an editor, the `editor/` directory lifts into `packages/@bnto/editor/`. The extraction boundary is the `editor/` module — store, hooks, and adapters move. UI components stay in `apps/web` (or move to `@bnto/ui`).

```
packages/@bnto/editor/          # Extracted from apps/web/editor/
├── store/                      # Zustand store (headless, no DOM)
├── hooks/                      # React hooks
├── adapters/                   # Definition ↔ renderer adapters
└── index.ts

apps/web/components/editor/     # Stays in app (UI layer)
apps/web/components/bento/      # Stays in app
apps/web/components/code-editor/ # Stays in app
```

**The rule:** Don't extract until there's a real second consumer. Design for extraction. Don't execute it prematurely.

---

## Relationship to Existing Packages

```
@bnto/nodes          Types, schemas, recipes, validation, definition CRUD
     ↓               (pure functions — no framework dependency)
     ↓
@bnto/core           Transport-agnostic API (workflows, executions, auth)
     ↓               (the editor store is NOT in core — it's a separate domain)
     ↓
apps/web/editor/     Editor state layer (store, hooks, adapters)
     ↓               (future: @bnto/editor)
     ↓
apps/web/components/ Editor UI (BentoCanvas, CodeEditor, chrome)
```

**Why not put the editor store in `@bnto/core`?**

Core is the transport-agnostic API for data operations (CRUD workflows, executions, auth). The editor is a **client-side authoring experience** — it doesn't talk to a backend, it doesn't need transport adapters. Different domain, different lifecycle. Keeping them separate follows the Bento Box Principle (single responsibility per package).

When `@bnto/editor` is extracted, it will be a peer of `@bnto/core`, not inside it.

---

## Entry Points

Two ways to start editing:

1. **`loadRecipe(slug)`** — Load a predefined recipe from `@bnto/nodes`. The bento box fills with pre-arranged compartments. Good for customizing existing recipes.

2. **`createBlank()`** — Start with an empty bento box (root group node, no children). Add compartments from the palette. Good for building from scratch.

Both produce the same `EditorState` shape. Same store, same hooks, same operations.

---

## Implementation Sequence

The layered architecture dictates the build order:

| Sprint        | Wave   | What                               | Layer                          |
| ------------- | ------ | ---------------------------------- | ------------------------------ |
| **Sprint 4**  | Wave 1 | Definition CRUD pure functions     | Layer 1 (`@bnto/nodes`)        |
| **Sprint 4**  | Wave 2 | Editor store + hooks + adapters    | Layer 2-3 (`apps/web/editor/`) |
| **Sprint 4**  | Wave 3 | Bento box visual editor components | Layer 4 (visual skin)          |
| **Sprint 4**  | Wave 4 | Execution visualization + export   | Layer 4 (visual skin)          |
| **Sprint 4B** | Wave 1 | JSON Schema generation             | Layer 1 (`@bnto/nodes`)        |
| **Sprint 4B** | Wave 2 | CodeMirror 6 foundation            | Layer 4 (code skin)            |
| **Sprint 4B** | Wave 3 | Slash commands + command registry  | Layer 4 (code skin)            |
| **Sprint 4B** | Wave 4 | Store sync + split view            | Layer 3-4 (sync + UI)          |
| **Sprint 4B** | Wave 5 | Breadcrumbs, polish, E2E           | Layer 4 (code skin)            |

Sprint 4 Waves 1-2 are prerequisites for Sprint 4B — the code editor consumes the shared store.

---

## Design Decisions

| Decision                        | Choice                                                  | Rationale                                                                                                                                                                                    |
| ------------------------------- | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Visual metaphor**             | Bento box grid (not conveyor belt pipeline)             | Spatial arrangement is simpler, more intuitive for non-developers. No edge management complexity. Matches the product name.                                                                  |
| **No edges in visual editor**   | Execution order = position order                        | Bento box metaphor is about arrangement, not wiring. Code editor can handle edges for advanced cases.                                                                                        |
| **RF as graph source of truth** | ReactFlow owns all node/edge state during editing       | Eliminates sync bugs between parallel stores. Definition only at load/export boundaries. Proven at scale in atomiton. See [ReactFlow Performance Patterns](#reactflow-performance-patterns). |
| **Thin Zustand for metadata**   | isDirty, validation, undo/redo, recipe metadata only    | Zustand handles what RF doesn't — editor-level concerns, not graph structure. No overlap with RF state.                                                                                      |
| **Switchable editors**          | Visual ↔ Code on the fly via shared store               | Power users switch freely. No lock-in to one editing mode.                                                                                                                                   |
| **Store location**              | Co-located in `apps/web/editor/`, future `@bnto/editor` | Follow co-location rule. Extract when desktop needs it.                                                                                                                                      |
| **Store not in `@bnto/core`**   | Separate domain (authoring ≠ transport)                 | Core is for backend communication. Editor is client-side authoring. Different concerns.                                                                                                      |
| **Dumb components**             | All business logic in store/hooks/pure fns              | Components render, nothing more. Makes editors interchangeable.                                                                                                                              |
| **Visual editor tech**          | React Flow (`@xyflow/react`)                            | Already in use for bento box prototype. Grid layout, zoom-to-fit, node positioning.                                                                                                          |
| **Code editor tech**            | CodeMirror 6                                            | 60x smaller than Monaco. CSS variable theming. Mobile support. Headless state. See [code-editor.md](code-editor.md).                                                                         |

---

## References

| Document                             | What It Covers                                                             |
| ------------------------------------ | -------------------------------------------------------------------------- |
| [visual-editor.md](visual-editor.md) | Bento box visual editor — compartment design, grid layout, execution state |
| [code-editor.md](code-editor.md)     | CodeMirror 6 — tech choice, slash commands, JSON Schema, theming           |
| [PLAN.md](../PLAN.md) → Sprint 4     | Visual editor task breakdown (Waves 1-4)                                   |
| [PLAN.md](../PLAN.md) → Sprint 4B    | Code editor task breakdown (Waves 1-5)                                     |
| [pricing-model.md](pricing-model.md) | Recipe editor is free. Create, run, export = free. Save, share = Pro.      |
