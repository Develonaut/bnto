# Editor UX Direction — March 2026

**Created:** March 5, 2026
**Status:** Decision captured — informs Sprint 5 implementation
**Context:** Based on codebase review, Notion strategy docs, and research into comparable products (n8n, Make, Zapier canvas, Retool)

---

## The Core Tension

The editor is architecturally sound (store, adapters, schema-driven forms, controlled RF, action/hook layering are all correct). The felt problem is that nodes are visually passive — there's no affordance connecting canvas interaction to the config panel, and the editor can't run anything yet, so nothing feels finished.

**The single highest-leverage change: wire execution (Sprint 5 Wave 3).** An editor that can't run doesn't feel like a tool regardless of polish level.

---

## Design North Star

**Mini Motorways first level:** A world with just enough in it to tell you what to do next. Buildings pop in with spring. The system is simple but lets complexity grow. The UI that doesn't exist can't confuse anyone.

The visual editor is a bento box, not a flowchart. No edges. Position implies order. This is correct and must be maintained.

---

## Decisions

### Node Inline Controls: Minimal (Delete Only, On Hover)

**Decision:** Add one inline control to nodes — a delete × in the top-right corner, visible on hover only. Ghost styling, destructive on hover. Hidden for I/O nodes (protected).

**Rationale:** Every button on a node is a question the user has to answer. The config panel is already the config surface — clicking the node opens it. The toolbar handles add, navigate, undo/redo. Arrow reorder buttons belong in the LayerPanel, not on nodes. The Mini Motorways rule: the UI that doesn't exist can't confuse anyone.

**Do NOT add:** Arrow reorder buttons, explicit "open config" button, duplicate button. These increase cognitive load without adding capability the user doesn't already have.

### Card vs Surface: Keep Card + Pressable, Add Hover Overlay

**Decision:** Do NOT switch to `Surface` as the base node primitive. Keep `Pressable asChild` wrapping `Card` as the node. Add the delete control as an absolutely-positioned hover overlay with `stopPropagation`, not as an inline button inside the card.

**Rationale:**

- The `Pressable` spring + `Card` elevation-driven execution states are the Mini Motorways identity of the editor. Switching to Surface loses both.
- The whole card surface as a click target is intentional — one pressable thing, one action (select + open config). This matches Mini Motorways: tap the building.
- The tension between "card is a big click target" and "I also need delete" is solved by the overlay pattern, not by switching primitives.

**Implementation pattern:**

```tsx
<div className="group relative">
  <Pressable asChild spring="bounciest" toggle active={selected}>
    <Card elevation={...} style={{ width: w, height: h }}>
      {/* icon + label */}
    </Card>
  </Pressable>

  {/* Hover overlay — only for non-I/O nodes */}
  {!isIoNode && (
    <button
      onClick={(e) => { e.stopPropagation(); onDelete(); }}
      className="absolute top-1.5 right-1.5 opacity-0 group-hover:opacity-100
                 transition-opacity size-5 rounded-md
                 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
      aria-label="Remove node"
    >
      <XIcon className="size-3" />
    </button>
  )}
</div>
```

The `group/group-hover` Tailwind pattern handles hover state in pure CSS. `stopPropagation` prevents × click from also selecting the node. No JS state needed.

### Node Reordering: LayerPanel Only

**Decision:** Node reordering belongs in the LayerPanel's `NodeList`, not on canvas nodes. The LayerPanel is already the right surface for recipe structure management. Drag-to-reorder in the list is more reliable than canvas arrow buttons and adds no visual noise to the canvas.

Drag-and-drop from the node palette to canvas position is a future polish pass (already in PLAN backlog).

### Node → Config Panel Connection: Visual Identity Echo

**Decision:** When a node is selected, the config panel header should echo the node — same Lucide icon (already in `ICON_COMPONENTS`), same category variant color, same label. Creates a direct visual thread between the canvas click and the panel that opens.

**Implementation:** Add `Icon` rendering to `ConfigPanelRoot` header before the heading text. Source from `ICON_COMPONENTS[typeInfo.iconKey]`. 30-minute change, high felt impact.

