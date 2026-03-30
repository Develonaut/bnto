"use client";

import {
  RecipeFlowHeader,
  RecipeFlowBackButton,
  RecipeFlowStatus,
  RecipeFlowActions,
  RunRecipeButton,
  ToolbarDownloadButton,
} from "./RecipeFlow";

export function BntoToolbar() {
  return (
    <RecipeFlowHeader>
      <RecipeFlowActions className="shrink-0">
        <RecipeFlowBackButton />
      </RecipeFlowActions>
      <RecipeFlowStatus />
      <RecipeFlowActions className="ml-auto shrink-0">
        <ToolbarDownloadButton />
        <RunRecipeButton />
      </RecipeFlowActions>
    </RecipeFlowHeader>
  );
}
