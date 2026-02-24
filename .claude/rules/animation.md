# Animation Standards

## Mini Motorways Motion Language

Two distinct motion modes inspired by Mini Motorways:

| Mode | Quality | Easing | Example |
|---|---|---|---|
| **Entrance** | Springy, elastic, playful | Spring curves (`linear()`) | Cards popping onto the grid, list items revealing |
| **Transition** | Smooth, deliberate, restrained | Ease-out (`cubic-bezier`) | Panel slides, state changes, color shifts, press interactions |
| **Emphasis** | Gentle pulse, no bounce | Ease-in-out | Demand timers pulsing, idle ambient presence |

**The insight:** When things *appear*, they have a satisfying springy pop -- like a building materializing on the map. Once present, all motion is smooth and calm. Entrances are playful; everything after is serene.

---

## Token Scale

All animation timings use CSS custom properties defined in `globals.mini-motorways-depth.css` and registered in `@theme inline`.

### Durations

| Token | CSS utility | Value | Use case |
|---|---|---|---|
| `instant` | `duration-instant` | 75ms | Micro-feedback (opacity toggles, icon swaps) |
| `fast` | `duration-fast` | 150ms | Panel slides, quick reveals, press interactions |
| `normal` | `duration-normal` | 300ms | Default transitions, fade in/out |
| `slow` | `duration-slow` | 500ms | Spring entrance animations |
| `slower` | `duration-slower` | 1200ms | Cinematic reveals |

### Easings: Smooth (transitions)

| Token | CSS utility | Value |
|---|---|---|
| `out` | `ease-out` | `cubic-bezier(0.22, 1, 0.36, 1)` |
| `in-out` | `ease-in-out` | `cubic-bezier(0.65, 0, 0.35, 1)` |

### Easings: Springs (entrances)

| Token | CSS utility | Character |
|---|---|---|
| `spring` | `ease-spring` | Barely perceptible settle -- default for entrance animations |
| `spring-bouncy` | `ease-spring-bouncy` | Noticeable elastic overshoot -- playful pop-in |
| `spring-bouncier` | `ease-spring-bouncier` | Pronounced bounce -- special emphasis moments |

Spring curves use CSS `linear()` with values extracted from `tailwindcss-motion`.

---

## Animation Classes

All entrance animations use `animation-fill-mode: both` so elements are invisible before the animation starts and retain their final state after.

| Class | Easing | Duration | Use Case |
|---|---|---|---|
| `animate-fade-in` | ease-out | 300ms | Overlays, subtle reveals |
| `animate-scale-in` | spring | 500ms | Cards, panels, buildings materializing |
| `animate-slide-up` | spring | 500ms | Content reveals, list items |
| `animate-slide-down` | spring | 500ms | Dropdowns, menus |
| `animate-pulse-soft` | ease-in-out | 2s infinite | Attention/urgency signals |
| `animate-breathe` | ease-in-out | 3s infinite | Ambient idle presence |

### Configurable per-element

| Variable | Default | Override example |
|---|---|---|
| `--scale-from` | `0.9` | `0.6` for dramatic hero pop-in |
| `--slide-distance` | `8px` | `16px` for larger sections |
| `--stagger-index` | `0` | Set via `style` in JSX for cascade position |
| `--stagger-interval` | `60ms` | Adjust cascade gap |

```tsx
// Standard card entrance
<Card className="motion-safe:animate-scale-in" />

// Hero element -- more dramatic spring
<HeroCard className="motion-safe:animate-scale-in"
          style={{ '--scale-from': '0.6' }} />

// Override easing for bouncier feel
<Badge className="motion-safe:animate-scale-in"
       style={{ animationTimingFunction: 'var(--ease-spring-bouncy)' }} />
```

---

## CSS vs. motion Library Decision Tree

```
Can CSS handle it?
  |-- Yes (entrance, hover, focus, opacity, transform, color, stagger)
  |     -> CSS animation/transition in globals.mini-motorways-depth.css
  |     Prefix with motion-safe: for accessibility
  |
  +-- No (exit on unmount, layout animations, gesture-based interaction)
        -> motion/react (AnimatePresence, layout prop, useSpring)
```

**CSS handles 95% of animations.** `motion/react` stays in the dependency tree for the cases CSS can't do.

### When CSS is right
- Entrance animations (fade, scale, slide -- use the animation classes)
- Hover/focus/active state changes (opacity, scale, color, shadow)
- Staggered list reveals (stagger cascade utility)
- Emphasis loops (pulse, breathe)
- Any animation driven by a pseudo-class
- The `.depth` + `.pressable` system (already CSS)

### When motion/react is right
- `AnimatePresence` for exit animations (unmount transitions -- CSS has no concept of "animate out before removing from DOM")
- Layout animations (`layout` prop -- smooth position/size changes when DOM reflows)
- Gesture-based interactions (drag, swipe, pinch)
- Complex orchestrated sequences that depend on JS state

---

## Stagger Cascade

Container class that staggers children's animation-delay using a CSS variable.

