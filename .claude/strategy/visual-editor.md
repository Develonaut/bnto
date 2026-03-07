# Visual Editor — Bento Box

**Last Updated:** March 2026
**Status:** Prototype on Motorway page — Sprint 4 Waves 3-4 build the real editor
**Architecture:** See [editor-architecture.md](editor-architecture.md) for the shared layer design, [node-architecture.md](node-architecture.md) for the three-layer node model

---

## What This Is

A visual recipe editor where nodes are **compartments in a bento box**. Users add, remove, arrange, and configure nodes by placing them on a grid — like arranging compartments in a real bento box. No wires, no flow arrows. Position implies execution order.

This is one of two editor modes. Users can switch to the [code editor](code-editor.md) (CodeMirror 6) on the fly. Both are views of the same `Definition` in the shared editor store.

---

## The Bento Box Metaphor

| Concept          | Bento Box                                | Editor                                        |
| ---------------- | ---------------------------------------- | --------------------------------------------- |
| **Box**          | The container with compartments          | The canvas with a grid background             |
| **Compartments** | Sized sections holding different items   | Nodes — surface Cards of varying width/height |
| **Arrangement**  | Items placed deliberately in the box     | Nodes positioned on the grid                  |
| **Order**        | Top-left to bottom-right (reading order) | Execution order follows position              |
| **Adding**       | Place a new item in an empty slot        | Click "Add" from the node palette             |
| **Removing**     | Take an item out of the box              | Select a compartment, press delete            |

The bento box is a natural fit for Bnto's product identity — the name literally comes from the Japanese lunch box.

---

## Visual Design

### CompartmentNode

Each node renders as a `.surface` Card — the same Motorway elevation system used across the app.

```
┌─────────────────────┐
│                     │
│     [icon 32px]     │  ← Large category icon (Lucide), muted foreground
│                     │
│      Compress       │  ← Label (font-display, semibold, sm)
│      image          │  ← Sublabel (font-mono, xs, muted)
│                     │
└─────────────────────┘
```

Each node type has a distinct icon (from Lucide) that acts as its "building silhouette" — you can identify what a node does without reading its label. Icons are mapped via `editor/adapters/nodeIcons.ts`. Category-driven variant colors are mapped via `editor/adapters/nodeColors.ts`.

**Properties:**

- Variable width/height based on node type tier (compact 100px, standard 140px, wide 200px, container 240px+)
- Category-driven variant colors via `.surface-{variant}` CSS classes — nodes of the same category share a color (image=primary, spreadsheet=secondary, file=accent, data=muted, control=warning)
- Elevation-driven execution state: idle → none/sm (resting), pending → sm (waiting), active → md (rising), completed → lg (springy pop)
- Springy entrance animation (`Animate.ScaleIn` with `spring-bouncy` easing)
- Pressable interaction (Motorway press effect)

### Grid Layout

Compartments tile in a **3-row bento layout** — predefined slots of varying sizes:

```
┌────────┬────────────┬────────┬──────────┐
│   1    │     2      │   3    │    4     │  Row 0: alternating small/wide
├────────┴────────────┼────────┴──────────┤
│         5           │        6          │  Row 1: two wide panels
├──────────┬──────────┼────────┬──────────┤
│    7     │    8     │   9    │   10     │  Row 2: four equal cells
└──────────┴──────────┴────────┴──────────┘
```

**Grid constants:**

- Cell base: 120px
- Gap: 20px
- Total width: 660px
- Background: warm grid lines (`BackgroundVariant.Lines`, gap=40, color=`var(--border)`)

The layout algorithm assigns compartments to predefined slots. As compartments are added, the viewport smoothly zooms out to accommodate — the "city growing" effect.

### No Edges

The visual editor does **not** show connections between nodes. Execution order is implied by position — top-left to bottom-right, following natural reading order. This is a deliberate simplification:

- **Simpler UX** — no edge creation, no connection validation, no spaghetti wires
- **Matches the metaphor** — bento boxes don't have wires between compartments
- **Code editor handles advanced cases** — users who need explicit edge control use the JSON editor

---

## Execution State Visualization

When a recipe runs, compartments visually reflect execution progress:

| Status      | Elevation   | Visual                            | Meaning                  |
| ----------- | ----------- | --------------------------------- | ------------------------ |
| `idle`      | `none`/`sm` | Flat, resting in the bento box    | Not running              |
| `pending`   | `sm`        | Slight lift, muted appearance     | Waiting in queue         |
| `active`    | `md`        | Rising up — "being serviced"      | Currently processing     |
| `completed` | `lg`        | Full springy pop to max elevation | Done — satisfying bounce |

