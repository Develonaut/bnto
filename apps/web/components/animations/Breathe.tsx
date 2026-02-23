"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { easings } from "./config";

interface BreatheProps {
  children: ReactNode;
  /** Peak scale factor. */
  scale?: number;
  /** Full cycle duration (seconds). */
  duration?: number;
  className?: string;
}

/**
 * Destinations needing connection.
 *
 * Gentle scale oscillation that draws attention without aggression.
 * Infinite loop — use for status indicators, "needs action" elements.
 */
export function Breathe({
  children,
  scale = 1.04,
  duration = 2,
  className,
}: BreatheProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      animate={{ scale: [1, scale, 1] }}
      transition={{
        duration,
        ease: easings.inOut,
        repeat: Infinity,
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
