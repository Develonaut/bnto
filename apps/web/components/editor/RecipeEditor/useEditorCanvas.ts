"use client";

import { useState, useCallback, useEffect, useMemo, useRef } from "react";
import { useReactFlow, useStoreApi } from "@xyflow/react";
import { usePrevious } from "@/components/ui/usePrevious";
import { useEditorStore, useEditorActions, definitionToBento } from "@/editor";
import { useDefinitionSync } from "@/editor/hooks/useDefinitionSync";
import { useEditorSelection } from "@/editor/hooks/useEditorSelection";
import type { CompartmentNodeType } from "../canvas/CompartmentNode";

/**
 * useEditorCanvas — all editor orchestration logic.
 *
 * Owns selection, sync, position registration, recipe change,
 * sidebar toggle, and default nodes computation. The component
 * that consumes this hook is a pure render shell.
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
  const { setPositionGetter, loadRecipe, createBlank } = useEditorActions();
  const definition = useEditorStore((s) => s.definition);
  const recipeName = useEditorStore((s) => s.definition.name ?? "Untitled");
  const { setNodes } = useReactFlow<CompartmentNodeType>();
  const storeApi = useStoreApi<CompartmentNodeType>();

  /* Register position getter so the store can read RF positions for undo. */
  useEffect(() => {
    setPositionGetter(() => {
      const positions: Record<string, { x: number; y: number }> = {};
      for (const node of storeApi.getState().nodes) {
        positions[node.id] = node.position;
      }
      return positions;
    });
  }, [setPositionGetter, storeApi]);

  /* Seed initial nodes via defaultNodes (uncontrolled mode).
   * RF reads this once on mount and owns the state from there.
   * eslint-disable: definition captured at mount time intentionally. */
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const defaultNodes = useMemo(
    () => definitionToBento(definition).nodes as CompartmentNodeType[],
    [],
  );

  /* Incremental sync: definition changes -> RF node additions/removals. */
  useDefinitionSync();

  /* Full replacement sync: when the definition reference changes entirely
   * (loadRecipe, undo/redo, param updates), re-convert to bento nodes.
   * Preserves RF selection state so param edits in the config panel
   * don't deselect the active node and close the panel. */
  const prevDefRef = useRef(definition);
  useEffect(() => {
    if (prevDefRef.current === definition) return;
    prevDefRef.current = definition;
    const layout = definitionToBento(definition);
    const freshNodes = layout.nodes as CompartmentNodeType[];
    setNodes((prev) => {
      const selectedIds = new Set(
        prev.filter((n) => n.selected).map((n) => n.id),
      );
      return freshNodes.map((n) => ({
        ...n,
        selected: selectedIds.has(n.id),
      }));
    });
  }, [definition, setNodes]);

  const handleRecipeChange = useCallback(
    (slug: string) => {
      setActiveSlug(slug);
      if (slug === "blank") {
        createBlank();
      } else {
        loadRecipe(slug);
      }
    },
    [loadRecipe, createBlank],
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