### Config Panel Position: Right Slide-In (Keep As-Is)

**Decision:** The current right-side slide-in panel pattern is correct. This is the pattern n8n, Make, and Retool all converged on. Do not rethink it.

**What to improve (not rethink):**

- Add node icon to panel header (see above)
- Empty state: show node palette or "add a node" prompt instead of just "Select a node to configure"
- For nodes with many params (Loop, Group): add visual grouping with light dividers. `SchemaField` can support a `group` key.

### Empty Canvas: Palette Open by Default

**Decision:** On blank canvas (no `?from=` slug), the node palette menu should open automatically on first load with a "Add your first step" hint. Not a persistent sidebar — just the palette menu pre-opened. Close on first node add.

**Rationale:** Two scaffolded I/O nodes with nothing in between and a closed toolbar reads as empty and intimidating. The first frame should tell the user what to do.

**Additional:** Selecting the Input node on a blank canvas should immediately open the config panel (auto-select + config open). Don't make the user discover this.

### The Gap Between I/O Nodes: Dashed Placeholder Slot (Visual Only, Not in State)

**Decision:** When the canvas has only Input + Output nodes (nothing in between), render a `PlaceholderSlot` component at the center bento slot. Clicking it opens the node palette. It is NOT a node in state — it's a render-time affordance derived from a canvas condition.

**Why not in state:** A phantom node in state would need to be guarded against export, execution, validation, and undo/redo. It's visual scaffolding. Keep it in the render layer.

**Derived condition:** `nodes.length === 2 && nodes[0].data.nodeType === 'input' && nodes[1].data.nodeType === 'output'`

**Implementation:** `PlaceholderSlot` renders inside `BentoCanvas` when the condition is true. Uses `Card variant="muted"` with dashed border, `PlusIcon` centered, subtle pulse animation on the border. Click handler calls `openNodePalette()` from `useEditorPanels`. Disappears as soon as a real node is added to state.

**Rationale:** Visual affordance that the middle is empty and waiting. Mini Motorways equivalent: the road network shows you where roads can go.

### Node Visual Hierarchy: Three-Tier System

**Decision:** Establish three distinct visual tiers on the canvas through a combination of size, color, elevation, and selection treatment. Every visual difference communicates information — nothing is decorative.

**The tiers:**

| Tier       | Nodes             | At Rest                                     | Selected                                    | Reasoning                                                                                                                                                                   |
| ---------- | ----------------- | ------------------------------------------- | ------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Structural | `input`, `output` | `color="muted"`, `elevation="sm"`, `160×90` | `elevation="md"`, subtle lift               | These are the bento box walls, not compartments. Muted cream reads as "part of the canvas". Wider/shorter shape signals "different kind of thing" before the label is read. |
| Processing | all others        | `elevation="md"`, `120×120`, white card     | `elevation="lg"` + `ring-2 ring-primary/60` | Lifted above the I/O anchors. Square shape. Category left border (3px) gives category signal without dominating.                                                            |
| Selected   | any node          | —                                           | Ring + elevation                            | Ring is unmistakable at a glance. Elevation change alone (current state) is too subtle.                                                                                     |

**Why `color="muted"` for I/O:** The `muted` Surface variant in the design system renders as warm off-cream — effectively the canvas background color. A muted card visually sinks into the canvas rather than rising above it. This is the right metaphor: Input/Output are the _container_, not a step inside it.

**Why different shape (not just color):** Shape communicates category-level difference even in peripheral vision, before the user focuses. The same rounded square for all three types means the user has to read labels to understand structure. `160×90` vs `120×120` breaks that sameness immediately.

**Why a ring for selected (not just elevation):** Elevation changes are too subtle for selection state — especially on a light background. The `ring-2 ring-primary/60 ring-offset-2` pattern creates a visible terracotta outline that reads as "active focus" at a glance. This is how most design systems handle selection (n8n, Figma, Retool).

**I/O Pressable behavior:** I/O nodes remove `toggle` and `active` from their `Pressable` wrapper. They can receive clicks for selection feedback, but shouldn't feel like pressable buttons — they're structural, not configurable. The config panel does not open for I/O node clicks in the same way.

