"use client";

import { type ComponentProps } from "react";
import { motion, useReducedMotion } from "motion/react";

import { durations, easings } from "./config";

interface PathDrawProps {
  /** The SVG path `d` attribute. */
  d: string;
  /** Stroke color. */
  stroke?: string;
  /** Stroke width. */
  strokeWidth?: number;
  /** Animation duration (seconds). */
  duration?: number;
  /** Extra delay before the animation starts (seconds). */
  delay?: number;
  /** Additional props forwarded to the motion.path element. */
  pathProps?: Omit<ComponentProps<typeof motion.path>, "d" | "transition">;
}

/**
 * Roads being constructed.
 *
 * SVG path stroke animation — the line draws itself from start to end.
 * Must be used inside an `<svg>` element.
 *
 * ```tsx
 * <svg viewBox="0 0 200 100">
 *   <PathDraw d="M 10 50 Q 100 10 190 50" stroke="currentColor" />
 * </svg>
 * ```
 */
export function PathDraw({
  d,
  stroke = "currentColor",
  strokeWidth = 2,
  duration = durations.slow,
  delay = 0,
  pathProps,
}: PathDrawProps) {
  const prefersReduced = useReducedMotion();

  return (
    <motion.path
      d={d}
      stroke={stroke}
      strokeWidth={strokeWidth}
      fill="none"
      strokeLinecap="round"
      initial={prefersReduced ? undefined : { pathLength: 0, opacity: 0 }}
      animate={prefersReduced ? undefined : { pathLength: 1, opacity: 1 }}
      transition={
        prefersReduced
          ? undefined
          : { duration, ease: easings.out, delay }
      }
      {...pathProps}
    />
  );
}
