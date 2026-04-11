"use client";

import { SlideUp, StepperIndicator } from "@bnto/ui";
import { useRecipeStepperStore } from "../_stores/recipeStepperContext";

/** Shows the stepper indicator only after files have been added. */
export function ConditionalStepperIndicator() {
  const hasFiles = useRecipeStepperStore((s) => s.files.length > 0);
  if (!hasFiles) return null;
  return (
    <SlideUp>
      <StepperIndicator />
    </SlideUp>
  );
}
