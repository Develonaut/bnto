"use client";

import type { ReactNode } from "react";

interface RecipeStepperToolbarProps {
  children: ReactNode;
}

/**
 * Responsive toolbar container for Steps 2-3.
 *
 * Mobile: wraps to two rows. Desktop: single row with left/center/right sections.
 */
export function RecipeStepperToolbar({ children }: RecipeStepperToolbarProps) {
  return (
    <div
      role="toolbar"
      aria-label="Recipe actions"
      className="flex min-h-[4.5rem] flex-wrap items-center gap-3 md:flex-nowrap md:gap-4"
    >
      {children}
    </div>
  );
}
