"use client";

import { useEffect, useRef } from "react";
import { useReactFlow, useStore } from "@xyflow/react";
import { FIT_VIEW_OPTIONS } from "../../constants";
import { PLACEHOLDER_ID } from "../../helpers/injectPlaceholder";

/**
 * useCanvasFitView — fits viewport when node count changes.
 *
 * Priority: selected node > all processing nodes > placeholder.
 * Initial mount uses duration: 0 (instant); subsequent changes animate.
 */
function useCanvasFitView() {
  const { fitView, getNodes } = useReactFlow();
  const prevCountRef = useRef<number | null>(null);
  const nodeCount = useStore((s) => s.nodes.length);
  const hasProcessingNodes = useStore((s) => s.nodes.some((n) => n.type === "compartment"));

  useEffect(() => {
    if (nodeCount === 0) return;
    const isInitial = prevCountRef.current === null;
    prevCountRef.current = nodeCount;

    requestAnimationFrame(() => {
      const selected = getNodes().find((n) => n.selected);
      const includeNodes = selected
        ? [{ id: selected.id }]
        : hasProcessingNodes
          ? undefined
          : [{ id: PLACEHOLDER_ID }];
      fitView({ ...FIT_VIEW_OPTIONS, duration: isInitial ? 0 : 300, nodes: includeNodes });
    });
  }, [nodeCount, fitView, hasProcessingNodes, getNodes]);
}

export { useCanvasFitView };
