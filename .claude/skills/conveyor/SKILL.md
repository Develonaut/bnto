---
name: conveyor
description: Iterate on conveyor belt connection system — build, test, and advance phases
---

# Conveyor Belt Connection System — Iteration Skill

You are working on the **conveyor belt connection system** for the Motorway design system. This is a Mini Motorways-style "level" built with React Flow — surface nodes as buildings, animated conveyor belts as roads, sushi pieces as traveling data items. It lives on the Motorway showcase page (`/motorway`).

## Before Every Iteration

1. **Read the research document:** `.claude/research/conveyor-belt-connections.md` — single source of truth for current phase, design decisions, open questions, and progress log.
2. **Read the Motorway page:** `apps/web/app/(dev)/motorway/page.tsx` — understand the section structure.
3. **Read existing conveyor components:** Check `apps/web/app/(dev)/motorway/Conveyor*.tsx`, `StationNode.tsx`, `ConveyorEdge.tsx` for what's already been built.
4. **Read globals.css conveyor section:** Search for "Conveyor Belt" in `apps/web/app/globals.css` for the belt CSS classes and keyframes.
5. **Read the surface system:** Search for `.surface` in `surface.css` to understand shadow tokens, light angle, and elevation tiers.

## The Concept

A **Mini Motorways level** on the Motorway page. React Flow provides the invisible grid. Nodes are `.surface` Cards (the "buildings"). Connections are thick animated conveyor belts (the "roads"). Data items (sushi) ride the belts between processing stations.

The grid is invisible — users see a clean canvas with buildings and roads, like a bird's-eye view of a sushi factory floor. Under the hood, React Flow handles positioning, interaction, and SVG edge rendering.

## Architecture

```
ConveyorShowcase (showcase wrapper)
  └─ ConveyorCanvas (React Flow <ReactFlow> instance)
       ├─ nodeTypes: { station: StationNode }     ← .surface Card
       ├─ edgeTypes: { conveyor: ConveyorEdge }   ← thick animated SVG path
       ├─ SVG <defs> for belt shadow filter
       └─ hardcoded nodes[] + edges[] data (Phase 1)
```

**Key files:**
- `ConveyorShowcase.tsx` — section wrapper, lazy-loads the canvas
- `ConveyorCanvas.tsx` — React Flow instance with custom types
- `StationNode.tsx` — custom node: `.surface` Card with Handles
- `ConveyorEdge.tsx` — custom edge: thick SVG path with animated dash pattern
- `globals.css` — belt keyframes, shadow filter, CSS fallback classes

**Lazy loading:** React Flow is heavy (~45KB gz). The canvas component must be loaded via `next/dynamic` with `ssr: false` to avoid SSR issues and keep the bundle lean for other pages.

## Phases

| Phase | What | Status |
|---|---|---|
| **1** | React Flow canvas + depth nodes + thick belt edges | In progress |
| **2** | Animated belt surface + theming + speed variants | Not started |
| **3** | Sushi piece animation (data items traveling belts) | Not started |
| **4** | Interactive editing (drag, connect, delete) | Not started |
| **5** | Recipe data integration (parse .bnto.json → graph) | Not started |
| **6** | Execution visualization (real-time progress) | Not started |

## Rules for Each Iteration

### Visual Quality

- **Match the Mini Motorways aesthetic.** Clean, minimal, colorful buildings on a quiet grid. The showcase should look like a level from the game.
- **Use bnto theme tokens** — `var(--muted)`, `var(--border)`, `var(--primary)`, depth variant tokens. Never hardcode colors.
- **Surface nodes must use the real `.surface` system** — not approximations. Apply `elevation="sm"` and `surface-{variant}` classes.
- **Belt shadow matches `.surface` language** — hard-edge, warm-tinted, directional (follows `--light-angle: 135deg`).
- **Respect `motion-safe:`** — all belt animations guarded. Static fallback for reduced motion.
- **Invisible grid** — React Flow's `<Background>` is either hidden or shows a very subtle dot pattern. Never a visible line grid.

### Code Quality

