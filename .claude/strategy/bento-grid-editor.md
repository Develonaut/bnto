# Bento Grid Editor — Unified Recipe View

**Status:** Proposed
**Last Updated:** March 29, 2026
**Supersedes:** Schema-driven config plan (folded into Phase 1 of this work)

---

## Vision

Replace both the ReactFlow canvas editor and the hand-written run page with a single **bento grid view** that toggles between run mode and edit mode. The recipe IS a bento box — compartments arranged in a CSS grid, each containing a focused piece of the recipe experience.

The outer container is a `<Surface elevation="none" border="dashed">` — literally a bento box. Inside, `<Card>` compartments hold recipe info, pipeline nodes, config controls, I/O, and settings. Each compartment loads independently and springs up via `<Card loading>` when ready.

**Why this is right:**

1. **ReactFlow solves a problem bnto doesn't have.** Recipes are linear pipelines, not arbitrary directed graphs. A vertical node list communicates execution order better than a canvas with edges.
2. **The bento grid IS the brand.** The product name, the visual metaphor, and the UI layout become one thing.
3. **Performance.** Drop `@xyflow/react` (SVG canvas, zoom/pan, minimap, synthetic node injection). Replace with CSS Grid + standard React components.
4. **Seamless mode toggle.** Authors see the same view as end users, with edit affordances layered on. No separate `/editor` route or mental model switch.
5. **Progressive loading.** Each compartment is a `<Card loading>` that springs up independently when its data arrives. Suspense boundaries at the compartment level, not the page level.

---

## Layout

### The Grid

```
+------------------------------------------------------------------+
|  Surface elevation="none" border="dashed" rounded="2xl"          |
|                                                                  |
|  +----------------+  +----------------+  +---------------------+ |
|  |                |  |                |  |                     | |
|  | Recipe Info    |  |    Input       |  |  Node Config        | |
|  | (title, desc)  |  |  (drop zone)  |  |  (SchemaForm)       | |
|  |                |  |                |  |                     | |
|  +----------------+  +----------------+  |                     | |
|  +----------------+  +----------------+  |                     | |
|  |                |  |                |  |                     | |
|  |  Toolbar       |  |   Pipeline     |  |                     | |
|  |  (run, code,   |  |   (node list)  |  |                     | |
|  |   settings)    |  |                |  |                     | |
|  |                |  +----------------+  |                     | |
|  +----------------+  +----------------+  +---------------------+ |
|                       |    Output      |                         |
|                       |  (download)    |                         |
|                       +----------------+                         |
+------------------------------------------------------------------+
```

Built with existing primitives:

```tsx
<Surface elevation="none" border="dashed" rounded="2xl">
  <Grid cols={6} rows={6} gap="md" animated>
    <GridItem colSpan={2} rowSpan={3}>
      <RecipeInfoCard /> {/* title, description, tags */}
    </GridItem>
    <GridItem colSpan={2} rowSpan={2} colStart={3}>
      <InputCard /> {/* drop zone / file upload */}
    </GridItem>
    <GridItem colSpan={2} rowSpan={6} colStart={5}>
      <NodeConfigCard /> {/* SchemaForm for selected node */}
    </GridItem>
    <GridItem colSpan={2} rowSpan={3} rowStart={4}>
      <ToolbarCard /> {/* run, code view, settings */}
    </GridItem>
    <GridItem colSpan={2} rowSpan={3} colStart={3} rowStart={3}>
      <PipelineCard /> {/* ordered node list */}
    </GridItem>
    <GridItem colSpan={2} colStart={3} rowStart={6}>
      <OutputCard /> {/* download results */}
    </GridItem>
  </Grid>
</Surface>
```

### Compartment Behavior by Mode

| Compartment         | Run Mode                                                                                         | Edit Mode                                                                            |
| ------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| **Recipe Info**     | Title + description (read-only). SEO heading server-rendered                                     | Title + description (editable). Slug, category                                       |
| **Input**           | Active drop zone. File list with delete. Accept types shown                                      | File type constraints. Accept config                                                 |
| **Pipeline**        | Execution progress per node (spinners, checkmarks). Collapsed to summary for single-node recipes | Ordered node list. Click to select → config panel. Drag to reorder. Add from palette |
| **Node Config**     | Processing node params via SchemaForm. Presets, sliders, selects                                 | Same + delete node button, advanced params, conditional visibility                   |
| **Toolbar**         | Run button, progress bar, download all                                                           | Run + code view toggle + recipe settings + help                                      |
| **Output**          | Download individual results, download all                                                        | Output format config, download                                                       |
| **Recipe Settings** | Hidden                                                                                           | Iteration mode (auto/explicit), pipeline settings                                    |

