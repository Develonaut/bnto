/**
 * Hook for exporting the current editor state as a Recipe or .bnto.json file.
 *
 * Validates the definition before export. Invalid definitions cannot
 * be exported — the hook returns validation errors to show the user.
 */

"use client";

import { useCallback } from "react";
import { definitionToRecipe, validateDefinition } from "@bnto/nodes";
import type { RecipeMetadata } from "@bnto/nodes";
import type { Recipe } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface ExportResult {
  /** The exported recipe, or null if validation failed. */
  recipe: Recipe | null;
  /** Validation errors that prevent export. Empty = valid. */
  errors: ReturnType<typeof validateDefinition>;
}

interface EditorExportResult {
  /** Export current definition as a Recipe object. */
  exportAsRecipe: (metadata?: RecipeMetadata) => ExportResult;
  /** Download current definition as a .bnto.json file. */
  download: (metadata?: RecipeMetadata) => void;
  /** Whether the current definition is valid for export. */
  canExport: boolean;
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

function useEditorExport(): EditorExportResult {
  const definition = useEditorStore((s) => s.definition);
  const validationErrors = useEditorStore((s) => s.validationErrors);

  const canExport = validationErrors.length === 0;

  const exportAsRecipe = useCallback(
    (metadata?: RecipeMetadata): ExportResult => {
      const errors = validateDefinition(definition);
      if (errors.length > 0) {
        return { recipe: null, errors };
      }
      const recipe = definitionToRecipe(definition, metadata);
      return { recipe, errors: [] };
    },
    [definition],
  );

  const download = useCallback(
    (metadata?: RecipeMetadata) => {
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
