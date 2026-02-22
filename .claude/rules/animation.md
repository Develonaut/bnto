# Animation Standards

## Token Scale

All animation timings use the shared token scale defined in `@theme inline` (CSS) and `apps/web/lib/motion.ts` (JS). Never use raw numbers.

| Token | CSS utility | JS constant | Value | Use case |
|---|---|---|---|---|
| `instant` | `duration-instant` | `durations.instant` | 75ms | Micro-feedback (opacity toggles, icon swaps) |
| `fast` | `duration-fast` | `durations.fast` | 150ms | Panel slides, quick reveals |
| `normal` | `duration-normal` | `durations.normal` | 300ms | Default transitions, fade in/out |
| `slow` | `duration-slow` | `durations.slow` | 500ms | Emphasis animations |
| `slower` | `duration-slower` | `durations.slower` | 1200ms | Cinematic reveals |

Easings:

| Token | CSS utility | JS constant | Value |
|---|---|---|---|
| `out` | `ease-out` | `easings.out` | `cubic-bezier(0.33, 1, 0.68, 1)` |
| `in-out` | `ease-in-out` | `easings.inOut` | `cubic-bezier(0.65, 0, 0.35, 1)` |

## CSS vs. motion Library Decision Tree

```
Can CSS handle it?
  |-- Yes (hover, focus, opacity, transform, color) -> CSS transition/animation
  |     Use Tailwind utilities: transition-*, duration-*, ease-*
  |     Prefix with motion-safe: for accessibility
  |
  +-- No (stagger, spring, layout, AnimatePresence, complex orchestration)
        -> motion library with shared presets
          import { transitions, durations, easings } from "@/lib/motion"
```

**Push simple animations to CSS.** The motion library adds JS overhead. A `scale(1.02)` on hover is CSS's job, not motion's.

### When CSS is right
- Hover/focus/active state changes (opacity, scale, color, shadow)
- Simple enter transitions (fade in via `transition-opacity`)
- Keyframe loops (gradient rotation, color cycling)
- Any animation driven by a pseudo-class

### When motion is right
- `AnimatePresence` for exit animations (unmount transitions)
- Staggered list reveals
- Layout animations (panel width transitions with AnimatePresence)
- Spring physics
- Orchestrated multi-element sequences

## Shared Presets

For motion library consumers, use the shared transition presets:

```typescript
import { transitions, durations, easings } from "@/lib/motion";

// Presets
transitions.reveal   // { duration: 0.3, ease: easings.out }    -- default entrance/exit
transitions.panel    // { duration: 0.15, ease: easings.inOut }  -- quick panel slide
transitions.cinematic // { duration: 1.2, ease: easings.out }   -- slow reveal

// Compose with overrides
transition={{ ...transitions.reveal, delay: 0.1 }}
```

## motion-safe Requirement

**Every animation MUST respect `prefers-reduced-motion`.** No exceptions.

### CSS animations
Prefix with `motion-safe:`:
```tsx
className="motion-safe:transition-opacity motion-safe:duration-normal"
```

### motion library animations
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

## Rules

1. **Use tokens, not raw values.** `duration-normal` not `duration-300`. `durations.normal` not `0.3`.
2. **CSS first.** Only reach for the motion library when CSS can't handle the animation.
3. **Always guard with motion-safe.** Every animation path must have a reduced-motion fallback.
4. **Don't touch vendor animations.** shadcn primitive transitions (dialog, sheet, accordion) keep their defaults.
5. **Shared presets for motion library.** Import from `@/lib/motion`, don't inline `{ duration: 0.3, ease: "easeOut" }`.
