# Bnto — Design Language

**Created:** February 21, 2026
**Updated:** February 23, 2026 (Mini Motorways brand vision, animation primitives, font update)
**Status:** Active

---

## Core Thesis

**Warm, organized, satisfying. Like a well-packed bento box on a Mini Motorways map.**

Bnto's visual identity draws from the intersection of the bento box metaphor (organized compartments, purposeful arrangement) and the clean, warm, geometric aesthetic of Mini Motorways. The result feels like a tool that was made by someone who cares — warm without being cute, simple without being simplistic, satisfying without being flashy.

The aesthetic is a spatial feeling, not a color palette.

---

## The Bento Box IS the Language

The bento box isn't just a product metaphor — it's the visual system:

- **Organized compartments** → UI sections with clear boundaries and generous spacing
- **Each piece intentional** → No decoration that doesn't serve comprehension
- **Beautiful through structure** → Layout and spacing do the work, not color or effects
- **Satisfying to open** → Animations that feel like revealing something well-prepared

---

## References

| Reference | What we take from it |
|-----------|---------------------|
| **Mini Motorways** (Dinosaur Polo Club) | Warm cream backgrounds, bold-but-muted accent colors, clean geometry, rounded shapes. The feeling of watching a city grow — buildings popping up, roads drawing themselves, cars finding their way. Calm, purposeful, satisfying |
| **Mini Motorways map palette** | Each map has 4-5 distinct colors on a warm base. Colors are bold enough to be functional (distinguish districts) but muted enough to feel calm. Three color temperatures coexist: warm (buildings), cool (water/parks), neutral (roads) |
| **Blake Wood's art direction** | Early color tests show deliberate palette constraint — each map is a mood, not a rainbow. The palette serves the game, not the other way around |
| **Japanese bento boxes** | Precision, economy, spatial harmony. Visual beauty as a byproduct of thoughtful arrangement |
| **shadcn/ui** | Production-grade component library. Clean, accessible, composable. The substrate our personality layers onto |

---

## Brand Personality

**Wordmark:** `bnto` — all lowercase, always. Four small rounded letters. No capitalization, no camel case, not even at the start of a sentence in brand contexts. The roundedness and lowercase together signal warmth and approachability before the user reads a single word of copy.

**Voice:** Plain language. "It just works" as north star phrase. No jargon unless the user asked for it. Talking to a designer who needs to compress 40 images, not an engineer who wants to build a pipeline.

| Trait | We are | We're not |
|-------|--------|-----------|
| **Temperature** | Warm | Cold, clinical, corporate |
| **Tone** | Inviting | Impressive, intimidating |
| **Complexity** | Simple | Simplistic, dumbed-down |
| **Feeling** | Satisfying | Flashy, attention-seeking |
| **Motion** | Purposeful | Decorative, bouncy for fun |

---

## Color Philosophy

Three color temperatures coexist on a warm base — like a Mini Motorways map where buildings, water, and roads are all distinct but harmonious.

### The Palette

| Role | Token | Light Mode | Dark Mode | Character |
|------|-------|------------|-----------|-----------|
| **Background** | `--background` | Warm cream | Cool slate | The map surface |
| **Foreground** | `--foreground` | Warm dark brown | Warm off-white | Readable, never pure black/white |
| **Primary** | `--primary` | Terracotta (hue 35) | Same | Buildings — the main interactive color. CTAs, active states |
| **Secondary** | `--secondary` | Soft teal (hue 189) | Dark teal | Water/parks — the cool counterpoint. Secondary actions |
| **Accent** | `--accent` | Golden yellow (hue 91) | Richer golden | Landmarks — highlights, badges, attention |
| **Muted** | `--muted` | Warm off-white | Dark slate | Roads — background surfaces, subtle separation |
| **Destructive** | `--destructive` | Warm red | Slightly lighter red | Errors, delete actions |

### Color Rules

