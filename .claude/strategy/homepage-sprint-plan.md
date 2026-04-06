# Homepage & Site Polish — Sprint Plan

**Created:** April 6, 2026
**Branch:** `feat/homepage-cli-first`
**Goal:** Bring the landing page up to Motorways design system standard, one section at a time. Each piece is shippable independently.
**References:** [brand-messaging-audit.md](brand-messaging-audit.md), [landing-page-inspiration.md](landing-page-inspiration.md), [design-language.md](design-language.md)

---

## Philosophy

Every piece is a standalone PR. No piece depends on another. The order is a recommendation based on impact, not a dependency chain. Skip around if you want. The only hard rule: **each piece should make the site visibly better on its own.**

**Mascots:** Kawaii sushi characters from [Catalyst Labs](https://creativemarket.com/catalyststuff) (~$6 each). Purchase as you reach each section — nothing waits on them. See Piece 11 for the full character roster, category mapping, and placement strategy. The Motorways animation system (springs, stagger, surfaces, elevation) carries the visual identity on its own; the characters add personality on top.

**`<SpringIn>` component:** A new entrance animation component (backlog item in PLAN.md) that brings the springable surface "grounded → raised" animation into the entrance animation family. Elements start flat/muted and spring up to their elevated state — the "building materializing on the map" feeling. Composable with `<Stagger>`. Once built, `<SpringIn>` becomes the preferred entrance animation for any section with cards (Pieces 4, 5, 6, 8, 10). Use `<ScaleIn>` / `<SlideUp>` for non-card elements (text, buttons, terminals).

---

## Piece 1: Copy Polish (no component changes)

**Scope:** Text-only changes across the landing page. No new components, no layout changes, no animations. Just better words.

**Files touched:** `apps/web/` — page components + i18n strings

| Current                                                                          | Proposed                                                                     | File            |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- | --------------- |
| Hero sub: "Build a node for anything. Chain nodes into recipes..." (4 sentences) | "15 recipes included. Or pack your own." (1 sentence)                        | Hero section    |
| Divider: "Composable by design."                                                 | "What's in the box"                                                          | Section divider |
| Divider: "Open source."                                                          | "Open kitchen"                                                               | Section divider |
| Section: "How it works"                                                          | "What's in the box"                                                          | Section header  |
| Section: "No catch"                                                              | "No mystery meat" or keep "No catch"                                         | Section header  |
| Footer tagline: "Task automation, from simple to powerful."                      | "Pack. Run. Done."                                                           | Footer          |
| Pitch 1: "Composable nodes. Build any workflow..."                               | "Pick your ingredients. Each node does one thing. Chain them into anything." | Pitch points    |
| Pitch 2: "15 recipes included. One command: bnto run"                            | "15 house specials. One command: `bnto run`."                                | Pitch points    |
| Pitch 3: "Runs on your machine. Files never leave your device"                   | "Your kitchen, your rules. Files never leave your machine."                  | Pitch points    |
| Pitch 4: "Open source (MIT). Extend it, fork it, contribute"                     | "Open kitchen. MIT licensed. Peek behind the counter anytime."               | Pitch points    |
| "How it works" body (3 sentences)                                                | "One node, one job. Chain them together, run them anywhere."                 | How it works    |

**Definition of done:** All copy updated, `task ui:build` passes, screenshots regenerated if layout shifted.

---

## Piece 2: Nav Restructure

**Scope:** Redesign the top navigation. Remove FAQ from top nav, rename "Create (beta)" to "Editor (beta)", add an Explore dropdown mega-menu.

**Current nav:** `[bnto]  Create(beta)  Explore  FAQ  [theme] [auth]`

**Target nav:** `[bnto]  Explore ▾  Editor (beta)  [GitHub ★]  [Get started]`

**Sub-tasks:**

1. Rename "Create (beta)" → "Editor (beta)" in nav
2. Remove FAQ from top nav (keep in footer)
3. Build `ExploreDropdown` mega-menu component — recipes grouped by category, sourced from `core.registry`
4. Add "Get started" CTA button (links to `/explore` or GitHub install instructions)
5. Add GitHub star link (icon only, subtle)
6. Move theme toggle to footer or remove from nav

**Mascot opportunity:** Small character icons (24-32px) next to category names in the Explore mega-menu. Image = sushi roll, Spreadsheet = bento box, File = onigiri, Video = octopus. Not required — text-only categories work fine.

**Definition of done:** Nav renders correctly on desktop + mobile, Explore dropdown shows all recipes by category, E2E nav tests updated.

---

## Piece 3: Hero Section Animations

**Scope:** Add Motorways entrance animations to the existing hero section. No layout changes — just make what's already there spring to life.

**Sub-tasks:**

1. `SlideUp` on hero headline ("Pack your workflow.")
2. `FadeIn` with delay on sub-headline
3. `ScaleIn` with `spring-bouncy` on the CTA buttons (staggered)
4. `ScaleIn` on the terminal mockup card
5. Wrap pitch points in `Stagger` + `SlideUp`
6. Scroll-trigger setup — `IntersectionObserver` utility that adds animation classes on viewport entry

**Mascot opportunity:** Primary mascot (sushi roll with belt, winking) as a large floating character next to the headline. `ScaleIn` with `spring-bouncy` entrance. **Purchase: "Cute Sushi Salmon Roll Cartoon" from Catalyst Labs (~$6).**

**Definition of done:** Hero animates on page load. Below-fold sections animate on scroll. `motion-safe` respected throughout. No animation on `prefers-reduced-motion`.

---

## Piece 4: Explore Page Spring Animations

**Scope:** Add Motorways entrance animations to the `/explore` recipe card grid. Cards should stagger-cascade with spring pop-in.

**Sub-tasks:**

1. Wrap recipe cards in `Stagger` container
2. Each card uses `ScaleIn` with stagger index
3. Category filter tabs get spring selection animation
4. `FadeIn` on the page header

**Definition of done:** `/explore` cards pop in with stagger cascade on page load. Category switching re-triggers the cascade. Screenshots regenerated.

---

## Piece 5: "What's in the Box" Section Redesign

**Scope:** Redesign the "How it works" section into a visual 3-step flow using Motorways surfaces.

**Current:** Text-heavy explanation + terminal mockup showing compression demo.

**Target:** Three cards in a row showing Pick → Pack → Run, each as a Motorways surface card with elevation, spring entrance on scroll.

**Sub-tasks:**

1. Design 3 step cards: "Pick" (choose a recipe), "Pack" (configure nodes), "Run" (one command)
2. Each card is a `<Card>` with `ScaleIn` entrance, staggered left to right
3. Connecting arrows or flow indicators between cards (CSS, not SVG — simple `→` or dashed line)
4. Section header: "What's in the box" with "Nodes are compartments. Recipes are the box." as sub-text

**Mascot opportunity:** One character per step card — chopsticks sushi = Pick, square bento = Pack, floating maki = Run. **Purchase: "Sushi With Chopstick Cartoon" (~$6), "Cute Sushi Cartoon Illustration" (square, ~$6), "Cute Astronaut Surfing On Sushi" or similar action pose (~$6).** Characters sit inside or above each card as visual anchors.

**Definition of done:** Section renders 3 step cards with spring animations. Mobile stacks vertically. Screenshots regenerated.

---

## Piece 6: Recipe Showcase Section ("House Specials")

**Scope:** Add a dedicated recipe showcase section below "What's in the box" — a warm card grid showing the 15 recipes, grouped by category.

**Current:** Recipes only appear in the scrolling marquee (which is cool but not browsable) and on `/explore`.

**Target:** A curated grid section with category headers (Image, Spreadsheet, File, Video), each recipe as a compact card. Spring stagger entrance on scroll.

**Sub-tasks:**

1. Create `RecipeShowcase` section component
2. Group recipes by category with warm headers
3. Each recipe card: icon + name + one-liner + feature tags (reuse explore card or simplified version)
4. `Stagger` + `ScaleIn` cascade per category group
5. "Browse all recipes →" link to `/explore` at bottom
6. Category header characters: Image = sushi roll, Spreadsheet = bento box, File = onigiri, Video = octopus. **Purchase: "Cute Panda Sushi Cartoon" or primary mascot (~$6), "Cute Sushi Cartoon Illustration" (square, reuse from Piece 5), "Cute Onigiri Cartoon Illustration" (~$6), "Cute Octopus Holding Takoyaki" (~$6).** If not ready yet, existing line icons work fine

**Definition of done:** Section shows all 15 recipes in categorized grid. Spring animations on scroll. Links to individual recipe pages work.

---

## Piece 7: "Open Kitchen" Section Polish

**Scope:** Polish the existing "Open source" / "No catch" section. Better copy, Motorways surface treatment, animation.

**Current:** Split layout with heading left, strikethrough anti-pattern list right. Functional but flat.

**Target:** Same layout but with Motorways card surfaces, spring entrance, and warmer copy.

**Sub-tasks:**

1. Update copy: "No catch" → "No mystery meat" (or keep if we prefer)
2. Update copy: section header "Free recipes that stay free. Open source you can verify."
3. Wrap anti-pattern list in a `<Card>` with elevation
4. `SlideUp` stagger on the strikethrough items
5. `ScaleIn` on the GitHub CTA button
6. "MIT Licensed · Open Source" badge gets a surface treatment

**Definition of done:** Section has Motorways surfaces and animations. Copy updated. Screenshots regenerated.

---

## Piece 8: "Build Your Own" Section (new)

**Scope:** Add a new section teasing the visual editor and `.bnto.json` format.

**Current:** This section doesn't exist.

**Target:** A section showing a `.bnto.json` code snippet alongside a teaser for the editor, with a CTA to open the editor.

**Sub-tasks:**

1. Create `BuildYourOwn` section component
2. Left: copy — "Pack your own bento. Any node, any combination."
3. Right: `.bnto.json` code preview in a `<Card>` with mono font, syntax highlighting
4. CTA: "Open Editor (beta) →" button
5. `ScaleIn` entrance on the code card, `SlideUp` on the copy
6. Consider a small animated preview of the editor canvas (stretch goal — a static screenshot is fine for v1)

**Definition of done:** New section renders between "Open Kitchen" and Footer. Links to `/editor`. Spring animations on scroll.

---

## Piece 9: Footer Refresh

**Scope:** Polish the footer with updated messaging and structure.

**Current:** Logo + tagline + recipe links by category + company links. Functional.

**Target:** Same structure but with updated tagline ("Pack. Run. Done."), better visual treatment, and relocated items (FAQ link, theme toggle if moved here).

**Sub-tasks:**

1. Update tagline: "Task automation, from simple to powerful." → "Pack. Run. Done."
2. Add FAQ link to company column
3. Consider adding "Docs" link (even if it just goes to GitHub README for now)
4. Visual polish: better spacing, subtle separator, warm treatment
5. Move theme toggle here if removed from nav (Piece 2)

**Definition of done:** Footer updated. All links work. Theme toggle relocated if applicable.

---

## Piece 10: Recipe Page Animations

**Scope:** Add Motorways entrance animations to individual recipe pages (`/compress-images`, `/clean-csv`, etc.).

**Current:** RecipeShell renders statically.

**Target:** Recipe page content springs in on load — header, config panel, drop zone all animate.

**Sub-tasks:**

1. `SlideUp` on recipe page header (title + description)
2. `ScaleIn` on the file drop zone card
3. `FadeIn` on the config section
4. Feature tags get `SlideUp` with stagger
5. JSON-LD description section gets `FadeIn`

**Definition of done:** Recipe pages animate on load. All 15 recipe pages render correctly. Screenshots regenerated for pages project.

---

## Piece 11: Mascots & Illustrations — Catalyst Labs Kawaii Sushi

**Scope:** Integrate purchased kawaii sushi character illustrations across the site. NOT a blocker for any other piece — each section works without them, but they add tremendous personality.

**Source:** [Catalyst Labs on Creative Market](https://creativemarket.com/catalyststuff) — kawaii Japanese food character illustrations. ~$6 each, consistent art style (thick outlines, bold colors, kawaii faces). Same artist ensures cohesive look across all characters.

### Character Roster & Site Placement

Purchase these illustrations as each section is built. The character column tells you which to buy when you reach that piece.

| Character                            | Description                                               | Site Placement                                                                  | Which Piece           |
| ------------------------------------ | --------------------------------------------------------- | ------------------------------------------------------------------------------- | --------------------- |
| **Sushi roll with belt**             | Salmon nigiri with nori belt, winking, finger-up pose     | **Primary mascot** — hero section, brand identity, favicon candidate            | Piece 3 (hero)        |
| **Square bento sushi**               | Maki roll with visible compartments inside, heart bubble  | **"What's in the box"** — the product metaphor incarnate (compartments = nodes) | Piece 5               |
| **Sushi in chopsticks**              | Nigiri being picked up by chopsticks, content expression  | **"Pick" step card** — choosing/selecting gesture                               | Piece 5               |
| **Maki on chopstick**                | Round roll skewered on a chopstick, floating with shadow  | **"Run" step card** — in motion, processing                                     | Piece 5               |
| **Rice + salmon pair**               | Two pieces wrapped together, hugging                      | **"Pack" step card** or composability messaging                                 | Piece 5 or 6          |
| **Onigiri with salmon**              | Triangle rice ball, happy salmon face in center           | **File category icon** — simple, fundamental building block                     | Piece 6               |
| **Octopus with takoyaki**            | Happy octopus holding takoyaki skewer, hachimaki headband | **Video category icon** or fun accent (404 page, loading state)                 | Piece 6 or standalone |
| **Cute sushi roll** (round, pink bg) | Classic maki roll face, rosy cheeks                       | **Image category icon** — visually rich, primary capability                     | Piece 6               |

### Category → Character Mapping

| Node Category   | Character                                 | Reasoning                                            |
| --------------- | ----------------------------------------- | ---------------------------------------------------- |
| **Image**       | Sushi roll with belt (primary mascot)     | Most visually rich — represents the hero capability  |
| **Spreadsheet** | Square bento sushi (compartments visible) | Grid = rows and columns = compartments               |
| **File**        | Onigiri (salmon center)                   | Simple, fundamental, the building block              |
| **Video**       | Octopus with takoyaki                     | Animated, lively, multiple "arms" = multiple streams |

### Placement Strategy

Characters drop into sections as those sections are built. Nothing waits on illustrations.

- **Hero section** (Piece 3): Primary mascot (sushi with belt) as a large floating character alongside the headline. `ScaleIn` with `spring-bouncy` entrance
- **"What's in the box" cards** (Piece 5): One character per step — chopsticks picking = Pick, bento square = Pack, floating maki = Run
- **Recipe category headers** (Piece 6): One character per category on the Recipe Showcase and Explore page
- **Nav dropdown** (Piece 2): Small character icons next to category names in the Explore mega-menu
- **404 page** (standalone): Octopus looking confused
- **Footer** (Piece 9): Small mascot waving or holding a to-go box
- **Recipe pages** (Piece 10): Category character as a small accent on individual recipe pages

### Format & Integration

- Purchase as PNG from Creative Market, then trace/convert to SVG for crisp rendering at any size (or use as-is if the PNGs are high enough resolution)
- Recolor to harmonize with bnto's warm palette (terracotta, teal, golden on cream) if the original colors clash
- Each illustration is a standalone React component wrapping an `<img>` or inline SVG
- Size variants: hero (200-300px), section accent (100-150px), category icon (40-60px), nav icon (24-32px)

**When to do this:** Purchase illustrations as you reach each piece. The primary mascot (sushi with belt) is highest priority — buy it when starting Piece 3 (hero animations) or Piece 5 ("What's in the box"). Category icons can wait until Piece 6 (recipe showcase).

**Budget:** ~$48-60 for 8 illustrations at $6 each. All from one artist for visual consistency.

**Definition of done:** Characters integrated into relevant sections. Each character renders at appropriate sizes. Colors harmonize with bnto palette. `motion-safe` respected on any character animations.

---

## Piece 12: FAQ Page Polish (low priority)

**Scope:** Minor polish to the FAQ page since it's moving out of top nav.

**Sub-tasks:**

1. Add `ScaleIn` entrance on FAQ accordion items
2. Consider reorganizing questions by category
3. Ensure the page is discoverable from footer

**Definition of done:** FAQ page has spring animations. Accessible from footer.

---

## Quick Reference: Impact vs Effort

| Piece                       | Impact | Effort                     | Depends on              |
| --------------------------- | ------ | -------------------------- | ----------------------- |
| 1. Copy polish              | High   | Low                        | Nothing                 |
| 2. Nav restructure          | High   | Medium                     | Nothing                 |
| 3. Hero animations          | High   | Low                        | Nothing                 |
| 4. Explore animations       | Medium | Low                        | Nothing                 |
| 5. "What's in the box"      | High   | Medium                     | Piece 1 (for copy)      |
| 6. Recipe showcase          | Medium | Medium                     | Nothing                 |
| 7. "Open kitchen" polish    | Medium | Low                        | Piece 1 (for copy)      |
| 8. "Build your own"         | Medium | Medium                     | Nothing                 |
| 9. Footer refresh           | Low    | Low                        | Nothing                 |
| 10. Recipe page animations  | Medium | Low                        | Nothing                 |
| 11. Mascots (Catalyst Labs) | High   | Low (purchase + integrate) | Nothing (buy per-piece) |
| 12. FAQ polish              | Low    | Low                        | Nothing                 |

**Recommended order:** 1 → 3 → 2 → 4 → 5 → 7 → 6 → 8 → 9 → 10 → 12 → 11

Start with copy (instant improvement, zero risk), then animations (biggest visual bang for smallest effort), then structural changes (nav, new sections).
