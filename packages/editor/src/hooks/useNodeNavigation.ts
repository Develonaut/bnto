/**
 * useNodeNavigation — prev/next/delete logic for cycling through nodes.
 *
 * Derives navigation state from the store's nodes, configs, and selection.
 * Returns handlers + disabled flags for toolbar buttons.
 */

"use client";

import { useCallback, useMemo } from "react";
import { useEditorStore } from "./useEditorStore";
import { useEditorActions } from "./useEditorActions";
import { isIoNodeType } from "../helpers/isIoNodeType";

function useNodeNavigation() {
  const { removeSelectedNode, selectedNodeId: selId, selectNode } = useEditorActions();
  const nodes = useEditorStore((s) => s.nodes);
  const configs = useEditorStore((s) => s.configs);

  const isSelectedIo = selId ? isIoNodeType(configs[selId]?.nodeType ?? "") : false;
  const canDelete = !!selId && !isSelectedIo;

  const nodeIds = useMemo(() => nodes.map((n) => n.id), [nodes]);
  const selIndex = selId ? nodeIds.indexOf(selId) : -1;
  const canPrev = selIndex > 0;
  const canNext = selIndex >= 0 && selIndex < nodeIds.length - 1;

  const handlePrev = useCallback(() => {
    if (selIndex > 0) selectNode(nodeIds[selIndex - 1]!);
  }, [selIndex, nodeIds, selectNode]);

  const handleNext = useCallback(() => {
    if (selIndex >= 0 && selIndex < nodeIds.length - 1) selectNode(nodeIds[selIndex + 1]!);
  }, [selIndex, nodeIds, selectNode]);

  return {
    canPrev,
    canNext,
    canDelete,
    handlePrev,
    handleNext,
    removeSelectedNode,
  };
}

export { useNodeNavigation };
