# Landing Page Inspiration & Direction

**Created:** April 6, 2026
**Status:** Active reference for homepage redesign (`feat/homepage-cli-first`)

---

## Reference Sites

Three primary references for bnto's landing page. Each brings something different. Together they define the target.

### 1. Charm (charm.sh) — Personality & Warmth

**What we take:**

- Unapologetic personality. Witty, human copy. Not afraid to be fun
- Mascots and character-driven branding for each tool
- Card-based grid layout with warm, color-rich surfaces
- Playful naming and micro-interactions that reward exploration
- The confidence to be whimsical in a developer space and still command respect

**Key insight:** Charm proves that "fun" and "credible" are not opposites. Their personality IS their credibility — it signals taste, care, and a team that sweats the details.

### 2. Bun (bun.sh) — Warm Palette & Polish

**What we take:**

- Warm accent colors (orange, pink) on off-white backgrounds — very close to our cream + terracotta palette
- Cute mascot that adds personality without being childish
- Pill-shaped buttons, generous rounded corners everywhere
- Premium feel through spacing and restraint, not darkness or complexity
- Balance of "serious about performance, fun to work with"

**Key insight:** Bun's off-white base (`#f7f7f4`) is nearly identical to bnto's warm cream background. Their approach to warmth through color temperature rather than illustration is a direct reference for our token system.

### 3. Deno (deno.com) — Cozy Atmosphere & Illustration

**What we take:**

- Lo-fi illustrated mascot and atmospheric scene-setting (the rainy window, the coffee)
- "Cozy productivity" feeling — warm, inviting, makes you want to use the tool
- Dark charcoal paired with warm taupes and creams
- Illustration-forward hero that tells a story, not just shows a product
- The feeling that this tool was made by people who care about the experience

**Key insight:** Deno's hero illustration does the work of 1000 words. It communicates the entire brand personality in one image. bnto needs an equivalent "signature visual moment."

---

## The bnto Hero: A Mini Motorways Level

The hero section should feel like looking at a Mini Motorways level in play. Not a screenshot of the game — an original composition in our design language that evokes the same feeling: warm, geometric, satisfying, alive.

### The Vision

Imagine a warm cream canvas. Rounded, colorful nodes (terracotta, teal, golden) are connected by smooth, flowing paths — like Mini Motorways roads. Files flow along the paths between nodes like little cars. Buildings (nodes) pop in with springy animations. The whole thing breathes with gentle, purposeful motion.

This is bnto's "cozy rainy window" moment (Deno) and "glamorous terminal" moment (Charm) rolled into one. It communicates: **workflows are satisfying when they're well-organized.**

### Visual Elements

| Element              | Mini Motorways Analog              | bnto Implementation                                            |
| -------------------- | ---------------------------------- | -------------------------------------------------------------- |
| **Nodes**            | Buildings (houses, shops)          | Rounded cards in terracotta, teal, golden — each a recipe step |
| **Connections**      | Roads between buildings            | Smooth curved paths between nodes                              |
| **Files in transit** | Cars on roads                      | Small rounded rectangles flowing along paths                   |
| **Background**       | The map surface                    | Warm cream with subtle grid texture                            |
| **Animation**        | Buildings popping up, cars flowing | Spring entrances for nodes, smooth flow for files              |

### Animation Approach

All CSS-first, using our existing animation system:

- **Node pop-in:** `ScaleIn` with `spring-bouncy` — buildings materializing on the map
- **File flow:** CSS `offset-path` animation along the connection curves — cars on roads
- **Idle breathing:** Gentle `Breathe` on active nodes — the level is alive
- **Staggered entrance:** `Stagger` cascade so nodes appear one by one on scroll — the level builds itself

The hero should feel alive but calm. Like watching a Mini Motorways level that's running smoothly — satisfying, not stressful.

### Copy Direction

Inspired by Charm's confidence and Deno's warmth:

- **Headline:** Short, punchy, verb-first. Not "The workflow automation platform" — more like "Pack your workflow."
- **Sub:** One sentence that explains what bnto does in plain language. No jargon
- **CTA:** `cargo install bnto-cli` front and center — the CLI is the product
- **Tone:** Warm, inviting, slightly playful. Like a friend showing you a tool they love

### What We Do NOT Do

- No dark hero section (we're warm cream, not midnight purple)
- No 3D renders or glass-morphism (we're 2D, flat, geometric — like the game)
- No stock photography or generic "developer at laptop" imagery
- No feature matrix in the hero (save that for below the fold)
- No "enterprise" language. We're open source, free, fun
- No auto-playing video. The animation IS the hero

---

## Page Sections (Below the Fold)

### Recipe Showcase

Like Charm's tool grid — each recipe as a warm card with its icon, name, and one-line description. Spring entrance on scroll. Links to the dedicated `/compress-images`, `/clean-csv` pages.

### How It Works

Three steps, illustrated simply:

1. Pick a recipe (or write your own `.bnto.json`)
2. Run it (`bnto run compress-images photos/`)
3. Done. Files processed, right where you left them

### Node Types

Show what bnto can do — image, CSV, file, video nodes. Each node type as a small, rounded, color-coded card. The "building types" on our map.

### Open Source

MIT licensed. The engine compiles everywhere. Community-first. Link to GitHub, crates.io, contributing guide.

---

## Design Principles for the Landing Page

1. **The hero is a level, not a screenshot.** An original animated composition, not a product demo
2. **Warm cream is the canvas.** The page background IS the Mini Motorways map surface
3. **Springy everything.** Every entrance animation uses our spring system. Nothing fades — things pop in
4. **Copy has personality.** Write like Charm, not like AWS
5. **CLI first, always.** `cargo install bnto-cli` is the primary CTA, not "Sign up"
6. **Show, don't tell.** The animated hero demonstrates what bnto does better than any paragraph
7. **Restraint over spectacle.** The animation should feel satisfying, not overwhelming. Calm productivity, not a fireworks show

---

## Relationship to Existing Docs

This document is the **creative direction** for the landing page. It builds on:

- [design-language.md](design-language.md) — Visual identity tokens (colors, type, radius, animation). Non-negotiable foundation
- [animation.md](../scopes/web/animation.md) — Animation implementation system (spring curves, CSS classes, components)
- [theming.md](../scopes/web/theming.md) — Exact token values (OKLCH colors, shadow scale, radius scale)
- [seo.md](../scopes/web/seo.md) — URL strategy, metadata, structured data for recipe pages

The design language is locked. This document adds landing-page-specific creative direction on top of it.

**Mascot decision (April 6, 2026):** Kawaii sushi characters from [Catalyst Labs](https://creativemarket.com/catalyststuff) on Creative Market. This delivers on the Charm-inspired vision — each tool/category gets its own character mascot, but with Japanese food instead of abstract creatures. See [brand-messaging-audit.md](brand-messaging-audit.md) for the full character roster and placement strategy, and [homepage-sprint-plan.md](homepage-sprint-plan.md) Piece 11 for purchase-per-section integration plan.
