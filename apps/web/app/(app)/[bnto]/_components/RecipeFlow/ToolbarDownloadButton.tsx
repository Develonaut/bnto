"use client";

import { Button, DownloadIcon } from "@bnto/ui";
import { useRecipeFlowStore, useRecipeFlowActions } from "../../_stores/recipeFlowContext";

/** Download all button — only visible in step 3, enabled on completion. */
export function ToolbarDownloadButton() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const resolved = useRecipeFlowStore((s) => s.resolvedPhase);
  const actions = useRecipeFlowActions();

  if (step !== 3) return null;

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
