"use client";

import { type ReactNode } from "react";
import { AnimatePresence } from "motion/react";

type PresenceMode = "sync" | "wait" | "popLayout";

interface PresenceGateProps {
  children: ReactNode;
  /** How entering and exiting elements interact. */
  mode?: PresenceMode;
  /** Fire onExitComplete when all exiting elements are removed. */
  onExitComplete?: () => void;
}

/**
 * AnimatePresence wrapper for conditional content.
 *
 * Enables exit animations on children that unmount.
 * Wrap any conditionally rendered animated content with this.
 *
 * ```tsx
 * <PresenceGate>
 *   {isVisible && (
 *     <PopIn key="modal">
 *       <Modal />
 *     </PopIn>
 *   )}
 * </PresenceGate>
 * ```
 */
export function PresenceGate({
  children,
  mode = "sync",
  onExitComplete,
}: PresenceGateProps) {
  return (
    <AnimatePresence mode={mode} onExitComplete={onExitComplete}>
      {children}
    </AnimatePresence>
  );
}
