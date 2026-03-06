"use client";

import type { ReactNode } from "react";

import { AnimatePresence, motion, type Transition } from "motion/react";

/**
 * Animates a keyed list with enter, exit, and layout transitions.
 *
 * Uses motion/react for the parts CSS can't do:
 *   - AnimatePresence delays unmount until exit animation completes
 *   - layout prop FLIP-animates neighbors when items enter/exit
 *   - Spring physics create the physical "push/pull" on neighbors
 *
 * CSS still drives entrance animations via the render callback
 * (ScaleIn, etc) for non-list contexts. This component owns
 * enter + exit + layout for list items.
 */

interface PresenceListProps {
  /** Current active keys. Items removed from this array will exit-animate. */
  keys: string[];
  /** Render function called for each visible item. */
  children: (key: string) => ReactNode;
}

// Bouncy spring — matches the spring-bouncier CSS curve feel.
// Low damping = pronounced overshoot on enter + layout shifts.
const enterTransition: Transition = {
  type: "spring",
  stiffness: 400,
  damping: 17,
};

// Exit is snappy tween — quick pop-up then collapse to nothing.
// Spring exit felt too subtle; a fast tween with keyframes gives
// the dramatic "pop off the map" feel.
const exitTransition: Transition = {
  type: "tween",
  duration: 0.25,
  ease: [0.22, 1, 0.36, 1], // ease-out
};

// Snappier spring for layout shifts (neighbors moving)
const layoutTransition: Transition = {
  type: "spring",
  stiffness: 400,
  damping: 22,
};

export function PresenceList({ keys, children }: PresenceListProps) {
  return (
    <AnimatePresence mode="popLayout">
      {keys.map((key) => (
        <motion.div
          key={key}
          layout
          initial={{ scale: 0.7, opacity: 0 }}
          animate={{ scale: 1, opacity: 1, transition: enterTransition }}
          exit={{
            scale: [1, 1.1, 0],
            opacity: [1, 1, 0],
            transition: exitTransition,
          }}
          transition={layoutTransition}
        >
          {children(key)}
        </motion.div>
      ))}
    </AnimatePresence>
  );
}