### Springable Loading

Each compartment is a `<Card loading={isLoading}>`. The loading flow:

1. Grid mounts immediately — the outer `Surface` and `Grid` are static layout
2. Each `Card` starts with `loading={true}` — grounded, muted, skeleton content
3. As data arrives (definition parsed, schema resolved, files selected), individual cards flip `loading={false}`
4. Card springs up with `spring="bounciest"` — face rises, walls appear, content pops in
5. Compartments load independently — fast ones pop first, slow ones follow

```tsx
function InputCard({ files, isReady }: InputCardProps) {
  return (
    <Card loading={!isReady} elevation="sm">
      {!isReady ? <Skeleton className="h-full w-full" /> : <DropZone files={files} />}
    </Card>
  );
}
```

This gives us **progressive disclosure** for free — the grid is the skeleton, each compartment pops in as it becomes ready. No page-level loading spinner.

### Future: Dynamic Grid

The grid layout is statically defined for now (fixed `cols`/`rows`/`GridItem` placement). Future iterations could make this dynamic:

- Recipes with many nodes → pipeline compartment grows, config panel scrolls
- Single-node recipes → pipeline collapses, config takes center stage
- During execution → input/output compartments expand, config shrinks
- Responsive → mobile stacks vertically, tablet shows 2 columns

The `Grid` and `GridItem` API already supports this — it's just a matter of computing `colSpan`/`rowSpan` from recipe shape.

---

## Architecture

### What Stays (renderer-agnostic)

These layers have zero ReactFlow coupling and work with any renderer:

| Layer                   | Location                                      | Why it stays                                                                                        |
| ----------------------- | --------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| **Editor Store**        | `packages/editor/src/store/`                  | `EditorState` is abstract — nodes, edges, configs, undo/redo, execution state. No renderer types    |
| **Actions**             | `packages/editor/src/actions/`                | 31 pure functions. Take `EditorState`, return `Partial<EditorState>`. Zero imports from ReactFlow   |
| **Services/Clients**    | `packages/editor/src/services/`, `clients/`   | Thin wrappers around actions + `storeApi.setState()`. Domain-namespaced API                         |
| **Definition adapters** | `packages/editor/src/adapters/`               | `definitionToGraph()`, `rfNodesToDefinition()` — logic is renderer-agnostic (types need decoupling) |
| **ConfigPanel**         | `packages/editor/src/components/ConfigPanel/` | Reads from store, calls `editor.definition.updateParams()`. Zero ReactFlow deps                     |
| **Toolbars/Menus**      | `packages/editor/src/components/`             | `EditorToolbar`, `EditorMenuPanel`, `RunPanel`, `NodePaletteDialog` — all store-driven              |
| **I/O Renderers**       | `packages/editor/src/components/`             | `InputRenderer`, `OutputRenderer` — store-driven                                                    |
| **@bnto/form**          | `packages/@bnto/form/`                        | `SchemaForm`, controls, schema resolution — the config UI engine                                    |
| **Engine + core**       | `packages/@bnto/nodes`, `@bnto/core`          | Definition format, node schemas, execution — untouched                                              |

### What Goes (ReactFlow-specific)

| Layer                     | Location                                       | Why it goes                                                                                                      |
| ------------------------- | ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **Custom nodes**          | `packages/editor/src/components/nodes/`        | `CompartmentNode`, `IoNode`, `AddDividerNode`, `ContainerGroupNode`, `PlaceholderNode` — all ReactFlow `Node<T>` |
| **Canvas shell**          | `packages/editor/src/components/EditorCanvas/` | `CanvasShell`, `Canvas`, `CanvasInner` — `<ReactFlow>` wrappers                                                  |
| **Rendering pipeline**    | `packages/editor/src/hooks/`                   | `useLayoutNodes`, `useExecutionNodes`, `usePlaceholderNodes`, `useAddDividerNodes` — RF-specific transforms      |
| **RF adapter types**      | `packages/editor/src/adapters/types.ts`        | `BentoNode = Node<CompartmentNodeData>` — tightly coupled to RF's `Node` type                                    |
| **`@xyflow/react` dep**   | `package.json`                                 | The library itself                                                                                               |
| **Hand-written configs**  | `apps/web/.../configs/`                        | 9 config components, registry, types — replaced by SchemaForm                                                    |
| **Run page phase system** | `apps/web/.../[bnto]/`                         | `RecipeShell`, `RecipePhaseContent`, `PhaseIndicator`, 3-step linear flow — replaced by grid compartments        |

