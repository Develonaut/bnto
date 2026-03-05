/**
 * Hook for exporting the current editor state as a Recipe or .bnto.json file.
 *
 * Export is event-driven (user clicks Export), not reactive rendering.
 * State is read imperatively at call time via storeApi.getState() —
 * no reactive subscriptions to nodes/configs/metadata. Only
 * validationErrors is subscribed (drives the rendered canExport flag).
 */

"use client";

import { useCallback } from "react";
import { definitionToRecipe, validateDefinition } from "@bnto/nodes";
import type { RecipeMetadata as NodeRecipeMetadata, Recipe } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";
import { useEditorStoreApi } from "./useEditorStoreApi";
import { rfNodesToDefinition } from "../adapters/rfNodesToDefinition";
import { downloadBlob } from "@bnto/core";

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

function useEditorExport() {
  const validationErrors = useEditorStore((s) => s.validationErrors);
  const storeApi = useEditorStoreApi();

  const canExport = validationErrors.length === 0;

  const exportAsRecipe = useCallback(
    (metadata?: NodeRecipeMetadata): ExportResult => {
      const { nodes, configs, recipeMetadata } = storeApi.getState();
      const definition = rfNodesToDefinition(nodes, recipeMetadata, configs);

      const errors = validateDefinition(definition);
      if (errors.length > 0) {
        return { recipe: null, errors };
      }
      const recipe = definitionToRecipe(definition, metadata);
      return { recipe, errors: [] };
    },
    [storeApi],
  );

  const download = useCallback(
    (metadata?: NodeRecipeMetadata) => {
      const result = exportAsRecipe(metadata);
      if (!result.recipe) return;

      const json = JSON.stringify(result.recipe.definition, null, 2);
      downloadBlob(new Blob([json], { type: "application/json" }), `${result.recipe.slug}.bnto.json`);
    },
    [exportAsRecipe],
  );

  return { exportAsRecipe, download, canExport };
}

export { useEditorExport };
export type { ExportResult, EditorExportResult };
