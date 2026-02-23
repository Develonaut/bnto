"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { durations, easings } from "./config";

interface ExitDissolveProps {
  children: ReactNode;
  /** Exit animation duration (seconds). */
  duration?: number;
  className?: string;
}

/**
 * Removing a road.
 *
 * Gentle opacity + scale-down exit animation.
 * Less dramatic than PopIn's reverse — for non-emphatic removal.
 */
export function ExitDissolve({
  children,
  duration = durations.fast,
  className,
}: ExitDissolveProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      initial={{ opacity: 1, scale: 1 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.98 }}
      transition={{ duration, ease: easings.out }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
