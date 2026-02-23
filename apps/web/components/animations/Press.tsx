"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { durations, easings } from "./config";

interface PressProps {
  children: ReactNode;
  /** Vertical distance the element "sinks" on hover (px). */
  depth?: number;
  className?: string;
}

/**
 * Pressing a building into the map.
 *
 * On hover, the element's shadow collapses and it translates down —
 * like pushing it flush with the surface. On active/click, fully flush.
 *
 * The shadow removal creates a tactile "pressed in" feeling that
 * pairs with the warm elevated shadows in the theme.
 */
export function Press({
  children,
  depth = 2,
  className,
}: PressProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      whileHover={{
        y: depth,
        boxShadow: "0 0 0 0 transparent",
        transition: { duration: durations.fast, ease: easings.out },
      }}
      whileTap={{
        y: depth + 1,
        scale: 0.99,
        boxShadow: "0 0 0 0 transparent",
        transition: { duration: durations.fast / 2, ease: easings.out },
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
