"use client";

import { Button, ArrowLeftIcon } from "@bnto/ui";
import { useRecipeFlowStore, useRecipeFlowActions } from "../../_stores/recipeFlowContext";

/** Back button — context-aware label based on active step. */
export function RecipeFlowBackButton() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const isProcessing = useRecipeFlowStore((s) => s.isProcessing);
  const actions = useRecipeFlowActions();

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
