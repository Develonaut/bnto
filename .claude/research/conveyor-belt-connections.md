# Conveyor Belt Connections — Research & Progress

**Status:** Phase 3 — Sushi pieces traveling on belt paths
**Last Updated:** 2026-02-26
**Location:** Motorway page (`/dev/motorway`) → "Conveyor Belts" section (bottom)

---

## Concept

Build a **Mini Motorways-style level** on the Motorway showcase page. React Flow provides the invisible grid and layout engine. Nodes are bnto depth cards (the "buildings"). Connections are animated conveyor belts (the "roads"). The whole thing looks like a bird's-eye view of a sushi factory floor where data flows between processing stations.

This is the future recipe pipeline visualizer — how users will see their `.bnto.json` recipes rendered as interactive maps.

### Visual Language

| Game Element | Bnto Equivalent | Implementation |
|---|---|---|
| Buildings (colored blocks) | Recipe nodes (stations) | `.surface` Cards as React Flow nodes |
| Roads (thick colored paths) | Conveyor belts (data connections) | Custom React Flow edge with thick SVG stroke + animated dash |
| Cars moving on roads | Sushi pieces on belts | SVG element animated via `getPointAtLength()` |
| Invisible grid | Layout engine | React Flow + ELK.js orthogonal routing |
| Map terrain | Background | Subtle grid or clean background |
| Roundabouts | Merge/fan-in nodes | Junction circles at node anchors |

### Key Visual Properties

1. **Invisible grid** — React Flow's grid is hidden. Nodes snap to it but the user sees a clean canvas.
2. **Surface nodes** — each node is a `.surface` Card with wall + shadow. Colors match node type.
3. **Belt connections** — thick SVG paths (20-28px stroke) with animated dash pattern for surface texture.
4. **Belt shadow** — SVG `drop-shadow` filter matching the warm `.surface` shadow language.
5. **Orthogonal routing** — belts turn at right angles with rounded corners (like game roads).
6. **Stations on top** — surface nodes render above belt layer (natural z-ordering hides junction seams).

---

## Reference: Mini Motorways

See `/Users/ryan/Documents/Mini Motorways Reference/` for game screenshots.

Key observations:
- **Munich screenshot:** Thick dark roads, rounded corners, golden highway overlay, buildings on top of roads. Junctions merge seamlessly because buildings cover the seam.
- **Tokyo Night:** Dark mode. Roads lighter against dark ground. Strong depth/shadow on buildings.
- **LA Harbor:** Clean two-station setup — simple road connecting two colored buildings. The simplest case.
- **Color palette:** Each road type = one solid color. No gradients on surface.
- **Layering:** Ground → roads → buildings. Our equivalent: canvas bg → belt edges → surface nodes.

**Adaptation:** Roads → conveyor belts. Cars → sushi. Buildings → stations. The `.surface` system already matches the game's building aesthetic.

---

## Technical Approach

### React Flow as the Foundation

React Flow (`@xyflow/react`) is the layout and interaction engine. It provides:
- Node positioning on an invisible grid
- SVG edge rendering (where we draw conveyor belts)
- Zoom, pan, and viewport controls
- Connection creation (drag from node to node)
- Selection and keyboard navigation

**Nodes** are custom React Flow node components that render `.surface` Cards.
**Edges** are custom React Flow edge components that render thick animated SVG paths.

The grid is invisible by default — React Flow's `<Background>` component is either hidden or shows a very subtle dot pattern (like graph paper viewed from far away).

### ELK.js for Layout

ELK.js computes node positions and edge routes automatically:
```js
{
  "elk.algorithm": "layered",
  "elk.direction": "RIGHT",           // horizontal pipeline flow
  "elk.edgeRouting": "ORTHOGONAL",    // right-angle turns
  "elk.spacing.nodeNode": "80",
  "elk.spacing.edgeEdge": "30",
  "elk.spacing.edgeNode": "40",
}
```

This means we define the graph (nodes + connections) and ELK computes WHERE everything goes. React Flow renders it.

### Belt Edge Rendering

Custom `ConveyorEdge` component using React Flow's `getSmoothStepPath()`:

```tsx
function ConveyorEdge({ sourceX, sourceY, targetX, targetY, ...props }) {
  const [path] = getSmoothStepPath({
    sourceX, sourceY, targetX, targetY,
    borderRadius: 14,  // rounded corners at turns
  });

  return (
    <>
      {/* Belt body — thick stroke */}
      <path
        d={path}
        stroke="var(--muted)"
        strokeWidth={24}
        strokeLinecap="round"
        fill="none"
        filter="url(#belt-shadow)"
      />
      {/* Belt surface — animated dashes for texture */}
      <path
        d={path}
        stroke="var(--border)"
        strokeWidth={24}
        strokeLinecap="round"
        strokeDasharray="4 16"
        fill="none"
        className="motion-safe:animate-convey"
      />
    </>
  );
}
```

