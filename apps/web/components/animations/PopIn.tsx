"use client";

import { type ComponentProps, type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { springs } from "./config";

interface PopInProps {
  children: ReactNode;
  /** Extra delay before the animation starts (seconds). */
  delay?: number;
  /** Override the default spring. */
  spring?: ComponentProps<typeof motion.div>["transition"];
  className?: string;
}

/**
 * Buildings appearing on the map.
 *
 * Scales from 0 → 1.05 → 1 with spring overshoot + opacity fade.
 * The signature bnto entrance animation.
 */
export function PopIn({ children, delay = 0, spring, className }: PopInProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.6 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.95 }}
      transition={{
        type: "spring",
        ...springs.snappy,
        delay,
        ...spring,
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
