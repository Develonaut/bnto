/**
 * Hook for exporting the current editor state as a Recipe or .bnto.json file.
 *
 * Before export, patches current ReactFlow positions into the definition
 * so the exported recipe reflects the visual layout.
 */

"use client";

import { useCallback } from "react";
import { useReactFlow } from "@xyflow/react";
import { definitionToRecipe, validateDefinition } from "@bnto/nodes";
import type { RecipeMetadata, Recipe, Definition } from "@bnto/nodes";
import { useEditorStore } from "./useEditorStore";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

interface ExportResult {
  recipe: Recipe | null;
  errors: ReturnType<typeof validateDefinition>;
}

interface EditorExportResult {
  exportAsRecipe: (metadata?: RecipeMetadata) => ExportResult;
  download: (metadata?: RecipeMetadata) => void;
  canExport: boolean;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Patch RF node positions into a Definition for export. */
function patchPositions(
  definition: Definition,
  positionMap: Map<string, { x: number; y: number }>,
): Definition {
  return {
    ...definition,
    nodes: (definition.nodes ?? []).map((child) => {
      const pos = positionMap.get(child.id);
      return pos ? { ...child, position: pos } : child;
    }),
  };
}

// ---------------------------------------------------------------------------
// Hook
// ---------------------------------------------------------------------------

function useEditorExport(): EditorExportResult {
  const definition = useEditorStore((s) => s.definition);
  const validationErrors = useEditorStore((s) => s.validationErrors);
  const { getNodes } = useReactFlow();

  const canExport = validationErrors.length === 0;

  const exportAsRecipe = useCallback(
    (metadata?: RecipeMetadata): ExportResult => {
      // Build position map from RF nodes — each node's data.nodeId links
      // back to the Definition node ID
      const rfNodes = getNodes();
      const positionMap = new Map<string, { x: number; y: number }>();
      for (const n of rfNodes) {
        const nodeId = (n.data as Record<string, unknown>)?.nodeId;
        if (typeof nodeId === "string") {
          positionMap.set(nodeId, n.position);
        }
      }
      const patched = patchPositions(definition, positionMap);

      const errors = validateDefinition(patched);
      if (errors.length > 0) {
        return { recipe: null, errors };
      }
      const recipe = definitionToRecipe(patched, metadata);
      return { recipe, errors: [] };
    },
    [definition, getNodes],
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
