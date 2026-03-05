# Editor User Journey

**Created:** March 4, 2026
**Status:** Strategy document — guides editor implementation from backlog through delivery
**Related:** [editor-architecture.md](editor-architecture.md), [visual-editor.md](visual-editor.md), [io-nodes.md](io-nodes.md), [pricing-model.md](pricing-model.md)

---

## What This Is

The complete user journey for the bnto recipe editor — from discovery to export/save. This is the "home base" for all editor work. Agents reference it when picking up tasks. Each implementation wave links back to journey stages. E2E tests map to journey flows.

**Relationship to other docs:**
- [editor-architecture.md](editor-architecture.md) — HOW the editor is built (layers, store, hooks, components)
- [visual-editor.md](visual-editor.md) — HOW the bento box canvas looks and behaves
- [io-nodes.md](io-nodes.md) — HOW input/output nodes make recipes self-describing
- **This doc** — WHAT the user experiences end-to-end, and HOW we verify it works

---

## Journey Stages

| Stage | Name | Entry Condition | Exit Condition | Key Interaction |
|-------|------|----------------|----------------|-----------------|
| 1 | **Discover** | User is on a recipe page or navigates to `/editor` | Editor route loads | Recipe page bridge button, nav link, direct URL |
| 2 | **Enter** | Editor route loaded | Canvas rendered with initial state | Auto-scaffold I/O nodes (blank) or load predefined recipe (`?from={slug}`) |
| 3 | **Build** | Canvas has nodes | User has configured at least one processing node | Add nodes from palette, configure via config panel, remove/rearrange |
| 4 | **Test** | Recipe has input + processing + output nodes | Execution completes with results | Drop files into input node, click Run, see progress, receive output |
| 5 | **Refine** | First execution complete | User is satisfied with results | Adjust parameters, re-run, compare output |
| 6 | **Export** | Recipe is valid | `.bnto.json` downloaded | Click Export, file saves to disk |
| 7 | **Save** | User is authenticated (Free Account+) | Recipe persisted to Convex | Click Save, recipe appears in My Recipes |

---

## Entry Points

### `/editor` route

- **Access:** AccountGated — unauthenticated users see sign-in prompt (Free Account+)
- **Blank canvas:** Auto-scaffolds Input + Output compartments. User fills the middle.
- **From recipe:** `/editor?from={slug}` loads a predefined recipe's definition into the editor. All nodes pre-populated, fully configured, ready to run or customize.

### "Open in Editor" bridge button

Recipe pages (`/compress-images`, etc.) get an "Open in Editor" button that navigates to `/editor?from={slug}`. This bridges the casual experience (recipe pages) with the power-user experience (editor).

**The two experiences are complementary:**
- Recipe pages = casual, single-purpose, zero learning curve
- Editor = power user, compose anything, full control

### Navigation

- `/editor` appears in the app navigation (authenticated users only)
- Accessible from recipe page bridge buttons

---

## Interaction Model

### Config Panel = Universal Interaction Surface

The config panel (right sidebar) is the primary interaction surface for ALL node types. When a node is selected on the canvas, the config panel shows that node's schema-driven form fields.

**For Input nodes:** The config panel renders a file dropzone + file list. This is where users add files to the recipe.

**For Output nodes:** The config panel renders download controls — file list with download buttons, auto-download toggle, ZIP option.

**For Processing nodes:** The config panel renders the node's parameter form — quality sliders, format dropdowns, column mappings, etc. All auto-generated from `@bnto/nodes` `ParameterSchema`.

**Key insight:** The canvas shows WHAT nodes exist and their arrangement. The config panel shows HOW each node is configured. Click a compartment to see its details.

### Elevation-Driven Execution Progress

The bento box IS the progress indicator. Compartments physically rise as they execute:

| Execution State | Elevation | Visual Effect |
|----------------|-----------|---------------|
| `idle` | `sm` | Resting in the bento box |
| `pending` | `sm` | Slight lift, muted appearance — waiting in queue |
| `active` | `md` | Rising up — node is processing |
| `completed` | `lg` | Full pop with spring bounce — done |

No separate progress bar needed. The canvas communicates state through the Motorway elevation system, with springy Card animations creating the Mini Motorways "building materializing" feel.