1. **Three temperatures, always.** Warm (terracotta), cool (teal), golden (accent). Removing any one makes the palette feel flat
2. **Same hues across modes.** Primary is terracotta in both light and dark. Secondary is teal in both. The lightness shifts, the identity stays
3. **Warm shadows.** Light mode shadows use `hsl(8 19% 15%)` — a warm dark brown. Never cold gray or pure black shadows in light mode
4. **Cool slate dark mode.** Dark backgrounds carry a subtle blue tint (hue 269), not pure neutral gray. Warm foreground colors on cool backgrounds create depth

### Where Color Appears

| Context | Color | Why |
|---------|-------|-----|
| Primary buttons, CTAs | Terracotta | Main interactive signal |
| Secondary buttons, badges | Soft teal | De-emphasized but distinct actions |
| Highlighted items, active states | Golden | Draws the eye without competing with primary |
| Error states, delete | Warm red | Urgency signal |

### Where Color Does NOT Appear

| Context | Why not |
|---------|---------|
| Body text | Always foreground color. No colored body text |
| Backgrounds | Always `--background` or `--card`. Never a colored fill |
| Navigation chrome | Structural, not decorative. Uses foreground/muted |
| Icons (general) | Foreground or muted-foreground. Color only for status icons |

---

## Typography

Three-font system: **DM Sans** (display) + **Inter** (body) + **Geist Mono** (code).

| Role | Font | Treatment | Why |
|------|------|-----------|-----|
| **Display / Headings** | DM Sans (semibold/bold) | Large, tight leading, `font-display` | Geometric with soft rounding — closest Google Fonts match to Mini Motorways' Helvetica. Warm character pairs with terracotta |
| **Body** | Inter (regular) | Standard readable size, generous line-height, `font-sans` | The universal UI font. Clean, highly legible at all sizes. Pairs perfectly with DM Sans |
| **Code / Technical** | Geist Mono | Monospaced, small, uppercase tracking for labels, `font-mono` | Logs, `.bnto.json` previews, execution output, node type labels |
| **Badges / Tags** | DM Sans or Inter | Uppercase, `tracking-widest`, small | Industrial stamp feel for metadata |

