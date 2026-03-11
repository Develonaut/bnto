"use client";

import { useCallback, useEffect, useRef, type ReactNode } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  BackgroundVariant,
  useReactFlow,
  useStore,
} from "@xyflow/react";
import type { NodeChange, EdgeChange, Edge } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { cn } from "@bnto/ui";

import { CompartmentNode, IoNode, PlaceholderNode, ContainerGroupNode, AddDividerNode } from "../nodes";
import type { BentoNode } from "../../adapters/types";
import { FIT_VIEW_OPTIONS } from "../../constants";
import { PLACEHOLDER_ID } from "../../helpers/injectPlaceholder";

/**
 * A React Flow canvas styled as a bento box grid.
 *
 * Two rendering modes:
 *
 *   Controlled (nodes + onNodesChange) — editor mode. External store
 *     owns node state, passes it as props. RF renders but doesn't own.
 *
 *   Uncontrolled (defaultNodes) — read-only showcase. `defaultNodes`
 *     seeds RF's internal store on mount. RF owns node state.
 *
 * Children are rendered inside ReactFlow — use for `<Panel>` overlays.
 */

type CanvasProps = {
  /** Controlled mode: nodes from external store. */
  nodes?: BentoNode[];
  /** Controlled mode: RF change handler. */
  onNodesChange?: (changes: NodeChange<BentoNode>[]) => void;
  /** Controlled mode: edges from external store. */
  edges?: Edge[];
  /** Controlled mode: edge change handler. */
  onEdgesChange?: (changes: EdgeChange[]) => void;
  /** Uncontrolled mode: initial nodes (RF owns state after mount). */
  defaultNodes?: BentoNode[];
  /** Canvas height in px. Default: 480 */
  height?: number;
  /** Enable drag, select, pan, zoom. Default: false (read-only). */
  interactive?: boolean;
  /** Disable specific behaviors (overrides interactive). */
  disable?: {
    drag?: boolean;
    pan?: boolean;
    select?: boolean;
  };
  /**
   * When true, skips the internal ReactFlowProvider wrapper.
   * Use this when the canvas is embedded inside a parent that already
   * provides a ReactFlowProvider.
   */
  standalone?: boolean;
  /** Called when a node is clicked. Receives the node ID. */
  onNodeClick?: (nodeId: string) => void;
  /** Called when the empty canvas background is clicked (not a node). */
  onPaneClick?: () => void;
  /** ReactFlow children — Panel overlays, Controls, etc. */
  children?: ReactNode;
  /** Override container classes. */
  className?: string;
};

const EMPTY_EDGES: never[] = [];
const PRO_OPTIONS = { hideAttribution: true } as const;

/** Module-level constant — stable reference prevents RF from
 * unmounting/remounting all nodes on every render. */
const NODE_TYPES = {
  compartment: CompartmentNode,
  io: IoNode,
  placeholder: PlaceholderNode,
  containerGroup: ContainerGroupNode,
  addDivider: AddDividerNode,
} as const;

/* ── Inner canvas — must live inside ReactFlowProvider ──────── */

type CanvasInnerProps = {
  nodes?: BentoNode[];
  onNodesChange?: (changes: NodeChange<BentoNode>[]) => void;
  edges?: Edge[];
  onEdgesChange?: (changes: EdgeChange[]) => void;
  defaultNodes?: BentoNode[];
  interactive?: boolean;
  disable?: CanvasProps["disable"];
  onNodeClick?: (nodeId: string) => void;
  onPaneClick?: () => void;
  children?: ReactNode;
};

function CanvasInner({
  nodes,
  onNodesChange,
  edges,
  onEdgesChange,
  defaultNodes,
  interactive = false,
  disable,
  onNodeClick,
  onPaneClick,
  children,
}: CanvasInnerProps) {
  const { fitView, getNodes } = useReactFlow();

  /* Fit viewport whenever the node count changes (add/remove).
   * Initial mount uses duration: 0 (instant); subsequent changes
   * animate smoothly so the user sees the viewport adjust.
   *
   * Priority: selected node > all processing nodes > placeholder.
   * Reading selection inside rAF ensures we get the latest state
   * after React has flushed (e.g. auto-select on addNode). */
  const prevCountRef = useRef<number | null>(null);
  const nodeCount = useStore((s) => s.nodes.length);
  const hasProcessingNodes = useStore((s) => s.nodes.some((n) => n.type === "compartment"));

  useEffect(() => {
    if (nodeCount === 0) return;
    const isInitial = prevCountRef.current === null;
    prevCountRef.current = nodeCount;
    // Defer fitView so RF has time to measure node DOM dimensions.
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

  // Controlled mode: nodes + onNodesChange props
  // Uncontrolled mode: defaultNodes prop
  const isControlled = !!onNodesChange;

  /** Skip clicks on placeholder nodes and interactive child elements (buttons). */
  const handleNodeClick = useCallback(
    (_event: React.MouseEvent, node: { id: string; type?: string }) => {
      if (!onNodeClick) return;
      if (node.type === "placeholder" || node.type === "containerGroup" || node.type === "addDivider") return;
      const target = _event.target as HTMLElement;
      if (target.closest("button, a, input, select, textarea")) return;
      onNodeClick(node.id);
    },
    [onNodeClick],
  );

  return (
    <ReactFlow<BentoNode>
      {...(isControlled
        ? { nodes, onNodesChange, edges: edges ?? EMPTY_EDGES, onEdgesChange }
        : { defaultNodes, edges: EMPTY_EDGES })}
      nodeTypes={NODE_TYPES}
      onNodeClick={onNodeClick ? handleNodeClick : undefined}
      onPaneClick={onPaneClick}
      nodesDraggable={interactive && !disable?.drag}
      nodesConnectable={false}
      elementsSelectable={interactive && !disable?.select}
      panOnDrag={interactive && !disable?.pan}
      zoomOnScroll={false}
      zoomOnPinch={false}
      zoomOnDoubleClick={false}
      minZoom={FIT_VIEW_OPTIONS.maxZoom}
      maxZoom={FIT_VIEW_OPTIONS.maxZoom}
      preventScrolling={interactive}
      proOptions={PRO_OPTIONS}
    >
      <Background variant={BackgroundVariant.Lines} gap={40} color="var(--border)" />
      {children}
    </ReactFlow>
  );
}

/* ── Public canvas wrapper ──────────────────────────────────── */

export function Canvas({
  nodes,
  onNodesChange,
  edges,
  onEdgesChange,
  defaultNodes,
  height,
  interactive = false,
  disable,
  standalone = false,
  onNodeClick,
  onPaneClick,
  children,
  className,
}: CanvasProps) {
  const inner = (
    <CanvasInner
      nodes={nodes}
      onNodesChange={onNodesChange}
      edges={edges}
      onEdgesChange={onEdgesChange}
      defaultNodes={defaultNodes}
      interactive={interactive}
      disable={disable}
      onNodeClick={onNodeClick}
      onPaneClick={onPaneClick}
    >
      {children}
    </CanvasInner>
  );

  return (
    <div
      className={cn(
        "w-full overflow-hidden rounded-xl border border-border bg-background",
        className,
      )}
      style={height ? { height } : undefined}
    >
      {standalone ? inner : <ReactFlowProvider>{inner}</ReactFlowProvider>}
    </div>
  );
}
