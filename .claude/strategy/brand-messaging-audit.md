# Brand & Messaging Audit

**Created:** April 6, 2026
**Updated:** April 6, 2026 (added mascot strategy — Catalyst Labs kawaii sushi characters)
**Purpose:** Review current copy, identify improvements, outline landing page structure and nav redesign

---

## The Bento Box Metaphor — Our Secret Weapon

bnto has a natural metaphor that most dev tools would kill for: **the bento box.** It's not forced — it's literally what the product does. Nodes are compartments. Recipes are the box. You pack your workflow.

This metaphor unlocks a whole vocabulary that's warm, visual, and memorable:

| Concept             | Current Term     | Bento Metaphor    | Notes                                            |
| ------------------- | ---------------- | ----------------- | ------------------------------------------------ |
| A workflow pipeline | Recipe           | Recipe            | Already good — cooking/prep metaphor             |
| A single node       | Node             | Compartment       | "Nodes are compartments" is already in our copy  |
| Node types          | Categories       | Ingredients       | Nodes are the ingredients you pack into your box |
| Running a recipe    | Execution        | Serving / Plating | "Serve your workflow"                            |
| Predefined recipes  | Built-in recipes | House specials    | Like a restaurant's signature dishes             |
| Custom recipes      | Custom recipes   | Build your own    | "Build your own bento" / "Pack your own"         |
| The engine          | Engine           | Kitchen           | Where the cooking happens                        |
| CLI                 | CLI              | Counter service   | Fast, direct, no-frills                          |
| Browser execution   | Browser          | Takeout window    | Grab and go, no install                          |
| `.bnto.json` files  | Recipe files     | The menu          | Portable, shareable recipe cards                 |
| Output files        | Results          | The finished box  | Ready to enjoy                                   |

**This vocabulary should leak into our copy everywhere** — not forced on every sentence, but sprinkled naturally so the whole site feels cohesive.

---

## Mascot & Character Strategy — Catalyst Labs Kawaii Sushi

**Decision (April 6, 2026):** bnto's visual identity gets kawaii sushi characters from [Catalyst Labs on Creative Market](https://creativemarket.com/catalyststuff). Think Charm's approach — each tool has its own character mascot — but with Japanese food instead of abstract creatures. The bento/sushi metaphor that's already in our copy now gets a visual embodiment.

**Why this artist:** Consistent thick-outline style, bold colors, kawaii faces, ~$6/illustration. All from one artist ensures visual cohesion across the site. The style pairs naturally with Motorways' warm surfaces and spring animations — playful, approachable, premium.

**The roster:**

| Character                                 | Catalyst Labs Product                        | bnto Role                                           | Purchase When  |
| ----------------------------------------- | -------------------------------------------- | --------------------------------------------------- | -------------- |
| Sushi roll with belt (winking, finger up) | "Cute Sushi Salmon Roll Cartoon"             | **Primary mascot** — hero, brand identity           | Piece 3 (hero) |
| Square bento sushi (compartments visible) | "Cute Sushi Cartoon Illustration"            | **"Pack" step** — compartments = nodes metaphor     | Piece 5        |
| Sushi in chopsticks (content expression)  | "Sushi With Chopstick Cartoon"               | **"Pick" step** — choosing/selecting                | Piece 5        |
| Maki on chopstick (floating)              | "Cute Astronaut Surfing On Sushi" or similar | **"Run" step** — in motion, processing              | Piece 5        |
| Rice + salmon pair (hugging)              | "Cute Sushi And Friend Illustration"         | **Composability** — nodes working together          | Piece 6        |
| Onigiri with salmon center                | "Cute Onigiri Cartoon Illustration"          | **File category icon** — fundamental building block | Piece 6        |
| Octopus with takoyaki (hachimaki)         | "Cute Octopus Holding Takoyaki"              | **Video category icon** — lively, multi-stream      | Piece 6        |
| Classic maki roll (round, rosy cheeks)    | "Cute Panda Sushi Cartoon"                   | **Image category icon** — primary capability        | Piece 6        |