**Category left border:** Processing nodes get a `border-l-[3px]` in their category color (`categoryBorderColor` map, matches `CompartmentVariant`). 3px is visible without dominating. I/O nodes get no pip — their color is already their identity signal.

### Execution Progress: Elevation Sequence (Sprint 5 Wave 3)

**Decision:** The elevation-driven execution sequence is the most signature UX moment in the editor and should be the focus of Sprint 5 Wave 3. Prioritize this over node visual polish.

| State       | Elevation | Visual                        |
| ----------- | --------- | ----------------------------- |
| `idle`      | `sm`      | Resting in bento box          |
| `pending`   | `sm`      | Slight lift, muted appearance |
| `active`    | `md`      | Rising — node is processing   |
| `completed` | `lg`      | Spring pop to max elevation   |

The spring animation on Card elevation creates the Mini Motorways "building materializing" feel. When a recipe runs, compartments pop up in sequence — like buildings appearing on the map. **This is the moment the editor becomes Bnto.**

### Run Button: Center Toolbar, Primary Variant

**Decision:** Run button goes center-left in the toolbar, primary variant (terracotta), between the node navigation group and undo group. Prominent but not overwhelming.

Sprint 5 Wave 3 responsibility. The editor without a run button is an editor that can't be finished.

### What to NOT Copy From Comparable Products

| Product           | Pattern                                 | Verdict                                               |
| ----------------- | --------------------------------------- | ----------------------------------------------------- |
| n8n               | Edge drawing / wire connections         | ❌ Already decided — no edges, position implies order |
| Make (Integromat) | Modal inspector over canvas             | ❌ Hides canvas during config, wrong for Bnto         |
| Zapier            | Linear step list masquerading as canvas | ❌ Bnto's spatial grid is the differentiator          |
| Retool            | Dense property inspector with tabs      | ❌ Too enterprise-y, wrong tone                       |

**What to steal:** React Flow canvas + right-side inspector is the correct pattern (n8n, Retool both use it). You're already doing this.

---

## Priority Order for Sprint 5

1. **Wave 3: Wire execution** — Run → WASM → elevation progress → download. Nothing else makes the editor feel real until this ships.
2. **Wave 2 refinement: Empty canvas entry** — Palette pre-open, auto-select Input node, dashed placeholder slot.
3. **ConfigPanel header echo** — Add icon to panel header. 30 minutes, high felt impact.
4. **Node hover delete** — Single inline control on non-I/O nodes.
5. **SchemaForm grouping** — Visual dividers for multi-param nodes.

Phases 2-3 of the CompartmentNode visual redesign (elevation execution states, bento grid varied sizes) are already in the backlog and should land in Sprint 5 Wave 3 alongside execution wiring.

---

## Editor Entry Point Strategy

**Decision (March 2026):** The editor is opt-in for power users — not the default experience for recipe pages.

**Three entry points:**

| Entry Point                   | Route                   | Intent                                 | Users                   |
| ----------------------------- | ----------------------- | -------------------------------------- | ----------------------- |
| Nav "Create"                  | `/editor` (no params)   | Build a new recipe from scratch        | Power users, developers |
| Recipe page CTA               | `/editor?from={slug}`   | Fork and customize a predefined recipe | Curious/advanced users  |
| My Recipes (Sprint 5 Wave 4+) | `/editor?id={recipeId}` | Edit a saved recipe                    | Returning users         |

**Predefined recipe pages are NOT replaced by the editor.** The 3-step flow (Files → Configure → Results) on `/compress-images` etc. is the right home for casual users. Don't push them into an editor canvas — they just want their files processed.

**"Customize in Editor" button** on recipe pages is secondary — a power-user CTA below the primary file drop, not promoted to primary. Consider stronger copy: "Open in Editor" or "Build your own version" reads as more of an invitation than "Customize in Editor".

**The readonly editor (future state):** Eventually predefined recipe pages may embed a readonly canvas view showing the recipe flow — purely informational, not interactive. This is a different component from the full editor (shares visual language, not functionality). Do not conflate the two when building Sprint 5.

