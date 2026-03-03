"use client";

import { useEffect, useMemo, useRef } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  BackgroundVariant,
  useReactFlow,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

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
 */

type BentoCanvasProps = {
  nodes: CompartmentNodeType[];
  /** Canvas height in px. Default: 480 */
  height?: number;
};

/* ── Inner canvas — must live inside ReactFlowProvider ──────── */

function BentoCanvasInner({ nodes }: { nodes: CompartmentNodeType[] }) {
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
      /* Read-only for showcase — no drag, no connect */
      nodesDraggable={false}
      nodesConnectable={false}
      elementsSelectable={false}
      panOnDrag={false}
      zoomOnScroll={false}
      zoomOnPinch={false}
      zoomOnDoubleClick={false}
      preventScrolling={false}
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
    </ReactFlow>
  );
}

/* ── Public canvas wrapper ──────────────────────────────────── */

export function BentoCanvas({ nodes, height = 480 }: BentoCanvasProps) {
  return (
    <div
      className="w-full overflow-hidden rounded-xl border border-border bg-background"
      style={{ height }}
    >
      <ReactFlowProvider>
        <BentoCanvasInner nodes={nodes} />
      </ReactFlowProvider>
    </div>
  );
}
