# Editor Architecture

**Last Updated:** March 2026
**Status:** Architecture defined — informing Sprint 4 + 4B implementation

---

## What This Is

The recipe editor lets users create, customize, and export `.bnto.json` recipes. There are **two editors** — a visual bento box grid and a code editor — but they share the same foundation. Users can switch between them on the fly. Both are free (see [pricing-model.md](pricing-model.md)).

This document defines the **shared editor layer** — the architecture that makes both editors work from the same state, the package boundaries, and the layering rules.

---

## Two Editors, One Foundation

| Editor | What It Is | Who It's For | Technology |
|--------|-----------|--------------|------------|
| **Bento Box** (visual) | Spatial grid of compartments — add, remove, arrange, configure nodes | Visual thinkers, casual users | React Flow (`@xyflow/react`) |
| **Code Editor** (JSON) | Schema-aware `.bnto.json` text editor with slash commands | Power users, developers | CodeMirror 6 |

Both editors are **dumb views of the same store.** Switching between them is instant — render a different component, same state. No data migration, no serialization delay, no loss.

The visual editor shows a bento box where each compartment is a node. The code editor shows the same recipe as formatted JSON. Edit in either, see the change in both.

---

## Layered Architecture

The editor follows the same layered pattern as `@bnto/core`. For the bento box visual editor, **ReactFlow is the source of truth for graph state** — see [ReactFlow Performance Patterns](#reactflow-performance-patterns) for the full model. The Zustand store is thinned to metadata only (dirty flag, validation errors, undo/redo history, recipe metadata).

```
@bnto/nodes (pure types + functions)
         ↓
ReactFlow store (graph state) + thin Zustand (editor metadata)
         ↓
React hooks (RF-first mutations + business logic)
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

### Layer 2: State Management (ReactFlow + thin Zustand)

For the visual editor, **ReactFlow owns graph state** — nodes, edges, positions, selection, viewport. A thin Zustand store holds editor-level metadata that RF doesn't manage. See [ReactFlow Performance Patterns](#reactflow-performance-patterns) for the full model.

```typescript
// ReactFlow owns (via useStore / useReactFlow):
//   - nodes[] with full data (type, name, parameters in node.data)
//   - edges[]
//   - selection
//   - viewport (pan, zoom)
//   - nodeLookup Map (O(1) access)

// Thin Zustand store (editor metadata only):
interface EditorState {
  isDirty: boolean;
  validationErrors: ValidationError[];
  recipeMetadata: { name: string; slug: string; description: string };
  undoStack: CompartmentNodeType[][];   // RF node snapshots
  redoStack: CompartmentNodeType[][];
}
```

**Key rules:**
- **No `definition` in the store during editing.** Definition exists only at load/export boundaries
- **No `selectedNodeId` in Zustand.** RF owns selection state
- Undo/redo via RF node snapshots (`getNodes()` before mutation, `setNodes()` on undo)
- Factory pattern: `createEditorStore()` — thin wrapper for metadata + undo history
- Graph mutations go through `setNodes()` / `setEdges()`, never through Zustand actions
- For the code editor (future), the store mediates between CM6 document state and the shared `Definition` format. The code editor's relationship to the store differs from the visual editor's — see [Switchable Editors](#switchable-editors)

### Layer 3: React Hooks

Thin reactive bindings that subscribe to store slices. No business logic — just select and return.

```typescript
// Slice selectors — re-render only when your slice changes
useEditorDefinition()        // → Definition
useSelectedNode()            // → node data + schema for selected node
useEditorActions()           // → action dispatchers
useEditorDirty()             // → boolean
useValidationErrors()        // → ValidationError[]
useNodePalette()             // → available node types grouped by category
useEditorExport()            // → { exportAsRecipe, download }
```

**Rules:**
- Hooks select specific slices, never the whole store
- No transforms in hooks — if you need derived data, add a selector to the store
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

### ReactFlow as Single Source of Truth

**The core insight: ReactFlow IS the store for graph state.**

During editing, ReactFlow owns all node/edge state — positions, data, selection, viewport. There is no parallel Zustand store mirroring graph structure. The `Definition` type (from `@bnto/nodes`) only exists at **boundaries**: on load (Definition → RF nodes) and on export (RF nodes → Definition).

```
Load:   Definition → definitionToRFNodes() → setNodes()     [one-time conversion]
Edit:   RF owns everything. Mutations go through setNodes()  [during editing]
Export: getNodes() → rfNodesToDefinition() → Definition      [on-demand conversion]
```

**Why this matters:** A parallel store that mirrors RF state creates sync bugs. Every `useEffect` that watches one store and writes to another is a race condition waiting to happen. The fix isn't better sync — it's eliminating the need for sync entirely.

**Six principles (from atomiton):**

1. **RF IS the store.** No parallel Zustand store for graph structure. `useEditorStore` = `useStore` from `@xyflow/react` with shallow equality.
2. **Definition only at boundaries.** `definitionToRFNodes()` on load. `rfNodesToDefinition()` on export. During editing, RF owns all state.
3. **Pure builder functions.** Node creation/updates are pure functions. Hooks orchestrate: pure function → `setNodes()`.
4. **O(1) node access.** RF's internal `nodeLookup` Map — never `nodes.find()` for lookups.
5. **DOM-direct execution state.** Progress via data attributes and CSS variables during execution. Zero re-renders.
6. **No sync effects.** Zero `useEffect` watching one store and writing to another. Single source of truth eliminates sync bugs entirely.

### Three-Layer Hook Pattern

All editor mutations follow the same layering: pure functions → RF-first hooks → business hooks. Each layer has a single responsibility.

```
Layer 1: Pure Functions (@bnto/nodes + editor/adapters/)
         No React. No store. Fully testable in isolation.
         createBlankDefinition(), addNode(), definitionToRFNodes()

              ↓

Layer 2: RF-First Hooks (editor/hooks/)
         Call pure functions → setNodes(). Operate on RF state directly.
         useAddNode(), useRemoveNode(), useUpdateNodeParams()

              ↓

Layer 3: Business Hooks (editor/hooks/)
         Compose RF-first hooks + side effects (undo snapshots, dirty tracking, validation).
         useEditorActions(), useEditorExport(), useEditorUndoRedo()
```

**Layer 1 — Pure functions** are the foundation. They accept data and return data. No React, no store, no DOM. These are where the real logic lives and where the heaviest testing happens.

```typescript
// Pure function — no React, easy to test
import { addNode } from "@bnto/nodes";
const updated = addNode(definition, "image", { x: 100, y: 200 });
```

**Layer 2 — RF-first hooks** bridge pure functions to ReactFlow's store. Each hook does one thing: call a pure function, then call `setNodes()` or `setEdges()`. These hooks use `useReactFlow()` to get `setNodes`/`getNodes`.

```typescript
// RF-first hook — pure function → setNodes()
function useAddNode() {
  const { setNodes } = useReactFlow();
  return useCallback((nodeType: NodeTypeName, position: XYPosition) => {
    const rfNode = createCompartmentNode(nodeType, position);
    setNodes(prev => [...prev, rfNode]);
    return rfNode.id;
  }, [setNodes]);
}
```

**Layer 3 — Business hooks** compose RF-first hooks with cross-cutting concerns. Undo/redo snapshots, dirty flag, validation, export. These are the hooks that components import.

```typescript
// Business hook — composes RF-first + side effects
function useEditorActions() {
  const addNode = useAddNode();
  const { pushUndo } = useEditorUndoRedo();
  return {
    addNode: (type, position) => {
      pushUndo();  // snapshot before mutation
      return addNode(type, position);
    },
  };
}
```

**The rule:** Components only import Layer 3. Layer 3 composes Layer 2. Layer 2 calls Layer 1. Never skip layers.

### State Ownership Table

Clear separation between what ReactFlow owns and what thin Zustand owns. No overlap.

| State | Owner | Access Pattern | Why |
|---|---|---|---|
| **Node positions** | ReactFlow | `useStore(s => s.nodes)` | RF handles drag, zoom-to-fit, viewport math |
| **Node data** (type, name, params) | ReactFlow (`node.data`) | `useStore(s => s.nodeLookup.get(id))` | Keeps all node state in one place — no sync needed |
| **Node selection** | ReactFlow | `useStore(s => s.nodeLookup.get(id)?.selected)` | RF handles click-to-select, multi-select, keyboard |
| **Viewport** (pan, zoom) | ReactFlow | `useStore(s => s.transform)` | RF handles gestures, fitView, zoom controls |
| **Edges** | ReactFlow | `useStore(s => s.edges)` | RF handles edge routing, hit testing |
| **isDirty** | Zustand (thin) | `useEditorStore(s => s.isDirty)` | Not graph state — UI metadata |
| **validationErrors** | Zustand (thin) | `useEditorStore(s => s.validationErrors)` | Derived from definition, not graph state |
| **recipeMetadata** (name, slug, description) | Zustand (thin) | `useEditorStore(s => s.recipeMetadata)` | Recipe-level metadata, not per-node |
| **undoStack / redoStack** | Zustand (thin) | `useEditorUndoRedo()` | Snapshots of RF node arrays |

**The thin Zustand store** holds only what ReactFlow doesn't: editor-level metadata, undo/redo history, and validation state. It never duplicates graph state.

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

ENTRY: definitionToRFNodes(definition) → CompartmentNodeType[]
  - Called once on recipe load or createBlank()
  - Computes bento slot positions from node index
  - Stores nodeType, name, parameters in RF node.data
  - After this call, Definition is discarded

EXIT: rfNodesToDefinition(nodes) → Definition
  - Called on export (download .bnto.json) or save
  - Reads node.data fields, extracts positions from RF node.position
  - Builds a valid Definition from RF state
  - Pure function — does not modify RF state
```

**Entry boundary:** When a recipe loads (predefined or blank), `definitionToRFNodes()` converts the `Definition` into an array of `CompartmentNodeType[]` and calls `setNodes()`. After this, the original `Definition` is not stored anywhere — RF owns the data.

**Exit boundary:** When the user exports or saves, `rfNodesToDefinition()` reads `getNodes()` from RF and constructs a `Definition`. This is a read-only operation — it doesn't change RF state.

**No mid-editing Definition.** During editing, there is no `Definition` object being maintained. Mutations go through `setNodes()`. This eliminates the entire category of sync bugs where Definition and RF nodes drift apart.

### O(1) Node Access via `nodeLookup`

ReactFlow (v12.10+) maintains an internal `nodeLookup` Map alongside the `nodes` array. Use it for direct access instead of `nodes.find()`.

```typescript
// O(1) — use for targeted lookups
const node = useStore(s => s.nodeLookup.get(nodeId));

// O(n) — avoid for single lookups, fine for rendering all nodes
const allNodes = useStore(s => s.nodes);
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
[data-execution-state="running"] { /* pulse animation */ }
[data-execution-state="completed"] { /* success styling */ }
[data-execution-state="error"] { /* error styling */ }
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

| Sprint | Wave | What | Layer |
|--------|------|------|-------|
| **Sprint 4** | Wave 1 | Definition CRUD pure functions | Layer 1 (`@bnto/nodes`) |
| **Sprint 4** | Wave 2 | Editor store + hooks + adapters | Layer 2-3 (`apps/web/editor/`) |
| **Sprint 4** | Wave 3 | Bento box visual editor components | Layer 4 (visual skin) |
| **Sprint 4** | Wave 4 | Execution visualization + export | Layer 4 (visual skin) |
| **Sprint 4B** | Wave 1 | JSON Schema generation | Layer 1 (`@bnto/nodes`) |
| **Sprint 4B** | Wave 2 | CodeMirror 6 foundation | Layer 4 (code skin) |
| **Sprint 4B** | Wave 3 | Slash commands + command registry | Layer 4 (code skin) |
| **Sprint 4B** | Wave 4 | Store sync + split view | Layer 3-4 (sync + UI) |
| **Sprint 4B** | Wave 5 | Breadcrumbs, polish, E2E | Layer 4 (code skin) |

Sprint 4 Waves 1-2 are prerequisites for Sprint 4B — the code editor consumes the shared store.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Visual metaphor** | Bento box grid (not conveyor belt pipeline) | Spatial arrangement is simpler, more intuitive for non-developers. No edge management complexity. Matches the product name. |
| **No edges in visual editor** | Execution order = position order | Bento box metaphor is about arrangement, not wiring. Code editor can handle edges for advanced cases. |
| **RF as graph source of truth** | ReactFlow owns all node/edge state during editing | Eliminates sync bugs between parallel stores. Definition only at load/export boundaries. Proven at scale in atomiton. See [ReactFlow Performance Patterns](#reactflow-performance-patterns). |
| **Thin Zustand for metadata** | isDirty, validation, undo/redo, recipe metadata only | Zustand handles what RF doesn't — editor-level concerns, not graph structure. No overlap with RF state. |
| **Switchable editors** | Visual ↔ Code on the fly via shared store | Power users switch freely. No lock-in to one editing mode. |
| **Store location** | Co-located in `apps/web/editor/`, future `@bnto/editor` | Follow co-location rule. Extract when desktop needs it. |
| **Store not in `@bnto/core`** | Separate domain (authoring ≠ transport) | Core is for backend communication. Editor is client-side authoring. Different concerns. |
| **Dumb components** | All business logic in store/hooks/pure fns | Components render, nothing more. Makes editors interchangeable. |
| **Visual editor tech** | React Flow (`@xyflow/react`) | Already in use for bento box prototype. Grid layout, zoom-to-fit, node positioning. |
| **Code editor tech** | CodeMirror 6 | 60x smaller than Monaco. CSS variable theming. Mobile support. Headless state. See [code-editor.md](code-editor.md). |

---

## References

| Document | What It Covers |
|----------|---------------|
| [visual-editor.md](visual-editor.md) | Bento box visual editor — compartment design, grid layout, execution state |
| [code-editor.md](code-editor.md) | CodeMirror 6 — tech choice, slash commands, JSON Schema, theming |
| [PLAN.md](../PLAN.md) → Sprint 4 | Visual editor task breakdown (Waves 1-4) |
| [PLAN.md](../PLAN.md) → Sprint 4B | Code editor task breakdown (Waves 1-5) |
| [pricing-model.md](pricing-model.md) | Recipe editor is free. Create, run, export = free. Save, share = Pro. |
