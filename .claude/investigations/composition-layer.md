# Investigation: Composition Layer (Sprint M)

**Date:** February 28, 2026
**Status:** Complete — findings inform Sprint M plan

---

## Problem Statement

The bnto codebase has a powerful CSS surface system and well-designed layout components, but className patterns leak into consumer code. Pages and blocks compose via raw Tailwind strings instead of components. This investigation catalogs the patterns, quantifies the scale, and identifies the components needed to fix it.

---

## Part 1: Existing Component Inventory

### Layout Primitives (exist, need wider adoption)

| Component | File | Props | Adoption |
|-----------|------|-------|----------|
| **Stack** | `components/ui/Stack.tsx` | `gap`, `align`, `justify`, `as` | ~50% — 13 files still use raw `flex flex-col gap-*` |
| **Row** | `components/ui/Row.tsx` | `gap`, `align`, `justify`, `wrap`, `as` | ~40% — 18 files still use raw `flex items-center gap-*` |
| **Grid** | `components/ui/Grid.tsx` | `cols`, `rows`, `gap`, `flow`, `align`, `justify`, `animated`, `as` + `Grid.Item` | ~60% — 9 files use raw `grid` |
| **Container** | `components/ui/Container.tsx` | `size`, `as` | Good adoption |
| **BentoGrid** | `components/ui/BentoGrid.tsx` | `cols`, `gap`, `minRowHeight`, `uniform` | Good adoption |
| **Divider** | `components/ui/Divider.tsx` | `label` | Niche, adequate |
| **AppShell** | `components/ui/AppShell.tsx` | Header, Main, Content sub-components | Good adoption |

### Typography Primitives (exist, good adoption)

| Component | File | Props |
|-----------|------|-------|
| **Heading** | `components/ui/Heading.tsx` | `level`, `size`, `as` |
| **Text** | `components/ui/Text.tsx` | `size`, `color`, `leading`, `weight`, `balance`, `mono`, `as` |

### Animation Components (exist, good adoption)

`Animate.Stagger`, `Animate.ScaleIn`, `Animate.FadeIn`, `Animate.SlideUp`, `Animate.SlideDown`, `Animate.PulseSoft`, `Animate.Breathe`, `Animate.BouncyStagger`, `Animate.InView` — all in `components/ui/Animate.tsx`.

### CSS System (exists, no React API)

| CSS Class | Defined In | React Component? | Used By |
|-----------|-----------|-------------------|---------|
| `.surface` | `surface.css:24` | NO | Card (internal), Button (internal) |
| `.pressable` | `surface.css:240` | NO | Button (internal) |
| `.elevation-sm/md/lg/none` | `surface.css:86-108` | NO | Card, Button (internal) |
| `.surface-primary/secondary/...` | `surface.css:122-194` | NO | Button (internal) |

---

## Part 2: className Leaking Catalog

### Pattern A: Vertical flex (`flex flex-col gap-*`) — 13 files

Should be `<Stack gap="...">`. Existing component handles this perfectly.

| File | Line(s) | Example |
|------|---------|---------|
| `app/(app)/page.tsx` | 32 | `<div className="flex flex-col gap-4">` |
| `components/blocks/Footer.tsx` | multiple | 5 instances of raw flex-col |
| `components/blocks/HeroSidebar.tsx` | multiple | 4 instances |
| `components/blocks/MobileNavMenu.tsx` | multiple | 6 instances |
| `app/(app)/_components/TrustLayout.tsx` | 32 | `<div className="flex flex-col gap-5">` |
| `app/(app)/_components/BragLayout.tsx` | multiple | 4 instances |
| `components/blocks/Pricing.tsx` | multiple | 3 instances |
| `app/(app)/[bnto]/_components/configs/*` | multiple | Config panels |
| `RecipeShell.tsx` | multiple | space-y-6, space-y-4 |
| `PhaseFlowShowcase.tsx` | multiple | Showcase layout |

### Pattern B: Horizontal flex (`flex items-center gap-*`) — 18 files

Should be `<Row gap="..." align="center">`. Most common gap is `gap-3` (no token — needs "base").

| File | Line(s) | Example |
|------|---------|---------|
| `app/(app)/[bnto]/_components/FileCard.tsx` | 73 | `<div className="flex min-w-0 flex-1 items-center gap-3">` |
| `components/blocks/Navbar.tsx` | 53 | `<div className="flex items-center gap-3.5 px-6 py-3">` |
| `app/(app)/_components/TrustLayout.tsx` | 36 | `<div className="flex items-center gap-3">` |
| `components/blocks/Footer.tsx` | multiple | Navigation rows |
| `components/blocks/HeroSidebar.tsx` | 67 | Feature list items |
| `components/blocks/RecipeGrid.tsx` | multiple | Card internal layout |
| `ExecutionProgress.tsx` | multiple | Status rows |
| `BrowserExecutionResults.tsx` | multiple | File result rows |
| `NavUser.tsx` | multiple | Dropdown items |
| *...8 more files* | | |

### Pattern C: Icon badge (centered icon in colored circle) — 6+ instances

Exact same 60+ character className pattern repeated identically.