Two layered paths:
1. **Base** — solid thick stroke (belt body, muted color)
2. **Surface** — dashed stroke on top (moving segments, border color)

The dash animation (`stroke-dashoffset`) creates the scrolling belt texture. This is the SVG equivalent of the CSS `linear-gradient` + `background-position` animation.

---

## Phases

### Phase 1: React Flow Canvas + Static Scene ✓ COMPLETE

**Goal:** A React Flow canvas on the Motorway page showing a pre-built "level" — 5 surface nodes connected by thick belt edges.

**Delivered:**
- `ConveyorShowcase.tsx` — blueprint declaring stations + belts
- `ConveyorCanvas.tsx` — pure React Flow renderer (props-in, scene-out)
- `StationNode.tsx` — custom node: `.surface` Card with invisible handles
- `ConveyorEdge.tsx` — custom edge: flat thick SVG path with animated dashes
- `conveyor.css` — belt keyframes, variant color tokens, speed variants
- 5-station image processing pipeline with fork/merge

### Phase 2: Flat Belts + Variant Colors + Seamless Junctions ✓ COMPLETE

**Delivered:**
- **Flat belts** — no shadow, no depth. Belts are flush to the ground surface, only buildings have elevation (matches Mini Motorways reference exactly)
- **Variant colors** — each belt tinted to match source station variant via `color-mix()` CSS tokens (`.belt-primary`, `.belt-secondary`, `.belt-accent`, `.belt-muted`, `.belt-success`)
- **Seamless junctions** — handles invisible (zero-size), belts slide under card edges. Buildings cover junction seams naturally
- **Animated dash surface** — `stroke-dashoffset` keyframe with `motion-safe:` guard
- **Speed variants** — `.belt-surface-fast` (0.5s) and `.belt-surface-paused` via CSS classes
- **Simplified SVG** — 2-layer approach (body + surface) down from original 5-layer

### Phase 3: Sushi Piece Animation ✓ COMPLETE

**Delivered:**
- `BeltPiece.tsx` — three sushi piece types rendered as SVG shapes
  - **Maki** — circle with center dot (roll cross-section)
  - **Nigiri** — rounded rectangle with inner accent
  - **Onigiri** — diamond with center dot
- Pieces travel along belt paths via SVG `<animateMotion>` (rotate="auto" keeps orientation)
- Multiple pieces staggered per belt (configurable count + speed)
- Piece colors inherit from belt variant via CSS custom properties
- `.belt-pieces` wrapper hidden in `prefers-reduced-motion: reduce`
- Preview row in ConveyorShowcase showing all 3 types across all 5 variant colors
- Configurable per-belt: `pieces`, `pieceSpeed`, `pieceType` in ConveyorEdgeData

### Phase 4: Interactive Editing

- Drag nodes to reposition
- Drag from node port to create new connection
- Delete connections
- Add/remove nodes
- ELK.js auto-layout button (recompute positions)
- Minimap for navigation on complex recipes

### Phase 5: Recipe Data Integration

- Parse `.bnto.json` into React Flow nodes + edges
- Map node types to station variants (image → primary, csv → secondary, etc.)
- Show real recipe pipeline structure
- Bi-directional: editing the graph updates the recipe JSON

### Phase 6: Execution Visualization

- Tie belt animation to real recipe execution state
- Sushi piece enters at input node when user drops a file
- Piece travels through pipeline, pauses at each station during processing
- Station glows/pulses while processing
- Completed: piece arrives at output with success indicator
- Failed: piece turns red, error treatment on the station

---

## Research Notes

### React Flow Custom Nodes

React Flow nodes are React components. They receive `data` props and render whatever you want. Our `StationNode` renders a `.surface` Card:

```tsx
function StationNode({ data }: NodeProps<StationData>) {
  return (
    <Card elevation="sm" className={`surface-${data.variant}`}>
      <Handle type="target" position={Position.Left} />
      <Text>{data.label}</Text>
      <Handle type="source" position={Position.Right} />
    </Card>
  );
}
```

`Handle` components are React Flow's connection points — where belts attach to stations. They can be styled to look like belt rollers or hidden entirely (belt connects to the card edge).

### React Flow Custom Edges

Full SVG control. The `BaseEdge` convenience component is optional — we render raw `<path>` elements for maximum control. `getSmoothStepPath()` generates orthogonal paths with `borderRadius` for rounded corners.

Edge props include `sourceX`, `sourceY`, `targetX`, `targetY`, `sourcePosition`, `targetPosition`. These are all we need to compute the belt path.

### SVG Dash Animation for Belt Texture

