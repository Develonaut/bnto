/**
 * Animation tokens for motion library consumers.
 *
 * CSS consumers use the Tailwind utilities: duration-instant, duration-fast,
 * duration-normal, duration-slow, duration-slower, ease-out, ease-in-out.
 *
 * JS consumers (motion/framer-motion) import these constants.
 */

/** Duration constants in seconds */
export const durations = {
  instant: 0.075,
  fast: 0.15,
  normal: 0.3,
  slow: 0.5,
  slower: 1.2,
} as const;

/** Cubic-bezier easing curves */
export const easings = {
  out: [0.33, 1, 0.68, 1] as [number, number, number, number],
  inOut: [0.65, 0, 0.35, 1] as [number, number, number, number],
};

/** Transition presets for motion library consumers */
export const transitions = {
  /** Default entrance/exit */
  reveal: { duration: durations.normal, ease: easings.out },
  /** Quick panel slide */
  panel: { duration: durations.fast, ease: easings.inOut },
  /** Slow cinematic reveal */
  cinematic: { duration: durations.slower, ease: easings.out },
} as const;
