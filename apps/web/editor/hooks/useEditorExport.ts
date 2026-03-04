/**
 * Hook for exporting the current editor state as a Recipe or .bnto.json file.
 *
 * Reads RF nodes directly and reconstructs a Definition via
 * rfNodesToDefinition. No patching needed — RF is the source of truth.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { definitionToRecipe, validateDefinition } from "@bnto/nodes";
import type { RecipeMetadata as NodeRecipeMetadata, Recipe } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import type { BentoNode } from "../adapters/types";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface ExportResult {
  recipe: Recipe | null;
  errors: ReturnType<typeof validateDefinition>;
}

interface EditorExportResult {
  exportAsRecipe: (metadata?: NodeRecipeMetadata) => ExportResult;
  download: (metadata?: NodeRecipeMetadata) => void;
  canExport: boolean;
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

function useEditorExport(): EditorExportResult {
  const validationErrors = useEditorStore((s) => s.validationErrors);
  const recipeMetadata = useEditorStore((s) => s.recipeMetadata);
  const { getNodes } = useReactFlow<BentoNode>();

  const canExport = validationErrors.length === 0;

  const exportAsRecipe = useCallback(
    (metadata?: NodeRecipeMetadata): ExportResult => {
      const rfNodes = getNodes();
      const rootDefinition = {
        ...recipeMetadata,
        position: { x: 0, y: 0 },
        metadata: {},
        parameters: {},
        inputPorts: [] as never[],
        outputPorts: [] as never[],
      };
      const definition = rfNodesToDefinition(rfNodes, rootDefinition);

      const errors = validateDefinition(definition);
      if (errors.length > 0) {
        return { recipe: null, errors };
      }
      const recipe = definitionToRecipe(definition, metadata);
      return { recipe, errors: [] };
    },
    [recipeMetadata, getNodes],
  );

  const download = useCallback(
    (metadata?: NodeRecipeMetadata) => {
      const result = exportAsRecipe(metadata);
      if (!result.recipe) return;

      const json = JSON.stringify(result.recipe.definition, null, 2);
      const blob = new Blob([json], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `${result.recipe.slug}.bnto.json`;
      a.click();
      URL.revokeObjectURL(url);
    },
    [exportAsRecipe],
  );

  return { exportAsRecipe, download, canExport };
}

export { useEditorExport };
export type { ExportResult, EditorExportResult };