### What's New (bento grid renderer)

| Component            | Purpose                                                                                            |
| -------------------- | -------------------------------------------------------------------------------------------------- |
| **`RecipeGrid`**     | The outer `Surface` + `Grid` layout. Accepts `mode: "run" \| "edit"` and `definition`              |
| **`RecipeInfoCard`** | Title, description, tags. Editable in edit mode                                                    |
| **`InputCard`**      | Drop zone (run) or input type config (edit). Reuses existing `DropZone` component                  |
| **`PipelineCard`**   | Vertical ordered list of nodes. Click to select, drag to reorder (edit). Progress indicators (run) |
| **`NodeConfigCard`** | `SchemaForm` for the selected node's parameters. The config panel, but in a grid compartment       |
| **`ToolbarCard`**    | Run button, code view toggle, recipe settings. Mode-aware actions                                  |
| **`OutputCard`**     | Download results (run). Output format config (edit)                                                |

### Type Decoupling

The editor's `BentoNode` type is currently `Node<CompartmentNodeData>` from ReactFlow. This needs to become renderer-agnostic:

```typescript
// Before (coupled to ReactFlow)
import type { Node } from "@xyflow/react";
type BentoNode = Node<CompartmentNodeData, "compartment" | "io" | ...>;

// After (renderer-agnostic)
interface EditorNode {
  id: string;
  type: string;
  data: NodeData;       // visual fields (icon, label, status)
  position?: Position;  // optional — grid renderer ignores this
}
```

The store, actions, and services work with `EditorNode`. The grid renderer reads `EditorNode[]` and renders them as list items in the pipeline card.

---

## Naming Standardization

With the unified view, terminology aligns across run and edit:

| Concept                | Old (divergent)                                                  | New (unified)                                                                                       |
| ---------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| User-facing values     | `config` (run) / `parameters` (editor)                           | **`parameters`** — matches engine                                                                   |
| Updating values        | `setConfig(obj)` (run) / `updateParams(nodeId, params)` (editor) | **`updateParams`**                                                                                  |
| Execution lifecycle    | `RunPhase` (run) / `ExecutionPhase` (editor)                     | **`ExecutionPhase`** — `uploading` is a transport concern on the phase, not a separate phase        |
| 3-step visual progress | `activePhase` (1/2/3)                                            | **`step`** — avoids collision with execution phase. Or removed entirely (grid makes steps implicit) |
| The view itself        | "Recipe page" (run) / "Editor"                                   | **Recipe view** with `mode: "run" \| "edit"`                                                        |
| The flow state         | `recipeFlowStore` / `EditorStore`                                | **`EditorStore`** — one store for both modes                                                        |

---

## URL Strategy

| URL                          | What it shows                  | Mode                                      |
| ---------------------------- | ------------------------------ | ----------------------------------------- |
| `/compress-images`           | Predefined recipe in run mode  | `run` (default for public visitors)       |
| `/compress-images?mode=edit` | Same recipe, edit mode         | `edit` (for authors/collaborators)        |
| `/r/[id]` (future)           | User-created recipe            | `run` or `edit` based on ownership        |
| `/editor` (deprecated)       | Currently the ReactFlow editor | Redirects to recipe URL with `?mode=edit` |

The `[bnto]/page.tsx` server component stays — it handles SSG, metadata, JSON-LD. The client component inside mounts the bento grid with the appropriate mode.

---

## Run Mode Flow (End User)

The 3-step linear flow (upload → configure → run → download) becomes implicit in the grid:

1. **Page loads** — Grid mounts. Input card has drop zone. Config card shows defaults. Pipeline shows node list. All cards spring in
2. **User drops files** — Input card shows file list. Config card is interactive. Run button enables in toolbar
3. **User adjusts params** — Config card updates live. Same SchemaForm as editor
4. **User clicks Run** — Pipeline card shows per-node progress (spinner/checkmark). Toolbar shows overall progress bar
5. **Execution completes** — Output card springs in with download buttons. Input card shows before/after comparison

