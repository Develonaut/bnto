"use client";

import type { ReactNode } from "react";

import { AnimatePresence, motion } from "motion/react";

/**
 * Delays unmount of a single child until its exit animation completes.
 * Thin wrapper around motion/react's AnimatePresence for single elements.
 */

interface PresenceProps {
  /** Whether the child should be visible. */
  visible: boolean;
  /** Called after the exit animation finishes. */
  onExitComplete?: () => void;
  children: ReactNode;
}

export function Presence({ visible, onExitComplete, children }: PresenceProps) {
  return (
    <AnimatePresence onExitComplete={onExitComplete}>
      {visible && (
        <motion.div
          initial={{ scale: 0.9, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          exit={{ scale: 0.9, opacity: 0 }}
          transition={{ type: "spring", stiffness: 350, damping: 40 }}
        >
          {children}
        </motion.div>
      )}
    </AnimatePresence>
  );
}