```tsx
<div className="stagger-cascade">
  {items.map((item, i) => (
    <div key={item.id}
         style={{ '--stagger-index': i } as React.CSSProperties}
         className="motion-safe:animate-slide-up">
      ...
    </div>
  ))}
</div>
```

Override the interval: `style={{ '--stagger-interval': '80ms' }}` on the container.

Reduced motion automatically zeroes all stagger delays.

---

## motion-safe Requirement

**Every animation MUST respect `prefers-reduced-motion`.** No exceptions.

### CSS animations
Prefix with `motion-safe:`:
```tsx
className="motion-safe:animate-scale-in"
```

### motion/react animations
Use `useReducedMotion()` from `motion/react` and render a static fallback:
```tsx
const prefersReduced = useReducedMotion();
if (prefersReduced) return <div>{children}</div>;
return <motion.div animate={...}>{children}</motion.div>;
```

### JS-driven animations (setTimeout, RAF)
Check `matchMedia` and skip the animation:
```tsx
const prefersReduced =
  typeof window !== "undefined" &&
  window.matchMedia("(prefers-reduced-motion: reduce)").matches;
```

---

## Component API (`Animate.*`)

**Props over classNames.** Consumers outside `components/ui/` should never touch `motion-safe:animate-*` classes or `--scale-from` / `--stagger-index` CSS variables directly. Use the `Animate` composition components instead.

```tsx
import { Animate } from "@/components/ui/animate";

// Stagger cascade with springy cards
<Animate.Stagger className="grid grid-cols-3 gap-4">
  {items.map((item, i) => (
    <Animate.ScaleIn key={item.id} index={i} from={0.85}>
      <Card>...</Card>
    </Animate.ScaleIn>
  ))}
</Animate.Stagger>

// Hero pop-in with bouncy spring
<Animate.ScaleIn from={0.5} easing="spring-bouncier">
  <Card>Hero Card</Card>
</Animate.ScaleIn>
```

### Components

| Component | Extra Props | Underlying CSS |
|---|---|---|
| `Animate.Stagger` | `interval?: number \| string` | `stagger-cascade`, `--stagger-interval` |
| `Animate.ScaleIn` | `from?: number` | `motion-safe:animate-scale-in`, `--scale-from` |
| `Animate.FadeIn` | -- | `motion-safe:animate-fade-in` |
| `Animate.SlideUp` | `distance?: number \| string` | `motion-safe:animate-slide-up`, `--slide-distance` |
| `Animate.SlideDown` | `distance?: number \| string` | `motion-safe:animate-slide-down`, `--slide-distance` |
| `Animate.PulseSoft` | -- | `motion-safe:animate-pulse-soft` |
| `Animate.Breathe` | -- | `motion-safe:animate-breathe` |

### Shared Props (entrance components)

| Prop | Type | Description |
|---|---|---|
| `index` | `number` | Position in stagger cascade. Sets `--stagger-index` |
| `easing` | `"spring" \| "spring-bouncy" \| "spring-bouncier" \| "out" \| "in-out"` | Override animation easing |
| `asChild` | `boolean` | Merge onto child element instead of wrapping in a `<div>` |
| `className` | `string` | Additional classes |
| `style` | `CSSProperties` | Additional inline styles |

### Depth caveat

By default, `Animate.*` components render a wrapper `<div>`. This is intentional -- opacity animation flattens `transform-style: preserve-3d`, which breaks the `.depth` 3D effect on Cards. The wrapper isolates the animation from the depth element.

Use `asChild` only when the child does NOT use `.depth`:

```tsx
// GOOD -- wrapper div isolates animation from depth
<Animate.ScaleIn from={0.85} index={i}>
  <Card>...</Card>
</Animate.ScaleIn>

// GOOD -- asChild on non-depth element
<Animate.FadeIn asChild>
  <span>No depth here</span>
</Animate.FadeIn>

// BAD -- asChild merges opacity animation onto depth element
<Animate.ScaleIn asChild>
  <Card>Depth will break!</Card>
</Animate.ScaleIn>
```

---

## Rules

1. **Use tokens, not raw values.** `duration-normal` not `duration-300`. `var(--ease-spring)` not inline `linear(...)`.
2. **Use `Animate.*` components, not raw classes.** Consumers outside `components/ui/` should use `<Animate.ScaleIn>` not `className="motion-safe:animate-scale-in"`.
3. **CSS first.** Only reach for `motion/react` when CSS can't handle the animation (exit, layout, gesture).
4. **Always guard with motion-safe.** Every animation path must have a reduced-motion fallback. `Animate.*` components handle this automatically.
5. **Don't touch vendor animations.** shadcn primitive transitions (dialog, sheet, accordion) keep their defaults.
6. **Entrances are springy, transitions are smooth.** Use `--ease-spring` for things appearing. Use `--ease-out` for state changes.
7. **Compositor-only properties.** Animate `opacity`, `scale`, `translate`, `rotate` only. Never animate `width`, `height`, `top`, `left`, `margin`, `padding`.