**Category mapping:**

| Node Category | Character                   | Why                                       |
| ------------- | --------------------------- | ----------------------------------------- |
| Image         | Sushi roll with belt        | Most visually rich — hero capability      |
| Spreadsheet   | Square bento (compartments) | Grid = rows/columns = compartments        |
| File          | Onigiri                     | Simple, fundamental, the building block   |
| Video         | Octopus with takoyaki       | Animated, lively, multiple arms = streams |

**Budget:** ~$48-60 total for 8 characters. Purchase as each section is built.

**Integration:** Purchase PNGs, trace/convert to SVG for resolution independence. Recolor to harmonize with bnto palette if needed. Each character becomes a React component. Size variants: hero (200-300px), section accent (100-150px), category icon (40-60px), nav icon (24-32px).

**This completes the Charm-inspired vision.** Charm has cute creatures for each CLI tool. bnto has cute sushi for each capability. The bento metaphor is no longer just in the copy — it's in the visual identity.

---

## Visual Observations (from Screenshots)

### What the Motorways Design System Gets Right

The Motorways design system is the best part of the entire product surface. It's genuinely special:

- **The 3D surface system** — Cards with directional shadows that respond to elevation. None of this is visible on the landing page
- **Spring animations** — bouncy/bouncier/bounciest springs with grounded-to-elevated transitions. The landing page has zero spring animations
- **The warm palette in action** — Primary (terracotta), Secondary (teal), Accent (golden), all on warm cream. Beautiful together but the landing page only uses primary sparingly
- **Component polish** — Dormant buttons that wake on hover, radial dial gauges, toggle buttons with spring press. These are delightful interactions that nobody visiting the homepage would ever know about
- **Surface variants** — Solid, dashed, none borders. The dashed border on placeholder surfaces is a lovely detail

**The gap:** The Motorways system is built for Mini Motorways-level delight. The landing page delivers SaaS-template-level functionality. The design system has personality to burn; the page doesn't spend any of it.

### Landing Page Issues (Visual)

1. **No animation on the page.** Zero springs, zero entrance effects, zero delight. The page loads fully rendered and static. Compare to Charm where tools have character, or Deno where the dinosaur illustration immediately signals personality
2. **The terminal mockup is the visual centerpiece** — and it's fine, but it's just a code block. It doesn't move, doesn't demonstrate, doesn't delight. Bun's terminal animation auto-types commands and shows output appearing in real time
3. **The recipe card grid on /explore has no spring entrance.** Cards just... appear. The Motorways system has `ScaleIn` with bounciest springs specifically designed for card entrances
4. **No color variety in the sections.** Every section is cream background + dark text. No section uses the secondary (teal) or accent (golden) palettes. The page feels monochrome despite having a rich token system
5. **The pitch point icons are the only color** — the terracotta checkmarks and feature badges. Everything else is warm gray on cream
6. **No hero illustration or visual moment.** The "hero" is text + terminal. There's no signature visual that says "this is bnto, not any other dev tool"

### Explore Page Issues (Visual)

1. **Generic card grid** — Functional but could be any product. No warmth, no delight
2. **Recipe icons are abstract line art** — They're clean but forgettable. No personality
3. **Category filter tabs** are fine but could be warmer (rounded pills with spring selection animation)
4. **No stagger animation** — All 15 cards appear at once. A stagger cascade would make browsing feel alive

### What's Actually Working

1. **The warm cream background** — Sets the right tone immediately. Reads as "cozy productivity"
2. **The nav bar** — Clean pill shape, properly rounded, good spacing
3. **The footer** — Well-organized recipe links by category. Actually a useful reference
4. **The anti-pattern strikethrough list** in the "Open source" section — Clever, memorable, trust-building
5. **"Pack your workflow."** — Strong headline that lands instantly

---

## Current Copy Audit

### What's Working

