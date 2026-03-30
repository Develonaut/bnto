"use client";

import { useRecipeFlowStore, useRecipeFlowActions } from "../../_stores/recipeFlowContext";
import { RunButton } from "../RunButton";

/** Primary run CTA — reads phase and file count from the context store. */
export function RunRecipeButton() {
  const phase = useRecipeFlowStore((s) => s.resolvedPhase);
  const hasFiles = useRecipeFlowStore((s) => s.files.length > 0);
  const actions = useRecipeFlowActions();

  return <RunButton phase={phase} hasFiles={hasFiles} onRun={actions.run} />;
}
