"use client";

import { useEffect, useMemo, useRef, type ReactNode } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  Background,
  BackgroundVariant,
  useReactFlow,
  useStore,
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
 * Two rendering modes:
 *
 *   Controlled (nodes prop) — read-only showcase. External state
 *     passed as `nodes` prop, ReactFlow renders it directly.
 *
 *   Uncontrolled (defaultNodes prop) — editor mode. `defaultNodes`
 *     seeds RF's internal store on mount. RF owns node state.
 *     Use `useReactFlow().setNodes()` for programmatic updates.
 *     Drag, select, and pan are handled internally by RF.
 *
 * Children are rendered inside ReactFlow — use for `<Panel>` overlays
 * (e.g., floating toolbar, minimap, controls).
 */

type BentoCanvasProps = {
  /** Initial nodes — RF manages state internally after mount. */
  defaultNodes: CompartmentNodeType[];
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

const EMPTY_EDGES: never[] = [];
const PRO_OPTIONS = { hideAttribution: true } as const;

/* ── Inner canvas — must live inside ReactFlowProvider ──────── */

type BentoCanvasInnerProps = {
  defaultNodes: CompartmentNodeType[];
  interactive?: boolean;
  disable?: BentoCanvasProps["disable"];
  children?: ReactNode;
};

function BentoCanvasInner({
  defaultNodes,
  interactive = false,
  disable,
  children,
}: BentoCanvasInnerProps) {
  const { fitView } = useReactFlow();
  const nodeTypes = useMemo(() => ({ compartment: CompartmentNode }), []);

  /* Fit all nodes on initial mount only — frames the default layout.
   * After mount, the palette and sidebar own fitView targeting. */
  const hasFitted = useRef(false);
  const nodeCount = useStore((s) => s.nodes.length);

  useEffect(() => {
    if (hasFitted.current || nodeCount === 0) return;
    hasFitted.current = true;
    fitView({ duration: 0, padding: 0.2 });
  }, [nodeCount, fitView]);

  return (
    <ReactFlow<CompartmentNodeType>
      defaultNodes={defaultNodes}
      edges={EMPTY_EDGES}
      nodeTypes={nodeTypes}
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
