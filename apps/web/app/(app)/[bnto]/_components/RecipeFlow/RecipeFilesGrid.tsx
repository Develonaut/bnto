"use client";

import { useCallback } from "react";
import { core } from "@bnto/core";
import { StepperContent } from "@bnto/ui";
import {
  useRecipeFlowStore,
  useRecipeFlowInstance,
  useRecipeFlowActions,
} from "../../_stores/recipeFlowContext";
import { RecipeFileGrid } from "../RecipeFileGrid";

/** File grid for Steps 2-3 — reads from the context store. */
export function RecipeFilesGrid() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const files = useRecipeFlowStore((s) => s.files);
  const instance = useRecipeFlowInstance();
  const browserExec = core.executions.useExecutionState(instance);
  const actions = useRecipeFlowActions();
  const handleDeleteFile = useCallback((i: number) => () => actions.deleteFile(i), [actions]);

  if (step < 2) return null;

  return (
    <StepperContent value={String(step)}>
      <RecipeFileGrid
        files={files}
        activeStep={step}
        browserExec={browserExec}
        onDeleteFile={handleDeleteFile}
        onDownload={actions.downloadResult}
      />
    </StepperContent>
  );
}
