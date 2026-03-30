"use client";

import { Button, DownloadIcon } from "@bnto/ui";
import { useRecipeStepperStore, useRecipeStepperActions } from "../../_stores/recipeStepperContext";

/** Download all button — enabled on completion. */
export function DownloadAllButton() {
  const resolved = useRecipeStepperStore((s) => s.resolvedStep);
  const actions = useRecipeStepperActions();

  return (
    <Button
      variant="outline"
      size="icon"
      elevation="sm"
      disabled={resolved !== "completed"}
      onClick={actions.downloadAll}
      aria-label="Download all"
      data-testid="download-all-button"
    >
      <DownloadIcon className="size-4" />
    </Button>
  );
}
