"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { springs } from "./config";

interface LayoutShiftProps {
  children: ReactNode;
  /** Spring transition override. */
  spring?: { stiffness?: number; damping?: number };
  className?: string;
}

/**
 * Rearranging the map.
 *
 * Wrapper that enables automatic position and size animations
 * when the element's layout changes (reordering, expanding, collapsing).
 * Uses Motion's layout animation engine.
 */
export function LayoutShift({
  children,
  spring,
  className,
}: LayoutShiftProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      layout
      transition={{
        type: "spring",
        ...springs.smooth,
        ...spring,
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
