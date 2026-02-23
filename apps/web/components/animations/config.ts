/**
 * Shared animation constants for Mini Motorways-inspired motion.
 *
 * All animation primitives import from here — never inline raw spring
 * values or durations in components.
 */

/** Spring presets — physics-based, interruptible, natural-feeling. */
export const springs = {
  /** Buildings popping up — quick with slight overshoot. */
  snappy: { stiffness: 400, damping: 25 },
  /** Roads drawing in — purposeful, less bounce. */
  smooth: { stiffness: 300, damping: 30 },
  /** Soft reveals — slow settle. */
  gentle: { stiffness: 200, damping: 20 },
} as const;

/** Duration presets (seconds) for non-spring animations. */
export const durations = {
  fast: 0.15,
  normal: 0.3,
  slow: 0.5,
} as const;

/** Easing curves for CSS-driven or tween-based animations. */
export const easings = {
  out: [0.33, 1, 0.68, 1] as const,
  inOut: [0.65, 0, 0.35, 1] as const,
} as const;

/** Default stagger delay between children (seconds). */
export const staggerDelay = 0.06;