- **Co-locate in the Motorway page folder:** `apps/web/app/(dev)/motorway/`
- **One component per file:** `ConveyorCanvas.tsx`, `StationNode.tsx`, `ConveyorEdge.tsx`, etc.
- **Lazy load React Flow:** Use `next/dynamic` with `ssr: false` for the canvas component.
- **CSS in globals.css** for keyframes and belt base classes. Component-specific styles via Tailwind utilities.
- **Keep the showcase read-only for Phase 1.** No interaction needed yet — just a beautiful static scene.
- **Follow the `ShowcaseSection` pattern** — already wired with `id="conveyor"`.

### React Flow Specifics

- **Custom node type registration:** `nodeTypes={{ station: StationNode }}` — memoize the object to avoid re-renders.
- **Custom edge type registration:** `edgeTypes={{ conveyor: ConveyorEdge }}` — same memoization.
- **Use `getSmoothStepPath`** for orthogonal belt routing with `borderRadius` for rounded corners.
- **SVG `<defs>`** for shared filters (belt shadow). Define once in the React Flow SVG layer.
- **Handles** are React Flow's connection points. Style them as belt rollers or hide them (`opacity: 0`, `width: 0`).
- **`fitView`** on the React Flow instance so the scene auto-fits the viewport.
- **Disable interaction for Phase 1:** `nodesDraggable={false}`, `nodesConnectable={false}`, `elementsSelectable={false}`.

### Belt Edge Rendering

Two-layer SVG path approach:

1. **Base path** — solid thick stroke, muted color (belt body/frame)
2. **Surface path** — dashed stroke on top, animated `stroke-dashoffset` (moving belt segments)

Both use `strokeLinecap="round"` for rounded endpoints. `getSmoothStepPath` with `borderRadius` for rounded orthogonal corners.

Shadow via SVG `<feDropShadow>` filter or CSS `filter: drop-shadow(...)`.

### Iteration Workflow

1. **Read** the research doc to know the current phase and what's next
2. **Build** the next increment (one meaningful visual step)
3. **Verify** — run `task ui:build` to confirm compilation. Visually verify on the Motorway page if dev server is running.
4. **Update** the research doc:
   - Move the phase status forward if completed
   - Add entry to the Progress Log with date and what was done
   - Record design decisions or discoveries
   - Update open questions (close resolved ones, add new ones)
5. **Show** the user what was built (describe the visual result)

### When Adding Dependencies

Phase 1 requires `@xyflow/react`. When adding:
- Install: `pnpm add @xyflow/react --filter @bnto/web`
- Import React Flow's base CSS in the canvas component (not globally)
- Verify bundle impact with the build

ELK.js (`elkjs`) is deferred to Phase 4+ when auto-layout is needed.

## Key Reference Files

| File | What |
|---|---|
| `.claude/research/conveyor-belt-connections.md` | Research doc — phases, decisions, progress |
| `apps/web/app/(dev)/motorway/page.tsx` | Motorway showcase page |
| `apps/web/app/(dev)/motorway/ShowcaseSection.tsx` | Section wrapper pattern |
| `apps/web/app/(dev)/motorway/CardShowcase.tsx` | Example: simple showcase |
| `apps/web/app/(dev)/motorway/AnimationShowcase.tsx` | Example: complex showcase with state |
| `apps/web/app/surface.css` | Surface system, elevation tiers, shadow tokens |
| `.claude/rules/animation.md` | Motion language rules |
| `.claude/rules/theming.md` | Color tokens, radius, shadows |
| `.claude/rules/components.md` | Component patterns |

## Mini Motorways Reference

Visual reference images: `/Users/ryan/Documents/Mini Motorways Reference/`

- `blake-wood-munich.jpg` — best road detail (thick roads, rounded corners, buildings on top)
- `blake-wood-tokyonight.jpg` — dark mode, layering visible
- `blake-wood-colours.jpg` — color palette variety across 12 city themes
- `blake-wood-earlyconcept.jpg` — city concept art, overhead map aesthetic
- `1_D_5HwcEgOLoCiBNy9MoA1g.webp` — title screen, simplest road (two buildings, one path)

**Remember:** We're adapting the road language into conveyor belts + sushi. Same visual properties (thick, rounded, shadowed, animated), different metaphor.
