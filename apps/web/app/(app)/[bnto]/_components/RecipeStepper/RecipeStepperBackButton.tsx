"use client";

import { Button, ArrowLeftIcon } from "@bnto/ui";
import { useRecipeStepperStore, useRecipeStepperActions } from "../../_stores/recipeStepperContext";

/** Back button — context-aware label based on active step. */
export function RecipeStepperBackButton() {
  const step = useRecipeStepperStore((s) => s.activeStep);
  const isProcessing = useRecipeStepperStore((s) => s.isProcessing);
  const actions = useRecipeStepperActions();

  const label = step === 3 ? "Back to configure" : "Back to file selection";

  return (
    <Button
      variant="ghost"
      size="icon"
      elevation="sm"
      disabled={isProcessing}
      onClick={actions.back}
      aria-label={label}
      data-testid="back-button"
    >
      <ArrowLeftIcon className="size-4" />
    </Button>
  );
}
