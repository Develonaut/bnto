"use client";

import { useCallback } from "react";
import { ReactFlow, Background, BackgroundVariant } from "@xyflow/react";
import type { BentoNode } from "../../adapters/types";
import { FIT_VIEW_OPTIONS } from "../../constants";
import { CANVAS_NODE_TYPES } from "./canvasNodeTypes";
import { useCanvasFitView } from "./useCanvasFitView";
import type { CanvasProps } from "./canvasTypes";

const EMPTY_EDGES: never[] = [];
const PRO_OPTIONS = { hideAttribution: true } as const;
const SYNTHETIC_TYPES = new Set(["placeholder", "containerGroup", "addDivider"]);

type CanvasInnerProps = Omit<CanvasProps, "height" | "standalone" | "className">;

/** Build interaction flags from the interactive + disable config. */
function interactionProps(interactive: boolean, disable?: CanvasInnerProps["disable"]) {
  return {
    nodesDraggable: interactive && !disable?.drag,
    nodesConnectable: false,
    elementsSelectable: interactive && !disable?.select,
    panOnDrag: interactive && !disable?.pan,
    zoomOnScroll: interactive,
    zoomOnPinch: interactive,
    zoomOnDoubleClick: false,
    minZoom: interactive ? FIT_VIEW_OPTIONS.minZoom : FIT_VIEW_OPTIONS.maxZoom,
    maxZoom: FIT_VIEW_OPTIONS.maxZoom,
    preventScrolling: interactive,
  } as const;
}

/** Resolve controlled vs uncontrolled node props. */
function resolveNodeProps(props: CanvasInnerProps) {
  return props.onNodesChange
    ? {
        nodes: props.nodes,
        onNodesChange: props.onNodesChange,
        edges: props.edges ?? EMPTY_EDGES,
        onEdgesChange: props.onEdgesChange,
      }
    : { defaultNodes: props.defaultNodes, edges: EMPTY_EDGES };
}

/** Filters clicks on synthetic or interactive child elements. */
function useFilteredNodeClick(onNodeClick: ((id: string) => void) | undefined) {
  return useCallback(
    (_event: React.MouseEvent, node: { id: string; type?: string }) => {
      if (!onNodeClick || SYNTHETIC_TYPES.has(node.type ?? "")) return;
      if ((_event.target as HTMLElement).closest("button, a, input, select, textarea")) return;
      onNodeClick(node.id);
    },
    [onNodeClick],
  );
}

function CanvasInner(props: CanvasInnerProps) {
  const { interactive = false, disable, onNodeClick, onPaneClick, children } = props;
  useCanvasFitView();
  const handleNodeClick = useFilteredNodeClick(onNodeClick);

  return (
    <ReactFlow<BentoNode>
      {...resolveNodeProps(props)}
      nodeTypes={CANVAS_NODE_TYPES}
      onNodeClick={onNodeClick ? handleNodeClick : undefined}
      onPaneClick={onPaneClick}
      {...interactionProps(interactive, disable)}
      proOptions={PRO_OPTIONS}
    >
      <Background variant={BackgroundVariant.Lines} gap={40} color="var(--border)" />
      {children}
    </ReactFlow>
  );
}

export { CanvasInner };