```css
@keyframes convey-svg {
  to {
    stroke-dashoffset: -20px; /* negative = forward direction */
  }
}

.belt-surface {
  stroke-dasharray: 4 16;
  animation: convey-svg 1.2s infinite linear;
}
```

`stroke-dashoffset` animation is compositor-friendly in most browsers. The dash pattern (4px dash, 16px gap in a 20px cycle) creates evenly spaced belt segments.

### Junction Blending

Two natural solutions (both apply):
1. **Node overlay** — station cards sit on top of belts (React Flow renders HTML nodes above SVG edges). The card body + surface shadow covers where belts meet the station.
2. **Junction circles** — filled SVG circles at Handle positions, radius = belt width / 2, same color as belt. Covers any remaining seam.

### Bundle Size

| Library | Size | Load strategy |
|---|---|---|
| `@xyflow/react` | ~45KB gz | `next/dynamic` with `ssr: false` |
| `elkjs` (web worker) | ~100KB gz | Dynamic import inside canvas component |

Both lazy-loaded — zero cost to tool pages. Only the Motorway dev page and future recipe editor load them.

### Library Comparison (why React Flow)

| Library | Stars | Custom Edges | Orthogonal Routing | Community |
|---|---|---|---|---|
| **React Flow** | ~35K | Full SVG control | `getSmoothStepPath` + ELK.js | Huge, active |
| Rete.js | ~12K | Manual SVG | Manual | Small |
| react-diagrams | ~8K | Link segments | Custom | Minimal |

React Flow is the clear winner for edge customization depth, community size, and maintenance. MIT licensed.

---

## CSS Reference

### Belt animation (globals.css)

The CSS belt classes (`.conveyor-belt`, `@keyframes convey`) are defined in `globals.css` for both the CSS demo and as reference. The React Flow implementation uses SVG equivalents:

- CSS `linear-gradient` → SVG `stroke-dasharray`
- CSS `background-position` animation → SVG `stroke-dashoffset` animation
- CSS `border-top/bottom` → SVG layered paths (base + surface)
- CSS `box-shadow` → SVG `<feDropShadow>` filter

### Belt color tokens

| Token | Use |
|---|---|
| `var(--muted)` | Belt body (base track color) |
| `var(--border)` | Belt ridges (dash pattern color) |
| `var(--surface-muted-wall)` | Belt rail edges |
| `var(--surface-muted-shadow)` | Belt cast shadow |

---

## Open Questions

- [x] ~~Should the React Flow background show a very subtle dot grid?~~ Yes — subtle dots, gap=20, size=1, color=var(--border).
- [x] ~~Handle styling — visible belt rollers, or invisible?~~ Invisible. Belts slide under card edges seamlessly.
- [x] ~~Should belts change color based on the source node variant?~~ Yes — source variant. Each belt inherits color from its source station.
- [x] ~~What recipe should the "level" depict?~~ Image processing pipeline: Drop Files → Compress → Resize/Convert (fork) → Download (merge).
- [x] ~~Should the canvas have zoom/pan controls?~~ No — fixed viewport for showcase. fitView with padding=0.2.
- [ ] Belt borders — optional per theme. Some themes (e.g. dark Tokyo) may need a subtle border to distinguish belts from the background. Light themes don't need it. Could add a `.belt-bordered` modifier class.
- [x] ~~Sushi piece design — what does a data item look like on the belt?~~ Three types: maki (circle+dot), nigiri (rounded rect+inner rect), onigiri (diamond+dot). Simple CSS-styled SVG shapes with body + detail layers.

---

## Progress Log

| Date | Phase | What happened |
|---|---|---|
| 2026-02-26 | Research | Initial research complete. Compared React Flow, Rete.js, SVG techniques, Canvas approaches. |
| 2026-02-26 | Research | Pivoted from CSS-only Phase 1 to React Flow foundation. Updated all phases. |
| 2026-02-26 | Phase 1 | React Flow canvas + surface nodes + thick belt edges on Motorway page. Composability refactor: scene data in Showcase, canvas is pure renderer |
| 2026-02-26 | Phase 2 | Flat belts (no shadow), variant color tokens via color-mix(), invisible handles for seamless junctions, speed variants, stronger dash visibility. Simplified from 5-layer to 2-layer SVG. Wider station spacing, larger cards. |
| 2026-02-26 | Phase 2 | Added optional bordered prop for dark themes, Replay button for entrance animations, staggered entrance sequence (stations bounce in → belts fade in after). |
| 2026-02-26 | Phase 3 | Sushi pieces: BeltPiece.tsx with 3 types (maki, nigiri, onigiri), SVG animateMotion along belt paths, variant color inheritance, reduced-motion guard. Preview row in showcase. |