No phase indicator needed — the grid's state IS the progress indicator. Empty output card = not run yet. Spinning pipeline = running. Output card with files = done.

---

## Edit Mode Flow (Author)

Same grid, with additional affordances:

1. **Pipeline card** — Nodes are clickable to select. Drag handle for reorder. "+" button to add from palette. Swipe/delete to remove
2. **Node config card** — Shows delete button, all params (including advanced/conditional). Visibility toggles
3. **Recipe info card** — Title and description are editable fields
4. **Toolbar card** — Code view toggle (shows raw `.bnto.json`), recipe settings panel (iteration mode), help
5. **Recipe settings** — Shown in toolbar card or as a dedicated compartment. Iteration mode, pipeline settings

The edit mode toggle is a simple button in the toolbar or nav. Authors see a "Preview" button that switches to run mode — they see exactly what end users see.

---

## Testing Strategy (TDD-First)

Every phase writes tests before implementation. The existing test suite is extensive and acts as a safety net — if existing tests break, we've changed behavior we shouldn't have.

### Test Layers

| Layer              | What                                                                             | Tool                              | When                                                                         |
| ------------------ | -------------------------------------------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------------- |
| **Pure functions** | `extractProcessingNodes`, param defaults derivation, definition walkers          | Vitest unit tests                 | Phase 1 — write first, implement second                                      |
| **Hooks**          | `useRecipeConfig`, `useRecipeParameters`                                         | Vitest `renderHook`               | Phase 1-2 — test state derivation logic                                      |
| **Components**     | Grid compartments render correctly per mode, loading states, springable entrance | Vitest + React Testing Library    | Phase 2-3 — render tests for each compartment                                |
| **Store**          | Unified store actions, mode transitions, parameter updates                       | Vitest unit tests on pure actions | Phase 2-3 — test actions in isolation                                        |
| **E2E**            | Full user journeys (drop files → configure → run → download)                     | Playwright                        | Phase 2+ — update existing browser journey specs once grid replaces run page |

### Existing Tests That Must Keep Passing

These suites validate behavior that doesn't change — the grid is a new renderer, not new execution logic:

- `packages/@bnto/form/` — SchemaForm, controls, schema resolution. Untouched
- `packages/@bnto/nodes/` — catalog, schemas, validation. Untouched
- `packages/@bnto/registry/` — recipes, node types. Untouched
- `packages/editor/src/actions/` — all 31 pure action tests. Untouched (actions are renderer-agnostic)
- `packages/editor/src/store/` — store tests for state shape, undo/redo. Untouched
- `apps/web/e2e/journeys/browser/` — execution journey tests. Updated in later phases when the UI changes

### Tests That Will Be Deleted

- `apps/web/.../configs/__tests__/types.test.ts` — tests hand-written config types (deleted in Phase 1)
- `apps/web/e2e/journeys/editor/` — editor-specific E2E tests that assert ReactFlow canvas behavior (replaced in Phase 3+)

### New Tests Per Phase

**Phase 1:**

- `extractProcessingNodes.test.ts` — walks definition, returns processing nodes, filters I/O and containers
- `deriveDefaults.test.ts` — collects parameter defaults from definition nodes
- `RecipeConfigSection.test.ts` — renders SchemaForm with correct schema per node type

**Phase 2:**

- `RecipeGrid.test.ts` — renders correct compartments for run mode
- Per-compartment render tests (`InputCard.test.ts`, `PipelineCard.test.ts`, etc.)
- Loading state tests — verify `<Card loading>` skeleton → content transition

**Phase 3:**

- Mode toggle tests — same grid, different affordances per mode
- Edit-specific interaction tests (reorder, add, delete nodes)

---

## Implementation Phases

### Phase 1: Schema-Driven Config (foundation)

Kill hand-written config components. Make `SchemaForm` the single config renderer for both run and edit.

**Tests first:**

- `extractProcessingNodes.test.ts` — definition walker, filters I/O and containers
- `deriveDefaults.test.ts` — collects defaults from node parameters
- Update `RecipeConfigSection` tests — renders SchemaForm, not registry lookup

