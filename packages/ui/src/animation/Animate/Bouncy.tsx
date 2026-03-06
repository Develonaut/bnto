"use client";

import type { ReactNode } from "react";

import { motion, useReducedMotion, type Transition } from "motion/react";

interface BouncyProps {
  /** Starting scale (0-1). Default 0.7. */
  from?: number;
  /** Spring stiffness. Default 400. */
  stiffness?: number;
  /** Spring damping. Lower = bouncier. Default 17. */
  damping?: number;
  className?: string;
  children?: ReactNode;
}

const Bouncy = ({
  from = 0.7,
  stiffness = 400,
  damping = 17,
  className,
  children,
}: BouncyProps) => {
  const prefersReduced = useReducedMotion();

  if (prefersReduced) {
    return <div className={className}>{children}</div>;
  }

  const transition: Transition = {
    type: "spring",
    stiffness,
    damping,
  };

  return (
    <motion.div
      className={className}
      initial={{ scale: from, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      transition={transition}
    >
      {children}
    </motion.div>
  );
};
Bouncy.displayName = "Animate.Bouncy";

export { Bouncy };
export type { BouncyProps };