The progression flows through compartments in order — each physically rises as it processes, then pops to full height when complete. The Card `.surface` spring animations create the Mini Motorways "building materializing" feel automatically. As the recipe runs, compartments pop up one by one in sequence — like buildings appearing on the map.

---

## Interaction Model

### Adding Nodes

1. Click "Add" in the toolbar or open the NodePalette
2. Select a node type from the palette (10 types, grouped by category)
3. Compartment appears in the next available slot with default parameters
4. All operations dispatch to the shared editor store

### Configuring Nodes

1. Click a compartment to select it
2. NodeConfigPanel opens with schema-driven form fields
3. Fields auto-generated from `ParameterSchema` (Atomiton pattern)
4. `visibleWhen` / `requiredWhen` handled reactively
5. Parameter changes dispatch `updateParams` to store

### Removing Nodes

1. Select a compartment
2. Press delete or click "Remove" in toolbar
3. Compartment removed, remaining compartments reflow

### Rearranging

1. Drag compartments to reposition (when interactive mode enabled)
2. Grid snapping keeps things aligned
3. Position changes dispatch `moveNode` to store

---

## Technology

**React Flow (`@xyflow/react`)** powers the canvas:

- Node positioning on an invisible grid
- Zoom-to-fit with smooth animation
- Custom node components (`CompartmentNode`)
- Grid background rendering

React Flow is already used in the Motorway page prototype. The editor builds on this foundation.

**Lazy loaded** via `next/dynamic({ ssr: false })` — zero cost to non-editor pages.

---

## Existing Prototype

The Motorway showcase page (`/motorway`) has a working bento box prototype:

| File                                               | Purpose                                                           |
| -------------------------------------------------- | ----------------------------------------------------------------- |
| `apps/web/app/(dev)/motorway/BentoBoxShowcase.tsx` | Interactive add/remove controller with execution simulation       |
| `apps/web/app/(dev)/motorway/BentoCanvas.tsx`      | React Flow canvas with warm grid, zoom-to-fit                     |
| `apps/web/app/(dev)/motorway/CompartmentNode.tsx`  | Surface Card node with variant colors and status-driven elevation |

This prototype is read-only (no drag, no connect). Sprint 4 Wave 3 upgrades it to a full interactive editor wired to the shared store.

---

## Compound Nodes & Recursive Composition

The bento grid currently renders a **flat view of the root group's children**. In the [three-layer node model](node-architecture.md), a recipe is a compound node (a root `group` with children). The bento grid shows those top-level children as compartments.

**Today:** Casual users see a flat grid of compartments. Each compartment is either an I/O node or a processing node (engine primitive). No nesting is exposed in the UI.

**Future:** Power users will be able to "drill into" a compound node to reveal its internal subgraph. A recipe used as a node inside another recipe would appear as a single compartment that can be expanded. The bento grid would render the subgraph's children when expanded, using the same compartment layout recursively.

This is a UI extension — the data model already supports nesting via `PipelineNode.children` and `Definition.nodes`. The editor store and execution engine handle recursive structures. The visual editor just needs to add drill-in/drill-out navigation when compound nodes are introduced to the UI.

---

## What This Is NOT

- **Not the conveyor belt.** The conveyor belt system (stations + animated belts + sushi pieces) is a Motorway showcase feature, not the editor. See [conveyor-belt.md](conveyor-belt.md) for that R&D log.
- **Not the code editor.** The code editor is a separate skin on the same store. See [code-editor.md](code-editor.md).
- **Not a DAG/graph editor.** No wires, no directed acyclic graph. It's a spatial arrangement tool.

---

## References

| Document                                         | What It Covers                                                           |
| ------------------------------------------------ | ------------------------------------------------------------------------ |
| [editor-architecture.md](editor-architecture.md) | Shared editor layer — store, hooks, package strategy, switchable editors |
| [code-editor.md](code-editor.md)                 | CodeMirror 6 code editor — tech choice, slash commands, JSON Schema      |
| [conveyor-belt.md](conveyor-belt.md)             | Conveyor belt showcase — Motorway page R&D (not the editor)              |
| [node-architecture.md](node-architecture.md)     | Three-layer node model — primitives, control flow, compound nodes        |
| [PLAN.md](../PLAN.md) → Sprint 4                 | Visual editor task breakdown (Waves 1-4)                                 |
