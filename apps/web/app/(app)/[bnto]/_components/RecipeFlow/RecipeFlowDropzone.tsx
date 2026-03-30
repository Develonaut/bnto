"use client";

import { StepperContent } from "@bnto/ui";
import { useRecipeFlowStore, useRecipeFlowDefn } from "../../_stores/recipeFlowContext";
import { RecipeDropzone } from "../RecipeDropzone";

/** Step 1 dropzone — self-gated, only visible when no files are selected. */
export function RecipeFlowDropzone() {
  const step = useRecipeFlowStore((s) => s.activeStep);
  const defn = useRecipeFlowDefn();

  if (step !== 1) return null;

  return (
    <StepperContent value="1">
      <RecipeDropzone acceptLabel={defn.acceptLabel} />
    </StepperContent>
  );
}
