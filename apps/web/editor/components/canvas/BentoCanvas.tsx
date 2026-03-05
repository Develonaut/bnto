"use client";

import { useEffect, useRef, type ReactNode } from "react";
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

import { CompartmentNode } from "./CompartmentNode";
import type { BentoNode } from "@/editor/adapters/types";

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

type BentoCanvasProps = {
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
    zoom?: boolean;
    select?: boolean;
  };
  /**
   * When true, skips the internal ReactFlowProvider wrapper.
   * Use this when the canvas is embedded inside a parent that already
   * provides a ReactFlowProvider.
   */
  standalone?: boolean;
  /** ReactFlow children — Panel overlays, Controls, etc. */
  children?: ReactNode;
  /** Override container classes. */
  className?: string;
};

const EMPTY_EDGES: never[] = [];
const PRO_OPTIONS = { hideAttribution: true } as const;

/** Module-level constant — stable reference prevents RF from
 * unmounting/remounting all nodes on every render. */
const NODE_TYPES = { compartment: CompartmentNode } as const;

/* ── Inner canvas — must live inside ReactFlowProvider ──────── */

type BentoCanvasInnerProps = {
  nodes?: BentoNode[];
  onNodesChange?: (changes: NodeChange<BentoNode>[]) => void;
  edges?: Edge[];
  onEdgesChange?: (changes: EdgeChange[]) => void;
  defaultNodes?: BentoNode[];
  interactive?: boolean;
  disable?: BentoCanvasProps["disable"];
  children?: ReactNode;
};

function BentoCanvasInner({
  nodes,
  onNodesChange,
  edges,
  onEdgesChange,
  defaultNodes,
  interactive = false,
  disable,
  children,
}: BentoCanvasInnerProps) {
  const { fitView } = useReactFlow();

  /* Fit all nodes on initial mount only. */
  const hasFitted = useRef(false);
  const nodeCount = useStore((s) => s.nodes.length);

  useEffect(() => {
    if (hasFitted.current || nodeCount === 0) return;
    hasFitted.current = true;
    // Defer fitView to the next animation frame so RF has time to
    // measure node DOM dimensions before calculating viewport bounds.
    requestAnimationFrame(() => {
      fitView({ duration: 0, padding: 0.2 });
    });
  }, [nodeCount, fitView]);

  // Controlled mode: nodes + onNodesChange props
  // Uncontrolled mode: defaultNodes prop
  const isControlled = !!onNodesChange;

  return (
    <ReactFlow<BentoNode>
      {...(isControlled
        ? { nodes, onNodesChange, edges: edges ?? EMPTY_EDGES, onEdgesChange }
        : { defaultNodes, edges: EMPTY_EDGES })}
      nodeTypes={NODE_TYPES}
      nodesDraggable={interactive && !disable?.drag}
      nodesConnectable={false}
      elementsSelectable={interactive && !disable?.select}
      panOnDrag={interactive && !disable?.pan}
      zoomOnScroll={interactive && !disable?.zoom}
      zoomOnPinch={interactive && !disable?.zoom}
      zoomOnDoubleClick={false}
      preventScrolling={!interactive}
      proOptions={PRO_OPTIONS}
    >
      <Background
        variant={BackgroundVariant.Lines}
        gap={40}
        color="var(--border)"
      />
      {children}
    </ReactFlow>
  );
}

/* ── Public canvas wrapper ──────────────────────────────────── */

export function BentoCanvas({
  nodes,
  onNodesChange,
  edges,
  onEdgesChange,
  defaultNodes,
  height,
  interactive = false,
  disable,
  standalone = false,
  children,
  className,
}: BentoCanvasProps) {
  const inner = (
    <BentoCanvasInner
      nodes={nodes}
      onNodesChange={onNodesChange}
      edges={edges}
      onEdgesChange={onEdgesChange}
      defaultNodes={defaultNodes}
      interactive={interactive}
      disable={disable}
    >
      {children}
    </BentoCanvasInner>
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
