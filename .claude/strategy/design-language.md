# Bnto — Design Language

**Created:** February 21, 2026
**Updated:** February 24, 2026

---

## Core Thesis

**Warm, organized, satisfying. Like a well-packed bento box on a Mini Motorways map.**

The visual identity draws from the bento box metaphor (organized compartments, purposeful arrangement) and the clean, warm, geometric aesthetic of Mini Motorways. Warm without being cute, simple without being simplistic.

---

## References

| Reference | What we take from it |
|-----------|---------------------|
| **Mini Motorways** | Warm cream backgrounds, bold-but-muted accent colors, clean geometry, rounded shapes. Buildings popping up, roads drawing themselves — calm, purposeful, satisfying |
| **Japanese bento boxes** | Precision, economy, spatial harmony. Beauty as a byproduct of thoughtful arrangement |
| **shadcn/ui** | Production-grade component substrate our personality layers onto |

---

## Brand Personality

**Wordmark:** `bnto` — all lowercase, always. Four small rounded letters. No capitalization even at sentence start in brand contexts.

**Voice:** Plain language. "It just works." No jargon unless the user asked for it.

| Trait | We are | We're not |
|-------|--------|-----------|
| Temperature | Warm | Cold, clinical |
| Tone | Inviting | Impressive, intimidating |
| Complexity | Simple | Simplistic |
| Feeling | Satisfying | Flashy |
| Motion | Purposeful | Decorative |

---

## Color Philosophy

Three color temperatures on a warm base — like a Mini Motorways map where buildings, water, and roads coexist harmoniously.

| Role | Token | Character |
|------|-------|-----------|
| **Background** | `--background` | Warm cream (light) / cool slate (dark) — the map surface |
| **Primary** | `--primary` | Terracotta — buildings, main interactive color |
| **Secondary** | `--secondary` | Soft teal — water/parks, cool counterpoint |
| **Accent** | `--accent` | Golden yellow — landmarks, highlights |
| **Destructive** | `--destructive` | Warm red — errors, delete actions |

**Rules:** Same hues across modes (lightness shifts, identity stays). Warm shadows in light mode (`hsl(8 19% 15%)`). Cool slate dark mode (subtle blue tint, not neutral gray).

For exact token values: see [theming.md](../rules/theming.md).

---

## Typography

**DM Sans** (display) + **Inter** (body) + **Geist Mono** (code). Loaded via `next/font/google`.

DM Sans: geometric with soft rounding — warm character pairs with terracotta. Inter: universal UI font, clean at all sizes. Geist Mono: for logs, `.bnto.json` previews, node type labels.

For font class usage: see [theming.md](../rules/theming.md).

---

## Shape & Radius

Generously rounded. Default to `rounded-lg` (20px base). Round more for prominent surfaces, tighter for small inline elements. Never sharp corners in brand UI.

For exact radius tokens: see [theming.md](../rules/theming.md).

---

## Animation Language

Inspired by Mini Motorways — buildings popping up, roads drawing themselves, neighborhoods growing. Every animation serves comprehension, never decoration.

**Two motion modes:**
- **Entrances:** Springy, elastic (buildings materializing) — CSS spring curves
- **Transitions:** Smooth, deliberate (state changes, press interactions) — ease-out curves

**CSS-first.** 95% of animations use CSS keyframes + custom properties. `motion/react` reserved for exit animations (`AnimatePresence`), layout animations, and gesture interactions.

For implementation details, tokens, and the `Animate.*` component API: see [animation.md](../rules/animation.md).

---

## Principles

1. **Warm, organized, satisfying** — Every choice should feel like opening a well-packed bento box
2. **Three color temperatures** — Warm (terracotta), cool (teal), golden (accent). Removing any one breaks harmony
3. **Animations serve comprehension** — Show what appeared, what moved, what changed
4. **Composable, not pervasive** — Effects are wrappers. Base components stay clean
5. **Restraint over spectacle** — If you notice the animation consciously, it's too strong
6. **CSS-first** — Simple states in CSS. Motion library for springs, presence, layout only
7. **Round more, not less** — Generous radii. The brand is warm and friendly
8. **The work is the hero** — The workflow, the files, the results are the content
