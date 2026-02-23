"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { springs } from "./config";

type Direction = "top" | "bottom" | "left" | "right";

const offsets: Record<Direction, { x?: number; y?: number }> = {
  top: { y: -24 },
  bottom: { y: 24 },
  left: { x: -24 },
  right: { x: 24 },
};

interface SlideInProps {
  children: ReactNode;
  /** Direction the element slides in from. */
  from?: Direction;
  /** Offset distance in pixels. */
  offset?: number;
  /** Extra delay before the animation starts (seconds). */
  delay?: number;
  className?: string;
}

/**
 * Roads drawing into place.
 *
 * Slides from an offset position and decelerates to rest.
 * Purposeful movement that commits to a position.
 */
export function SlideIn({
  children,
  from = "bottom",
  offset,
  delay = 0,
  className,
}: SlideInProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  const defaultOffset = offsets[from];
  const initial = {
    opacity: 0,
    x: offset !== undefined ? (from === "left" ? -offset : from === "right" ? offset : 0) : (defaultOffset.x ?? 0),
    y: offset !== undefined ? (from === "top" ? -offset : from === "bottom" ? offset : 0) : (defaultOffset.y ?? 0),
  };

  return (
    <motion.div
      initial={initial}
      animate={{ opacity: 1, x: 0, y: 0 }}
      exit={{
        opacity: 0,
        x: initial.x * 0.5,
        y: initial.y * 0.5,
      }}
      transition={{
        type: "spring",
        ...springs.smooth,
        delay,
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
