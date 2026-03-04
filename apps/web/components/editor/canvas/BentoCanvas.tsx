"use client";

import { useEffect, useMemo, useRef, type ReactNode } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  BackgroundVariant,
  useReactFlow,
  type OnNodesChange,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { cn } from "@/lib/cn";

import { CompartmentNode, type CompartmentNodeType } from "./CompartmentNode";

/**
 * A React Flow canvas styled as a bento box grid.
 *
 * Compartments (surface cards) sit on a warm grid background — matching
 * the Mini Motorways editing view where buildings appear on the map.
 * Grid lines use BackgroundVariant.Lines (not dots) to match the
 * gameplay screenshot's visible graph-paper grid.
 *
 * Automatically adjusts the viewport with a smooth zoom animation
 * when nodes are added or removed — the "city growing" effect from
 * Mini Motorways where the camera pulls back as neighborhoods expand.
 *
 * Two modes via the `interactive` prop:
 *   false (default) — read-only showcase. No drag, no zoom, no select.
 *   true — editor mode. Nodes draggable, selectable, canvas pannable.
 *     Consumer provides `onNodesChange` for controlled position state.
 *     No edge connections — execution order is positional in bento grid.
 *
 * Children are rendered inside ReactFlow — use for `<Panel>` overlays
 * (e.g., floating toolbar, minimap, controls).
 */

type BentoCanvasProps = {
  nodes: CompartmentNodeType[];
  /** Canvas height in px. Default: 480 */
  height?: number;
  /** Enable drag, select, pan, zoom. Default: false (read-only). */
  interactive?: boolean;
  /** Controlled mode callback — required when interactive. */
  onNodesChange?: OnNodesChange<CompartmentNodeType>;
  /**
   * When true, skips the internal ReactFlowProvider wrapper.
   * Use this when the canvas is embedded inside a parent that already
   * provides a ReactFlowProvider (e.g., RecipeEditor). This lets
   * sibling hooks (useEditorSelection, useEditorUndoRedo) share the
   * same RF context as the canvas.
   */
  standalone?: boolean;
  /** ReactFlow children — Panel overlays, Controls, etc. */
  children?: ReactNode;
  /** Override container classes (e.g., strip border when embedded). */
  className?: string;
};

/* ── Inner canvas — must live inside ReactFlowProvider ──────── */

function BentoCanvasInner({
  nodes,
  interactive = false,
  onNodesChange,
  children,
}: Pick<BentoCanvasProps, "nodes" | "interactive" | "onNodesChange" | "children">) {
  const { fitView } = useReactFlow();
  const nodeTypes = useMemo(() => ({ compartment: CompartmentNode }), []);

  /* Track previous count to distinguish first render from updates.
   * First render = instant fitView (no animation). Subsequent changes
   * = smooth 600ms zoom transition. */
  const prevCount = useRef(0);
  const nodeCount = nodes.length;

  useEffect(() => {
    const prev = prevCount.current;
    prevCount.current = nodeCount;

    /* Empty canvas — nothing to fit. */
    if (nodeCount === 0) return;

    const timer = setTimeout(() => {
      fitView({ duration: prev === 0 ? 0 : 600, padding: 0.2 });
    }, 80);
    return () => clearTimeout(timer);
  }, [nodeCount, fitView]);

  return (
    <ReactFlow<CompartmentNodeType>
      nodes={nodes}
      edges={[]}
      nodeTypes={nodeTypes}
      onNodesChange={interactive ? onNodesChange : undefined}
      nodesDraggable={interactive}
      nodesConnectable={false}
      elementsSelectable={interactive}
      panOnDrag={interactive}
      zoomOnScroll={interactive}
      zoomOnPinch={interactive}
      zoomOnDoubleClick={false}
      preventScrolling={!interactive}
      proOptions={{ hideAttribution: true }}
    >
      {/* Grid lines matching Mini Motorways editing view —
       * warm lines on cream background, like graph paper.
       * Gap=40 gives a subtle density similar to the game. */}
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
  height = 480,
  interactive = false,
  onNodesChange,
  standalone = false,
  children,
  className,
}: BentoCanvasProps) {
  const inner = (
    <BentoCanvasInner
      nodes={nodes}
      interactive={interactive}
      onNodesChange={onNodesChange}
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
      style={{ height }}
    >
      {standalone ? inner : <ReactFlowProvider>{inner}</ReactFlowProvider>}
    </div>
  );
}