**Then implement:**

- Create `extractProcessingNodes(definition)` utility
- Create `useRecipeConfig(definition)` hook
- Rewrite `RecipeConfigSection` to use `SchemaForm`
- Derive defaults from definition instead of `DEFAULT_CONFIGS`
- Delete 15 hand-written config files

**Verify:** `task ui:test` — all existing tests pass, new tests pass, deleted test files don't break anything.

### Phase 2: Bento Grid Layout

Build the grid view as the new recipe renderer. Initially for run mode only, replacing the current run page.

**Tests first:**

- `RecipeGrid.test.ts` — renders 6 compartments in correct grid positions
- Per-compartment tests — correct content for run mode, loading states
- Springable loading tests — `<Card loading={true}>` renders skeleton, `loading={false}` renders content

**Then implement:**

- Create `RecipeGrid` (outer Surface + Grid)
- Create compartment cards: `RecipeInfoCard`, `InputCard`, `PipelineCard`, `NodeConfigCard`, `ToolbarCard`, `OutputCard`
- Each card uses `<Card loading>` for springable entrance
- Wire to existing `useRecipeFlow` / `recipeFlowStore` for execution state
- Reuse existing components inside cards: `DropZone`, `SchemaForm`, `RunButton`, download UI

**Verify:** `task ui:test` + visual check on each recipe page. E2E browser journey tests updated if needed.

### Phase 3: Edit Mode

Add edit mode to the bento grid. This is where ReactFlow gets replaced.

**Tests first:**

- Mode toggle tests — `mode="edit"` shows edit affordances, `mode="run"` hides them
- Pipeline card edit tests — select, reorder, add, delete interactions
- Store unification tests — `EditorStore` drives both modes correctly

**Then implement:**

- Add `mode` prop to `RecipeGrid` (`"run" | "edit"`)
- Pipeline card gains: click-to-select, drag-to-reorder, add-from-palette, delete
- Config card gains: delete node button, advanced params, conditional visibility
- Recipe info card gains: editable title/description
- Toolbar card gains: code view, recipe settings
- Wire to `EditorStore` instead of `recipeFlowStore` (or unify them)

**Verify:** `task ui:test` + visual check. Edit mode works in the bento grid.

### Phase 4: Cleanup

Remove everything that's been replaced.

- Delete ReactFlow custom nodes, canvas shell, rendering pipeline hooks
- Remove `@xyflow/react` dependency
- Delete the `/editor` page (or make it a redirect)
- Delete the old run page components (phase system, RecipeShell, etc.)
- Decouple `BentoNode` type from ReactFlow's `Node` type
- Update/delete tests that reference removed code

**Verify:** `task check` — full quality gate. No references to removed code. Bundle size decreased.

### Phase 5: Polish (future)

- Dynamic grid layouts based on recipe shape
- Responsive breakpoints (mobile stacks, tablet 2-col)
- Transition animations between modes
- Compartment resize on execution state changes

---

## What This Unlocks

1. **User-created recipes** (`/r/[id]`) — fetch definition from Convex, render the same grid. Zero per-recipe work
2. **Embeddable recipes** — the grid is a self-contained component. Could be embedded in docs, blog posts, other sites
3. **Mobile-first editing** — CSS Grid is responsive by nature. ReactFlow's canvas zoom/pan doesn't work well on mobile
4. **Collaborative editing (future)** — the grid view is simpler to sync than a canvas with arbitrary node positions
5. **Performance** — drop `@xyflow/react`, SVG rendering, synthetic node injection. Ship less JS, render faster

---

## Risks & Mitigations

| Risk                                    | Mitigation                                                                                                                                                                                 |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Complex recipes don't fit in a list** | Bnto recipes are linear pipelines today. If branching/merging is needed later, we can add a canvas view as an alternative mode — but that's a product decision, not a technical constraint |
| **Loss of drag-to-position**            | Nodes have no meaningful x/y position in linear pipelines. Vertical order = execution order. Drag-to-reorder in the list is the right interaction                                          |
| **SEO regression**                      | `[bnto]/page.tsx` stays as a server component with SSG. The grid is a client component inside it. Same metadata, same JSON-LD                                                              |
| **Scope creep**                         | Phases are shippable independently. Phase 1 (schema config) and Phase 2 (grid run mode) ship value on their own                                                                            |
