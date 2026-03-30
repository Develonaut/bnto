"use client";

import type { ReactNode } from "react";
import { FileUpload } from "@bnto/ui";
import {
  useRecipeFlowStore,
  useRecipeFlowActions,
  useRecipeFlowDefn,
} from "../../_stores/recipeFlowContext";

/** FileUpload wrapper — reads files/processing state from the recipe flow store. */
export function RecipeFlowFileUpload({ children }: { children: ReactNode }) {
  const files = useRecipeFlowStore((s) => s.files);
  const isProcessing = useRecipeFlowStore((s) => s.isProcessing);
  const actions = useRecipeFlowActions();
  const defn = useRecipeFlowDefn();

  return (
    <FileUpload
      value={files}
      onValueChange={actions.setFiles}
      accept={defn.dropzoneAccept}
      multiple
      disabled={isProcessing}
    >
      {children}
    </FileUpload>
  );
}
