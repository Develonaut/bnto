# Bnto — Design Language

**Created:** February 21, 2026
**Updated:** February 24, 2026

---

## Core Thesis

**Warm, organized, satisfying. Like a well-packed bento box on a Mini Motorways map.**

The visual identity draws from the bento box metaphor (organized compartments, purposeful arrangement) and the clean, warm, geometric aesthetic of Mini Motorways. Warm without being cute, simple without being simplistic.

---

## References

| Reference                | What we take from it                                                                                                                                                |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Mini Motorways**       | Warm cream backgrounds, bold-but-muted accent colors, clean geometry, rounded shapes. Buildings popping up, roads drawing themselves — calm, purposeful, satisfying |
| **Japanese bento boxes** | Precision, economy, spatial harmony. Beauty as a byproduct of thoughtful arrangement                                                                                |
| **shadcn/ui**            | Production-grade component substrate our personality layers onto                                                                                                    |

---

## Brand Personality

**Wordmark:** `bnto` — all lowercase, always. Four small rounded letters. No capitalization even at sentence start in brand contexts.

**Voice:** Plain language. "It just works." No jargon unless the user asked for it.

| Trait       | We are     | We're not                |
| ----------- | ---------- | ------------------------ |
| Temperature | Warm       | Cold, clinical           |
| Tone        | Inviting   | Impressive, intimidating |
| Complexity  | Simple     | Simplistic               |
| Feeling     | Satisfying | Flashy                   |
| Motion      | Purposeful | Decorative               |

---

## Color Philosophy

Three color temperatures on a warm base — like a Mini Motorways map where buildings, water, and roads coexist harmoniously.

| Role            | Token           | Character                                                |
| --------------- | --------------- | -------------------------------------------------------- |
| **Background**  | `--background`  | Warm cream (light) / cool slate (dark) — the map surface |
| **Primary**     | `--primary`     | Terracotta — buildings, main interactive color           |
| **Secondary**   | `--secondary`   | Soft teal — water/parks, cool counterpoint               |
| **Accent**      | `--accent`      | Golden yellow — landmarks, highlights                    |
| **Destructive** | `--destructive` | Warm red — errors, delete actions                        |

**Rules:** Same hues across modes (lightness shifts, identity stays). Warm shadows in light mode (`hsl(8 19% 15%)`). Cool slate dark mode (subtle blue tint, not neutral gray).

For exact token values: see [theming.md](../rules/theming.md).

---

## Typography

**Geist** (display) + **Inter** (body) + **Geist Mono** (code). Loaded via `next/font/google`.

Geist: clean geometric sans-serif — modern character pairs with the warm palette. Inter: universal UI font, clean at all sizes. Geist Mono: for logs, `.bnto.json` previews, node type labels.

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

## Editor Animation Language (The Mini Motorways Moment)

The editor's animation identity is drawn _directly_ from Mini Motorways gameplay. This is not metaphor — it is a literal design reference. The game's animations are what make it feel satisfying. We replicate those exact beats:

### Node Entrance: The Building Pop-In ⚠️ PROTECTED — DO NOT REMOVE

This is the single most important animation in the editor. It is the visual identity of Bnto.

In Mini Motorways, when a building appears on the map, it does a quick scale-up spring — fast in, slight overshoot, settle. It feels like it _materialized from the ground_. The entire satisfaction of placing a new building comes from this moment. **Every node added to the Bnto canvas must do exactly this.**

```tsx
// CompartmentNode.tsx — THIS WRAPPER IS NON-NEGOTIABLE
<ScaleIn from={0.7} easing="spring-bouncy">
  <Pressable asChild spring="bounciest" ...>
    <Card ...>
      {/* node content */}
    </Card>
  </Pressable>
</ScaleIn>
```

**What `ScaleIn from={0.7} easing="spring-bouncy"` does:**

- Scales the node in from 70% → 100% on mount
- Spring easing means it slightly overshoots 100% then settles — the "pop"
- Duration: ~250–300ms total, felt as instant but satisfying
- Combined with `spring="bounciest"` on `Pressable`, every subsequent press echoes that same spring feel

**Why this cannot be removed or weakened:**

- The springy buttons are why users describe the site as feeling "alive" and "fun"
- The node pop-in is the direct Mini Motorways analog — this is not metaphor, it's intentional design theft from a game whose feel we want to replicate
- A fade-in would feel like a web page loading. A linear scale would feel like software. A spring pop feels like a building appearing on a map.
- Users adding nodes to a canvas is the core creative action in the editor. It should feel _good_ every time.

**If you are an agent reading this:** Do not remove `ScaleIn`, do not replace it with `FadeIn`, do not flatten `spring-bouncy` to `ease-out`. If you need to refactor `CompartmentNode.tsx` for any reason, verify that the `ScaleIn` wrapper survives the refactor. This is load-bearing UI.

### Button Press: Springy, Tactile — Core Brand Feel

Every button on the site uses `spring="bounciest"` through the `Pressable` system. The springy, elastic press feel is **not decoration** — it is a core part of why the interface feels alive, warm, and satisfying. This is a deliberate match to the Mini Motorways building-tap feel.

The site's visual identity is: **warm cream + terracotta + springy buttons**. Remove any of these three and it stops feeling like Bnto.

**Rule:** All interactive surfaces (`Button`, `Pressable`, toolbar controls, node cards) keep their spring animations. No agent should flatten these to standard CSS transitions or `ease-out` curves.

### Execution Elevation Sequence: Traffic Flowing

When a recipe runs, nodes animate through an elevation sequence:

| Status      | Elevation            | Metaphor                           |
| ----------- | -------------------- | ---------------------------------- |
| `idle`      | `sm`                 | Building sitting on the map        |
| `pending`   | `sm` (muted)         | Building acknowledged, waiting     |
| `active`    | `md` → `lg` (spring) | Building processing — rising       |
| `completed` | `lg` (hold)          | Building done — fully materialized |

This is the "traffic flowing through the map" moment. Nodes pop up in sequence as execution progresses, exactly like Mini Motorways buildings lighting up as cars pass through. **This is the signature UX moment of the editor.** It must be implemented in Sprint 5 Wave 3 and must use spring animation for the elevation change (not a CSS transition).

### Edit ↔ Run Mode Transition

When the editor enters run mode (Sprint 6), the canvas grid fades and panels slide out. The transition should feel like pressing "Play" in Mini Motorways — the editing scaffolding disappears and the simulation begins. Grid fade: CSS `opacity` transition (~300ms ease-out). Panel slide-out: `Animate.FadeIn` reversed or `AnimatePresence` exit.

### Node Deletion

When a node is deleted via the hover overlay × button, the node should exit with a quick scale-down spring — the reverse of the entrance. Not just a DOM removal. `AnimatePresence` + `ScaleIn` in reverse (scale to 0.7, opacity 0).

### What We Do NOT Do

- No fade-only entrances for nodes. Fade alone feels like a webpage loading, not a building appearing.
- No linear scale animations. Spring overshoot is essential — it's what makes it feel physical.
- No instant DOM adds without animation. Every node addition must have an entrance.
- No decorative animations that don't communicate state change. Every motion has a reason.

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