### No Edges

Execution order = compartment position (array order in the definition). No wires, no drag-to-connect. The bento box metaphor is about spatial arrangement, not wiring. This is a deliberate simplification documented in [editor-architecture.md](editor-architecture.md#no-edges-in-the-visual-editor).

---

## User Flows

### Flow 1: Casual User (recipe page → editor)

```
1. User is on /compress-images, has used it before
2. Notices "Open in Editor" button
3. Clicks it → navigates to /editor?from=compress-images
4. Editor loads with Input (file-upload) + Image Compress + Output (download) pre-configured
5. User drops 3 JPEGs into the Input node's config panel dropzone
6. Clicks Run → compartments pop up in sequence (idle → active → completed)
7. Output node's config panel shows 3 compressed files with download buttons
8. User downloads, satisfied
9. Curiosity: adds a "Resize" node from the palette between Compress and Output
10. Re-runs with resize → gets compressed AND resized images
11. Exports the custom recipe as .bnto.json
```

### Flow 2: Power User (blank canvas)

```
1. User navigates to /editor directly
2. Blank canvas: Input + Output compartments auto-scaffolded
3. Opens node palette → adds "CSV Clean" processing node
4. Selects Input node → config panel shows dropzone → drops a CSV file
5. Selects CSV Clean node → config panel shows column operations
6. Configures: remove empty rows, trim whitespace
7. Clicks Run → compartments pop in sequence
8. Selects Output node → config panel shows cleaned CSV for download
9. Exports as my-csv-cleaner.bnto.json
10. Saves to account (Free tier: 3 recipes max)
```

### Flow 3: Returning User (saved recipe)

```
1. User navigates to My Recipes (/my-recipes)
2. Sees their saved recipes → clicks one to open in editor
3. Editor loads with their saved definition
4. Makes adjustments, re-runs, re-saves
```

---

## Success Criteria

### 1. Task Completion Rate

**Test:** Build a compress-images recipe from scratch (blank canvas), run it with 3 test images, download results. Must complete in under 5 minutes by a user who has never seen the editor before.

**Acceptance:**
- Blank canvas auto-scaffolds Input + Output
- Adding a processing node takes < 3 clicks
- Config panel renders appropriate controls for each node type
- Run executes successfully with visible progress
- Output is downloadable

### 2. Round-Trip Fidelity

**Test:** Export a recipe as `.bnto.json`, re-import it into the editor. The loaded state must be identical to the exported state.

**Acceptance:**
- All nodes present with correct types and positions
- All parameters preserved (values, not just keys)
- Validation passes on the re-imported recipe
- Running the re-imported recipe produces identical output

### 3. Predefined Recipe Parity

**Test:** Every predefined recipe that runs on its recipe page must also run correctly when loaded in the editor via `?from={slug}`.

**Acceptance:**
- All 6 Tier 1 recipes load correctly in the editor
- File input works via the Input node's config panel dropzone
- Execution produces the same output as the recipe page
- Output is downloadable from the Output node's config panel

---

## E2E Test Matrix

See [journeys/editor.md](../journeys/editor.md) for the full test matrix.

---

## Phased Delivery

### Wave 1: Production Route + Entry (refs: Discover + Enter)

Get the editor accessible as a real route with proper entry points.

- `/editor` route with AccountGate (sign-in prompt for unauthenticated users)
- `?from={slug}` query param loads predefined recipe from `@bnto/nodes` registry
- Auto-scaffold Input + Output compartments for blank canvas
- Navigation link in app nav (authenticated users only)
- "Open in Editor" bridge button on recipe pages

**Exit criteria:** User can navigate to `/editor`, see a canvas, and load a predefined recipe.

### Wave 2: Input/Output Nodes (refs: Build + Test, Interaction Model)

Make recipes self-describing with explicit I/O nodes so custom editor creations can run.

- `input` and `output` node types in `@bnto/nodes` with schemas
- Input node compartment on canvas with config panel = dropzone + file list
- Output node compartment on canvas with config panel = download list + auto-download toggle
- Update all 6 predefined recipe definitions with explicit I/O nodes
- Generic `InputRenderer` and `OutputRenderer` driven by node parameters
- Input/Output nodes are always present — users can configure but not delete them

**Exit criteria:** A recipe with I/O nodes can accept files and deliver results through the editor's config panel.

**Dependency:** Sprint 4C (I/O Nodes) provides the `@bnto/nodes` foundation.

### Wave 3: Execution Integration (refs: Test + Refine)

Wire the Run button to the browser WASM engine so recipes execute inside the editor.

- Connect Run button → `core.executions.createExecution()` → browser WASM engine
- Elevation-driven progress: compartments pop as nodes execute (idle → active → completed)
- Results routed to Output node's config panel (download list)
- Auto-download toggle on Output node
- Reset/re-run flow (clear results, re-execute)
- Error states on individual compartments (node failure → destructive variant)

**Exit criteria:** User can drop files, click Run, see compartments pop, and download results — all inside the editor.

### Wave 4: Save + Bridge (refs: Save, Feature Funnel)

Persistence for authenticated users and the recipe page bridge.

- Save recipe to Convex (authenticated users)
- Tier limits: Free = 3 saved recipes, Pro = unlimited
- "Open in Editor" button on all 6 recipe pages → `/editor?from={slug}`
- My Recipes integration (load saved recipes into editor)
- Dirty state tracking (unsaved changes warning)

**Exit criteria:** Authenticated user can save, reload, and manage recipes. Recipe pages bridge to the editor.

### Wave 5: E2E + Polish (refs: E2E Matrix, Success Criteria)

Comprehensive test coverage and keyboard shortcuts.

- Full E2E test suite covering all journey flows (see [journeys/editor.md](../journeys/editor.md))
- Keyboard shortcuts: Cmd-Z/Cmd-Shift-Z (undo/redo), Delete (remove node), Cmd-Enter (run), Cmd-S (export)
- Journey matrix verified against all three success criteria
- Accessibility audit (focus management, screen reader labels)

**Exit criteria:** All E2E tests pass. All three success criteria met.

---

## Future Direction

### Recipe Page Convergence (Vision)

Recipe pages (`/compress-images`) are essentially read-only editor views — same compartment grid, same Input/Output pattern, same execution flow. Long-term, recipe pages become locked-down editor views rendered from the same component system.

```
Today:
  /compress-images  → RecipeShell (hardcoded per-slug config)
  /editor           → RecipeEditor (generic, schema-driven)

Future:
  /compress-images  → RecipeEditor (locked mode, pre-loaded definition)
  /editor           → RecipeEditor (full mode, blank or loaded)
```

Two modes, one rendering engine. The recipe page is a RecipeEditor with:
- Definition pre-loaded and locked (no add/remove nodes)
- Config panel open by default for the Input node
- "Open in Editor" button unlocks full editing

**This is vision, not commitment.** Recipe pages work well today with their purpose-built UX. Convergence happens when the editor is mature enough that a locked-down editor view is genuinely better than the current recipe page experience.

### Community Recipe URL Namespace (Open Decision)

When community recipes ship, predefined root-level slugs (`bnto.io/compress-images`) can't serve all recipes — uniqueness is per-author, not global. The URL namespace needs a strategy:

**Options under consideration:**
- `/@user/slug` — Twitter-style author namespace
- `/r/{id}` — Short ID, author shown on page
- `/recipes/{user}/{slug}` — Explicit namespace

**Not decided yet.** This needs its own strategy discussion when community recipes are closer to shipping. Current predefined recipes keep their root-level slugs regardless.

---

## Out of Scope

These are explicitly NOT part of this journey:

- **Code editor** — Sprint 4B, separate feature with its own persona (`/code-editor-expert`). Shares the headless store but is a distinct editing experience
- **Edges/connections** — Design decision: no edges in the visual editor. Order = position
- **Inline controls on nodes** — Future polish. Config panel is the interaction surface
- **Server-node execution** — M4/Pro. Browser execution only for now
- **Sharing/collaboration** — Pro feature, depends on save infrastructure
- **Container node nesting** — Group/loop as collapsible sub-canvases. Future enhancement
- **Drag-and-drop from palette** — Click-to-add for now. Drag is a polish pass
