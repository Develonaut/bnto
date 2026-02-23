"use client";

import { type ReactNode } from "react";
import { motion, useReducedMotion } from "motion/react";

import { staggerDelay } from "./config";

interface StaggerCascadeProps {
  children: ReactNode;
  /** Delay between each child animation (seconds). */
  stagger?: number;
  /** Delay before the first child starts (seconds). */
  delay?: number;
  className?: string;
}

/**
 * Houses appearing in a neighborhood.
 *
 * Parent orchestrates children appearing one after another.
 * Wrap each child in PopIn (or any entrance animation) for
 * the individual element effect.
 *
 * ```tsx
 * <StaggerCascade>
 *   {items.map(item => (
 *     <PopIn key={item.id}>
 *       <Card>{item.name}</Card>
 *     </PopIn>
 *   ))}
 * </StaggerCascade>
 * ```
 */
export function StaggerCascade({
  children,
  stagger = staggerDelay,
  delay = 0,
  className,
}: StaggerCascadeProps) {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  return (
    <motion.div
      initial="hidden"
      animate="visible"
      exit="hidden"
      variants={{
        hidden: {},
        visible: {
          transition: {
            staggerChildren: stagger,
            delayChildren: delay,
          },
        },
      }}
      className={className}
    >
      {children}
    </motion.div>
  );
}
