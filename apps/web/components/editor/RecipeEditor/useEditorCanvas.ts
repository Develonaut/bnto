"use client";

import { useState, useCallback, useEffect, useMemo } from "react";
import { useReactFlow, useStoreApi } from "@xyflow/react";
import { getRecipeBySlug, createBlankDefinition } from "@bnto/nodes";
import { usePrevious } from "@/components/ui/usePrevious";
import { useEditorStore, definitionToBento } from "@/editor";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import type { BentoNode } from "@/editor/adapters/types";
import { useEditorStoreApi } from "@/editor/hooks/useEditorStoreApi";

/**
 * useEditorCanvas — all editor orchestration logic.
 *
 * RF is the single source of truth for node state. This hook owns:
 * - Recipe loading (definition → RF nodes directly)
 * - Node getter registration (store reads RF for undo snapshots)
 * - Selection, sidebar toggle, default nodes
 *
 * No sync effects — mutations go straight to RF via useAddNode etc.
 */

interface UseEditorCanvasOptions {
  initialSlug?: string;
}

function useEditorCanvas({ initialSlug }: UseEditorCanvasOptions = {}) {
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [activeSlug, setActiveSlug] = useState<string>(initialSlug ?? "blank");

  const { selectedNodeId } = useEditorSelection();
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  /* Keep showing the last node's config while the panel slides out. */
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const storeApi = useEditorStoreApi();
  const recipeName = useEditorStore((s) => s.recipeMetadata.name ?? "Untitled");
  const { setNodes } = useReactFlow<BentoNode>();
  const rfStoreApi = useStoreApi<BentoNode>();

  /* Register node getter so the store can read RF nodes for undo snapshots. */
  useEffect(() => {
    storeApi.getState().setNodeGetter(() =>
      rfStoreApi.getState().nodes as BentoNode[],
    );
  }, [storeApi, rfStoreApi]);

  /* Seed initial nodes via defaultNodes (uncontrolled mode).
   * RF reads this once on mount and owns the state from there. */
  const defaultNodes = useMemo(() => {
    if (initialSlug && initialSlug !== "blank") {
      const recipe = getRecipeBySlug(initialSlug);
      if (recipe) return definitionToBento(recipe.definition).nodes as BentoNode[];
    }
    return definitionToBento(createBlankDefinition()).nodes as BentoNode[];
    // eslint-disable-next-line react-hooks/exhaustive-deps -- captured at mount time intentionally
  }, []);

  /* Recipe change handler — converts definition to RF nodes directly. */
  const handleRecipeChange = useCallback(
    (slug: string) => {
      setActiveSlug(slug);
      const { loadRecipe, createBlank } = storeApi.getState();

      if (slug === "blank") {
        createBlank();
        const blank = createBlankDefinition();
        setNodes(definitionToBento(blank).nodes as BentoNode[]);
      } else {
        loadRecipe(slug);
        const recipe = getRecipeBySlug(slug);
        if (recipe) {
          setNodes(definitionToBento(recipe.definition).nodes as BentoNode[]);
        }
      }
    },
    [storeApi, setNodes],
  );

  const handleReset = useCallback(() => {
    handleRecipeChange(activeSlug);
  }, [handleRecipeChange, activeSlug]);

  const toggleSidebar = useCallback(() => {
    setSidebarCollapsed((prev) => !prev);
  }, []);

  return {
    sidebarCollapsed,
    toggleSidebar,
    selectedNodeId,
    configNodeId,
    recipeName,
    activeSlug,
    handleRecipeChange,
    handleReset,
    defaultNodes,
  };
}

export { useEditorCanvas };
