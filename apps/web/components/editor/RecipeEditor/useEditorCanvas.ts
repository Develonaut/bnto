"use client";

import { useState, useCallback } from "react";
import { usePrevious } from "@/components/ui/usePrevious";
import { useEditorStore } from "@/editor";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import { useEditorStoreApi } from "@/editor/hooks/useEditorStoreApi";

/**
 * useEditorCanvas — all editor orchestration logic.
 *
 * The store owns all state (controlled mode). This hook owns:
 * - Recipe loading (delegates to store.loadRecipe)
 * - Selection, sidebar toggle
 * - Passes store's nodes/onNodesChange to BentoCanvas
 */

interface UseEditorCanvasOptions {
  initialSlug?: string;
}

function useEditorCanvas({ initialSlug }: UseEditorCanvasOptions = {}) {
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);
  const [activeSlug, setActiveSlug] = useState<string>(initialSlug ?? "blank");

  const { selectedNodeId } = useEditorSelection();
  const prevSelectedNodeId = usePrevious(selectedNodeId);
  const configNodeId = selectedNodeId ?? prevSelectedNodeId ?? null;

  const storeApi = useEditorStoreApi();
  const recipeName = useEditorStore((s) => s.recipeMetadata.name ?? "Untitled");
  const nodes = useEditorStore((s) => s.nodes);
  const edges = useEditorStore((s) => s.edges);
  const onNodesChange = useEditorStore((s) => s.onNodesChange);
  const onEdgesChange = useEditorStore((s) => s.onEdgesChange);

  /* Seed initial recipe into store on first render. useState
   * initializer runs exactly once — safe for Strict Mode. */
  useState(() => {
    if (initialSlug && initialSlug !== "blank") {
      storeApi.getState().loadRecipe(initialSlug);
    }
  });

  /* Recipe change handler — delegates to store. */
  const handleRecipeChange = useCallback(
    (slug: string) => {
      setActiveSlug(slug);
      const { loadRecipe, createBlank } = storeApi.getState();
      if (slug === "blank") {
        createBlank();
      } else {
        loadRecipe(slug);
      }
    },
    [storeApi],
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
    nodes,
    edges,
    onNodesChange,
    onEdgesChange,
  };
}

export { useEditorCanvas };
