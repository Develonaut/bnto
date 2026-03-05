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
  const storeApi = useEditorStoreApi();

  const canExport = validationErrors.length === 0;

  const exportAsRecipe = useCallback(
    (metadata?: NodeRecipeMetadata): ExportResult => {
      const { nodes, configs, recipeMetadata } = storeApi.getState();
      const rootDefinition = {
        ...recipeMetadata,
        position: { x: 0, y: 0 },
        metadata: {},
        parameters: {},
        inputPorts: [] as never[],
        outputPorts: [] as never[],
      };
      const definition = rfNodesToDefinition(nodes, rootDefinition, configs);

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
