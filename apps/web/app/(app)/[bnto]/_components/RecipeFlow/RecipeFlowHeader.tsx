"use client";

import type { ReactNode } from "react";
import { StepperContent } from "@bnto/ui";
import { useRecipeFlowStore } from "../../_stores/recipeFlowContext";

interface RecipeFlowHeaderProps {
  children: ReactNode;
}

/**
 * Responsive toolbar for Steps 2-3.
 *
 * Wraps in StepperContent for the active step. Mobile: wraps to two rows.
 * Desktop: single row with left/center/right sections.
 */
export function RecipeFlowHeader({ children }: RecipeFlowHeaderProps) {
  const step = useRecipeFlowStore((s) => s.activeStep);
  if (step < 2) return null;

  return (
    <StepperContent value={String(step)}>
      <div
        role="toolbar"
        aria-label="Recipe actions"
        className="flex min-h-[4.5rem] flex-wrap items-center gap-3 md:flex-nowrap md:gap-4"
      >
        {children}
      </div>
    </StepperContent>
  );
}