**Nav label:** Change "Create" → "**New Recipe**" now. It's clearer about what you're creating and matches the mental model of the product (recipes, not generic content). No need to wait for the editor to ship.

**"Customize in Editor" button copy:** Change to "**Open in Editor**" — cleaner, more direct, and pairs naturally with the "New Recipe" nav label. "Customize" implies minor tweaks; "Open" implies full access.

**Future consideration — "Build your own version"** or a similar possessive CTA could be A/B tested once there's enough traffic to measure conversion from recipe pages into the editor.

---

## Vision: Edit Mode ↔ Run Mode (Mini Motorways Pattern)

**Concept (March 2026):** The editor and the recipe execution experience are the same surface, not two different screens — but with two modes. Like Mini Motorways: you pause to edit the road network (the bento grid is visible, nodes are moveable, config panels open), then unpause to run it (the grid fades away, traffic flows, you watch the result).

**This is not MVP.** This is the north star the MVP should not foreclose. All Sprint 5 and 5A/5B decisions should be made in a way that doesn't make this harder to build later.

**The mode switch:**

- **Edit mode:** Canvas grid visible. Nodes are pressable/selectable. Config panel slides in. LayerPanel open. Toolbar shows node management controls. Run button in toolbar (disabled if no processing nodes).
- **Run mode:** Canvas grid fades or hides. Nodes are static (no clicking). Config panel closed. LayerPanel closed. Toolbar collapses to just a stop/cancel button. Nodes animate through elevation sequence as execution progresses. Results appear at the Output node.
- **Transition:** A single button (the Run button) triggers the mode switch. Pressing Stop returns to edit mode at exactly the same canvas state.

**Why this is better than two screens:**

- Recipe pages (`/compress-images`) remain the casual entry point — drop files, run, download. No editor.
- The editor is the power-user surface. Once you're in the editor, you shouldn't have to leave it to run your recipe. Edit → run → tweak → run again is the power-user loop.
- Shared components: `CompartmentNode`, `EditorToolbar`, `ConfigPanel` all already exist. Run mode is mostly a _rendering state_, not a new screen.
- The elevation-driven execution sequence (idle → pending → active → completed) is already planned for Sprint 5 Wave 3 — it's the visual language of run mode.

**Implementation path (not Sprint 5 — future sprint):**

1. Sprint 5 Wave 3 wires execution to the existing editor screen (edit mode only, run starts but editor doesn't change modes yet).
2. Future sprint: Add `editorMode: "edit" | "run"` to the editor store. Run mode hides the canvas grid, closes panels, collapses toolbar. Animate the transition.
3. Future sprint: Predefined recipe pages (`/compress-images`) optionally embed a readonly run-mode-only editor view — the flow diagram that shows what's happening to your files.

**What not to build now:** Don't build the mode switch in Sprint 5. Wire execution first. The mode switch is polish that requires execution to exist. Doing it before execution is wired is building UI for a feature that doesn't work yet.

**Nav label:** "Create" is correct for MVP. Future consideration: rename to "New Recipe" once the editor ships and users understand what it does.

## What's Actually Good (Don't Touch)

- The layered architecture: pure actions → thin wrapper hooks → consumer hooks → dumb components. This is correct.
- The store ownership model: Zustand owns, RF renders as controlled props. No sync bugs.
- SchemaForm + controls registry: schema-driven forms with no manual field switching. Right call.
- The slide-in panel pattern for config. Right call.
- No edges in the visual editor. Right call.
- The bottom-center floating toolbar. Right call.
- `ScaleIn from={0.7} easing="spring-bouncy"` on node entrance. This is the Mini Motorways moment.

---

## References

- [visual-editor.md](../strategy/visual-editor.md) — canvas design, grid layout, execution states
- [editor-architecture.md](../strategy/editor-architecture.md) — store, hooks, adapters
- [editor-user-journey.md](../strategy/editor-user-journey.md) — user flows, success criteria
- [design-language.md](../strategy/design-language.md) — Mini Motorways reference, motion principles
- PLAN.md → Sprint 5 — production route task breakdown
