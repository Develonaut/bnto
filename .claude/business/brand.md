# Brand & Domain Decision

**Decision Date:** February 2026
**Status:** Final (brand name & domain) / In Progress (visual identity)
**Previously:** Notion — "Brand & Domain Decision"

---

## Brand Name

**bnto**

- Pronounced "bento" (like the Japanese lunch box)
- Always written as `bnto` — all lowercase, always. Four small rounded letters. No capitalization, no camel case, not even at the start of a sentence in brand contexts.
- The lowercase is intentional and part of the brand. It signals warmth and approachability before the user reads a single word of copy.
- Lowercase `bnto` everywhere: wordmark, repo names, package names, CLI binary, copy, marketing

---

## Domain

| Domain | Purpose |
|---|---|
| **bnto.io** | Primary platform URL |

---

## GitHub

| Setting | Value |
|---|---|
| **Organization** | `Develonaut` |
| **Repo** | `bnto` (monorepo) |
| **Package prefix** | `@bnto/` |
| **Go module** | `github.com/Develonaut/bnto` |

---

## The bnto Metaphor

A bento box (弁当) is a Japanese lunch container with carefully organized compartments. Each section has one purpose, holds one thing, and fits together into a complete meal.

This metaphor runs deep through the project:

- **Workflow nodes** are compartments — each does one thing well
- **The .bnto.json file** is the box — portable, organized, complete
- **Boxes stack** — a flow can contain other flows, a box can contain other boxes. One compartment is still a bento box. The mental model never breaks.
- **The codebase** follows the Bento Box Principle — single responsibility, no grab bags, composable pieces

---

## Brand Personality

bnto should feel like the product: approachable, calm, and capable. The kind of tool that makes you feel like you know what you're doing — even if you've never automated anything before.

**Warm, not cold.** This is not a developer tool that tolerates non-technical users. It's a tool for everyone that also happens to be powerful under the hood.

**Inviting, not impressive.** We're not trying to signal enterprise credibility. We're trying to make someone feel like they can just open it and go.

**Simple, not simplistic.** Clean and uncluttered. Nothing that doesn't need to be there. But never dumbed down.

**Satisfying, not flashy.** Like a well-packed lunch box — everything in its place, nothing wasted, quietly delightful.

---

## Visual Identity Direction

### Primary Reference: Mini Motorways

The closest visual reference is the opening level of **Mini Motorways** — warm cream base, soft muted colors, rounded shapes, nothing harsh or sharp. It doesn't look like enterprise software. It looks like something you'd want to spend time with.

Key qualities:
- **Warm cream/off-white as the base** — not stark white, not dark mode gray. Something that feels like paper, like a workspace.
- **Soft, muted color palette** — terracotta, sage, warm sand, soft indigo. Nothing neon, nothing clinical.
- **Rounded, friendly shapes** — nodes, cards, buttons, containers. Everything has a gentle radius.
- **Calm spatial layout** — breathing room. Whitespace is intentional.
- **Tactile and real** — the visual language should feel like something physical. A bento box you could hold.

### What to Avoid

- Cold, sterile developer-tool aesthetics (dark mode everything, monospace everywhere)
- Enterprise SaaS chrome (dense tables, aggressive CTAs, trust badges)
- Overly playful or cartoon-like (we're still a serious tool)
- Harsh contrast or aggressive typography

### Typography Direction

- Warm, humanist sans-serif for UI and marketing — reads like a friendly voice, not a manual
- Monospace only where code appears — contextual, not dominant
- Weight hierarchy should guide, not shout

### Motion & Interaction

- Transitions should feel smooth and calm — nothing snappy or jarring
- Running a flow should feel satisfying — like watching compartments fill, not watching a progress bar spin
- Errors should feel gentle and helpful, not alarming

### UI Component Stack (Technical)

- **shadcn/ui** — base component library. All core UI elements come from shadcn.
- **Theming** — complete. Full design system defined in `.claude/rules/theming.md`. Key decisions:
  - **Font:** Geist (display/headings) + Inter (body) + Geist Mono (code) via `next/font/google`
  - **Background:** Warm cream `oklch(0.9899 0.0164 95.22)`
  - **Primary:** Terracotta `oklch(0.6751 0.1788 35.19)`
  - **Accent:** Golden yellow `oklch(0.8885 0.1338 91.06)`
  - **Border radius:** 1.25rem base — generously rounded everywhere
  - **Shadows:** Warm-tinted, never cold or gray

---

## Voice & Tone

**Plain language, always.** If a sentence requires technical knowledge to parse, rewrite it.

**Encouraging, not instructional.** "Drop your files and run" not "Initialize a workflow execution with the following parameters."

**Honest about simplicity.** We don't oversell. If something takes two steps, we say two steps.

**North star phrase: "It just works."** Every copy decision, every UI label, every onboarding step should reinforce that feeling.

---

## Naming Convention

| Context | Format | Example |
|---|---|---|
| Brand/marketing | `bnto` | "Built with bnto" |
| CLI binary | `bnto` | `bnto run workflow.bnto.json` |
| Workflow files | `.bnto.json` | `resize-images.bnto.json` |
| npm packages | `@bnto/` | `@bnto/core`, `@bnto/ui` |
| Go module | `github.com/Develonaut/bnto` | `import bnto "github.com/Develonaut/bnto"` |
| Internal code | `bnto` (lowercase) | `bntoService`, `bntoConfig` |

---

*The brand should make anyone — a designer, a developer, a solo founder — feel like bnto was built for them. Because it was.*