| File | Line | Pattern |
|------|------|---------|
| `FileCard.tsx` | 74 | `flex size-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-primary` |
| `FileUploadItemMetadata.tsx` | 29 | Same pattern, `size-8` variant |
| `BrowserExecutionResults.tsx` | 61 | Same pattern |
| `HeroSidebar.tsx` | 67 | Same pattern |
| `BragLayout.tsx` | 57 | Same pattern |
| `RecipeGrid.tsx` | 49 | `rounded-lg` variant (square) |

Also appears with `bg-destructive/10` (TrustLayout) and `bg-muted` (RecipeShell).

### Pattern D: Pill badge (`rounded-full` label) — 3+ instances

| File | Line | Pattern |
|------|------|---------|
| `RecipeGrid.tsx` | 81 | `bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs font-medium` |
| `RecipeLayoutShowcase.tsx` | 221 | Same pattern |
| Various config components | — | File type tags |

### Pattern E: `space-y-*` instead of Stack — 17 files

`space-y-*` uses margin-between-children, which is fragile (breaks with conditionally rendered children). `<Stack gap>` uses flexbox gap, which is robust.

Found in: RecipeShell, loading.tsx, ExecutionResults, ExecutionProgress, ErrorCard, BrowserExecutionProgress, PhaseFlowShowcase, ProgressShowcase, and 9 more.

### Pattern F: Raw surface/pressable in non-primitive code — 3 instances

| File | Line | Pattern |
|------|------|---------|
| `PhaseIndicator.tsx` | 66 | `surface pressable` classes with conditional className logic |
| `Tabs.tsx` | — | `pressable` class on TabsTrigger |

---

## Part 3: Gap Token Analysis

The `GapSize` type has a hole between `sm` (gap-2 / 8px) and `md` (gap-4 / 16px).

**Raw `gap-3` (12px) appearances in the codebase:**
- `FileCard.tsx:73` — `gap-3`
- `TrustLayout.tsx:36` — `gap-3`
- `HeroSidebar.tsx:67` — `gap-3`
- `Navbar.tsx:53` — `gap-3.5`
- `Footer.tsx` — `gap-3`
- Multiple config components — `gap-3`

This is the #1 reason developers fall back to raw className for layout — the token they need doesn't exist.

**Recommendation:** Add `"base"` = `gap-3` to `GapSize`. This follows the Tailwind naming convention (base = the natural default between sm and md).

---

## Part 4: Component Design Decisions

### Surface and Pressable: Separate or combined?

**Decision: Separate.** Reasoning:
- Surface is visual (elevation, variant, rounding). Pressable is behavioral (hover/active/focus interaction). Many surfaces are not interactive (static cards, info panels).
- Card already composes Surface internally. Surface being standalone means any element can get the 3D treatment without being a Card.
- Button composes both. Keeping them separate preserves this layering.
- CSS already composes naturally (`.surface.pressable`) — React should mirror this.

### Should Card/Button be rewritten?

**No.** Card gets Surface composed internally (same output, cleaner internal code). Button imports shared `SpringMode`/`SPRING_STYLES` from Pressable (deduplication). Neither public API changes. The refactoring is invisible to consumers.

### Why not a `<Box>` or `<Flex>` component?

- `<Box>` is too generic — it doesn't communicate intent. A `<div>` with a className is clearer.
- `<Flex direction="column">` is less semantic than `<Stack>`. The name "Stack" tells you it's vertical. The name "Row" tells you it's horizontal. `<Flex direction="row" align="center">` requires reading props to understand layout.
- React Native has both Box and Flex but their ecosystem agrees Stack/Row/Column are superior for readability.

### Why not a `<Spacer>` component?

Gap-based layout (Stack/Row) eliminates the need for spacer elements. Spacers are a workaround for margin-based layout. We use gap.

---

## Part 5: Migration Complexity Assessment

| Wave | Files Touched | Risk | Visual Change |
|------|--------------|------|---------------|
| Wave 1 (new components) | 6 new files | None | None — new code only |
| Wave 2 (Surface/Pressable) | 2 new files | None | None — new code only |
| Wave 3 (Card/Button refactor) | 2 files | Medium | Must be pixel-identical |
| Wave 4 (consumer migration) | ~20 files | Low | Identical layout, component-based |
| Wave 5 (docs) | 3 files | None | No code changes |

**Highest risk:** Wave 3 (Card/Button internals). Mitigated by screenshot regression — full E2E suite must produce identical screenshots before and after.

**Lowest risk:** Wave 4 (consumer migration). Each file is a mechanical replacement of `<div className="flex flex-col gap-4">` with `<Stack gap="md">`. The rendered HTML is functionally equivalent.

---

## Part 6: What's Working Well (don't change)

- **Dot-notation compliance is 100%.** All multi-part components use `Card.Header`, `Dialog.Title`, etc. No flat imports found.
- **createCn pattern is solid.** Variant resolution via `createCn()` wrapping `tailwind-variants` works well. New components (Badge, IconBadge) should follow this pattern.
- **Animate.* API is well-adopted.** Consumer code uses composition components, not raw animation classes. This is the model for the new Surface/Pressable components.
- **CSS surface system is excellent.** 479 lines of carefully crafted CSS. Don't touch it — just wrap it in React components.
- **Object.assign namespace pattern is consistent.** Use it for any new compound components (Grid.Item already does this).
