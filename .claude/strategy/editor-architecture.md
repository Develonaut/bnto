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

The editor follows the same layered pattern as `@bnto/core`:

```
@bnto/nodes (pure types + functions)
         ↓
Editor store (Zustand — headless state machine)
         ↓
React hooks (thin reactive bindings)
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

### Layer 2: Editor Store (Zustand)

Headless state machine. Wraps Layer 1 pure functions into reactive state with undo/redo. No React dependency — vanilla Zustand store.

```typescript
// Conceptual shape — not final API
interface EditorState {
  definition: Definition;
  selectedNodeId: string | null;
  isDirty: boolean;
  validationErrors: ValidationError[];
  undoStack: Definition[];
  redoStack: Definition[];
}

interface EditorActions {
  loadRecipe(slug: string): void;
  createBlank(): void;
  addNode(type: NodeTypeName): void;
  removeNode(id: string): void;
  selectNode(id: string | null): void;
  updateParams(nodeId: string, params: Record<string, unknown>): void;
  moveNode(nodeId: string, position: Position): void;
  undo(): void;
  redo(): void;
  resetDirty(): void;
}
```

**Key rules:**
- All actions delegate to Layer 1 pure functions — the store is a thin wrapper
- Undo/redo via definition snapshots (push to stack before each mutation)
- Factory pattern: `createEditorStore(initialDefinition)` — same pattern as `recipeFlowStore`
- The store is the **single source of truth**. Both editors read from and write to this store.

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
│   ├── store/                     # Zustand editor store
│   │   └── createEditorStore.ts   # Factory function
│   ├── hooks/                     # React hooks (thin selectors)
│   │   ├── useEditorDefinition.ts
│   │   ├── useSelectedNode.ts
│   │   ├── useEditorActions.ts
│   │   ├── useNodePalette.ts
│   │   └── useEditorExport.ts
│   ├── adapters/                  # Editor ↔ renderer adapters
│   │   ├── definitionToBento.ts   # Definition → BentoCanvas nodes
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
