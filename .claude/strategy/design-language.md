# Bnto - Design Language

**Created:** February 21, 2026
**Status:** Active

---

## Visual Identity Direction

Bnto's visual identity should emerge from the product's core metaphor: **organized compartments, clean structure, purposeful arrangement.** The bento box is the reference point -- precision, economy, and beauty through structure rather than decoration.

**Visual reference: Mini Motorways (first level).** Warm cream base, soft muted palette, rounded shapes. Not sterile. Not corporate. Not cold. The aesthetic goal is: warm, inviting, simple, satisfying.

**Brand personality:**
- Warm, not cold
- Inviting, not impressive
- Simple, not simplistic
- Satisfying, not flashy

**Wordmark:** `bnto` — all lowercase, always. Four small rounded letters. No capitalization, no camel case, not even at the start of a sentence in brand contexts. The roundedness and lowercase together signal warmth and approachability before the user reads a single word of copy. This is the brand.

**Brand voice:** Plain language. "It just works" as north star phrase. No jargon unless the user asked for it. We're talking to a designer who needs to compress 40 images, not an engineer who wants to build a pipeline.

**UI stack:**
- shadcn/ui for components (production-ready, accessible, composable)
- Magic UI for animations (purposeful motion, not decorative)
- Quicksand (Google Fonts via `next/font`) — rounded, warm, friendly at all sizes
- Theme: **fully defined and implemented** — see `rules/theming.md` for all tokens

**Palette at a glance:**
- Background: warm cream (`--background`)
- Primary: terracotta (`--primary`) — CTAs, active states
- Accent: golden yellow (`--accent`) — highlights
- Secondary: soft teal (`--secondary`)
- Radius: `1.25rem` base — generously rounded everywhere
- Shadows: warm-tinted (never cold gray)

**What we know:**
- Warm and inviting -- this is a tool for real people, not an enterprise dashboard
- The interface should feel like a well-organized workspace that's pleasant to spend time in
- Visual noise competes with task completion -- restraint is paramount
- The design should convey that it just works, not that it's impressive

---

## Typography

| Role | Font | Treatment |
|------|------|-----------|
| **Headlines** | Geist Sans (bold/black) | Large, tight leading |
| **Body** | Geist Sans (regular) | Standard readable size, generous line-height |
| **Technical labels** | Geist Mono | Uppercase, wide letter-spacing (`tracking-widest`), small size (10px) |
| **Data / metadata** | Geist Mono | Monospaced for alignment in technical contexts |
| **Badges / tags** | Geist Sans | Uppercase, `tracking-widest`, `text-[10px]`, `font-semibold` |
| **Accent headlines** | Geist Sans (italic or light) | For subheadings that need contrast against bold headlines |

**Font rationale:** Geist (Vercel's typeface) provides a geometric, technical character that matches the precision expected of a workflow tool. Geist Mono complements it for code-adjacent contexts (workflow definitions, execution logs, node types). Both loaded via `next/font/google` at zero cost.

---

## Color

Color direction will be established as the product develops. The following principles apply regardless of palette:

**Surface hierarchy:** A clear 3-tier depth system (Background, Card, Popover) using the same hue family at varying lightness levels. Surfaces should feel cohesive, not like random grays.

**Border radius:** `0.5rem` (8px) base -- geometric and precise.

**Restraint:** Color is used purposefully, not decoratively. Status colors (success, error, warning, running) are the primary use of chromatic color in a workflow tool. Accent colors should be reserved for interactive elements and primary CTAs.

---

## Composable Implementation

The design language is applied through **wrapper components and utility primitives**, not baked into every component. This keeps the UI package clean and lets individual pages/sections opt in to specific treatments.

### Layer 1: Background & Layout

Applied once at the page level. Always present.

| Component | Location | What it does |
|-----------|----------|--------------|
| `<GridUnderlay />` | `packages/@bnto/ui` | Faint dot-grid or line-grid background via CSS |

### Layer 2: Structural Wrappers

Composable components that wrap content to apply structured framing.

| Component | Location | What it does |
|-----------|----------|--------------|
| `<BoxFrame />` | `packages/@bnto/ui` | Technical border with optional corner marks and label slot |
| `<TechLabel />` | `packages/@bnto/ui` | Monospaced uppercase micro-label |
| `<SectionFrame />` | `packages/@bnto/ui` | Combines BoxFrame + TechLabel for full section treatment |

**Usage pattern:**
```tsx
<SectionFrame label="WORKFLOWS" id="workflows">
  <h2>Your automation library</h2>
  <p>...</p>
</SectionFrame>
```

### Layer 3: Content Effects

Applied to specific elements for emphasis. Opt-in, not default.

| Component | Location | What it does |
|-----------|----------|--------------|
| `<StatusIndicator />` | `packages/@bnto/ui` | Execution status with appropriate color treatment |

**Usage pattern:**
```tsx
<StatusIndicator status={execution.status}>
  Running node 3 of 7
</StatusIndicator>
```

### Layer 4: Interactive Effects

Interaction feedback and emphasis effects. Opt-in, used sparingly on featured content.

| Component | Location | What it does |
|-----------|----------|--------------|
| `<ShineBorder />` | `packages/@bnto/ui` | Rotating radial gradient border shimmer for featured cards |

### Utility Classes

For lightweight application without wrapper components:

| Class | Effect |
|-------|--------|
| `.tech-label` | Monospaced, uppercase, tracked, muted |
| `.grid-underlay` | Adds faint dot-grid background |
| `.box-frame` | Adds technical border treatment |

---

## Application by Context

How different areas of the product use the design language:

| Context | Treatments |
|---------|------------|
| **Workflow Editor** | GridUnderlay (canvas), BoxFrame (node containers), TechLabel (node types) |
| **Execution View** | StatusIndicator, TechLabel (timing/metadata), monospaced log output |
| **Dashboard** | SectionFrame (workflow list, recent executions), BoxFrame (summary cards) |
| **Landing Page** | SectionFrame (feature sections), ShineBorder (CTA), GridUnderlay (subtle) |
| **Settings** | Minimal treatment -- clean forms, clear labels, no decorative effects |

---

## Build Order

Implement in layers so each layer is independently shippable:

1. **Layer 1** -- GridUnderlay
2. **Layer 2** -- TechLabel, BoxFrame, SectionFrame
3. **Apply Layer 2** -- Wrap existing pages with new components
4. **Layer 3** -- StatusIndicator, contextual treatments
5. **Layer 4** -- Interactive effects for polish

Each layer should pass through the full verification cycle (lint, typecheck, build) before moving to the next.

---

## Principles

1. **Composable, not pervasive** -- Effects are applied by wrapping, never by modifying base primitives. A `<Button>` is still a button. A `<BoxFrame>` around it adds structure.
2. **Restraint over spectacle** -- Every effect at low opacity/intensity. The interface should feel calm and organized. If you notice the decoration consciously, it's too strong.
3. **Performance-first** -- CSS effects use `will-change` and GPU-composited properties. No layout thrash. Canvas effects use `requestAnimationFrame`.
4. **Progressive enhancement** -- The application is fully readable and functional without any effects. Effects layer on top for capable browsers. Respect `prefers-reduced-motion`.
5. **Structure communicates** -- In a workflow tool, visual hierarchy and spatial organization do more work than color or decoration. The layout itself should explain what's happening.
6. **The work is the hero** -- The workflow canvas, execution results, and node configurations are the content. The platform design serves comprehension, never competes with it.