**Font rationale:** DM Sans was chosen over Geist Sans for its rounder, warmer letterforms that complement the terracotta palette and generous border radii. Helvetica (Mini Motorways' actual font) can't be used on the web, but DM Sans captures the same geometric-clean-warm feeling. Inter is the workhorse — functional, invisible, never competes with the display font. Geist Mono for technical output because it's designed for code readability.

All fonts loaded via `next/font/google` with zero layout shift.

---

## Radius & Shape

| Token | Value | Use |
|-------|-------|-----|
| `rounded-sm` | `calc(1.25rem - 4px)` ≈ 1rem | Small inline elements — badges, tags |
| `rounded-md` | `calc(1.25rem - 2px)` ≈ 1.125rem | Inputs, small buttons |
| `rounded-lg` | `1.25rem` (20px) | **Default** — cards, panels, containers |
| `rounded-xl` | `calc(1.25rem + 4px)` ≈ 1.5rem | Feature cards, hero elements |
| `rounded-full` | `9999px` | Pill buttons, avatar circles |

**Philosophy:** Round more, not less. The brand is warm and friendly, not sharp. Default to `rounded-lg`. Go rounder for prominent surfaces, tighter only for small inline elements. Never use sharp corners in brand UI.

---

## Shadows

Light mode shadows are warm-tinted (`hsl(8 19% 15%)` — the warm dark brown from the foreground family). They never feel cold or clinical.

Dark mode shadows use pure black at higher opacity — the warm tinting isn't visible against dark surfaces.

The shadow scale provides consistent elevation:

| Level | Use case |
|-------|----------|
| `shadow-sm` | Subtle lift — inputs, small cards |
| `shadow` | Standard card elevation |
| `shadow-md` | Modals, dropdowns |
| `shadow-lg` | Popovers, overlays |
| `shadow-xl` | Featured elements, hero cards |

---

## Animation Language

Bnto's animations are inspired by Mini Motorways — **buildings popping up, roads drawing themselves, neighborhoods growing.** Every animation serves comprehension (show what appeared, what moved, what changed), never decoration.

### Motion Library

[Motion](https://motion.dev) v12 (the Framer Motion successor) — already installed. Spring physics, AnimatePresence, layout animations, GPU-accelerated.

### Shared Constants

All animation primitives share a single set of spring/duration/easing constants. Never inline raw values.

| Constant | Value | Character |
|----------|-------|-----------|
| `springs.snappy` | `stiffness: 400, damping: 25` | Buildings popping up — quick with slight overshoot |
| `springs.smooth` | `stiffness: 300, damping: 30` | Roads drawing in — purposeful, less bounce |
| `springs.gentle` | `stiffness: 200, damping: 20` | Soft reveals — slow settle |
| `durations.fast` | `0.15s` | Quick feedback (hover, press) |
| `durations.normal` | `0.3s` | Default transitions |
| `durations.slow` | `0.5s` | Emphasis animations |
| `easings.out` | `cubic-bezier(0.33, 1, 0.68, 1)` | Natural deceleration |
| `easings.inOut` | `cubic-bezier(0.65, 0, 0.35, 1)` | Symmetric transitions |

### Animation Primitives

Composable wrapper components in `apps/web/components/animations/`. Each wraps children with motion behavior. Import from `@/components/animations`.

#### `<PopIn>` — Buildings appearing on the map

Scale 0 → 1.05 → 1 with spring overshoot + opacity fade. The signature bnto entrance animation.

**When to use:**
- Cards, tiles, or grid items appearing on page load or after data fetch
- Modal/dialog content appearing (scale from center)
- Toast/notification appearing
- Any element that "arrives" — it wasn't there, now it is
- Hero page bento grid items on initial load
- Node palette items when palette opens
- Search results appearing after a query

**When NOT to use:**
- Content that slides in from a screen edge (use SlideIn)
- Elements that are always visible and just need attention (use Breathe)
- Reordering existing elements (use LayoutShift)

```tsx
<PopIn>
  <Card>New content</Card>
</PopIn>

<PopIn delay={0.2}>
  <Badge>Delayed entrance</Badge>
</PopIn>
```

#### `<SlideIn>` — Roads drawing into place

Translate from an offset position and decelerate to rest. Accepts `from` direction prop.

**When to use:**
- Side panels opening (from="right")
- Bottom sheets or action bars appearing (from="bottom")
- Toast notifications entering from the top (from="top")
- Dropdown content appearing below a trigger (from="top")
- Page section content entering on scroll (from="bottom")
- Navigation drawers or sidebars (from="left")
- Toolbar or floating action bar appearing

**When NOT to use:**
- Elements appearing "in place" without directional context (use PopIn)
- Lists of items — each item would slide the same direction, which looks odd (use StaggerCascade + PopIn)

```tsx
<SlideIn from="bottom">
  <ActionBar />
</SlideIn>

<SlideIn from="right" offset={40}>
  <SidePanel />
</SlideIn>
```

#### `<StaggerCascade>` — Houses appearing in a neighborhood

Parent orchestrates children appearing one after another with a configurable delay. Wrap each child in PopIn (or any entrance animation) for the individual element effect.

**When to use:**
- Grid of cards on page load (workflow list, bnto tool grid, dashboard)
- List items appearing after data fetch (execution history, log entries)
- Hero bento grid — the primary use case, each tool card pops in sequentially
- Tag/badge groups appearing
- Navigation menu items
- Any collection of similar items that should feel like they're "growing" into place

**When NOT to use:**
- Single elements (no stagger needed — just use PopIn directly)
- Elements that should appear simultaneously (just use PopIn on each)

```tsx
<StaggerCascade stagger={0.08}>
  {workflows.map(w => (
    <PopIn key={w.id}>
      <WorkflowCard workflow={w} />
    </PopIn>
  ))}
</StaggerCascade>
```

#### `<Breathe>` — Destinations needing connection

Gentle scale oscillation (1 → 1.04 → 1), infinite loop. Draws attention without aggression.

**When to use:**
- Status indicator showing "running" or "in progress" state
- Element that needs user attention ("connect this", "action needed")
- Live/active indicator dot
- Upload zone waiting for files
- "Processing..." state on a card or element

**When NOT to use:**
- Static content that doesn't need attention
- Multiple elements simultaneously (more than 2-3 breathing elements is distracting)
- Elements the user is actively interacting with (hover/click states use Press instead)

```tsx
<Breathe>
  <StatusDot status="running" />
</Breathe>

<Breathe scale={1.02} duration={3}>
  <DropZone>Drop files here</DropZone>
</Breathe>
```

#### `<PathDraw>` — Roads being constructed

SVG path stroke animation — the line draws itself from start to end. Must be used inside an `<svg>` element.

**When to use:**
- Progress indicators or loading bars
- Connection lines between nodes in a workflow editor
- Decorative illustrations that "draw themselves" on page load
- Step-by-step flow visualization (line connecting steps)
- Divider lines or borders that appear with a drawing effect

**When NOT to use:**
- Non-SVG content (this is SVG-only)
- Simple opacity transitions (overkill — use CSS)

```tsx
<svg viewBox="0 0 200 100" className="w-full h-auto">
  <PathDraw
    d="M 10 50 Q 100 10 190 50"
    stroke="currentColor"
    strokeWidth={2}
  />
</svg>
```

#### `<LayoutShift>` — Rearranging the map

Wrapper that adds `layout={true}` for automatic position and size animations when the element's layout changes.

**When to use:**
- Reorderable lists (drag-and-drop, sort changes)
- Expanding/collapsing panels where siblings need to reflow
- Accordion items — when one opens, others shift down smoothly
- Filter changes that reorder or remove grid items
- Tab content where the container height changes
- Any element whose position changes due to sibling changes

**When NOT to use:**
- Elements entering or exiting the DOM (use PopIn/ExitDissolve + PresenceGate)
- Elements that don't change position (no animation needed)

```tsx
{items.map(item => (
  <LayoutShift key={item.id}>
    <Card>{item.name}</Card>
  </LayoutShift>
))}
```

#### `<ExitDissolve>` — Removing a road

Gentle opacity + scale-down exit (scale 1 → 0.98, opacity 1 → 0). Less dramatic than PopIn's exit.

**When to use:**
- Dismissing a toast or notification (gentle fade-out)
- Removing a card from a list (item deleted, filtered out)
- Closing a non-modal overlay
- Content being replaced by new content (old content dissolves, new PopIn's)
- Removing tags or badges

**When NOT to use:**
- Modal/dialog closing with emphasis (use PopIn — its exit is more dramatic with scale-down)
- Panels sliding off-screen (use SlideIn — its exit reverses the slide)

```tsx
<PresenceGate>
  {isVisible && (
    <ExitDissolve key="notification">
      <Notification />
    </ExitDissolve>
  )}
</PresenceGate>
```

#### `<NumberRoll>` — Score incrementing

Animated number transitions — the value smoothly rolls from its previous value to the new one using spring physics.

**When to use:**
- Execution timing stats (duration counting up)
- Dashboard counters (total runs, files processed)
- File size displays that change (compression ratio)
- Progress percentages
- Any numeric value that updates and the change should feel meaningful

**When NOT to use:**
- Static numbers that don't change
- Numbers in dense data tables (too much motion in a table is noisy)
- Timestamps or dates

```tsx
<NumberRoll value={executionCount} />

<NumberRoll
  value={compressionRatio}
  decimals={1}
  format={(v) => `${v}%`}
/>
```

#### `<PresenceGate>` — AnimatePresence wrapper

Wraps conditionally rendered content to enable exit animations. Without this, elements that unmount disappear instantly.

**When to use:**
- Always wrap content that conditionally renders AND has exit animations
- Modal/dialog conditional rendering
- Toast/notification conditional rendering
- Tab content swapping
- Any `{condition && <AnimatedElement />}` pattern

**When NOT to use:**
- Elements that are always in the DOM (no conditional rendering)
- Elements that don't need exit animations (just hide with CSS)

```tsx
<PresenceGate>
  {showModal && (
    <PopIn key="modal">
      <Modal onClose={() => setShowModal(false)} />
    </PopIn>
  )}
</PresenceGate>

<PresenceGate mode="wait">
  <PopIn key={activeTab}>
    <TabContent tab={activeTab} />
  </PopIn>
</PresenceGate>
```

#### `<Press>` — Pressing a building into the map

On hover, shadow collapses to zero and element translates down. On active/click, fully flush with surface. Creates a tactile "pressed in" feeling.

**When to use:**
- Cards in a grid that are clickable (bnto tool cards, workflow cards)
- Buttons or CTAs that should feel tactile
- Interactive tiles or thumbnails
- Any elevated element (has shadow) that the user clicks/taps

**When NOT to use:**
- Elements without shadows (the shadow collapse is the whole effect)
- Text links or inline interactive elements
- Non-interactive elements (decorative cards that don't do anything)
- Elements inside a scrollable list where accidental hover would be distracting

```tsx
<Press>
  <Card className="shadow-md cursor-pointer">
    <h3>Compress Images</h3>
  </Card>
</Press>

{/* Combine with PopIn for entrance + interaction */}
<PopIn>
  <Press>
    <BntoCard bnto={bnto} />
  </Press>
</PopIn>
```

### Interaction Patterns

| Interaction | Effect | Mini Motorways equivalent |
|-------------|--------|--------------------------|
| **Hover on card** | Shadow reduces, subtle translateY push-down ("press into surface") | Selecting a building — it settles into the map |
| **Click/active** | Shadow fully removed, element flush with surface | Placing a road — committed to the position |
| **Hover reveal** | Child elements fade in (opacity 0 → 1) via `group-hover` + `group-focus-within` | Tooltip/info appearing on hover |
| **List item enter** | StaggerCascade with PopIn children | New buildings appearing across the city |
| **Panel expand** | SlideIn from bottom/side with LayoutShift on siblings | Map area expanding |
| **Status pulse** | Breathe on status indicator | Building needing attention |
| **Content swap** | PresenceGate with ExitDissolve → PopIn | Switching between map views |

### The Hero Page Pattern

Two-column hero: messaging on the left, bento grid on the right. The grid contains cards for each predefined bnto tool. On page load, cards appear via **StaggerCascade + PopIn** — one by one, like buildings popping up on a fresh Mini Motorways map. Each card has the **Press** interaction on hover (shadow flattens, card settles into the grid surface).

```tsx
// Conceptual structure
<Hero>
  <Hero.Messaging>
    <PopIn>
      <h1>Pick a tool. Drop your files.</h1>
      <p>Compress images, clean CSVs, rename files — done.</p>
    </PopIn>
  </Hero.Messaging>
  <Hero.Grid>
    <StaggerCascade>
      {bntos.map(bnto => (
        <PopIn key={bnto.slug}>
          <Press>
            <BntoCard bnto={bnto} />
          </Press>
        </PopIn>
      ))}
    </StaggerCascade>
  </Hero.Grid>
</Hero>
```

---

## Performance Contract

All animations follow these rules:

1. **GPU-composited properties only** — `transform`, `opacity`, `filter`. Never animate `width`, `height`, `top`, `left`, or any layout property
2. **`prefers-reduced-motion` respected** — Every animation component renders a static fallback when reduced motion is preferred. No exceptions
3. **Spring physics over duration-based** — Springs feel natural and respond to interruption. Duration-based animations feel mechanical
4. **Element budget** — Max 8-10 actively animating elements visible simultaneously. StaggerCascade naturally limits this by sequencing
5. **No new dependencies** — Everything runs on `motion` (already installed) or pure CSS
6. **CSS-first for simple states** — Hover opacity, focus rings, color transitions use CSS (`transition-*`). Motion library only for springs, AnimatePresence, layout, and stagger

---

## Composable Implementation

The design language is applied through **wrapper components**, not baked into base primitives. A `<Card>` is still a card. Wrapping it in `<PopIn>` makes it animate. Wrapping it in `<Press>` makes it interactive.

### Layer 1: Animation Primitives

Composable motion wrappers in `apps/web/components/animations/`.

| Component | File | Purpose |
|-----------|------|---------|
| `<PopIn>` | `PopIn.tsx` | Spring entrance animation |
| `<SlideIn>` | `SlideIn.tsx` | Directional slide entrance |
| `<StaggerCascade>` | `StaggerCascade.tsx` | Orchestrated children entrance |
| `<Breathe>` | `Breathe.tsx` | Infinite gentle attention pulse |
| `<PathDraw>` | `PathDraw.tsx` | SVG stroke animation |
| `<LayoutShift>` | `LayoutShift.tsx` | Automatic position/size animation |
| `<ExitDissolve>` | `ExitDissolve.tsx` | Gentle exit animation |
| `<NumberRoll>` | `NumberRoll.tsx` | Animated number transitions |
| `<PresenceGate>` | `PresenceGate.tsx` | AnimatePresence wrapper |
| `<Press>` | `Press.tsx` | Hover/active push-down interaction |
| Shared config | `config.ts` | Springs, durations, easings constants |

**Usage pattern:**
```tsx
<StaggerCascade>
  {items.map(item => (
    <PopIn key={item.id}>
      <Press>
        <Card>{item.name}</Card>
      </Press>
    </PopIn>
  ))}
</StaggerCascade>
```

### Layer 2: Structural Wrappers

Layout and framing components in `apps/web/components/`.

| Component | What it does |
|-----------|--------------|
| `<GridUnderlay>` | Faint dot-grid background via CSS |
| `<SectionFrame>` | Section wrapper with optional label |

### Layer 3: Status & Feedback

| Component | What it does |
|-----------|--------------|
| `<StatusIndicator>` | Execution status with Breathe animation when running |

---

## Application by Context

| Context | Animation treatments |
|---------|---------------------|
| **Landing page hero** | StaggerCascade + PopIn on bento grid cards, Press on hover |
| **Landing page sections** | PopIn on section content as it scrolls into view |
| **Workflow list** | StaggerCascade on workflow cards |
| **Execution view** | NumberRoll on timing stats, Breathe on running indicator, PopIn on log entries |
| **Node palette** | StaggerCascade + PopIn when palette opens |
| **Modal/dialog enter** | PopIn (scale from center) |
| **Panel expand** | SlideIn + LayoutShift on adjacent content |
| **Toast notifications** | SlideIn from top, ExitDissolve on dismiss |
| **Settings** | Minimal — no animation beyond standard transitions |

---

## Principles

1. **Warm, organized, satisfying** — Every design choice should feel like opening a well-packed bento box
2. **Three color temperatures** — Warm (terracotta), cool (teal), golden (accent). Removing any one breaks the harmony
3. **Animations serve comprehension** — Show what appeared, what moved, what changed. Never animate for decoration
4. **Composable, not pervasive** — Effects are wrappers. Base components stay clean
5. **Restraint over spectacle** — If you notice the animation consciously, it's too strong
6. **Performance is non-negotiable** — A dropped frame breaks the feeling more than a missing animation
7. **The work is the hero** — The workflow, the files, the results are the content. The platform serves comprehension, never competes
8. **Round more, not less** — The brand is warm and friendly. Default to generous radii
9. **CSS-first** — Push simple states to CSS. Motion library for springs, presence, layout, stagger only
10. **Progressive enhancement** — Fully functional without animations. Respect `prefers-reduced-motion`