1. **"Pack your workflow."** — Strong headline. Short, punchy, uses the metaphor perfectly
2. **"Nodes are compartments. Recipes are the box."** — This is the best line on the site. It teaches the mental model in 7 words
3. **"Free. Open source. Runs on your machine."** — Clear trust line, no fluff
4. **FAQ answers** — Honest, direct, no corporate speak
5. **The strikethrough anti-patterns** — "Signup required" / "File size limits" / "Daily usage caps" all struck through. This is effective and memorable

### What Needs Work

#### Hero Subheading (too long, too technical)

**Current:** "Build a node for anything. Chain nodes into recipes that automate your workflow. 15 recipes work out of the box. Need something custom? Compose your own."

**Problem:** 4 sentences, 29 words, mixes technical ("node", "chain") with marketing ("out of the box"). By the time you finish reading, you've forgotten the headline.

**Direction:** One sentence. Lean into the metaphor.

- "15 house specials included. Or pack your own."
- "Pick your ingredients, pack your box, run it anywhere."
- "Automate anything. 15 recipes included, or compose your own."

#### Section Headers (functional, not memorable)

**Current:** "How it works" / "No catch"

**Problem:** Generic. Every SaaS landing page has "How it works." These don't reinforce the brand or make you smile.

**Direction:** Use the metaphor:

- "How it works" → "What's in the box" or "How to pack a bento"
- "No catch" → "No hidden fees. No mystery meat." or "What you see is what you eat."

#### Divider Labels (bland)

**Current:** "Composable by design." / "Open source."

**Problem:** These read like architecture docs, not a landing page. "Composable by design" is accurate but doesn't make anyone feel anything.

**Direction:**

- "Composable by design." → "Mix and match." or "Pick your ingredients."
- "Open source." → "Open kitchen." (you can see exactly how the food is made)

#### Site Tagline (too generic)

**Current (footer):** "Task automation, from simple to powerful."

**Problem:** Could be any automation tool. Doesn't mention bento, recipes, nodes, or anything bnto-specific.

**Direction:**

- "Workflow automation, one compartment at a time."
- "Pack. Run. Done."
- "Automate your workflow. Serve it anywhere."

#### Pitch Points (functional but dry)

**Current:**

1. "Composable nodes. Build any workflow by chaining nodes together"
2. "15 recipes included. One command: bnto run"
3. "Runs on your machine. Files never leave your device"
4. "Open source (MIT). Extend it, fork it, contribute"

**Direction:** Same info, more personality:

1. "Pick your ingredients. Each node does one thing perfectly. Chain them into anything."
2. "15 house specials. One command: `bnto run`. Done before the kettle boils."
3. "Your kitchen, your rules. Files never leave your machine."
4. "Open kitchen. MIT licensed. Peek behind the counter anytime."

#### How It Works Description (wall of text)

**Current:** "Each node encapsulates one capability: compress an image, rename a file, clean a CSV, download a video. Chain nodes into recipes that automate your workflow. Recipes are portable .bnto.json files that run everywhere: CLI, browser, desktop."

**Problem:** 3 sentences that repeat what the heading already said. Too much detail for a landing page section.

**Direction:** Shorter, punchier, one sentence:

- "One node, one job. Chain them together, run them anywhere."

---

## Landing Page Structure (Proposed)

### Nav Bar

**Current nav:** `[bnto]  Create(beta)  Explore  FAQ  [theme] [auth]`

**Problems:**

- "Create (beta)" is buried and unclear — what am I creating?
- "FAQ" in the top nav is wasted prime real estate on a reference page
- No Explore dropdown — forces navigation to a separate page to browse recipes
- No install CTA — the primary action isn't represented
- Theme toggle in nav is a dev convenience, not a user need

**Proposed nav:**

```
[bnto]     Explore ▾     Editor (beta)     Docs                [GitHub ★]  [Get started]
```

**Explore dropdown** — a rich mega-menu inspired by Deno's product dropdowns and Charm's categorization:

```
┌──────────────────────────────────────────────────────────────┐
│                                                              │
│  RECIPES                              WHAT IT HANDLES        │
│                                                              │
│  Image                                Image                  │
│    Compress Images                      PNG, JPEG, WebP      │
│    Resize Images                        GIF, TIFF, BMP       │
│    Convert Format                                            │
│    Optimize for Web                   Spreadsheet            │
│    Generate Thumbnails                  CSV (any delimiter)  │
│    Strip EXIF                           JSON (from CSV)      │
│    Compress & Rename                                         │
│    Watermark Images                   File                   │
│                                         Any file (rename)    │
│  Spreadsheet                            Batch patterns       │
│    Clean CSV                                                 │
│    Rename Columns                     Video                  │
│    CSV to JSON                          YouTube, Vimeo       │
│    Merge CSV                            1000+ sites          │
│    Standardize CSV                                           │
│                                                              │
│  File                                                        │
│    Rename Files                                              │
│                                                              │
│  Video                                                       │
│    Download Video                                            │
│                                                              │
│  [Browse all recipes →]                                      │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Key changes:**

- "Create (beta)" → "Editor (beta)" — clearer what it is
- Add rich "Explore" dropdown that surfaces recipes grouped by category
- FAQ → footer only (reference page, not navigation)
- Add "Docs" link (placeholder until docs exist, links to README/GitHub for now)
- Primary CTA button: "Get started" (links to install instructions or `/explore`)
- GitHub star count as social proof
- Theme toggle → footer or removed

### Page Sections

```
┌──────────────────────────────────────────────────────────────┐
│                         NAV BAR                              │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│                      HERO SECTION                            │
│                                                              │
│   FREE · OPEN SOURCE · RUNS ON YOUR MACHINE                 │
│                                                              │
│   "Pack your workflow."                                      │
│   15 recipes included. Or pack your own.                     │
│                                                              │
│   [$ cargo install bnto]     [Try in browser →]              │
│                                                              │
│   ┌──────────────────────────────────────────────┐           │
│   │  Animated Mini Motorways-style hero          │           │
│   │  Nodes connected by curved paths             │           │
│   │  Files flowing between nodes like cars       │           │
│   │  Spring pop-in, gentle idle breathing        │           │
│   └──────────────────────────────────────────────┘           │
│                                                              │
├───────────── · WHAT'S IN THE BOX · ─────────────────────────┤
│                                                              │
│   "Nodes are compartments. Recipes are the box."             │
│                                                              │
│   ┌──────────┐   ┌──────────┐   ┌──────────┐               │
│   │  1. PICK │──▸│  2. PACK │──▸│  3. RUN  │               │
│   │          │   │          │   │          │               │
│   │ Choose a │   │ Chain    │   │ One cmd: │               │
│   │ recipe   │   │ nodes    │   │ bnto run │               │
│   └──────────┘   └──────────┘   └──────────┘               │
│                                                              │
├───────────── · HOUSE SPECIALS · ─────────────────────────────┤
│                                                              │
│   "15 recipes ready to run. Zero setup."                     │
│                                                              │
│   ┌─────────┐ ┌─────────┐ ┌─────────┐                      │
│   │ Compress│ │ Resize  │ │ Convert │  ... (dormant stagger) │
│   │ Images  │ │ Images  │ │ Format  │                       │
│   └─────────┘ └─────────┘ └─────────┘                      │
│                                                              │
│   [Browse all recipes →]                                     │
│                                                              │
├───────────── · YOUR KITCHEN, YOUR RULES · ──────────────────┤
│                                                              │
│   Terminal animation: bnto run compress-images photos/       │
│   (Shows files being processed with progress bars)           │
│                                                              │
│   ✓ Runs on your machine. Files never leave your device      │
│   ✓ Portable .bnto.json recipes. Run anywhere                │
│   ✓ Build a node for anything. Chain into recipes            │
│   ✓ Free forever. Open source. MIT licensed                  │
│                                                              │
├───────────── · OPEN KITCHEN · ──────────────────────────────┤
│                                                              │
│   "Free recipes that stay free.                              │
│    Open source you can verify."                              │
│                                                              │
│   ~~Signup required~~        ← strikethrough anti-patterns   │
│   ~~File size limits~~                                       │
│   ~~Daily usage caps~~                                       │
│   ~~Watermarks on output~~                                   │
│   ~~Quality reduction~~                                      │
│   ~~"Upgrade to continue"~~                                  │
│                                                              │
│   MIT LICENSED · OPEN SOURCE                                 │
│   [View on GitHub]                                           │
│                                                              │
├───────────── · BUILD YOUR OWN · ─────────────────────────────┤
│                                                              │
│   "Pack your own bento.                                      │
│    Any node, any combination."                               │
│                                                              │
│   ┌─ .bnto.json ──────────────┐                              │
│   │ {                          │  ← Animated code preview    │
│   │   "name": "my-workflow",   │                              │
│   │   "nodes": [...]           │                              │
│   │ }                          │                              │
│   └────────────────────────────┘                              │
│                                                              │
│   [Open Editor (beta) →]                                     │
│                                                              │
├──────────────────────────────────────────────────────────────┤
│                         FOOTER                               │
│                                                              │
│   [bnto]                                                     │
│   "Pack. Run. Done."                                         │
│                                                              │
│   IMAGE           DATA              FILE        COMPANY      │
│   Compress        Clean CSV         Rename      Privacy      │
│   Resize          Rename Cols       ...         GitHub       │
│   Convert         CSV to JSON                   FAQ          │
│   ...             ...                           Docs         │
│                                                              │
│   [Open source ↗]  [Buy me a coffee ☕]                      │
│                                                              │
│   MIT Licensed. Free and open source.                        │
└──────────────────────────────────────────────────────────────┘
```

### New Section: "Build Your Own"

This doesn't exist today. It should tease the visual editor and the `.bnto.json` format:

- Show a small code snippet of a `.bnto.json` file (the "recipe card")
- Link to the editor for visual composition
- Reinforce "pack your own" — this isn't just for our predefined recipes

### Removed / Relocated

- **FAQ** → Footer link only. FAQs are reference, not navigation
- **"Create (beta)" button** → Becomes "Editor (beta)" in nav. Clearer name
- **Theme toggle** → Move to footer or remove from nav entirely

---

## Messaging Cheat Sheet

Quick reference for anyone writing copy for bnto:

### Voice

| Do                             | Don't                                        |
| ------------------------------ | -------------------------------------------- |
| "Pack your workflow"           | "Orchestrate your automation pipeline"       |
| "15 house specials"            | "15 predefined workflow templates"           |
| "Your kitchen, your rules"     | "Full local execution with zero data egress" |
| "Open kitchen"                 | "Transparent open-source codebase"           |
| "Done before the kettle boils" | "Sub-100ms execution time"                   |
| "Pick your ingredients"        | "Select composable processing nodes"         |
| "No mystery meat"              | "Full transparency into processing logic"    |
| "Serve it anywhere"            | "Cross-platform execution targets"           |

### Recurring Phrases

- **"Pack your workflow."** — The headline. Use it
- **"Pack. Run. Done."** — The three-word summary. Footer, social bios, meta descriptions
- **"Nodes are compartments. Recipes are the box."** — The mental model explainer
- **"House specials"** — For predefined recipes
- **"Open kitchen"** — For open source messaging
- **"Your kitchen, your rules"** — For local execution / privacy
- **"No mystery meat"** — For the anti-pattern / trust section
- **"What's in the box"** — For the how-it-works section

### Copy Progression (how to introduce bnto)

For any context where you need to explain bnto in increasing detail:

1. **One word:** Workflows
2. **Three words:** Pack. Run. Done.
3. **One sentence:** Pack your workflow — automate anything with composable nodes that run anywhere
4. **One paragraph:** bnto is workflow automation through composable parts. Each node does one thing — compress an image, clean a CSV, rename files. Chain them into recipes. Run them from the CLI, the browser, or the desktop. 15 recipes included, or pack your own. Free, open source, MIT licensed.

### Copy Rules

1. **Short > long.** If you can say it in 5 words, don't use 15
2. **Metaphor > jargon.** "Pack your box" > "compose your pipeline"
3. **Show confidence.** No hedging ("might", "could", "helps"). Just state what it does
4. **Be specific.** "15 recipes" > "many recipes". "127ms" > "fast"
5. **Personality is earned through brevity.** A witty 5-word line > a clever 20-word line
6. **The bento metaphor extends naturally — use it.** Kitchen, ingredients, house specials, serving, plating. Don't force it, but don't abandon it either
7. **Technical accuracy under warm copy.** Never sacrifice correctness for whimsy. "Runs on your machine" is both warm AND accurate

---

## Section-by-Section Animation Plan

Every section should use the Motorways animation system. Currently zero animations are on the landing page.

| Section              | Animation                            | Component           | Trigger              |
| -------------------- | ------------------------------------ | ------------------- | -------------------- |
| Hero headline        | `SlideUp`                            | Text reveal         | Page load            |
| Hero subheading      | `FadeIn` with 100ms delay            | Subhead             | Page load            |
| Hero CTAs            | `ScaleIn` with `spring-bouncy`       | Buttons             | Page load, staggered |
| Hero illustration    | Custom CSS `offset-path` + `ScaleIn` | Nodes + paths       | Page load, staggered |
| "What's in the box"  | `ScaleIn` with `spring`              | Step cards 1→2→3    | Scroll into view     |
| "House specials"     | `Stagger` + `ScaleIn`                | Recipe cards        | Scroll into view     |
| Terminal demo        | `SlideUp`                            | Terminal frame      | Scroll into view     |
| Pitch points         | `SlideUp` stagger                    | Checkmark items     | Scroll into view     |
| Anti-pattern list    | `FadeIn` stagger                     | Strikethrough items | Scroll into view     |
| ".bnto.json" preview | `ScaleIn` with `spring-bouncy`       | Code card           | Scroll into view     |

**Scroll trigger:** Use `IntersectionObserver` with a simple `data-animate` attribute. When an element enters the viewport, add the animation class. CSS-first, no JS animation library.

---

## Nav Comparison: Current vs Proposed vs Inspiration

| Element          | Current (bnto)       | Charm            | Deno                 | Bun                   | Proposed                               |
| ---------------- | -------------------- | ---------------- | -------------------- | --------------------- | -------------------------------------- |
| **Logo**         | `bnto` wordmark      | `charm` wordmark | Deno dino + wordmark | Bun mascot + wordmark | `bnto` wordmark                        |
| **Primary nav**  | Create, Explore, FAQ | Home, Libs, Apps | Products ▾, Docs     | Docs, Blog, Discord   | Explore ▾, Editor (beta), Docs         |
| **Dropdown**     | None                 | None             | Rich mega-menus      | None                  | Explore dropdown (recipes by category) |
| **Primary CTA**  | None                 | None             | None                 | None                  | "Get started" button                   |
| **Social proof** | None                 | Social icons     | None                 | None                  | GitHub star count                      |
| **Footer refs**  | FAQ, Privacy, GitHub | Team, contact    | Comprehensive grid   | Discord, GitHub       | FAQ, Docs, Privacy, GitHub             |

---

## Priority Actions

### Quick Wins (copy changes, no new components)

1. Revise hero subheading — one sentence, use the metaphor
2. Revise section divider labels — "What's in the box", "Open kitchen"
3. Revise pitch points — add personality (ingredients, kitchen, house specials)
4. Revise footer tagline — "Pack. Run. Done." instead of "Task automation, from simple to powerful"
5. Rename "Create (beta)" → "Editor (beta)" in nav

### Medium (component changes)

6. Add scroll-triggered entrance animations to every section (use existing Motorways animation components)
7. Add recipe card stagger cascade on `/explore`
8. Redesign nav — add Explore dropdown, remove FAQ from top nav
9. Add "Build Your Own" section with `.bnto.json` preview and editor link

### Larger (new features)

10. Animated hero — Mini Motorways-style node graph with spring entrances and file flow
11. Rich Explore dropdown mega-menu with recipe + capability browsing
12. Terminal animation that auto-types and shows real output
